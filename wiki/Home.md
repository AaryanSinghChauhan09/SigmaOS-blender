# SigmaOS-blender

SigmaOS integration component for **3D creation suite**.

## Commands

```bash
sigmaos-blender install   # install package + deploy defaults
sigmaos-blender deploy    # copy config templates to $HOME
sigmaos-blender status    # binary/package/config report
sigmaos-blender reset     # remove deployed config
sigmaos-blender open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
