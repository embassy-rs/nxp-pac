# SVD import scratch output

Authoritative MetaPAC inputs have moved to
[`nxp-pac/data/peripherals`](../../../nxp-pac/data/peripherals/README.md).

`cargo run -p generator -- extract <chip>` writes scratch import output beneath
`raw/` by default. This is an explicit onboarding operation, not a refresh of
curated inputs. Review import output before incorporating any definitions, and
maintain the curated YAML directly thereafter.
