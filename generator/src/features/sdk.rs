//! Reads `FSL_FEATURE_*` macros from MCUXpresso SDK device headers.
//!
//! The headers are flat lists of `#define`s (no `#if` apart from the include guard), so no
//! preprocessor is needed. Shapes handled:
//! - `#define FSL_FEATURE_X (1)`, also hex and `U`/`UL` suffixes;
//! - constant per-instance macros, `#define FSL_FEATURE_X_SIZEn(x) (4)`;
//! - per-instance ternaries, `(((x) == LPUART0) ? (1) : (((x) == LPUART1) ? (0) : (-1)))`, where a
//!   final `-1` means "not this instance".
//!
//! Floating-point values are skipped.

use std::collections::BTreeMap;

use regex::Regex;

use super::Value;

/// Macros of `text` whose name starts with one of `prefixes`, by name. A per-instance macro keeps
/// its SDK name (`FSL_FEATURE_LPUART_FIFO_SIZEn`).
pub fn parse_features(text: &str, prefixes: &[String]) -> anyhow::Result<BTreeMap<String, Value>> {
    let mut out = BTreeMap::new();
    for (name, param, body) in defines(text) {
        if !prefixes.iter().any(|p| name.starts_with(p.as_str())) {
            continue;
        }
        let value = match param {
            None => parse_int(&body).map(Value::Int),
            Some(param) => parse_per_instance(&body, &param)?,
        };
        match value {
            Some(v) => {
                out.insert(name, v);
            }
            None => tracing::debug!("skipping {name}: unsupported value `{body}`"),
        }
    }
    Ok(out)
}

/// `(name, macro parameter, body)` of every `#define`, with comments removed and continuation
/// lines joined.
fn defines(text: &str) -> impl Iterator<Item = (String, Option<String>, String)> {
    let block_comment = Regex::new(r"(?s)/\*.*?\*/").unwrap();
    let line_comment = Regex::new(r"//[^\n]*").unwrap();
    let define = Regex::new(r"^\s*#\s*define\s+(\w+)(?:\((\w+)\))?\s+(.+?)\s*$").unwrap();

    let text = text.replace("\r\n", "\n").replace("\\\n", " ");
    let text = block_comment.replace_all(&text, " ");
    let text = line_comment.replace_all(&text, "");
    text.lines()
        .filter_map(|line| {
            let c = define.captures(line)?;
            Some((
                c[1].to_string(),
                c.get(2).map(|m| m.as_str().to_string()),
                c[3].to_string(),
            ))
        })
        .collect::<Vec<_>>()
        .into_iter()
}

/// An integer literal, possibly parenthesized and negative.
fn parse_int(body: &str) -> Option<i64> {
    let mut s = body.trim();
    while let Some(inner) = s.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
        s = inner.trim();
    }
    let (neg, s) = match s.strip_prefix('-') {
        Some(rest) => (true, rest.trim()),
        None => (false, s),
    };
    let s = s.trim_end_matches(['u', 'U', 'l', 'L']);
    let v = match s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        Some(hex) => i64::from_str_radix(hex, 16).ok()?,
        None if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) => s.parse().ok()?,
        None => return None,
    };
    Some(if neg { -v } else { v })
}

/// The body of a per-instance macro: a constant, or a ternary chain over instance names.
fn parse_per_instance(body: &str, param: &str) -> anyhow::Result<Option<Value>> {
    if let Some(v) = parse_int(body) {
        return Ok(Some(Value::Int(v)));
    }
    let p = regex::escape(param);
    let arm = Regex::new(&format!(
        r"\(\s*\(\s*{p}\s*\)\s*==\s*(\w+)\s*\)\s*\?\s*\(?\s*(-?\w+)\s*\)?\s*:"
    ))?;
    let mut map = BTreeMap::new();
    let mut end = 0;
    for c in arm.captures_iter(body) {
        let Some(v) = parse_int(&c[2]) else {
            return Ok(None);
        };
        map.insert(c[1].to_string(), v);
        end = c.get(0).unwrap().end();
    }
    if map.is_empty() {
        return Ok(None);
    }
    // The final `else`: -1 means "not this instance"; anything else applies to all others.
    let default = parse_int(
        body[end..]
            .trim_end_matches(|c: char| c == ')' || c.is_whitespace())
            .trim_start_matches(|c: char| c == '(' || c.is_whitespace()),
    );
    match default {
        Some(-1) => {}
        Some(v) => {
            map.insert("*".to_string(), v);
        }
        None => anyhow::bail!("unsupported per-instance macro body `{body}`"),
    }
    Ok(Some(Value::PerInstance(map)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = r#"
/* ----------------------------------------------------------------------------
   -- Device specific feature definitions
   ---------------------------------------------------------------------------- */
#ifndef _X_FEATURES_H_
#define _X_FEATURES_H_
/* @brief Has CTRL SWAP. */
#define FSL_FEATURE_LPUART_HAS_CTRL_SWAP (1)
#define FSL_FEATURE_LPUART_HAS_NO_THING (0) // trailing comment
#define FSL_FEATURE_LPUART_FIFO_SIZEn(x) (4)
#define FSL_FEATURE_LPUART_BASE_HEX (0x01000000u)
#define FSL_FEATURE_LPUART_FREQ (738.0f)
#define FSL_FEATURE_LPUART_INSTANCE_HAS_Xn(x) \
    (((x) == LPUART0) ? (1) : \
    (((x) == LPUART1) ? (1) : \
    (((x) == LPUART2) ? (0) : (-1))))
#define FSL_FEATURE_LPSPI_HAS_CCR1 (1)
#endif
"#;

    #[test]
    fn parses_every_shape() {
        let f = parse_features(HEADER, &["FSL_FEATURE_LPUART_".to_string()]).unwrap();
        assert_eq!(f["FSL_FEATURE_LPUART_HAS_CTRL_SWAP"], Value::Int(1));
        assert_eq!(f["FSL_FEATURE_LPUART_HAS_NO_THING"], Value::Int(0));
        assert_eq!(f["FSL_FEATURE_LPUART_FIFO_SIZEn"], Value::Int(4));
        assert_eq!(f["FSL_FEATURE_LPUART_BASE_HEX"], Value::Int(0x0100_0000));
        assert!(!f.contains_key("FSL_FEATURE_LPUART_FREQ"));
        assert!(!f.contains_key("FSL_FEATURE_LPSPI_HAS_CCR1"));
        let Value::PerInstance(m) = &f["FSL_FEATURE_LPUART_INSTANCE_HAS_Xn"] else {
            panic!("not per-instance");
        };
        assert_eq!(
            m.iter().map(|(k, v)| (k.as_str(), *v)).collect::<Vec<_>>(),
            [("LPUART0", 1), ("LPUART1", 1), ("LPUART2", 0)]
        );
    }

    #[test]
    fn ternary_default_other_than_minus_one_applies_to_all() {
        let f = parse_features(
            "#define FSL_FEATURE_I3C_Xn(x) (((x) == I3C0) ? (2) : (1))\n",
            &["FSL_FEATURE_I3C_".to_string()],
        )
        .unwrap();
        let Value::PerInstance(m) = &f["FSL_FEATURE_I3C_Xn"] else {
            panic!("not per-instance");
        };
        assert_eq!(m["I3C0"], 2);
        assert_eq!(m["*"], 1);
    }
}
