use nih_plug::prelude::*;
use std::sync::Arc;

use crate::dsp::osc::Wave;

#[derive(Enum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum WaveChoice {
    #[id = "saw"]
    Saw,
    #[id = "square"]
    Square,
    #[id = "tri"]
    Triangle,
    #[id = "sine"]
    Sine,
}

impl WaveChoice {
    pub fn to_wave(self) -> Wave {
        match self {
            Self::Saw => Wave::Saw,
            Self::Square => Wave::Square,
            Self::Triangle => Wave::Triangle,
            Self::Sine => Wave::Sine,
        }
    }
}

#[derive(Enum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum NoiseChoice {
    #[id = "white"]
    White,
    #[id = "pink"]
    Pink,
    #[id = "brown"]
    Brown,
    #[id = "digi"]
    Digital,
}

impl NoiseChoice {
    pub fn to_kind(self) -> crate::dsp::noise::NoiseKind {
        match self {
            Self::White => crate::dsp::noise::NoiseKind::White,
            Self::Pink => crate::dsp::noise::NoiseKind::Pink,
            Self::Brown => crate::dsp::noise::NoiseKind::Brown,
            Self::Digital => crate::dsp::noise::NoiseKind::Digital,
        }
    }
}

#[derive(Enum, PartialEq, Eq, Debug, Clone, Copy)]
pub enum FilterChoice {
    #[id = "lp"]
    Lowpass,
    #[id = "bp"]
    Bandpass,
    #[id = "hp"]
    Highpass,
}

impl FilterChoice {
    pub fn to_mode(self) -> crate::dsp::filter::FilterMode {
        match self {
            Self::Lowpass => crate::dsp::filter::FilterMode::Lowpass,
            Self::Bandpass => crate::dsp::filter::FilterMode::Bandpass,
            Self::Highpass => crate::dsp::filter::FilterMode::Highpass,
        }
    }
}

#[derive(Params)]
pub struct SunderParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<nih_plug_egui::EguiState>,

    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "glide"]
    pub glide: FloatParam,
    #[id = "legato"]
    pub legato: BoolParam,

    #[id = "o1wav"]
    pub osc1_wave: EnumParam<WaveChoice>,
    #[id = "o1mix"]
    pub osc1_mix: FloatParam,
    #[id = "o1oct"]
    pub osc1_oct: IntParam,
    #[id = "o1semi"]
    pub osc1_semi: IntParam,
    #[id = "o1pwm"]
    pub osc1_pwm: FloatParam,
    #[id = "uni"]
    pub unison: IntParam,
    #[id = "udet"]
    pub detune: FloatParam,
    #[id = "usprd"]
    pub stereo: FloatParam,

    #[id = "o2wav"]
    pub osc2_wave: EnumParam<WaveChoice>,
    #[id = "o2mix"]
    pub osc2_mix: FloatParam,
    #[id = "o2oct"]
    pub osc2_oct: IntParam,
    #[id = "o2semi"]
    pub osc2_semi: IntParam,
    #[id = "o2ct"]
    pub osc2_cents: FloatParam,
    #[id = "o2pwm"]
    pub osc2_pwm: FloatParam,
    #[id = "sync"]
    pub sync: BoolParam,

    #[id = "sub"]
    pub sub_mix: FloatParam,
    #[id = "subsq"]
    pub sub_square: BoolParam,
    #[id = "noise"]
    pub noise: FloatParam,
    #[id = "ntype"]
    pub noise_type: EnumParam<NoiseChoice>,

    #[id = "cut"]
    pub cutoff: FloatParam,
    #[id = "res"]
    pub res: FloatParam,
    #[id = "drv"]
    pub drive: FloatParam,
    #[id = "fenv"]
    pub filt_env: FloatParam,
    #[id = "ktrk"]
    pub keytrack: FloatParam,
    #[id = "fmode"]
    pub filt_mode: EnumParam<FilterChoice>,
    #[id = "fpole"]
    pub four_pole: BoolParam,

    #[id = "aatk"]
    pub amp_a: FloatParam,
    #[id = "adec"]
    pub amp_d: FloatParam,
    #[id = "asus"]
    pub amp_s: FloatParam,
    #[id = "arel"]
    pub amp_r: FloatParam,

    #[id = "fatk"]
    pub filt_a: FloatParam,
    #[id = "fdec"]
    pub filt_d: FloatParam,
    #[id = "fsus"]
    pub filt_s: FloatParam,
    #[id = "frel"]
    pub filt_r: FloatParam,
    #[id = "penv"]
    pub pitch_env: FloatParam,

    #[id = "lfohz"]
    pub lfo_rate: FloatParam,
    #[id = "lfoamt"]
    pub lfo_amt: FloatParam,
    #[id = "lfopit"]
    pub lfo_pitch: FloatParam,
    #[id = "lfopwm"]
    pub lfo_pwm: FloatParam,

    #[id = "chmix"]
    pub cho_mix: FloatParam,
    #[id = "chrate"]
    pub cho_rate: FloatParam,
    #[id = "chdpth"]
    pub cho_depth: FloatParam,
}

