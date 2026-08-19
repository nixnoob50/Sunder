use nih_plug::prelude::{Param, ParamSetter};
use nih_plug_egui::egui::{
    self, epaint::PathStroke, pos2, vec2, Color32, CornerRadius, FontId, Pos2, Rect, RichText,
    Sense, Stroke, StrokeKind, Ui,
};
use std::f32::consts::PI;
use std::sync::Arc;

use crate::params::{SunderParams, WaveChoice};
use crate::presets::{self, Category, Preset};

const BG: Color32 = Color32::from_rgb(18, 17, 16);
const PANEL: Color32 = Color32::from_rgb(32, 30, 28);
const PANEL_EDGE: Color32 = Color32::from_rgb(58, 52, 46);
const INSET: Color32 = Color32::from_rgb(14, 13, 12);
const AMBER: Color32 = Color32::from_rgb(232, 156, 64);
const AMBER_DIM: Color32 = Color32::from_rgb(168, 104, 42);
const CREAM: Color32 = Color32::from_rgb(228, 220, 204);
const MUTED: Color32 = Color32::from_rgb(138, 128, 114);
const LCD: Color32 = Color32::from_rgb(168, 214, 140);
const LCD_BG: Color32 = Color32::from_rgb(12, 18, 12);
const KNOB_METAL: Color32 = Color32::from_rgb(48, 46, 44);
const KNOB_TOP: Color32 = Color32::from_rgb(72, 68, 64);

pub struct GuiState {
    category: Category,
    selected: usize,
    save_name: String,
    status: String,
    user: Vec<Preset>,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            category: Category::Bass,
            selected: 0,
            save_name: String::new(),
            status: String::new(),
            user: presets::load_user_presets(),
        }
    }
}

pub fn draw(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter, state: &mut GuiState) {
    ui.visuals_mut().override_text_color = Some(CREAM);
    ui.visuals_mut().extreme_bg_color = INSET;
    ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(40, 38, 36);
    ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(58, 50, 42);
    ui.visuals_mut().widgets.active.bg_fill = AMBER_DIM;
    ui.visuals_mut().selection.bg_fill = AMBER.linear_multiply(0.35);
    ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0, MUTED);
    ui.spacing_mut().item_spacing = vec2(8.0, 8.0);

    header(ui, params, setter);
    ui.add_space(4.0);

    let body = ui.available_rect_before_wrap();
    let preset_w = 176.0;
    let mods_w = (body.width() - preset_w - 8.0).max(200.0);
    let body_h = body.height();

    ui.horizontal_top(|ui| {
        ui.set_max_width(body.width());

        ui.vertical(|ui| {
            ui.set_width(preset_w);
            ui.set_max_width(preset_w);
            panel(ui, "PRESETS", |ui| {
                preset_browser(
                    ui,
                    params,
                    setter,
                    state,
                    (body_h - 40.0).clamp(120.0, 520.0),
                );
            });
        });

        ui.vertical(|ui| {
            ui.set_width(mods_w);
            ui.set_max_width(mods_w);
            egui::ScrollArea::vertical()
                .id_salt("sunder_mods")
                .max_width(mods_w)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.set_width(mods_w);
                    ui.set_max_width(mods_w);
                    let two_col = mods_w >= 560.0;
                    let col_w = if two_col { (mods_w - 10.0) * 0.5 } else { mods_w };
                    let ks = ((col_w - 24.0) / 4.2).clamp(28.0, 44.0);

                    if two_col {
                        ui.columns(2, |cols| {
                            osc1_panel(&mut cols[0], params, setter, ks);
                            osc2_panel(&mut cols[1], params, setter, ks);
                        });
                        ui.columns(2, |cols| {
                            filter_panel(&mut cols[0], params, setter, ks);
                            unison_panel(&mut cols[1], params, setter, ks);
                        });
                        ui.columns(2, |cols| {
                            env_panel(&mut cols[0], params, setter, ks);
                            fx_panel(&mut cols[1], params, setter, ks);
                        });
                    } else {
                        osc1_panel(ui, params, setter, ks);
                        osc2_panel(ui, params, setter, ks);
                        filter_panel(ui, params, setter, ks);
                        unison_panel(ui, params, setter, ks);
                        env_panel(ui, params, setter, ks);
                        fx_panel(ui, params, setter, ks);
                    }
                });
        });
    });
}

