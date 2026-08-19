use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::params::SunderParams;
use nih_plug::prelude::*;
use nih_plug::wrapper::state::ParamValue;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Bass,
    Lead,
    Pad,
    Keys,
    Famous,
    Songs,
    Sfx,
}

impl Category {
    pub const ALL: [Category; 7] = [
        Category::Bass,
        Category::Lead,
        Category::Pad,
        Category::Keys,
        Category::Famous,
        Category::Songs,
        Category::Sfx,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Bass => "Bass",
            Self::Lead => "Lead",
            Self::Pad => "Pad",
            Self::Keys => "Keys",
            Self::Famous => "Famous",
            Self::Songs => "Songs",
            Self::Sfx => "SFX",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub category: Category,
    pub params: Patch,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Patch {
    pub gain: f32,
    pub glide: f32,
    pub osc1_wave: i32,
    pub osc1_mix: f32,
    pub osc1_oct: i32,
    pub osc1_semi: i32,
    pub osc1_pwm: f32,
    pub unison: i32,
    pub detune: f32,
    pub stereo: f32,
    pub osc2_wave: i32,
    pub osc2_mix: f32,
    pub osc2_oct: i32,
    pub osc2_semi: i32,
    #[serde(default)]
    pub osc2_cents: f32,
    pub osc2_pwm: f32,
    pub sync: bool,
    pub sub_mix: f32,
    pub sub_square: bool,
    pub noise: f32,
    #[serde(default)]
    pub noise_type: i32,
    pub cutoff: f32,
    pub res: f32,
    pub drive: f32,
    pub filt_env: f32,
    pub keytrack: f32,
    #[serde(default)]
    pub filt_mode: i32,
    #[serde(default)]
    pub four_pole: bool,
    pub amp_a: f32,
    pub amp_d: f32,
    pub amp_s: f32,
    pub amp_r: f32,
    pub filt_a: f32,
    pub filt_d: f32,
    pub filt_s: f32,
    pub filt_r: f32,
    pub lfo_rate: f32,
    pub lfo_amt: f32,
    #[serde(default)]
    pub lfo_pitch: f32,
    #[serde(default)]
    pub lfo_pwm: f32,
    pub cho_mix: f32,
    pub cho_rate: f32,
    pub cho_depth: f32,
    #[serde(default)]
    pub legato: bool,
}

impl Default for Patch {
    fn default() -> Self {
        let p = SunderParams::default();
        snapshot(&p)
    }
}

pub fn snapshot(p: &SunderParams) -> Patch {
    Patch {
        gain: p.gain.unmodulated_plain_value(),
        glide: p.glide.unmodulated_plain_value(),
        osc1_wave: p.osc1_wave.unmodulated_plain_value().to_index() as i32,
        osc1_mix: p.osc1_mix.unmodulated_plain_value(),
        osc1_oct: p.osc1_oct.unmodulated_plain_value(),
        osc1_semi: p.osc1_semi.unmodulated_plain_value(),
        osc1_pwm: p.osc1_pwm.unmodulated_plain_value(),
        unison: p.unison.unmodulated_plain_value(),
        detune: p.detune.unmodulated_plain_value(),
        stereo: p.stereo.unmodulated_plain_value(),
        osc2_wave: p.osc2_wave.unmodulated_plain_value().to_index() as i32,
        osc2_mix: p.osc2_mix.unmodulated_plain_value(),
        osc2_oct: p.osc2_oct.unmodulated_plain_value(),
        osc2_semi: p.osc2_semi.unmodulated_plain_value(),
        osc2_cents: p.osc2_cents.unmodulated_plain_value(),
        osc2_pwm: p.osc2_pwm.unmodulated_plain_value(),
        sync: p.sync.unmodulated_plain_value(),
        sub_mix: p.sub_mix.unmodulated_plain_value(),
        sub_square: p.sub_square.unmodulated_plain_value(),
        noise: p.noise.unmodulated_plain_value(),
        noise_type: p.noise_type.unmodulated_plain_value().to_index() as i32,
        cutoff: p.cutoff.unmodulated_plain_value(),
        res: p.res.unmodulated_plain_value(),
        drive: p.drive.unmodulated_plain_value(),
        filt_env: p.filt_env.unmodulated_plain_value(),
        keytrack: p.keytrack.unmodulated_plain_value(),
        filt_mode: p.filt_mode.unmodulated_plain_value().to_index() as i32,
        four_pole: p.four_pole.unmodulated_plain_value(),
        amp_a: p.amp_a.unmodulated_plain_value(),
        amp_d: p.amp_d.unmodulated_plain_value(),
        amp_s: p.amp_s.unmodulated_plain_value(),
        amp_r: p.amp_r.unmodulated_plain_value(),
        filt_a: p.filt_a.unmodulated_plain_value(),
        filt_d: p.filt_d.unmodulated_plain_value(),
        filt_s: p.filt_s.unmodulated_plain_value(),
        filt_r: p.filt_r.unmodulated_plain_value(),
        lfo_rate: p.lfo_rate.unmodulated_plain_value(),
        lfo_amt: p.lfo_amt.unmodulated_plain_value(),
        lfo_pitch: p.lfo_pitch.unmodulated_plain_value(),
        lfo_pwm: p.lfo_pwm.unmodulated_plain_value(),
        cho_mix: p.cho_mix.unmodulated_plain_value(),
        cho_rate: p.cho_rate.unmodulated_plain_value(),
        cho_depth: p.cho_depth.unmodulated_plain_value(),
        legato: p.legato.unmodulated_plain_value(),
    }
}

/// Restore a factory/user patch via plugin state, not GUI automation gestures.
/// Bitwig treats begin/end-set as the user grabbing knobs, which disables automation lanes.
pub fn apply(setter: &ParamSetter, patch: &Patch) {
    let mut state = setter.raw_context.get_state();
    write_patch(&mut state, patch);
    setter.raw_context.set_state(state);
}

fn write_patch(state: &mut PluginState, patch: &Patch) {
    let p = &mut state.params;
    p.insert("gain".into(), ParamValue::F32(patch.gain));
    p.insert("glide".into(), ParamValue::F32(patch.glide));
    p.insert("legato".into(), ParamValue::Bool(patch.legato));
    p.insert("o1wav".into(), ParamValue::String(wave_id(patch.osc1_wave).into()));
    p.insert("o1mix".into(), ParamValue::F32(patch.osc1_mix));
    p.insert("o1oct".into(), ParamValue::I32(patch.osc1_oct));
    p.insert("o1semi".into(), ParamValue::I32(patch.osc1_semi));
    p.insert("o1pwm".into(), ParamValue::F32(patch.osc1_pwm));
    p.insert("uni".into(), ParamValue::I32(patch.unison));
    p.insert("udet".into(), ParamValue::F32(patch.detune));
    p.insert("usprd".into(), ParamValue::F32(patch.stereo));
    p.insert("o2wav".into(), ParamValue::String(wave_id(patch.osc2_wave).into()));
    p.insert("o2mix".into(), ParamValue::F32(patch.osc2_mix));
    p.insert("o2oct".into(), ParamValue::I32(patch.osc2_oct));
    p.insert("o2semi".into(), ParamValue::I32(patch.osc2_semi));
    p.insert("o2ct".into(), ParamValue::F32(patch.osc2_cents));
    p.insert("o2pwm".into(), ParamValue::F32(patch.osc2_pwm));
    p.insert("sync".into(), ParamValue::Bool(patch.sync));
    p.insert("sub".into(), ParamValue::F32(patch.sub_mix));
    p.insert("subsq".into(), ParamValue::Bool(patch.sub_square));
    p.insert("noise".into(), ParamValue::F32(patch.noise));
    p.insert("ntype".into(), ParamValue::String(noise_id(patch.noise_type).into()));
    p.insert("cut".into(), ParamValue::F32(patch.cutoff));
    p.insert("res".into(), ParamValue::F32(patch.res));
    p.insert("drv".into(), ParamValue::F32(patch.drive));
    p.insert("fenv".into(), ParamValue::F32(patch.filt_env));
    p.insert("ktrk".into(), ParamValue::F32(patch.keytrack));
    p.insert("fmode".into(), ParamValue::String(filt_id(patch.filt_mode).into()));
    p.insert("fpole".into(), ParamValue::Bool(patch.four_pole));
    p.insert("aatk".into(), ParamValue::F32(patch.amp_a));
    p.insert("adec".into(), ParamValue::F32(patch.amp_d));
    p.insert("asus".into(), ParamValue::F32(patch.amp_s));
    p.insert("arel".into(), ParamValue::F32(patch.amp_r));
    p.insert("fatk".into(), ParamValue::F32(patch.filt_a));
    p.insert("fdec".into(), ParamValue::F32(patch.filt_d));
    p.insert("fsus".into(), ParamValue::F32(patch.filt_s));
    p.insert("frel".into(), ParamValue::F32(patch.filt_r));
    p.insert("lfohz".into(), ParamValue::F32(patch.lfo_rate));
    p.insert("lfoamt".into(), ParamValue::F32(patch.lfo_amt));
    p.insert("lfopit".into(), ParamValue::F32(patch.lfo_pitch));
    p.insert("lfopwm".into(), ParamValue::F32(patch.lfo_pwm));
    p.insert("chmix".into(), ParamValue::F32(patch.cho_mix));
    p.insert("chrate".into(), ParamValue::F32(patch.cho_rate));
    p.insert("chdpth".into(), ParamValue::F32(patch.cho_depth));
}

fn noise_id(i: i32) -> &'static str {
    match i {
        1 => "pink",
        2 => "brown",
        3 => "digi",
        _ => "white",
    }
}

fn filt_id(i: i32) -> &'static str {
    match i {
        1 => "bp",
        2 => "hp",
        _ => "lp",
    }
}

