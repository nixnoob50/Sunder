# Sunder

![Sunder title card](marketing/sunder-title-card.webp)

Version **0.6.1**. A virtual-analog (VA) subtractive synthesizer — a CLAP instrument modeled on analog poly synths, not FM, wavetable, or granular.

The engine is classic subtractive: two polyBLEP oscillators plus a sub, optional PWM, hard sync, and a cheap supersaw (extra phases into one filter). That mix goes through a driven state-variable filter (LP/BP/HP, 2- or 4-pole), then amp and filter ADSRs, a pitch envelope, an LFO, legato glide, noise, and a stereo chorus.

It’s 8-voice polyphonic, Linux/CLAP-only, and aimed at Bitwig. Think Juno / Jupiter / OB-style analog character, not DX7-style FM (the Famous bank has DX-inspired patches, but they’re still analog-style recreations on this VA engine). Factory and user preset banks are included.

See `CHANGELOG.md` for release notes and `AGENTS.md` for how the project is put together. 1080p marketing stills (including isolated hardware visualizations) live in `marketing/`.

## Download (prebuilt)

Prebuilt Linux `Sunder.clap` bundles are on the [GitHub Releases](https://github.com/nixnoob50/Sunder/releases) page.

```bash
mkdir -p ~/.clap
# After downloading Sunder.clap from the latest release:
cp -a ~/Downloads/Sunder.clap ~/.clap/
```

Then rescan plugins in Bitwig and look for **Sunder**. If Bitwig still shows an older UI after an update, remove the plugin from the track and add it again.

User presets are saved to `~/.local/share/sunder/presets/`. Factory patches are read-only.

## Requirements

### Runtime

- Linux (x86_64)
- A CLAP host — Bitwig is the primary target
- OpenGL-capable GPU/drivers (editor uses egui)

### Build

- [Rust](https://rustup.rs/) **1.80** or newer (`rustc` / `cargo`)
- Git (nih-plug is pulled from GitHub)
- Linux X11 / OpenGL development libraries

On Debian/Ubuntu:

```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential pkg-config \
  libasound2-dev libgl-dev libjack-dev \
  libx11-xcb-dev libxcb1-dev libxcb-dri2-0-dev \
  libxcb-icccm4-dev libxcursor-dev libxkbcommon-dev \
  libxcb-shape0-dev libxcb-xfixes0-dev
```

## Build

```bash
git clone https://github.com/nixnoob50/Sunder.git
cd Sunder
cargo xtask bundle sunder --release
```

The bundle is written to `target/bundled/Sunder.clap`. If `CARGO_TARGET_DIR` is set, look under that tree’s `bundled/Sunder.clap` instead.

## Install for Bitwig

```bash
mkdir -p ~/.clap
cp -a target/bundled/Sunder.clap ~/.clap/
```

Then rescan plugins in Bitwig. Look for **Sunder**.