fn osc1_panel(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter, ks: f32) {
    panel(ui, "OSCILLATOR 1", |ui| {
        wave_picker(ui, setter, &params.osc1_wave);
        ui.add_space(4.0);
        knob_row(ui, |ui| {
            knob(ui, setter, &params.osc1_mix, "MIX", ks);
            knob(ui, setter, &params.osc1_pwm, "PWM", ks);
            knob(ui, setter, &params.osc1_oct, "OCT", ks);
            knob(ui, setter, &params.osc1_semi, "SEMI", ks);
        });
    });
}

fn osc2_panel(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter, ks: f32) {
    panel(ui, "OSCILLATOR 2", |ui| {
        ui.horizontal(|ui| {
            wave_picker(ui, setter, &params.osc2_wave);
            latch(ui, setter, &params.sync, "SYNC");
        });
        knob_row(ui, |ui| {
            knob(ui, setter, &params.osc2_mix, "MIX", ks);
            knob(ui, setter, &params.osc2_pwm, "PWM", ks);
            knob(ui, setter, &params.osc2_oct, "OCT", ks);
            knob(ui, setter, &params.osc2_semi, "SEMI", ks);
        });
    });
}

fn filter_panel(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter, ks: f32) {
    panel(ui, "FILTER", |ui| {
        knob_row(ui, |ui| {
            knob(ui, setter, &params.cutoff, "CUTOFF", ks + 4.0);
            knob(ui, setter, &params.res, "RES", ks + 4.0);
            knob(ui, setter, &params.drive, "DRIVE", ks + 4.0);
        });
        knob_row(ui, |ui| {
            knob(ui, setter, &params.filt_env, "ENV", ks);
            knob(ui, setter, &params.keytrack, "KEY", ks);
        });
    });
}

fn unison_panel(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter, ks: f32) {
    panel(ui, "UNISON / SUB", |ui| {
        knob_row(ui, |ui| {
            knob(ui, setter, &params.unison, "VOICES", ks);
            knob(ui, setter, &params.detune, "DETUNE", ks);
            knob(ui, setter, &params.stereo, "WIDTH", ks);
        });
        knob_row(ui, |ui| {
            knob(ui, setter, &params.sub_mix, "SUB", ks);
            knob(ui, setter, &params.noise, "NOISE", ks);
            latch(ui, setter, &params.sub_square, "SQ SUB");
        });
    });
}

fn env_panel(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter, ks: f32) {
    let k = ks - 2.0;
    panel(ui, "ENVELOPES", |ui| {
        ui.label(RichText::new("AMP").small().color(MUTED));
        knob_row(ui, |ui| {
            knob(ui, setter, &params.amp_a, "A", k);
            knob(ui, setter, &params.amp_d, "D", k);
            knob(ui, setter, &params.amp_s, "S", k);
            knob(ui, setter, &params.amp_r, "R", k);
        });
        ui.label(RichText::new("FILTER").small().color(MUTED));
        knob_row(ui, |ui| {
            knob(ui, setter, &params.filt_a, "A", k);
            knob(ui, setter, &params.filt_d, "D", k);
            knob(ui, setter, &params.filt_s, "S", k);
            knob(ui, setter, &params.filt_r, "R", k);
        });
    });
}

fn fx_panel(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter, ks: f32) {
    panel(ui, "MOD / FX", |ui| {
        knob_row(ui, |ui| {
            knob(ui, setter, &params.lfo_rate, "RATE", ks);
            knob(ui, setter, &params.lfo_amt, "LFO", ks);
            knob(ui, setter, &params.glide, "GLIDE", ks);
        });
        knob_row(ui, |ui| {
            knob(ui, setter, &params.cho_mix, "CHORUS", ks);
            knob(ui, setter, &params.cho_rate, "C.RATE", ks);
            knob(ui, setter, &params.cho_depth, "C.DPTH", ks);
        });
    });
}

fn knob_row(ui: &mut Ui, add: impl FnOnce(&mut Ui)) {
    ui.horizontal_wrapped(add);
}

fn header(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter) {
    let w = ui.available_width();
    ui.allocate_ui_with_layout(
        vec2(w, 64.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let rect = ui.max_rect();
            ui.painter()
                .rect_filled(rect, CornerRadius::same(10), Color32::from_rgb(26, 24, 22));
            ui.painter().rect_stroke(
                rect,
                CornerRadius::same(10),
                Stroke::new(1.0, PANEL_EDGE),
                StrokeKind::Inside,
            );
            ui.painter().line_segment(
                [
                    pos2(rect.left() + 16.0, rect.bottom() - 2.0),
                    pos2(rect.right() - 16.0, rect.bottom() - 2.0),
                ],
                Stroke::new(2.0, AMBER.linear_multiply(0.55)),
            );

            ui.add_space(14.0);
            ui.label(
                RichText::new("SUNDER")
                    .font(FontId::proportional(24.0))
                    .color(AMBER)
                    .strong(),
            );
            ui.add_space(10.0);
            ui.label(RichText::new("VIRTUAL ANALOG  ·  CLAP").small().color(MUTED));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                ui.allocate_ui_with_layout(
                    vec2(118.0, 48.0),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        header_gain(ui, setter, &params.gain);
                    },
                );
            });
        },
    );
}

