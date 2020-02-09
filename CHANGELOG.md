## 0.5.0
- Added _private exports_! Any export name beginning with `_` will be considered private to the
  package which owns it. Packages may not directly reference the private exports of their
  dependencies.
- Fixed crash when loading packages from any parent directory `--dir ..` (3d3f804)
- Fixed `starpkg new` failing when used in a subdirectory of any existing package (#2, ff5c306)
- Customized the user guide theme (867432a, 0fd9197)
