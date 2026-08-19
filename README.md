# Sunder

Version **0.2.2**. Virtual analog CLAP synthesizer for Bitwig on Linux: two oscillators + sub, hard sync, PWM, supersaw unison, driven SVF, chorus, and a factory/user preset bank.

See `CHANGELOG.md` for release notes and `AGENTS.md` for how the project is put together.

## Build

```bash
cargo xtask bundle sunder --release
```

The bundle is written to `target/bundled/Sunder.clap`.

## Install for Bitwig

```bash
mkdir -p ~/.clap
cp -a target/bundled/Sunder.clap ~/.clap/
```

Then rescan plugins in Bitwig. Look for **Sunder**.

User presets are saved to `~/.local/share/sunder/presets/`. Factory patches are read-only.