fn header_gain(ui: &mut Ui, setter: &ParamSetter, param: &nih_plug::prelude::FloatParam) {
    let (rect, mut response) = ui.allocate_exact_size(vec2(44.0, 44.0), Sense::click_and_drag());
    let norm = param.modulated_normalized_value();
    interact_knob(ui, setter, param, &mut response, norm);
    if ui.is_rect_visible(rect) {
        paint_knob(ui, rect, norm, response.hovered() || response.dragged());
    }
    ui.add_space(6.0);
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.label(RichText::new("GAIN").small().color(MUTED));
        ui.label(
            RichText::new(param.normalized_value_to_string(norm, true))
                .small()
                .color(CREAM),
        );
    });
}

fn panel(ui: &mut Ui, title: &str, add: impl FnOnce(&mut Ui)) {
    egui::Frame::new()
        .fill(PANEL)
        .stroke(Stroke::new(1.0, PANEL_EDGE))
        .corner_radius(10)
        .inner_margin(egui::Margin {
            left: 10,
            right: 10,
            top: 8,
            bottom: 10,
        })
        .show(ui, |ui| {
            let w = ui.available_width();
            if w.is_finite() && w > 1.0 {
                ui.set_width(w);
            }
            ui.label(RichText::new(title).small().color(AMBER_DIM).strong());
            ui.add_space(4.0);
            add(ui);
        });
}

fn preset_browser(
    ui: &mut Ui,
    params: &Arc<SunderParams>,
    setter: &ParamSetter,
    state: &mut GuiState,
    list_height: f32,
) {
    ui.set_width(ui.available_width());
    ui.spacing_mut().item_spacing = vec2(4.0, 4.0);

    for cat in Category::ALL {
        let on = state.category == cat;
        let chip = egui::Button::new(
            RichText::new(cat.label())
                .small()
                .color(if on { BG } else { CREAM }),
        )
        .fill(if on { AMBER } else { Color32::from_rgb(42, 38, 34) })
        .corner_radius(4)
        .min_size(vec2(ui.available_width(), 22.0));
        if ui.add(chip).clicked() {
            state.category = cat;
            state.selected = 0;
        }
    }
    ui.add_space(4.0);

    let factory = presets::factory_presets();
    let factory_view: Vec<&Preset> = factory
        .iter()
        .filter(|p| p.category == state.category)
        .collect();
    let user_view: Vec<&Preset> = state
        .user
        .iter()
        .filter(|p| p.category == state.category)
        .collect();
    let mut names: Vec<(bool, String)> = Vec::new();
    for p in &factory_view {
        names.push((true, p.name.clone()));
    }
    for p in &user_view {
        names.push((false, p.name.clone()));
    }

    egui::Frame::new()
        .fill(LCD_BG)
        .corner_radius(6)
        .inner_margin(6)
        .stroke(Stroke::new(1.0, Color32::from_rgb(36, 52, 32)))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            egui::ScrollArea::vertical()
                .max_height(list_height)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::Min).with_cross_justify(true),
                        |ui| {
                            for (i, (factory_flag, name)) in names.iter().enumerate() {
                                let selected = state.selected == i;
                                let text = if *factory_flag {
                                    name.clone()
                                } else {
                                    format!("• {name}")
                                };
                                let color = if selected { AMBER } else { LCD };
                                if ui
                                    .selectable_label(
                                        selected,
                                        RichText::new(text).color(color).small().monospace(),
                                    )
                                    .clicked()
                                {
                                    state.selected = i;
                                    let patch = if *factory_flag {
                                        factory
                                            .iter()
                                            .find(|p| p.name == *name)
                                            .map(|p| p.params.clone())
                                    } else {
                                        state
                                            .user
                                            .iter()
                                            .find(|p| p.name == *name)
                                            .map(|p| p.params.clone())
                                    };
                                    if let Some(patch) = patch {
                                        presets::apply(params, setter, &patch);
                                        state.status = format!("Loaded {name}");
                                    }
                                }
                            }
                        },
                    );
                });
        });

    ui.add_space(6.0);
    ui.horizontal(|ui| {
        if amber_button(ui, "SAVE").clicked() {
            let name = if state.save_name.trim().is_empty() {
                "User Patch".to_string()
            } else {
                state.save_name.trim().to_string()
            };
            let preset = Preset {
                name: name.clone(),
                category: state.category,
                params: presets::snapshot(params),
            };
            match presets::save_user_preset(&preset) {
                Ok(_) => {
                    state.user = presets::load_user_presets();
                    state.status = format!("Saved {name}");
                    state.save_name.clear();
                }
                Err(e) => state.status = e,
            }
        }
        if amber_button(ui, "DEL").clicked() {
            if let Some((factory_flag, name)) = names.get(state.selected) {
                if !*factory_flag {
                    match presets::delete_user_preset(name) {
                        Ok(()) => {
                            state.user = presets::load_user_presets();
                            state.selected = 0;
                            state.status = format!("Deleted {name}");
                        }
                        Err(e) => state.status = e,
                    }
                } else {
                    state.status = "Factory is read-only".into();
                }
            }
        }
    });
    ui.add(
        egui::TextEdit::singleline(&mut state.save_name)
            .hint_text("preset name")
            .desired_width(ui.available_width()),
    );
    if !state.status.is_empty() {
        ui.label(RichText::new(&state.status).small().color(LCD));
    }
}

