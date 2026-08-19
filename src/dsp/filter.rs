//! Linear trapezoidal state-variable filter with LP / BP / HP taps.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilterMode {
    Lowpass,
    Bandpass,
    Highpass,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Svf {
    ic1eq: f32,
    ic2eq: f32,
}

impl Svf {
    pub fn reset(&mut self) {
        self.ic1eq = 0.0;
        self.ic2eq = 0.0;
    }

    /// Returns `(low, band, high)`.
    pub fn process(&mut self, input: f32, cutoff_hz: f32, res: f32, sample_rate: f32) -> (f32, f32, f32) {
        let sr = sample_rate.max(1.0);
        // Keep g = tan(pi f/sr) well away from the pole at Nyquist. Filter-env
        // can push cutoff to MHz; HP + high res there becomes a Nyquist oscillator.
        let cutoff = cutoff_hz.clamp(20.0, sr * 0.38);
        let g = (std::f32::consts::PI * cutoff / sr).tan().min(2.5);
        // High g + tiny k (high res) is what blew up HP. Keep a g-dependent floor on k.
        let k = (2.0 - 2.0 * res.clamp(0.0, 0.93)).max(0.1 + 0.2 * g);
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;

        let v3 = input - self.ic2eq;
        let v1 = a1 * self.ic1eq + a2 * v3;
        let v2 = self.ic2eq + a2 * self.ic1eq + a3 * v3;
        self.ic1eq = (2.0 * v1 - self.ic1eq).clamp(-8.0, 8.0);
        self.ic2eq = (2.0 * v2 - self.ic2eq).clamp(-8.0, 8.0);

        if !self.ic1eq.is_finite() || !self.ic2eq.is_finite() {
            self.reset();
            return (0.0, 0.0, 0.0);
        }
        if self.ic1eq.abs() < 1e-20 {
            self.ic1eq = 0.0;
        }
        if self.ic2eq.abs() < 1e-20 {
            self.ic2eq = 0.0;
        }

        let lp = v2.clamp(-4.0, 4.0);
        let bp = v1.clamp(-4.0, 4.0);
        let hp = (input - k * v1 - v2).clamp(-4.0, 4.0);
        (lp, bp, hp)
    }

    pub fn pick(lp: f32, bp: f32, hp: f32, mode: FilterMode) -> f32 {
        match mode {
            FilterMode::Lowpass => lp,
            FilterMode::Bandpass => bp,
            FilterMode::Highpass => hp,
        }
    }
}

#[inline]
pub fn drive(x: f32, amount: f32) -> f32 {
    let g = 1.0 + amount * 6.0;
    (x * g).tanh()
}
