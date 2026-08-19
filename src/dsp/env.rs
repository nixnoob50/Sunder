//! ADSR envelope.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Off,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Clone, Copy, Debug)]
pub struct Adsr {
    pub stage: Stage,
    pub value: f32,
}

impl Default for Adsr {
    fn default() -> Self {
        Self {
            stage: Stage::Off,
            value: 0.0,
        }
    }
}

impl Adsr {
    pub fn note_on(&mut self) {
        self.stage = Stage::Attack;
        if self.value <= 0.0 {
            self.value = 0.0;
        }
    }

    pub fn note_off(&mut self) {
        if self.stage != Stage::Off {
            self.stage = Stage::Release;
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    #[allow(dead_code)]
    pub fn is_active(&self) -> bool {
        self.stage != Stage::Off
    }

    pub fn tick(
        &mut self,
        sample_rate: f32,
        attack_ms: f32,
        decay_ms: f32,
        sustain: f32,
        release_ms: f32,
    ) -> f32 {
        let sr = sample_rate.max(1.0);
        match self.stage {
            Stage::Off => self.value = 0.0,
            Stage::Attack => {
                let step = 1.0 / (ms_to_samples(attack_ms, sr).max(1.0));
                self.value += step;
                if self.value >= 1.0 {
                    self.value = 1.0;
                    self.stage = Stage::Decay;
                }
            }
            Stage::Decay => {
                let sus = sustain.clamp(0.0, 1.0);
                let step = 1.0 / (ms_to_samples(decay_ms, sr).max(1.0));
                self.value -= step;
                if self.value <= sus {
                    self.value = sus;
                    self.stage = if sus <= 0.0001 {
                        Stage::Off
                    } else {
                        Stage::Sustain
                    };
                }
            }
            Stage::Sustain => self.value = sustain.clamp(0.0, 1.0),
            Stage::Release => {
                let step = 1.0 / (ms_to_samples(release_ms, sr).max(1.0));
                self.value -= step;
                if self.value <= 0.0 {
                    self.value = 0.0;
                    self.stage = Stage::Off;
                }
            }
        }
        self.value
    }
}

fn ms_to_samples(ms: f32, sr: f32) -> f32 {
    (ms.max(0.2) / 1000.0) * sr
}
