# Changelog

All notable changes to Sunder are documented here.

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
