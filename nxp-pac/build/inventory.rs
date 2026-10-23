use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn write(inputs: &Path, out: &Path) -> io::Result<Vec<PathBuf>> {
    fn collect(
        root: &Path,
        directory: &Path,
        names: &mut Vec<String>,
        directories: &mut Vec<PathBuf>,
    ) -> io::Result<()> {
        directories.push(directory.to_owned());
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                collect(root, &path, names, directories)?;
            } else if path.extension().is_some_and(|ext| ext == "yaml") {
                let relative = path
                    .strip_prefix(root)
                    .map_err(io::Error::other)?
                    .with_extension("");
                names.push(relative.to_string_lossy().replace('\\', "/"));
            }
        }
        Ok(())
    }
    let root = inputs.join("peripherals");
    let mut names = Vec::new();
    let mut directories = Vec::new();
    collect(&root, &root, &mut names, &mut directories)?;
    names.sort();
    directories.sort();
    let values = names
        .iter()
        .map(|name| format!("{name:?}"))
        .collect::<Vec<_>>()
        .join(",");
    fs::create_dir_all(out)?;
    fs::write(
        out.join("meta_peripherals.rs"),
        format!(
            "/// List of all nxp-pac peripherals, whether they are used or not.\npub const META_PERIPHERALS: &[&str] = &[{values}];\n"
        ),
    )?;
    Ok(directories)
}
