Trying to use a single workspace to build a project with crates of mixed targets
- share: host target (running tests) or arm-none-eabi target (when included by the `app`)
- app: arm-none-eabi building with arm-none-eabi

I want to run cargo b in the workspace.
This does not work:
`error: invalid register `r0`: unknown register` and so on
