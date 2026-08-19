use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::params::{SunderParams, WaveChoice};
use nih_plug::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Bass,
    Lead,
    Pad,
    Keys,
    Famous,
    Sfx,
}

impl Category {
    pub const ALL: [Category; 6] = [
        Category::Bass,
        Category::Lead,
        Category::Pad,
        Category::Keys,
        Category::Famous,
        Category::Sfx,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Bass => "Bass",
            Self::Lead => "Lead",
            Self::Pad => "Pad",
            Self::Keys => "Keys",
            Self::Famous => "Famous",
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
    pub osc2_pwm: f32,
    pub sync: bool,
    pub sub_mix: f32,
    pub sub_square: bool,
    pub noise: f32,
    pub cutoff: f32,
    pub res: f32,
    pub drive: f32,
    pub filt_env: f32,
    pub keytrack: f32,
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
    pub cho_mix: f32,
    pub cho_rate: f32,
    pub cho_depth: f32,
}

impl Default for Patch {
    fn default() -> Self {
        let p = SunderParams::default();
        snapshot(&p)
    }
}

pub fn snapshot(p: &SunderParams) -> Patch {
    Patch {
        gain: p.gain.value(),
        glide: p.glide.value(),
        osc1_wave: p.osc1_wave.value().to_index() as i32,
        osc1_mix: p.osc1_mix.value(),
        osc1_oct: p.osc1_oct.value(),
        osc1_semi: p.osc1_semi.value(),
        osc1_pwm: p.osc1_pwm.value(),
        unison: p.unison.value(),
        detune: p.detune.value(),
        stereo: p.stereo.value(),
        osc2_wave: p.osc2_wave.value().to_index() as i32,
        osc2_mix: p.osc2_mix.value(),
        osc2_oct: p.osc2_oct.value(),
        osc2_semi: p.osc2_semi.value(),
        osc2_pwm: p.osc2_pwm.value(),
        sync: p.sync.value(),
        sub_mix: p.sub_mix.value(),
        sub_square: p.sub_square.value(),
        noise: p.noise.value(),
        cutoff: p.cutoff.value(),
        res: p.res.value(),
        drive: p.drive.value(),
        filt_env: p.filt_env.value(),
        keytrack: p.keytrack.value(),
        amp_a: p.amp_a.value(),
        amp_d: p.amp_d.value(),
        amp_s: p.amp_s.value(),
        amp_r: p.amp_r.value(),
        filt_a: p.filt_a.value(),
        filt_d: p.filt_d.value(),
        filt_s: p.filt_s.value(),
        filt_r: p.filt_r.value(),
        lfo_rate: p.lfo_rate.value(),
        lfo_amt: p.lfo_amt.value(),
        cho_mix: p.cho_mix.value(),
        cho_rate: p.cho_rate.value(),
        cho_depth: p.cho_depth.value(),
    }
}

pub fn apply(p: &SunderParams, setter: &ParamSetter, patch: &Patch) {
    set_f(setter, &p.gain, patch.gain);
    set_f(setter, &p.glide, patch.glide);
    set_enum(setter, &p.osc1_wave, wave_from_i(patch.osc1_wave));
    set_f(setter, &p.osc1_mix, patch.osc1_mix);
    set_i(setter, &p.osc1_oct, patch.osc1_oct);
    set_i(setter, &p.osc1_semi, patch.osc1_semi);
    set_f(setter, &p.osc1_pwm, patch.osc1_pwm);
    set_i(setter, &p.unison, patch.unison);
    set_f(setter, &p.detune, patch.detune);
    set_f(setter, &p.stereo, patch.stereo);
    set_enum(setter, &p.osc2_wave, wave_from_i(patch.osc2_wave));
    set_f(setter, &p.osc2_mix, patch.osc2_mix);
    set_i(setter, &p.osc2_oct, patch.osc2_oct);
    set_i(setter, &p.osc2_semi, patch.osc2_semi);
    set_f(setter, &p.osc2_pwm, patch.osc2_pwm);
    set_b(setter, &p.sync, patch.sync);
    set_f(setter, &p.sub_mix, patch.sub_mix);
    set_b(setter, &p.sub_square, patch.sub_square);
    set_f(setter, &p.noise, patch.noise);
    set_f(setter, &p.cutoff, patch.cutoff);
    set_f(setter, &p.res, patch.res);
    set_f(setter, &p.drive, patch.drive);
    set_f(setter, &p.filt_env, patch.filt_env);
    set_f(setter, &p.keytrack, patch.keytrack);
    set_f(setter, &p.amp_a, patch.amp_a);
    set_f(setter, &p.amp_d, patch.amp_d);
    set_f(setter, &p.amp_s, patch.amp_s);
    set_f(setter, &p.amp_r, patch.amp_r);
    set_f(setter, &p.filt_a, patch.filt_a);
    set_f(setter, &p.filt_d, patch.filt_d);
    set_f(setter, &p.filt_s, patch.filt_s);
    set_f(setter, &p.filt_r, patch.filt_r);
    set_f(setter, &p.lfo_rate, patch.lfo_rate);
    set_f(setter, &p.lfo_amt, patch.lfo_amt);
    set_f(setter, &p.cho_mix, patch.cho_mix);
    set_f(setter, &p.cho_rate, patch.cho_rate);
    set_f(setter, &p.cho_depth, patch.cho_depth);
}

fn wave_from_i(i: i32) -> WaveChoice {
    match i {
        1 => WaveChoice::Square,
        2 => WaveChoice::Triangle,
        3 => WaveChoice::Sine,
        _ => WaveChoice::Saw,
    }
}

fn set_f(setter: &ParamSetter, param: &FloatParam, value: f32) {
    setter.begin_set_parameter(param);
    setter.set_parameter(param, value);
    setter.end_set_parameter(param);
}

fn set_i(setter: &ParamSetter, param: &IntParam, value: i32) {
    setter.begin_set_parameter(param);
    setter.set_parameter(param, value);
    setter.end_set_parameter(param);
}

fn set_b(setter: &ParamSetter, param: &BoolParam, value: bool) {
    setter.begin_set_parameter(param);
    setter.set_parameter(param, value);
    setter.end_set_parameter(param);
}

fn set_enum(setter: &ParamSetter, param: &EnumParam<WaveChoice>, value: WaveChoice) {
    setter.begin_set_parameter(param);
    setter.set_parameter(param, value);
    setter.end_set_parameter(param);
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