fn wave_id(i: i32) -> &'static str {
    match i {
        1 => "square",
        2 => "tri",
        3 => "sine",
        _ => "saw",
    }
}

pub fn user_dir() -> PathBuf {
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share"))
        })
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("sunder/presets")
}

pub fn load_user_presets() -> Vec<Preset> {
    let dir = user_dir();
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(&dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        if let Ok(text) = fs::read_to_string(&path) {
            if let Ok(p) = serde_json::from_str::<Preset>(&text) {
                out.push(p);
            }
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

pub fn save_user_preset(preset: &Preset) -> Result<PathBuf, String> {
    let dir = user_dir();
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let slug = sanitize(&preset.name);
    let path = dir.join(format!("{slug}.json"));
    let json = serde_json::to_string_pretty(preset).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(path)
}

pub fn delete_user_preset(name: &str) -> Result<(), String> {
    let path = user_dir().join(format!("{}.json", sanitize(name)));
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn sanitize(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    if s.is_empty() {
        "preset".into()
    } else {
        s
    }
}

pub fn factory_presets() -> &'static [Preset] {
    static FACTORY: OnceLock<Vec<Preset>> = OnceLock::new();
    FACTORY
        .get_or_init(|| {
            serde_json::from_str(include_str!("../presets/factory.json")).unwrap_or_default()
        })
        .as_slice()
}
