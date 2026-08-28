# Sunder — agent notes

Linux **CLAP** virtual-analog instrument for **Bitwig**. Crate name `sunder`, plugin name **Sunder**, CLAP id `com.sunder.va`, bundle `~/.clap/Sunder.clap`.

Do **not** rename the product. Do **not** add VST3/LV2, joke UI, macros, dice rolls, or CUDA.

## Stack

- Rust, **nih-plug** (git) + **nih-plug-egui** / **egui**
- CLAP only (`bundler.toml` → `Sunder.clap`)
- DSP: 8-voice poly, 2 polyBLEP oscs + sub, PWM, hard sync, cheap supersaw (extra phases into **one** filter), driven SVF (LP/BP/HP, 2 or 4 pole), amp + filter ADSR, pitch envelope (`pitch_env`), LFO → cutoff / pitch / PWM, legato glide (ms per octave), stereo chorus after voice sum, noise (white / pink / brown / digital LFSR) through the same amp envelope
- Pitch envelope (**P.ENV**): on note-on pitch starts high and falls exponentially; fall time follows **Filter Decay** (zaps / sweeps).
- No CUDA; Bitwig handles reverb/delay/EQ
- User presets: `~/.local/share/sunder/presets/`
- Star ratings: `~/.local/share/sunder/ratings.json` (factory bank stays read-only). All Sunder instances share this file; the editor refreshes from it so a rating change shows up in other open windows.
- Factory bank: `presets/factory.json` (embedded at compile time)

## Layout rules (`src/editor.rs`)

- Preset column is a **fixed ~252px** width so names and 1–5 stars sit on one row. Module panels use **leftover width only**.
- Category chips sit in **two columns** at the top of the preset sidebar. **Favorites** is a full-width chip above them (virtual view, not a `Category`). Save/Del and the name field are a **pinned footer** (bottom-up layout); **Refresh** joins that row when Favorites is dirty. The preset list only uses leftover height.
- The loaded preset name is shown **centered in the title bar**, not under the preset list.
- Prefer side-by-side modules via explicit half-width verticals (`module_pair`). Do **not** use nested `ui.columns` or `egui::Grid` plus `set_clip_rect` for modules — those overlap frames and clip knob labels.
- Pack knobs to fill column width (size from ~4.5 slots); keep filter/env/fx rows dense rather than leaving empty panel right sides.
- Do not size the modules pane with `available_width()` *inside* a horizontal layout if that still reports the full window width. Measure leftover width from the parent rect first.
- Header Gain lives in a small LTR box beside the knob (label + dB), not floating below.
- Click a preset name to load. Stars sit to the **right** of the name; click the current star again to clear. Factory is read-only; Save/Delete is for user patches.

## Presets

- Original patch names. Famous bank is **inspired-by** analog/digital characters (Juno, JP-8, DX7, OB-X, etc.), not trademarked factory clones. Songs bank uses `Patch Name - Song Title` for inspired-by record synths.
- `Category`: Bass, Lead, Pad, Keys, Organ, Brass, Strings, Plucks, Bells, Famous, Songs, Sfx. Adding a category requires `Category::ALL`, `label()`, serde, and factory JSON. Favorites is **not** a category — it lists starred patches (rating 1–5), sorted by stars descending then name when you open the view or press **Refresh**. Rating while Favorites is open does not reorder the list.
- Wave indices: `0` saw, `1` square, `2` tri, `3` sine. Noise types: `0` white, `1` pink, `2` brown, `3` digital. Filter mode: `0` LP, `1` BP, `2` HP. Osc oct `[-2, 2]`, osc 2 cents `[-50, 50]`, unison `[1, 5]`.
- Glide is **ms per octave** and only slides on overlapping notes when Legato is on.
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
