//! Eight-voice VA engine: 2 osc + sub + supersaw unison into one filter.

use crate::dsp::env::{Adsr, Stage};
use crate::dsp::filter::{drive, Svf};
use crate::dsp::osc::{self, Wave};

pub const NUM_VOICES: usize = 8;
pub const MAX_UNISON: usize = 5;

#[derive(Clone, Copy, Debug)]
pub struct VoiceParams {
    pub osc1_wave: Wave,
    pub osc1_mix: f32,
    pub osc1_oct: i32,
    pub osc1_semi: i32,
    pub osc1_pwm: f32,
    pub unison: usize,
    pub detune_cents: f32,
    pub stereo: f32,
    pub osc2_wave: Wave,
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
    pub lfo: f32,
    pub glide_ms: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Voice {
    pub active: bool,
    pub note: u8,
    pub channel: u8,
    pub velocity: f32,
    pub age: u64,
    target_hz: f32,
    current_hz: f32,
    osc1: [f32; MAX_UNISON],
    osc2: f32,
    sub: f32,
    noise: u32,
    filter: Svf,
    amp: Adsr,
    filt: Adsr,
}

impl Default for Voice {
    fn default() -> Self {
        Self {
            active: false,
            note: 0,
            channel: 0,
            velocity: 0.0,
            age: 0,
            target_hz: 440.0,
            current_hz: 440.0,
            osc1: [0.0; MAX_UNISON],
            osc2: 0.0,
            sub: 0.0,
            noise: 1,
            filter: Svf::default(),
            amp: Adsr::default(),
            filt: Adsr::default(),
        }
    }
}

impl Voice {
    pub fn start(
        &mut self,
        note: u8,
        channel: u8,
        velocity: f32,
        age: u64,
        from_hz: f32,
        sample_rate: f32,
    ) {
        self.active = true;
        self.note = note;
        self.channel = channel;
        self.velocity = velocity.sqrt();
        self.age = age;
        self.target_hz = osc::midi_hz(note as f32);
        self.current_hz = if from_hz > 1.0 {
            from_hz
        } else {
            self.target_hz
        };
        self.osc1 = [0.0; MAX_UNISON];
        for (i, p) in self.osc1.iter_mut().enumerate() {
            *p = (i as f32 * 0.17) % 1.0;
        }
        self.osc2 = 0.31;
        self.sub = 0.0;
        self.noise = 0xA341_316C ^ (note as u32).wrapping_mul(0x9E37);
        self.filter.reset();
        self.amp.note_on();
        self.filt.note_on();
        let _ = sample_rate;
    }

    pub fn release(&mut self) {
        self.amp.note_off();
        self.filt.note_off();
    }

    pub fn kill(&mut self) {
        *self = Self::default();
    }

