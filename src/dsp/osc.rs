//! Band-limited analog-style oscillators (polyBLEP).

use std::f32::consts::TAU;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wave {
    Saw,
    Square,
    Triangle,
    Sine,
}

impl Wave {
    #[allow(dead_code)]
    pub fn from_index(i: i32) -> Self {
        match i {
            1 => Self::Square,
            2 => Self::Triangle,
            3 => Self::Sine,
            _ => Self::Saw,
        }
    }

    #[allow(dead_code)]
    pub fn to_index(self) -> i32 {
        match self {
            Self::Saw => 0,
            Self::Square => 1,
            Self::Triangle => 2,
            Self::Sine => 3,
        }
    }
}

#[inline]
fn wrap01(x: f32) -> f32 {
    x - x.floor()
}

#[inline]
fn polyblep(t: f32, dt: f32) -> f32 {
    if dt <= 0.0 {
        return 0.0;
    }
    if t < dt {
        let x = t / dt;
        x + x - x * x - 1.0
    } else if t > 1.0 - dt {
        let x = (t - 1.0) / dt;
        x * x + x + x + 1.0
    } else {
        0.0
    }
}

pub fn tick(phase: &mut f32, freq: f32, sample_rate: f32) -> (f32, bool) {
    let dt = (freq / sample_rate).clamp(0.0, 0.499);
    *phase += dt;
    let wrapped = *phase >= 1.0;
    if wrapped {
        *phase -= 1.0;
    }
    (dt, wrapped)
}

pub fn render(wave: Wave, phase: f32, dt: f32, pwm: f32) -> f32 {
    match wave {
        Wave::Saw => {
            let mut y = 2.0 * phase - 1.0;
            y -= polyblep(phase, dt);
            y
        }
        Wave::Square => {
            let pw = pwm.clamp(0.05, 0.95);
            let mut y = if phase < pw { 1.0 } else { -1.0 };
            y += polyblep(phase, dt);
            y -= polyblep(wrap01(phase - pw), dt);
            y * 0.8
        }
        Wave::Triangle => {
            let mut t = 2.0 * phase - 1.0;
            t = t.abs() * 2.0 - 1.0;
            t
        }
        Wave::Sine => (phase * TAU).sin(),
    }
}

pub fn midi_hz(note: f32) -> f32 {
    440.0 * 2f32.powf((note - 69.0) / 12.0)
}
