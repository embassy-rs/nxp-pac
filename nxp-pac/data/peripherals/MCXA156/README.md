# MCXA156 metadata corrections

Corrections use the MCX A156 reference manual (`MCXAP100M96FS6RM`, Rev. 7,
2024-12-09). Unresolved source conflicts are noted below.

Sources used to verify the corrections:

| Source | Revision | SHA-256 |
| --- | --- | --- |
| `MCXAP100M96FS6RM.pdf` | Rev. 7, 2024-12-09 | `962531bac7b68f4526ca844cb9497a566540c796feb833246485d0c8aa1fe20c` |
| `MCXAP100M96FS6.pdf` | Rev. 10, 2026-05-26 | `9c720d974792fb1fe990eda231e768f4bc15a4720c27009ac35cc7e46b43420b` |
| `MCXA156/MCXA156.xml` | `mcux-soc-svd` commit `7c1f00bdd6263f0b93466f5e73e5fc55a744b879` | `6f0da8b623578892908b8f08cba4f3e8417ad0fee247ca4f19f6ed4888066866` |

## Registers

- **SPC:** encoding 3 is reserved in SRAMCTL.VSM, ACTIVE_CFG.CORELDO_VDD_LVL,
  and LP_CFG.CORELDO_VDD_LVL (RM §25.7.6, §25.7.9, §25.7.11).
- **INPUTMUX:** ADC0/1, CMP0/1, and FlexPWM0/1 retain separate selector
  types. Their cross-instance sources differ (RM §13.5.1.17,
  §13.5.1.43, §13.5.1.56).
- **PORT:** PCR fieldsets preserve optional PFE, PV, DSE1 fields and MUX
  widths (RM §11.6.1). PORT4 retains the SVD's MUX values 0–6. The RM
  also lists value 7 for PORT4.PCR2/4/5/6 (§11.6.1.10, §11.6.1.12,
  §11.6.1.13); that source discrepancy remains unresolved.
- **SCG:** FIRCATC1–11 follow the pinned SVD at offsets 0x31C–0x344.
  RM §22.7.1.1 omits all eleven registers. The previous SVD pin,
  `da354fa1807150dcbe1e9bddbd92ab9b11fc6454`, defined only FIRCATC1–3.

## Pin routes

- **OPAMP0:** RM §45.1 explicitly prohibits using INP1 on this device.
  Accordingly, `MCXA156.json` omits DP1; P2_13 remains the DM0 input.
- **WUU0.P17:** `MCXA156.json` retains P4_6, following RM §16.1.3,
  Table 85. This conflicts with RM §10.3, Table 63, and datasheet
  `MCXAP100M96FS6`, Rev. 10, 2026-05-26, Table 45, which associate
  WUU0_IN17 with P4_4. The mapping is unresolved; do not rely on P17 as
  the sole wake source without hardware validation or vendor clarification.
  P4_4 and P4_6 are not advertised as interchangeable alternatives.
