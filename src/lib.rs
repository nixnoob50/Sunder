mod dsp;
mod editor;
mod params;
mod presets;

use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, resizable_window::ResizableWindow, EguiState};

use crate::dsp::chorus::Chorus;
use crate::dsp::voice::{Engine, VoiceParams};
use crate::params::SunderParams;

struct Sunder {
    params: Arc<SunderParams>,
    engine: Engine,
    chorus: Chorus,
    sample_rate: f32,
    lfo_phase: f32,
}

impl Default for Sunder {
    fn default() -> Self {
        Self {
            params: Arc::new(SunderParams::default()),
            engine: Engine::new(),
            chorus: Chorus::new(),
            sample_rate: 48_000.0,
            lfo_phase: 0.0,
        }
    }
}

impl Plugin for Sunder {
    const NAME: &'static str = "Sunder";
    const VENDOR: &'static str = "Sunder";
    const URL: &'static str = "https://github.com";
    const EMAIL: &'static str = "none";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        let egui_state: Arc<EguiState> = params.editor_state.clone();
        create_egui_editor(
            params.editor_state.clone(),
            editor::GuiState::default(),
            |ctx, _| {
                let mut visuals = egui::Visuals::dark();
                visuals.panel_fill = egui::Color32::from_rgb(18, 17, 16);
                visuals.window_fill = egui::Color32::from_rgb(18, 17, 16);
                visuals.extreme_bg_color = egui::Color32::from_rgb(14, 13, 12);
                ctx.set_visuals(visuals);
            },
            move |egui_ctx, setter, state| {
                ResizableWindow::new("sunder-window")
                    .min_size(egui::vec2(520.0, 420.0))
                    .show(egui_ctx, egui_state.as_ref(), |ui| {
                        ui.visuals_mut().panel_fill = egui::Color32::from_rgb(18, 17, 16);
                        editor::draw(ui, &params, setter, state);
                    });
            },
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        self.chorus.set_sample_rate(self.sample_rate);
        flush_denormals();
        true
    }

    fn reset(&mut self) {
        self.engine.reset();
        self.chorus.reset();
        self.lfo_phase = 0.0;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        flush_denormals();
        let sr = self.sample_rate;
        let mut next_event = context.next_event();
        let num_samples = buffer.samples();
        let output = buffer.as_slice();
        let channels = output.len();

        for sample_id in 0..num_samples {
            while let Some(event) = next_event {
                if event.timing() as usize > sample_id {
                    break;
                }
                match event {
                    NoteEvent::NoteOn {
                        note,
                        channel,
                        velocity,
                        ..
                    } => {
                        self.engine.note_on(
                            note,
                            channel,
                            velocity,
                            sr,
                            self.params.legato.value(),
                        );
                    }
                    NoteEvent::NoteOff { note, channel, .. } => {
                        self.engine.note_off(note, channel);
                    }
                    NoteEvent::Choke { note, channel, .. } => {
                        self.engine.note_off(note, channel);
                    }
                    _ => {}
                }
                next_event = context.next_event();
            }

            self.lfo_phase += self.params.lfo_rate.smoothed.next() / sr;
            if self.lfo_phase >= 1.0 {
                self.lfo_phase -= 1.0;
            }
            let lfo = (self.lfo_phase * std::f32::consts::TAU).sin();

            let vp = VoiceParams {
                osc1_wave: self.params.osc1_wave.value().to_wave(),
                osc1_mix: self.params.osc1_mix.smoothed.next(),
                osc1_oct: self.params.osc1_oct.value(),
                osc1_semi: self.params.osc1_semi.value(),
                osc1_pwm: self.params.osc1_pwm.smoothed.next(),
                unison: self.params.unison.value().clamp(1, 5) as usize,
                detune_cents: self.params.detune.value(),
                stereo: self.params.stereo.smoothed.next(),
                osc2_wave: self.params.osc2_wave.value().to_wave(),
                osc2_mix: self.params.osc2_mix.smoothed.next(),
                osc2_oct: self.params.osc2_oct.value(),
                osc2_semi: self.params.osc2_semi.value(),
                osc2_cents: self.params.osc2_cents.smoothed.next(),
                osc2_pwm: self.params.osc2_pwm.smoothed.next(),
                sync: self.params.sync.value(),
                sub_mix: self.params.sub_mix.smoothed.next(),
                sub_square: self.params.sub_square.value(),
                noise: self.params.noise.smoothed.next(),
                noise_kind: self.params.noise_type.value().to_kind(),
                cutoff: self.params.cutoff.smoothed.next(),
                res: self.params.res.smoothed.next(),
                drive: self.params.drive.smoothed.next(),
                filt_env: self.params.filt_env.value(),
                keytrack: self.params.keytrack.smoothed.next(),
                filt_mode: self.params.filt_mode.value().to_mode(),
                four_pole: self.params.four_pole.value(),
                amp_a: self.params.amp_a.value(),
                amp_d: self.params.amp_d.value(),
                amp_s: self.params.amp_s.smoothed.next(),
                amp_r: self.params.amp_r.value(),
                filt_a: self.params.filt_a.value(),
                filt_d: self.params.filt_d.value(),
                filt_s: self.params.filt_s.smoothed.next(),
                filt_r: self.params.filt_r.value(),
                lfo,
                lfo_cut: self.params.lfo_amt.smoothed.next(),
                lfo_pitch: self.params.lfo_pitch.smoothed.next(),
                lfo_pwm: self.params.lfo_pwm.smoothed.next(),
                glide_ms: self.params.glide.value(),
            };

            let mut mix_l = 0.0;
            let mut mix_r = 0.0;
            for voice in self.engine.voices.iter_mut() {
                if voice.active {
                    let (l, r) = voice.render(&vp, sr);
                    mix_l += l;
                    mix_r += r;
                }
            }

            let gain = self.params.gain.smoothed.next();
            mix_l *= gain;
            mix_r *= gain;
            let (l, r) = self.chorus.process(
                mix_l,
                mix_r,
                self.params.cho_mix.smoothed.next(),
                self.params.cho_rate.value(),
                self.params.cho_depth.smoothed.next(),
            );

            if channels >= 2 {
                output[0][sample_id] = l;
                output[1][sample_id] = r;
            } else if channels == 1 {
                output[0][sample_id] = 0.5 * (l + r);
            }
        }

        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for Sunder {
    const CLAP_ID: &'static str = "com.sunder.va";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("Virtual analog synthesizer: fat leads, scratchy leads, bass, pads");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
    ];
}

nih_export_clap!(Sunder);

#[allow(deprecated)]
fn flush_denormals() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::{_mm_getcsr, _mm_setcsr};
        #[cfg(target_arch = "x86")]
        use std::arch::x86::{_mm_getcsr, _mm_setcsr};
        _mm_setcsr(_mm_getcsr() | 0x8040);
    }
}
