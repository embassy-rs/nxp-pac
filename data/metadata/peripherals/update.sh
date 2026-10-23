#!/bin/bash
set -euo pipefail

echo "Automatic replacement of curated peripheral YAML is disabled." >&2
echo "Use 'cargo run -p generator -- extract <chip>' for explicit scratch import." >&2
echo "Maintain and review nxp-pac/data/peripherals YAML directly." >&2
exit 1