    pub fn render(&mut self, p: &VoiceParams, sample_rate: f32) -> (f32, f32) {
        if !self.active {
            return (0.0, 0.0);
        }

        let glide_s = (p.glide_ms.max(0.0) / 1000.0).max(0.0);
        if glide_s < 0.001 {
            self.current_hz = self.target_hz;
        } else {
            let coeff = (-1.0 / (glide_s * sample_rate)).exp();
            self.current_hz += (self.target_hz - self.current_hz) * (1.0 - coeff);
        }

        let n = p.unison.clamp(1, MAX_UNISON);
        let mut left = 0.0;
        let mut right = 0.0;
        let mut wrapped = false;
        for i in 0..n {
            let det = if n == 1 {
                0.0
            } else {
                let t = i as f32 / (n as f32 - 1.0);
                (t * 2.0 - 1.0) * p.detune_cents
            };
            let hz = self.current_hz * 2f32.powf(p.osc1_oct as f32 + p.osc1_semi as f32 / 12.0 + det / 1200.0);
            let (dt, wrap) = osc::tick(&mut self.osc1[i], hz, sample_rate);
            if i == 0 {
                wrapped = wrap;
            }
            let s = osc::render(p.osc1_wave, self.osc1[i], dt, p.osc1_pwm) * p.osc1_mix;
            let pan = if n == 1 {
                0.0
            } else {
                (i as f32 / (n as f32 - 1.0) * 2.0 - 1.0) * p.stereo.clamp(0.0, 1.0)
            };
            let angle = (pan + 1.0) * std::f32::consts::FRAC_PI_4;
            left += s * angle.cos();
            right += s * angle.sin();
        }
        let norm = 1.0 / (n as f32).sqrt();
        left *= norm;
        right *= norm;

        let hz2 = self.current_hz * 2f32.powf(p.osc2_oct as f32 + p.osc2_semi as f32 / 12.0);
        if p.sync && wrapped {
            self.osc2 = 0.0;
        }
        let (dt2, _) = osc::tick(&mut self.osc2, hz2, sample_rate);
        let o2 = osc::render(p.osc2_wave, self.osc2, dt2, p.osc2_pwm) * p.osc2_mix;
        left += o2 * 0.707;
        right += o2 * 0.707;

        let sub_hz = self.current_hz * 0.5;
        let (dts, _) = osc::tick(&mut self.sub, sub_hz, sample_rate);
        let sub_wave = if p.sub_square {
            Wave::Square
        } else {
            Wave::Sine
        };
        let sub = osc::render(sub_wave, self.sub, dts, 0.5) * p.sub_mix;
        left += sub * 0.707;
        right += sub * 0.707;

        if p.noise > 0.0001 {
            self.noise = self.noise.wrapping_mul(1664525).wrapping_add(1013904223);
            let nse = (self.noise as i32 as f32) * (1.0 / 2_147_483_648.0) * p.noise;
            left += nse;
            right += nse;
        }

        let amp = self.amp.tick(sample_rate, p.amp_a, p.amp_d, p.amp_s, p.amp_r);
        let fenv = self.filt.tick(sample_rate, p.filt_a, p.filt_d, p.filt_s, p.filt_r);
        if self.amp.stage == Stage::Off {
            self.active = false;
            return (0.0, 0.0);
        }

        let key_oct = (self.note as f32 - 60.0) / 12.0 * p.keytrack;
        let env_oct = fenv * p.filt_env * 8.0;
        let lfo_oct = p.lfo * 4.0;
        let cutoff = p.cutoff * 2f32.powf(key_oct + env_oct + lfo_oct);
        let mono = (left + right) * 0.5;
        let filtered = self.filter.process_lp(
            drive(mono, p.drive),
            cutoff,
            p.res,
            sample_rate,
        );
        let g = amp * self.velocity * 0.35;
        let wet = filtered * g;
        // Keep a touch of stereo from unison by using the pre-filter balance.
        let sum = (left.abs() + right.abs()).max(1e-6);
        (wet * (left.abs() / sum * 2.0).min(1.4), wet * (right.abs() / sum * 2.0).min(1.4))
    }
}

pub struct Engine {
    pub voices: [Voice; NUM_VOICES],
    next_age: u64,
    last_hz: f32,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            voices: [Voice::default(); NUM_VOICES],
            next_age: 1,
            last_hz: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.voices = [Voice::default(); NUM_VOICES];
        self.next_age = 1;
        self.last_hz = 0.0;
    }

    pub fn note_on(&mut self, note: u8, channel: u8, velocity: f32, sample_rate: f32) {
        let idx = self.alloc();
        let from = if self.last_hz > 1.0 {
            self.last_hz
        } else {
            osc::midi_hz(note as f32)
        };
        self.voices[idx].start(note, channel, velocity, self.next_age, from, sample_rate);
        self.next_age += 1;
        self.last_hz = osc::midi_hz(note as f32);
    }

    pub fn note_off(&mut self, note: u8, channel: u8) {
        for v in self.voices.iter_mut() {
            if v.active && v.note == note && v.channel == channel && v.amp.stage != Stage::Release {
                v.release();
            }
        }
    }

    #[allow(dead_code)]
    pub fn all_notes_off(&mut self) {
        for v in self.voices.iter_mut() {
            if v.active {
                v.release();
            }
        }
    }

    fn alloc(&mut self) -> usize {
        for (i, v) in self.voices.iter().enumerate() {
            if !v.active {
                return i;
            }
        }
        let mut best = 0;
        let mut best_score = f32::MAX;
        for (i, v) in self.voices.iter().enumerate() {
            let releasing = if v.amp.stage == Stage::Release { 0.0 } else { 10.0 };
            let score = releasing + v.amp.value - v.age as f32 * 1e-6;
            if score < best_score {
                best_score = score;
                best = i;
            }
        }
        self.voices[best].kill();
        best
    }
}
