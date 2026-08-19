# Changelog

All notable changes to Sunder are documented here.

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