impl Default for SunderParams {
    fn default() -> Self {
        Self {
            editor_state: nih_plug_egui::EguiState::from_size(820, 560),
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(-6.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-36.0),
                    max: util::db_to_gain(6.0),
                    factor: FloatRange::gain_skew_factor(-36.0, 6.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(10.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(1))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            glide: ms_param("Glide", 0.0, 0.0, 500.0),
            legato: BoolParam::new("Legato", false),
            osc1_wave: EnumParam::new("Osc1 Wave", WaveChoice::Saw),
            osc1_mix: unit_param("Osc1 Mix", 0.8),
            osc1_oct: IntParam::new("Osc1 Oct", 0, IntRange::Linear { min: -2, max: 2 }),
            osc1_semi: IntParam::new("Osc1 Semi", 0, IntRange::Linear { min: -12, max: 12 }),
            osc1_pwm: FloatParam::new(
                "Osc1 PWM",
                0.5,
                FloatRange::Linear { min: 0.05, max: 0.95 },
            )
            .with_smoother(SmoothingStyle::Linear(20.0))
            .with_step_size(0.01)
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            unison: IntParam::new("Unison", 1, IntRange::Linear { min: 1, max: 5 }),
            detune: FloatParam::new(
                "Detune",
                12.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 50.0,
                },
            )
            .with_step_size(0.1)
            .with_unit(" ct"),
            stereo: unit_param("Stereo", 0.4),
            osc2_wave: EnumParam::new("Osc2 Wave", WaveChoice::Saw),
            osc2_mix: unit_param("Osc2 Mix", 0.0),
            osc2_oct: IntParam::new("Osc2 Oct", 0, IntRange::Linear { min: -2, max: 2 }),
            osc2_semi: IntParam::new("Osc2 Semi", 0, IntRange::Linear { min: -12, max: 12 }),
            osc2_cents: FloatParam::new(
                "Osc2 Cents",
                0.0,
                FloatRange::Linear {
                    min: -50.0,
                    max: 50.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(20.0))
            .with_step_size(0.1)
            .with_unit(" ct"),
            osc2_pwm: FloatParam::new(
                "Osc2 PWM",
                0.5,
                FloatRange::Linear { min: 0.05, max: 0.95 },
            )
            .with_smoother(SmoothingStyle::Linear(20.0))
            .with_step_size(0.01)
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            sync: BoolParam::new("Sync", false),
            sub_mix: unit_param("Sub", 0.25),
            sub_square: BoolParam::new("Sub Square", false),
            noise: unit_param("Noise", 0.0),
            noise_type: EnumParam::new("Noise Type", NoiseChoice::White),
            cutoff: FloatParam::new(
                "Cutoff",
                1200.0,
                FloatRange::Skewed {
                    min: 40.0,
                    max: 18_000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(30.0))
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(1))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
            res: FloatParam::new("Resonance", 0.2, FloatRange::Linear { min: 0.0, max: 0.95 })
                .with_smoother(SmoothingStyle::Linear(30.0)),
            drive: unit_param("Drive", 0.15),
            filt_env: FloatParam::new(
                "Filt Env",
                0.25,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            )
            .with_step_size(0.01)
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            keytrack: unit_param("Keytrack", 0.2),
            filt_mode: EnumParam::new("Filter Mode", FilterChoice::Lowpass),
            four_pole: BoolParam::new("4 Pole", false),
            amp_a: ms_param("Amp Atk", 5.0, 0.2, 4000.0),
            amp_d: ms_param("Amp Dec", 120.0, 1.0, 4000.0),
            amp_s: unit_param("Amp Sus", 0.8),
            amp_r: ms_param("Amp Rel", 180.0, 1.0, 6000.0),
            filt_a: ms_param("Filt Atk", 8.0, 0.2, 4000.0),
            filt_d: ms_param("Filt Dec", 180.0, 1.0, 4000.0),
            filt_s: unit_param("Filt Sus", 0.35),
            filt_r: ms_param("Filt Rel", 220.0, 1.0, 6000.0),
            pitch_env: FloatParam::new(
                "Pitch Env",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 2.0,
                },
            )
            .with_step_size(0.01)
            .with_unit(" oct")
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            lfo_rate: FloatParam::new(
                "LFO Rate",
                0.4,
                FloatRange::Skewed {
                    min: 0.05,
                    max: 20.0,
                    factor: FloatRange::skew_factor(-1.5),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(40.0))
            .with_unit(" Hz"),
            lfo_amt: unit_param("LFO Cut", 0.0),
            lfo_pitch: unit_param("LFO Pitch", 0.0),
            lfo_pwm: unit_param("LFO PWM", 0.0),
            cho_mix: unit_param("Chorus", 0.0),
            cho_rate: FloatParam::new(
                "Chorus Rate",
                0.6,
                FloatRange::Linear {
                    min: 0.05,
                    max: 4.0,
                },
            )
            .with_unit(" Hz"),
            cho_depth: unit_param("Chorus Depth", 0.45),
        }
    }
}

fn unit_param(name: &'static str, default: f32) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Linear { min: 0.0, max: 1.0 })
        .with_smoother(SmoothingStyle::Linear(20.0))
        .with_step_size(0.01)
        .with_value_to_string(formatters::v2s_f32_rounded(2))
}

fn ms_param(name: &'static str, default: f32, min: f32, max: f32) -> FloatParam {
    FloatParam::new(
        name,
        default,
        FloatRange::Skewed {
            min,
            max,
            factor: FloatRange::skew_factor(-1.5),
        },
    )
    .with_step_size(0.1)
    .with_unit(" ms")
    .with_value_to_string(formatters::v2s_f32_rounded(0))
}
