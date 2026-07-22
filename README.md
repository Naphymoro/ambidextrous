# Ambidextrous

Ambidextrous is a cross-platform scientific compute planning and execution environment. It provides one shared core for a native CLI, a Tauri desktop terminal, and a local web interface.

## Product principles

- Terminal-first, with `ambi` and `ambidextrous` invoking the same CLI.
- Local-first and offline-capable by default.
- One command registry shared by CLI, desktop, and web interfaces.
- Benchmark before scale.
- Domain-neutral core with pluggable scientific modules.
- Explicit resource targets: local, WSL, Docker, Apptainer, SSH, Slurm, and PBS.
- No telemetry by default.

## Milestones

### Milestone 1

Production repository, Rust workspace, unified CLI, Tauri desktop shell, PTY terminal, local web interface, and cross-platform CI.

### Milestone 2

Restore and unify Legacy commands: `help`, `status`, `probe`, `modules`, `resources`, `executables`, `benchmark`, `plan`, `run`, `submit`, `jobs`, `logs`, `cancel`, `history`, `username`, `theme`, `doctor`, `version`, and `clear`.

### Milestone 3

Materials module using ASE with Quantum ESPRESSO, CP2K, GPAW, resource management, benchmarking, and scheduler integration.

## Workspace

```text
crates/ambi-core       shared command and domain core
crates/ambi-cli        native CLI (`ambi` and `ambidextrous`)
apps/desktop           Tauri desktop application
apps/web               local browser interface
modules/materials      ASE-backed materials module
.github/workflows      cross-platform build and test workflows
```

## Development status

The repository is being built milestone by milestone. Production readiness requires passing automated tests on Windows, Linux, and macOS before release.
