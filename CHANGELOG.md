# Changelog

All notable changes to Sunder are documented here.

## [0.5.1] — 2026-08-26

### Fixed
- Factory patches that were silent or near-silent from filter settings: high-cut HPs lowered (e.g. D50 Airbells, CZ Phase Lead, PPG Grit), HP 4-pole disabled, ultra-dark LPs opened (Soft Sub / Sine Weight / etc.)

## [0.5.0] — 2026-08-20

### Removed
- Drums factory category (and drum-only voice hacks: beater click, parallel sub thump)

### Added
- Brass factory category (30 patches) — saw + filter-env brass/horns/sections

### Changed
- Organ factory bank retuned: drawbar-style sine stacks, full sustain, leslie via chorus, percussion click

## [0.4.10] — 2026-08-20

### Changed
- Stronger drums: ~8 ms beater click (independent of amp body), parallel sub thump under the filter for pitched hits, higher makeup when P.ENV is used
- Kick factory patches nudged hotter (sub, click noise, gain)

## [0.4.9] — 2026-08-20

### Fixed
- Pitch envelope now starts at peak on note-on and falls exponentially (was rising with filter attack, then falling too slowly — kicks whooped instead of thumping)

### Changed
- Drums bank retuned: short pitch snaps (~25–55 ms), longer amp bodies, louder kicks, tighter noise hats/snares

## [0.4.8] — 2026-08-20

### Added
- Pitch envelope (P.ENV): filter ADSR also sweeps oscillator pitch in octaves — needed for kicks/toms

### Changed
- Drums factory bank retuned as true one-shots (pitch drop, noise hats/snares, zero sustain, keytrack off)

## [0.4.7] — 2026-08-20

### Added
- Factory categories Organ and Drums (30 patches each)

## [0.4.6] — 2026-08-20

### Fixed
- Preset sidebar Save/Del and name field stay inside the window (pinned footer; list uses leftover height)

## [0.4.5] — 2026-08-20

### Changed
- Loaded preset name is centered in the title bar; removed from under the preset list
- Preset categories are shown in two columns; Save/Del and the name field stay visible below the list
- Mouse wheel over a dial steps one value per notch (Voices 1→2, not 1→5)

## [0.4.4] — 2026-08-20

### Added
- Factory categories Strings, Plucks, and Bells (30 patches each)
- Extra factory patches so every category has at least 30 unique presets

## [0.4.3] — 2026-08-20

### Fixed
- Bandpass is gain-compensated so switching LP→BP no longer collapses the level
- 4-pole no longer cascades BP→BP (that stacked filter was nearly silent)
- A few BP / dark bass Songs and Famous patches opened up so they stay usable
- Mouse wheel over a knob adjusts the dial without scrolling the module pane

### Changed
- Denser module layout: larger knobs that fill the column, packed rows, tighter margins
- Module pairs use explicit widths instead of nested `ui.columns` (fixes overlapping panels and clipped knob labels)

## [0.4.2] — 2026-08-19

### Fixed
- Dark (brown) and pink noise are gain-matched so they are audible against the oscillators

## [0.4.1] — 2026-08-19

### Fixed
- High-pass (and high-resonance band-pass) no longer blows up and pegs the CPU when cutoff and resonance are high

## [0.4.0] — 2026-08-19

### Added
- Osc 2 fine detune (cents)
- LFO to cutoff, pitch, and PWM
- Filter LP / BP / HP plus a 4-pole cascade
- Legato glide (only while a note is held); glide time is milliseconds per octave
- Noise types (white, pink, brown, digital) with a type dropdown; noise shares the amp envelope
- Songs factory category with inspired-by record synths (`Patch Name - Song Title`)

### Changed
- Famous and Songs patches use 4-pole, osc-2 cents, PWM/pitch LFO, and legato where the original instrument needs them
- Chase Square - Axel F uses the user-tuned `gary-axel-f` patch, now with 4-pole, legato, and restored glide

## [0.2.1] — 2026-08-19

### Fixed
- Loading a factory or user preset no longer steals Bitwig automation lanes (cutoff and other knobs keep following the clip)

## [0.2.0] — 2026-08-19

### Added
- Custom dark/amber egui editor with analog-style knobs and a click-to-load preset browser
- Factory preset bank (Bass, Lead, Pad, Keys, Famous, SFX), including Juno / Jupiter-8 / DX7 / OB-X–style patches and extra scratchy and fat leads
- User preset save/delete under `~/.local/share/sunder/presets/`
- Resizable CLAP editor window

### Changed
- Module layout uses leftover width after the preset column (`ui.columns` / stacked panels) so frames and knobs stay on screen
- Bass factory patches sit lower (octave, sub, tighter cutoff)

## [0.1.0] — 2026-08-19

### Added
- Initial CLAP instrument: 8-voice VA engine (2 osc + sub, PWM, hard sync, unison, driven SVF, ADSR, LFO, glide, chorus)