fn amber_button(ui: &mut Ui, text: &str) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(text).small().color(BG).strong())
            .fill(AMBER)
            .corner_radius(4),
    )
}

fn wave_picker(ui: &mut Ui, setter: &ParamSetter, param: &nih_plug::prelude::EnumParam<WaveChoice>) {
    let current = param.value();
    ui.horizontal(|ui| {
        for (wave, label) in [
            (WaveChoice::Saw, "SAW"),
            (WaveChoice::Square, "SQR"),
            (WaveChoice::Triangle, "TRI"),
            (WaveChoice::Sine, "SIN"),
        ] {
            let on = current == wave;
            let btn = egui::Button::new(RichText::new(label).small().color(if on { BG } else { CREAM }))
                .fill(if on { AMBER } else { Color32::from_rgb(44, 40, 36) })
                .corner_radius(3)
                .min_size(vec2(40.0, 22.0));
            if ui.add(btn).clicked() {
                setter.begin_set_parameter(param);
                setter.set_parameter(param, wave);
                setter.end_set_parameter(param);
            }
        }
    });
}

fn latch(ui: &mut Ui, setter: &ParamSetter, param: &nih_plug::prelude::BoolParam, label: &str) {
    let on = param.value();
    ui.vertical(|ui| {
        ui.add_space(10.0);
        let (rect, response) = ui.allocate_exact_size(vec2(52.0, 28.0), Sense::click());
        let painter = ui.painter();
        painter.rect_filled(rect, CornerRadius::same(14), INSET);
        painter.rect_stroke(rect, CornerRadius::same(14), Stroke::new(1.0, PANEL_EDGE), StrokeKind::Inside);
        let knob_x = if on { rect.right() - 14.0 } else { rect.left() + 14.0 };
        painter.circle_filled(
            pos2(knob_x, rect.center().y),
            10.0,
            if on { AMBER } else { Color32::from_rgb(90, 86, 80) },
        );
        ui.label(RichText::new(label).small().color(if on { AMBER } else { MUTED }));
        if response.clicked() {
            setter.begin_set_parameter(param);
            setter.set_parameter(param, !on);
            setter.end_set_parameter(param);
        }
    });
}

fn knob<P: Param>(ui: &mut Ui, setter: &ParamSetter, param: &P, label: &str, size: f32) {
    ui.vertical(|ui| {
        ui.set_width(size + 4.0);
        ui.set_max_width(size + 4.0);
        ui.spacing_mut().item_spacing.y = 1.0;

        let (rect, mut response) = ui.allocate_exact_size(vec2(size, size), Sense::click_and_drag());
        let norm = param.modulated_normalized_value();
        interact_knob(ui, setter, param, &mut response, norm);
        paint_knob(ui, rect, norm, response.hovered() || response.dragged());

        ui.label(
            RichText::new(label)
                .font(FontId::proportional(10.0))
                .color(MUTED),
        );
        ui.label(
            RichText::new(pretty_value(param))
                .font(FontId::proportional(10.0))
                .color(CREAM),
        );
    });
}

