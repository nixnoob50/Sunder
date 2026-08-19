//! Linear trapezoidal state-variable lowpass with optional drive.

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

    pub fn process_lp(&mut self, input: f32, cutoff_hz: f32, res: f32, sample_rate: f32) -> f32 {
        let nyquist = sample_rate * 0.49;
        let cutoff = cutoff_hz.clamp(20.0, nyquist);
        let g = (std::f32::consts::PI * cutoff / sample_rate).tan();
        let k = 2.0 - 2.0 * res.clamp(0.0, 0.97);
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;

        let v3 = input - self.ic2eq;
        let v1 = a1 * self.ic1eq + a2 * v3;
        let v2 = self.ic2eq + a2 * self.ic1eq + a3 * v3;
        self.ic1eq = 2.0 * v1 - self.ic1eq;
        self.ic2eq = 2.0 * v2 - self.ic2eq;

        if self.ic1eq.abs() < 1e-20 {
            self.ic1eq = 0.0;
        }
        if self.ic2eq.abs() < 1e-20 {
            self.ic2eq = 0.0;
        }
        v2
    }
}

#[inline]
pub fn drive(x: f32, amount: f32) -> f32 {
    let g = 1.0 + amount * 6.0;
    (x * g).tanh()
}
