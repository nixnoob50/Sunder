# Sunder — agent notes

Linux **CLAP** virtual-analog instrument for **Bitwig**. Crate name `sunder`, plugin name **Sunder**, CLAP id `com.sunder.va`, bundle `~/.clap/Sunder.clap`.

Do **not** rename the product. Do **not** add VST3/LV2, joke UI, macros, dice rolls, or CUDA.

## Stack

- Rust, **nih-plug** (git) + **nih-plug-egui** / **egui**
- CLAP only (`bundler.toml` → `Sunder.clap`)
- DSP: 8-voice poly, 2 polyBLEP oscs + sub, PWM, hard sync, cheap supersaw (extra phases into **one** filter), driven SVF, amp + filter ADSR, LFO → cutoff, glide, stereo chorus after voice sum
- No CUDA; Bitwig handles reverb/delay/EQ
- User presets: `~/.local/share/sunder/presets/`
- Factory bank: `presets/factory.json` (embedded at compile time)

## Layout rules (`src/editor.rs`)

- Preset column is a **fixed ~176px** width. Module panels use **leftover width only**.
- Prefer `ui.columns` or a single stacked column. Do **not** use `egui::Grid` plus `set_clip_rect` for modules — that draws empty frames and clips titles off the right edge.
- Do not size the modules pane with `available_width()` *inside* a horizontal layout if that still reports the full window width. Measure leftover width from the parent rect first.
- Header Gain lives in a small LTR box beside the knob (label + dB), not floating below.
- Click a preset name to load. No Load button. Factory is read-only; Save/Delete is for user patches.

## Presets

- Original patch names. Famous bank is **inspired-by** analog/digital characters (Juno, JP-8, DX7, OB-X, etc.), not trademarked factory clones. Songs bank uses `Patch Name - Song Title` for inspired-by record synths.
- `Category`: Bass, Lead, Pad, Keys, Famous, Songs, Sfx. Adding a category requires `Category::ALL`, `label()`, serde, and factory JSON.
- Wave indices: `0` saw, `1` square, `2` tri, `3` sine. Osc oct `[-2, 2]`, unison `[1, 5]`.
- After changing `factory.json`, rebuild — the file is `include_str!`, not loaded from disk at runtime.
- For bass: keep cutoff low, sub high, `filt_env` modest (env is scaled by ~8 octaves in the voice).

## Build and install

```bash
cargo xtask bundle sunder --release
cp -f target/bundled/Sunder.clap ~/.clap/Sunder.clap
```

If `CARGO_TARGET_DIR` is set, the bundle is under that tree’s `bundled/Sunder.clap`. Bitwig often keeps the old editor binary until the plugin is **removed from the track and added again**.

Plugin version is `CARGO_PKG_VERSION` in `Cargo.toml`. Bump it and `CHANGELOG.md` together.

## Code map

| Path | Role |
| --- | --- |
| `src/lib.rs` | Plugin, process, `ResizableWindow` editor |
| `src/params.rs` | `SunderParams` |
| `src/editor.rs` | Dark/amber UI, knobs, preset browser |
| `src/dsp/` | osc, filter, env, chorus, voice |
| `src/presets.rs` | Factory embed + XDG user bank |
| `xtask/` | nih-plug bundler |
