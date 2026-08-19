//! Cheap stereo chorus after the voice sum.

const MAX_DELAY: usize = 8192;

pub struct Chorus {
    buf_l: Vec<f32>,
    buf_r: Vec<f32>,
    w: usize,
    phase: f32,
    sample_rate: f32,
}

impl Chorus {
    pub fn new() -> Self {
        Self {
            buf_l: vec![0.0; MAX_DELAY],
            buf_r: vec![0.0; MAX_DELAY],
            w: 0,
            phase: 0.0,
            sample_rate: 48_000.0,
        }
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr.max(1.0);
    }

    pub fn reset(&mut self) {
        self.buf_l.fill(0.0);
        self.buf_r.fill(0.0);
        self.w = 0;
        self.phase = 0.0;
    }

    pub fn process(&mut self, l: f32, r: f32, mix: f32, rate_hz: f32, depth: f32) -> (f32, f32) {
        let mix = mix.clamp(0.0, 1.0);
        if mix <= 0.0001 {
            self.buf_l[self.w] = l;
            self.buf_r[self.w] = r;
            self.w = (self.w + 1) % MAX_DELAY;
            return (l, r);
        }

        let sr = self.sample_rate;
        self.phase += rate_hz.max(0.01) / sr;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        let lfo = (self.phase * std::f32::consts::TAU).sin();
        let lfo2 = ((self.phase + 0.25) * std::f32::consts::TAU).sin();

        let base_ms = 18.0;
        let mod_ms = 6.0 * depth.clamp(0.0, 1.0);
        let delay_l = ((base_ms + mod_ms * lfo) * 0.001 * sr).clamp(1.0, (MAX_DELAY - 4) as f32);
        let delay_r = ((base_ms + mod_ms * lfo2) * 0.001 * sr).clamp(1.0, (MAX_DELAY - 4) as f32);

        self.buf_l[self.w] = l;
        self.buf_r[self.w] = r;

        let wet_l = read(&self.buf_l, self.w, delay_l);
        let wet_r = read(&self.buf_r, self.w, delay_r);
        self.w = (self.w + 1) % MAX_DELAY;

        (
            l * (1.0 - mix) + wet_l * mix,
            r * (1.0 - mix) + wet_r * mix,
        )
    }
}

fn read(buf: &[f32], w: usize, delay: f32) -> f32 {
    let n = buf.len();
    let pos = w as f32 - delay;
    let pos = if pos < 0.0 { pos + n as f32 } else { pos };
    let i = pos.floor() as usize % n;
    let f = pos.fract();
    let j = (i + 1) % n;
    buf[i] + (buf[j] - buf[i]) * f
}
