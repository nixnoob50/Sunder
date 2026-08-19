//! Analog-style noise: white, pink, brown, and Roland-ish digital LFSR.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoiseKind {
    White,
    Pink,
    Brown,
    Digital,
}

#[derive(Clone, Copy, Debug)]
pub struct Noise {
    lfsr: u32,
    pink_b0: f32,
    pink_b1: f32,
    pink_b2: f32,
    pink_b3: f32,
    pink_b4: f32,
    pink_b5: f32,
    pink_b6: f32,
    brown: f32,
}

impl Default for Noise {
    fn default() -> Self {
        Self {
            lfsr: 0xACE1u32,
            pink_b0: 0.0,
            pink_b1: 0.0,
            pink_b2: 0.0,
            pink_b3: 0.0,
            pink_b4: 0.0,
            pink_b5: 0.0,
            pink_b6: 0.0,
            brown: 0.0,
        }
    }
}

impl Noise {
    pub fn seed(&mut self, seed: u32) {
        self.lfsr = seed | 1;
        self.pink_b0 = 0.0;
        self.pink_b1 = 0.0;
        self.pink_b2 = 0.0;
        self.pink_b3 = 0.0;
        self.pink_b4 = 0.0;
        self.pink_b5 = 0.0;
        self.pink_b6 = 0.0;
        self.brown = 0.0;
    }

    pub fn next(&mut self, kind: NoiseKind) -> f32 {
        match kind {
            NoiseKind::White => self.white(),
            NoiseKind::Pink => self.pink(),
            NoiseKind::Brown => self.brown(),
            NoiseKind::Digital => self.digital(),
        }
    }

    fn step_lfsr(&mut self) -> u32 {
        let bit = ((self.lfsr >> 0) ^ (self.lfsr >> 2) ^ (self.lfsr >> 3) ^ (self.lfsr >> 5)) & 1;
        self.lfsr = (self.lfsr >> 1) | (bit << 15);
        self.lfsr
    }

    fn white(&mut self) -> f32 {
        self.step_lfsr();
        (self.lfsr as i16 as f32) * (1.0 / 32768.0)
    }

    /// Paul Kellet economy pink.
    fn pink(&mut self) -> f32 {
        let w = self.white();
        self.pink_b0 = 0.99886 * self.pink_b0 + w * 0.0555179;
        self.pink_b1 = 0.99332 * self.pink_b1 + w * 0.0750759;
        self.pink_b2 = 0.96900 * self.pink_b2 + w * 0.1538520;
        self.pink_b3 = 0.86650 * self.pink_b3 + w * 0.3104856;
        self.pink_b4 = 0.55000 * self.pink_b4 + w * 0.5329522;
        self.pink_b5 = -0.7616 * self.pink_b5 - w * 0.0168980;
        let y = self.pink_b0
            + self.pink_b1
            + self.pink_b2
            + self.pink_b3
            + self.pink_b4
            + self.pink_b5
            + self.pink_b6
            + w * 0.5362;
        self.pink_b6 = w * 0.115926;
        (y * 0.22).clamp(-1.0, 1.0)
    }

    /// Leaky integrator of white (~12 dB/oct). Makeup so the noise knob
    /// matches white/digital instead of disappearing under the oscillators.
    fn brown(&mut self) -> f32 {
        let w = self.white();
        self.brown = (self.brown + w * 0.08) * 0.985;
        (self.brown * 2.4).clamp(-1.0, 1.0)
    }

    /// 16-bit LFSR bitstream, closer to Roland analog digital-noise.
    fn digital(&mut self) -> f32 {
        self.step_lfsr();
        if (self.lfsr & 1) != 0 {
            0.7
        } else {
            -0.7
        }
    }
}