fn pretty_value<P: Param>(param: &P) -> String {
    let raw = param.normalized_value_to_string(param.modulated_normalized_value(), true);
    let mut num = String::new();
    let mut unit = String::new();
    let mut seen_unit = false;
    for c in raw.chars() {
        if !seen_unit && (c.is_ascii_digit() || c == '.' || c == '-' || c == '+') {
            num.push(c);
        } else {
            seen_unit = true;
            unit.push(c);
        }
    }
    if let Ok(v) = num.parse::<f32>() {
        let trimmed = if unit.contains("ms") {
            format!("{v:.0}")
        } else if unit.contains("Hz") || unit.contains("kHz") {
            if v >= 100.0 {
                format!("{v:.0}")
            } else {
                format!("{v:.1}")
            }
        } else {
            format!("{v:.2}")
        };
        format!("{trimmed}{}", unit.trim_start())
    } else {
        raw
    }
}

fn interact_knob<P: Param>(
    ui: &Ui,
    setter: &ParamSetter,
    param: &P,
    response: &mut egui::Response,
    current: f32,
) {
    let start_id = response.id.with("start");
    let amt_id = response.id.with("amt");

    if response.drag_started() {
        setter.begin_set_parameter(param);
        ui.memory_mut(|m| {
            m.data.insert_temp(start_id, current);
            m.data.insert_temp(amt_id, 0.0f32);
        });
    }
    if response.dragged() {
        let start = ui.memory(|m| m.data.get_temp::<f32>(start_id)).unwrap_or(current);
        let mut amt = ui.memory(|m| m.data.get_temp::<f32>(amt_id)).unwrap_or(0.0);
        let finer = ui.input(|i| i.modifiers.shift);
        amt += -response.drag_delta().y * if finer { 0.0012 } else { 0.0065 };
        ui.memory_mut(|m| m.data.insert_temp(amt_id, amt));
        let value = param.preview_plain((start + amt).clamp(0.0, 1.0));
        setter.set_parameter(param, value);
        response.mark_changed();
    }
    if response.drag_stopped() {
        setter.end_set_parameter(param);
    }
    if response.double_clicked() || (response.clicked() && ui.input(|i| i.modifiers.command)) {
        setter.begin_set_parameter(param);
        setter.set_parameter(param, param.default_plain_value());
        setter.end_set_parameter(param);
        response.mark_changed();
    }
    if response.hovered() {
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll.abs() > 0.1 {
            setter.begin_set_parameter(param);
            let next = if scroll > 0.0 {
                param.next_step(param.modulated_plain_value(), false)
            } else {
                param.previous_step(param.modulated_plain_value(), false)
            };
            setter.set_parameter(param, next);
            setter.end_set_parameter(param);
        }
    }
}

fn paint_knob(ui: &Ui, rect: Rect, norm: f32, hot: bool) {
    let painter = ui.painter();
    let center = rect.center();
    let r = rect.width() * 0.42;
    painter.circle_filled(center + vec2(0.0, 2.5), r + 3.0, Color32::from_black_alpha(90));
    painter.circle_filled(center, r + 3.0, Color32::from_rgb(22, 20, 18));
    painter.circle_stroke(
        center,
        r + 3.0,
        Stroke::new(1.0, if hot { AMBER_DIM } else { PANEL_EDGE }),
    );

    let start = 0.75 * PI;
    let sweep = 1.5 * PI;
    draw_arc(painter, center, r + 1.0, start, start + sweep, Stroke::new(3.0, Color32::from_rgb(28, 26, 24)));
    draw_arc(
        painter,
        center,
        r + 1.0,
        start,
        start + sweep * norm.clamp(0.0, 1.0),
        Stroke::new(3.0, if hot { AMBER } else { AMBER_DIM }),
    );

    painter.circle_filled(center, r - 1.0, KNOB_METAL);
    painter.circle_filled(center - vec2(r * 0.18, r * 0.22), r * 0.55, KNOB_TOP);
    painter.circle_stroke(center, r - 1.0, Stroke::new(1.0, Color32::from_rgb(92, 86, 78)));

    let angle = start + sweep * norm.clamp(0.0, 1.0);
    let inner = center + vec2(angle.cos(), angle.sin()) * (r * 0.18);
    let tip = center + vec2(angle.cos(), angle.sin()) * (r * 0.72);
    painter.line_segment([inner, tip], Stroke::new(2.2, AMBER));
    painter.circle_filled(center, 2.4, AMBER);
}

fn draw_arc(
    painter: &egui::Painter,
    center: Pos2,
    radius: f32,
    start: f32,
    end: f32,
    stroke: Stroke,
) {
    if end <= start + 0.001 {
        return;
    }
    let n = ((end - start).abs() * 24.0).ceil().max(2.0) as usize;
    let mut pts = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let a = start + (end - start) * t;
        pts.push(center + vec2(a.cos(), a.sin()) * radius);
    }
    painter.add(egui::Shape::line(pts, PathStroke::from(stroke)));
}
