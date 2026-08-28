# Sunder

Version **0.6.1**. A virtual-analog (VA) subtractive synthesizer — a CLAP instrument modeled on analog poly synths, not FM, wavetable, or granular.

The engine is classic subtractive: two polyBLEP oscillators plus a sub, optional PWM, hard sync, and a cheap supersaw (extra phases into one filter). That mix goes through a driven state-variable filter (LP/BP/HP, 2- or 4-pole), then amp and filter ADSRs, a pitch envelope, an LFO, legato glide, noise, and a stereo chorus.

It’s 8-voice polyphonic, Linux/CLAP-only, and aimed at Bitwig. Think Juno / Jupiter / OB-style analog character, not DX7-style FM (the Famous bank has DX-inspired patches, but they’re still analog-style recreations on this VA engine). Factory and user preset banks are included.

See `CHANGELOG.md` for release notes and `AGENTS.md` for how the project is put together. 1080p marketing stills (including isolated hardware visualizations) live in `marketing/`.

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
