use nih_plug::prelude::{Param, ParamSetter};
use nih_plug_egui::egui::{
    self, epaint::PathStroke, pos2, vec2, Align2, Color32, CornerRadius, FontId, Pos2, Rect,
    RichText, Sense, Stroke, StrokeKind, Ui,
};
use std::f32::consts::PI;
use std::sync::Arc;
use std::time::Duration;

use crate::params::{FilterChoice, NoiseChoice, SunderParams, WaveChoice};
use crate::presets::{self, Category, Preset, Ratings};

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
/// Name + 5 stars on one row. Modules use leftover width only.
const PRESET_COL_W: f32 = 252.0;
const STAR_PX: f32 = 12.0;
const STAR_GAP: f32 = 2.0;

pub struct GuiState {
    category: Category,
    /// Virtual Favorites view — not a `Category`. When true, the list is starred
    /// patches (all banks) sorted by rating then name.
    favorites: bool,
    selected: usize,
    /// Name of the last loaded or saved patch (shown in the title bar).
    loaded_name: String,
    save_name: String,
    status: String,
    user: Vec<Preset>,
    ratings: Ratings,
    ratings_gen: u64,
    /// Frozen Favorites order while rating. `None` when not in that view.
    fav_order: Option<Vec<(bool, String)>>,
    /// Show Refresh after ratings change in Favorites (list stays put until then).
    fav_dirty: bool,
}

impl Default for GuiState {
    fn default() -> Self {
        let (ratings, ratings_gen) = presets::snapshot_ratings();
        Self {
            category: Category::Bass,
            favorites: false,
            selected: 0,
            loaded_name: String::new(),
            save_name: String::new(),
            status: String::new(),
            user: presets::load_user_presets(),
            ratings,
            ratings_gen,
            fav_order: None,
            fav_dirty: false,
        }
    }
}

pub fn draw(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter, state: &mut GuiState) {
    let gen_before = state.ratings_gen;
    presets::sync_ratings(&mut state.ratings_gen, &mut state.ratings);
    if state.favorites && state.ratings_gen != gen_before {
        state.fav_dirty = true;
    }
    ui.ctx().request_repaint_after(Duration::from_millis(250));

    ui.visuals_mut().override_text_color = Some(CREAM);
    ui.visuals_mut().extreme_bg_color = INSET;
    ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(40, 38, 36);
    ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(58, 50, 42);
    ui.visuals_mut().widgets.active.bg_fill = AMBER_DIM;
    ui.visuals_mut().selection.bg_fill = AMBER.linear_multiply(0.35);
    ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0, MUTED);
    ui.spacing_mut().item_spacing = vec2(4.0, 4.0);

    header(ui, params, setter, &state.loaded_name);
    ui.add_space(2.0);

    let body = ui.available_rect_before_wrap();
    let preset_w = PRESET_COL_W;
    let mods_w = (body.width() - preset_w - 6.0).max(200.0);
    let body_h = body.height();

    ui.horizontal_top(|ui| {
        ui.set_max_width(body.width());
        ui.spacing_mut().item_spacing = vec2(6.0, 4.0);

        // Exact body height so the presets panel cannot grow past the window and clip Save/Del.
        ui.allocate_ui_with_layout(
            vec2(preset_w, body_h),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_width(preset_w);
                ui.set_max_width(preset_w);
                ui.shrink_clip_rect(ui.max_rect());
                presets_panel(ui, params, setter, state);
            },
        );

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
                    ui.spacing_mut().item_spacing = vec2(4.0, 4.0);
                    let two_col = mods_w >= 520.0;
                    let gap = 6.0;
                    let col_w = if two_col {
                        ((mods_w - gap) * 0.5).max(160.0)
                    } else {
                        mods_w
                    };
                    // Size knobs to fill ~4.5 slots so short rows do not leave a big empty right side.
                    let ks = ((col_w - 14.0) / 4.5).clamp(28.0, 48.0);

                    if two_col {
                        module_pair(ui, gap, col_w, |ui| {
                            osc1_panel(ui, params, setter, ks);
                        }, |ui| {
                            osc2_panel(ui, params, setter, ks);
                        });
                        module_pair(ui, gap, col_w, |ui| {
                            filter_panel(ui, params, setter, ks);
                        }, |ui| {
                            unison_panel(ui, params, setter, ks);
                        });
                        module_pair(ui, gap, col_w, |ui| {
                            env_panel(ui, params, setter, ks);
                        }, |ui| {
                            fx_panel(ui, params, setter, ks);
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
            ui.spacing_mut().item_spacing.x = 4.0;
            wave_picker(ui, setter, &params.osc2_wave);
            latch(ui, setter, &params.sync, "SYNC");
        });
        knob_row(ui, |ui| {
            knob(ui, setter, &params.osc2_mix, "MIX", ks);
            knob(ui, setter, &params.osc2_pwm, "PWM", ks);
            knob(ui, setter, &params.osc2_oct, "OCT", ks);
            knob(ui, setter, &params.osc2_semi, "SEMI", ks);
            knob(ui, setter, &params.osc2_cents, "CT", ks);
        });
    });
}

fn filter_panel(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter, ks: f32) {
    panel(ui, "FILTER", |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            filter_picker(ui, setter, &params.filt_mode);
            latch(ui, setter, &params.four_pole, "4 POLE");
        });
        knob_row(ui, |ui| {
            knob(ui, setter, &params.cutoff, "CUTOFF", ks);
            knob(ui, setter, &params.res, "RES", ks);
            knob(ui, setter, &params.drive, "DRIVE", ks);
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
            knob(ui, setter, &params.sub_mix, "SUB", ks);
            knob(ui, setter, &params.noise, "NOISE", ks);
            latch(ui, setter, &params.sub_square, "SQ SUB");
        });
        ui.horizontal(|ui| {
            ui.label(RichText::new("NOISE").small().color(MUTED));
            noise_picker(ui, setter, &params.noise_type);
        });
    });
}

fn env_panel(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter, ks: f32) {
    let k = (ks - 4.0).max(26.0);
    panel(ui, "ENVELOPES", |ui| {
        // Stacked (not nested columns) so labels stay visible and panels do not overlap.
        ui.label(RichText::new("AMP").small().color(MUTED));
        knob_row(ui, |ui| {
            knob(ui, setter, &params.amp_a, "A", k);
            knob(ui, setter, &params.amp_d, "D", k);
            knob(ui, setter, &params.amp_s, "S", k);
            knob(ui, setter, &params.amp_r, "R", k);
        });
        ui.add_space(2.0);
        ui.label(RichText::new("FILTER").small().color(MUTED));
        knob_row(ui, |ui| {
            knob(ui, setter, &params.filt_a, "A", k);
            knob(ui, setter, &params.filt_d, "D", k);
            knob(ui, setter, &params.filt_s, "S", k);
            knob(ui, setter, &params.filt_r, "R", k);
            knob(ui, setter, &params.pitch_env, "P.ENV", k);
        });
    });
}

fn fx_panel(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter, ks: f32) {
    panel(ui, "MOD / FX", |ui| {
        knob_row(ui, |ui| {
            knob(ui, setter, &params.lfo_rate, "RATE", ks);
            knob(ui, setter, &params.lfo_amt, "CUT", ks);
            knob(ui, setter, &params.lfo_pitch, "PIT", ks);
            knob(ui, setter, &params.lfo_pwm, "PWM", ks);
        });
        knob_row(ui, |ui| {
            knob(ui, setter, &params.glide, "GLIDE", ks);
            latch(ui, setter, &params.legato, "LEGATO");
            knob(ui, setter, &params.cho_mix, "CHORUS", ks);
            knob(ui, setter, &params.cho_rate, "C.RATE", ks);
            knob(ui, setter, &params.cho_depth, "C.DPTH", ks);
        });
    });
}

/// Side-by-side modules with explicit widths. Avoid `ui.columns` — nested columns
/// overlap frames and clip knob labels in egui.
fn module_pair(
    ui: &mut Ui,
    gap: f32,
    col_w: f32,
    left: impl FnOnce(&mut Ui),
    right: impl FnOnce(&mut Ui),
) {
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        ui.vertical(|ui| {
            ui.set_width(col_w);
            ui.set_max_width(col_w);
            left(ui);
        });
        ui.vertical(|ui| {
            ui.set_width(col_w);
            ui.set_max_width(col_w);
            right(ui);
        });
    });
}

fn knob_row(ui: &mut Ui, add: impl FnOnce(&mut Ui)) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(4.0, 4.0);
        add(ui);
    });
}

fn header(ui: &mut Ui, params: &Arc<SunderParams>, setter: &ParamSetter, loaded_name: &str) {
    let w = ui.available_width();
    ui.allocate_ui_with_layout(
        vec2(w, 52.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let rect = ui.max_rect();
            ui.painter()
                .rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(26, 24, 22));
            ui.painter().rect_stroke(
                rect,
                CornerRadius::same(8),
                Stroke::new(1.0, PANEL_EDGE),
                StrokeKind::Inside,
            );
            ui.painter().line_segment(
                [
                    pos2(rect.left() + 12.0, rect.bottom() - 2.0),
                    pos2(rect.right() - 12.0, rect.bottom() - 2.0),
                ],
                Stroke::new(2.0, AMBER.linear_multiply(0.55)),
            );

            if !loaded_name.is_empty() {
                ui.painter().text(
                    pos2(rect.center().x, rect.center().y),
                    Align2::CENTER_CENTER,
                    loaded_name,
                    FontId::proportional(14.0),
                    LCD,
                );
            }

            ui.add_space(12.0);
            ui.label(
                RichText::new("SUNDER")
                    .font(FontId::proportional(22.0))
                    .color(AMBER)
                    .strong(),
            );
            ui.add_space(8.0);
            ui.label(RichText::new("VIRTUAL ANALOG  ·  CLAP").small().color(MUTED));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                ui.allocate_ui_with_layout(
                    vec2(110.0, 42.0),
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

fn presets_panel(
    ui: &mut Ui,
    params: &Arc<SunderParams>,
    setter: &ParamSetter,
    state: &mut GuiState,
) {
    egui::Frame::new()
        .fill(PANEL)
        .stroke(Stroke::new(1.0, PANEL_EDGE))
        .corner_radius(8)
        .inner_margin(egui::Margin {
            left: 8,
            right: 8,
            top: 6,
            bottom: 8,
        })
        .show(ui, |ui| {
            let h = ui.available_height();
            let w = ui.available_width();
            if w.is_finite() && w > 1.0 {
                ui.set_width(w);
                ui.set_max_width(w);
            }
            if h.is_finite() && h > 1.0 {
                ui.set_min_height(h);
                ui.set_max_height(h);
            }
            ui.label(RichText::new("PRESETS").small().color(AMBER_DIM).strong());
            ui.add_space(2.0);
            preset_browser(ui, params, setter, state);
        });
}

fn panel(ui: &mut Ui, title: &str, add: impl FnOnce(&mut Ui)) {
    egui::Frame::new()
        .fill(PANEL)
        .stroke(Stroke::new(1.0, PANEL_EDGE))
        .corner_radius(8)
        .inner_margin(egui::Margin {
            left: 8,
            right: 8,
            top: 6,
            bottom: 10,
        })
        .show(ui, |ui| {
            let w = ui.available_width();
            if w.is_finite() && w > 1.0 {
                ui.set_width(w);
                ui.set_max_width(w);
            }
            ui.label(RichText::new(title).small().color(AMBER_DIM).strong());
            ui.add_space(2.0);
            add(ui);
        });
}

fn preset_browser(
    ui: &mut Ui,
    params: &Arc<SunderParams>,
    setter: &ParamSetter,
    state: &mut GuiState,
) {
    ui.set_width(ui.available_width());
    ui.spacing_mut().item_spacing = vec2(4.0, 4.0);

    let factory = presets::factory_presets();
    let names: Vec<(bool, String)> = if state.favorites {
        state
            .fav_order
            .clone()
            .unwrap_or_else(|| presets::favorite_entries(factory, &state.user, &state.ratings))
    } else {
        let mut names = Vec::new();
        for p in factory.iter().filter(|p| p.category == state.category) {
            names.push((true, p.name.clone()));
        }
        for p in state.user.iter().filter(|p| p.category == state.category) {
            names.push((false, p.name.clone()));
        }
        names
    };
    if state.selected >= names.len() {
        state.selected = 0;
    }

    // Bottom-up: pin Save/Del + name, then fill the leftover with categories + list.
    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
        ui.set_width(ui.available_width());

        if !state.status.is_empty() {
            ui.label(RichText::new(&state.status).small().color(MUTED));
        }
        ui.add(
            egui::TextEdit::singleline(&mut state.save_name)
                .hint_text("preset name")
                .desired_width(ui.available_width()),
        );
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
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
                        state.loaded_name = name.clone();
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
                                if state.loaded_name == *name {
                                    state.loaded_name.clear();
                                }
                                match presets::set_shared_rating(false, name, 0) {
                                    Ok((ratings, gen)) => {
                                        state.ratings = ratings;
                                        state.ratings_gen = gen;
                                    }
                                    Err(_) => {}
                                }
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
            if state.favorites && state.fav_dirty && amber_button(ui, "REFRESH").clicked() {
                refresh_favorites(state, factory);
            }
        });
        ui.add_space(4.0);

        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing = vec2(4.0, 4.0);

            let fav_on = state.favorites;
            let fav_chip = egui::Button::new(
                RichText::new("Favorites")
                    .small()
                    .color(if fav_on { BG } else { CREAM }),
            )
            .fill(if fav_on { AMBER } else { Color32::from_rgb(42, 38, 34) })
            .corner_radius(4)
            .min_size(vec2(ui.available_width(), 20.0));
            if ui.add(fav_chip).clicked() {
                if !state.favorites {
                    enter_favorites(state, factory);
                }
            }

            let cats = Category::ALL;
            let col_w = ((ui.available_width() - 4.0) * 0.5).max(64.0);
            for row in cats.chunks(2) {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    for &cat in row {
                        let on = !state.favorites && state.category == cat;
                        let chip = egui::Button::new(
                            RichText::new(cat.label())
                                .small()
                                .color(if on { BG } else { CREAM }),
                        )
                        .fill(if on { AMBER } else { Color32::from_rgb(42, 38, 34) })
                        .corner_radius(4)
                        .min_size(vec2(col_w, 20.0));
                        if ui.add(chip).clicked() {
                            state.favorites = false;
                            state.fav_order = None;
                            state.fav_dirty = false;
                            state.category = cat;
                            state.selected = 0;
                        }
                    }
                });
            }
            ui.add_space(4.0);

            // Pin the LCD list to leftover height and keep its clip inside that
            // slot. Replacing clip_rect on name rows let the full list paint
            // over the category chips.
            let list_w = ui.available_width();
            let list_h = ui.available_height().max(48.0);
            ui.allocate_ui_with_layout(
                vec2(list_w, list_h),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_width(list_w);
                    ui.set_max_width(list_w);
                    ui.set_min_height(list_h);
                    ui.set_max_height(list_h);
                    ui.shrink_clip_rect(ui.max_rect());
                    egui::Frame::new()
                        .fill(LCD_BG)
                        .corner_radius(6)
                        .inner_margin(6)
                        .stroke(Stroke::new(1.0, Color32::from_rgb(36, 52, 32)))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.set_max_height((list_h - 4.0).max(32.0));
                            egui::ScrollArea::vertical()
                                .id_salt("sunder_preset_list")
                                .max_height((list_h - 16.0).max(32.0))
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.spacing_mut().item_spacing.y = 1.0;
                                    if names.is_empty() && state.favorites {
                                        ui.label(
                                            RichText::new("No starred presets")
                                                .small()
                                                .color(MUTED),
                                        );
                                    }
                                    for (i, (factory_flag, name)) in names.iter().enumerate() {
                                        let selected = state.selected == i;
                                        let rating = state.ratings.get(*factory_flag, name);
                                        let text = if *factory_flag {
                                            name.clone()
                                        } else {
                                            format!("• {name}")
                                        };
                                        let color = if selected { AMBER } else { LCD };
                                        let mut load = false;
                                        let mut new_stars = None;
                                        ui.horizontal(|ui| {
                                            ui.spacing_mut().item_spacing.x = 4.0;
                                            let star_w = star_row_width();
                                            let name_w =
                                                (ui.available_width() - star_w - 4.0).max(48.0);
                                            let (name_rect, name_resp) = ui.allocate_exact_size(
                                                vec2(name_w, 16.0),
                                                Sense::click(),
                                            );
                                            if selected {
                                                ui.painter().rect_filled(
                                                    name_rect,
                                                    2.0,
                                                    AMBER.linear_multiply(0.22),
                                                );
                                            }
                                            ui.painter().with_clip_rect(name_rect).text(
                                                pos2(name_rect.left(), name_rect.center().y),
                                                Align2::LEFT_CENTER,
                                                &text,
                                                FontId::monospace(11.0),
                                                color,
                                            );
                                            if name_resp.clicked() {
                                                load = true;
                                            }
                                            new_stars = star_rating(ui, rating);
                                        });
                                        if load {
                                            state.selected = i;
                                            let loaded = if *factory_flag {
                                                factory.iter().find(|p| p.name == *name).map(|p| {
                                                    (p.category, p.params.clone())
                                                })
                                            } else {
                                                state.user.iter().find(|p| p.name == *name).map(
                                                    |p| (p.category, p.params.clone()),
                                                )
                                            };
                                            if let Some((cat, patch)) = loaded {
                                                presets::apply(setter, &patch);
                                                state.loaded_name = name.clone();
                                                state.category = cat;
                                                state.status.clear();
                                            }
                                        }
                                        if let Some(stars) = new_stars {
                                            let factory_flag = *factory_flag;
                                            let name = name.clone();
                                            match presets::set_shared_rating(
                                                factory_flag,
                                                &name,
                                                stars,
                                            ) {
                                                Ok((ratings, gen)) => {
                                                    state.ratings = ratings;
                                                    state.ratings_gen = gen;
                                                }
                                                Err(e) => state.status = e,
                                            }
                                            if state.favorites {
                                                state.fav_dirty = true;
                                            }
                                        }
                                    }
                                });
                        });
                },
            );
        });
    });
}

fn enter_favorites(state: &mut GuiState, factory: &[Preset]) {
    state.favorites = true;
    state.fav_order = Some(presets::favorite_entries(
        factory,
        &state.user,
        &state.ratings,
    ));
    state.fav_dirty = false;
    state.selected = 0;
}

fn refresh_favorites(state: &mut GuiState, factory: &[Preset]) {
    let follow = state.fav_order.as_ref().and_then(|order| order.get(state.selected).cloned());
    let order = presets::favorite_entries(factory, &state.user, &state.ratings);
    state.selected = follow
        .and_then(|(factory_flag, name)| {
            order
                .iter()
                .position(|(f, n)| *f == factory_flag && n == &name)
        })
        .unwrap_or(0);
    state.fav_order = Some(order);
    state.fav_dirty = false;
}

fn amber_button(ui: &mut Ui, text: &str) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(text).small().color(BG).strong())
            .fill(AMBER)
            .corner_radius(4),
    )
}

fn star_row_width() -> f32 {
    5.0 * STAR_PX + 4.0 * STAR_GAP
}

/// Click star N to set that rating. Click the current rating again to clear.
fn star_rating(ui: &mut Ui, rating: u8) -> Option<u8> {
    let mut new = None;
    ui.allocate_ui_with_layout(
        vec2(star_row_width(), STAR_PX),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = STAR_GAP;
            let size = vec2(STAR_PX, STAR_PX);
            for i in 1u8..=5 {
                let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
                paint_star(ui, rect, i <= rating);
                if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if resp.clicked() {
                    new = Some(if rating == i { 0 } else { i });
                }
            }
        },
    );
    new
}

fn paint_star(ui: &Ui, rect: Rect, filled: bool) {
    let painter = ui.painter();
    let c = rect.center();
    let r = rect.width() * 0.46;
    let mut pts = Vec::with_capacity(10);
    for i in 0..10 {
        let a = -0.5 * PI + i as f32 * PI / 5.0;
        let rad = if i % 2 == 0 { r } else { r * 0.40 };
        pts.push(pos2(c.x + rad * a.cos(), c.y + rad * a.sin()));
    }
    if filled {
        for i in 0..10 {
            painter.add(egui::Shape::convex_polygon(
                vec![c, pts[i], pts[(i + 1) % 10]],
                AMBER,
                Stroke::NONE,
            ));
        }
    } else {
        pts.push(pts[0]);
        painter.add(egui::Shape::line(
            pts,
            PathStroke::new(1.0, Color32::from_rgb(70, 92, 64)),
        ));
    }
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

fn noise_picker(ui: &mut Ui, setter: &ParamSetter, param: &nih_plug::prelude::EnumParam<NoiseChoice>) {
    let current = param.value();
    let text = match current {
        NoiseChoice::White => "WHITE",
        NoiseChoice::Pink => "PINK",
        NoiseChoice::Brown => "DARK",
        NoiseChoice::Digital => "DIGI",
    };
    egui::ComboBox::from_id_salt("sunder-noise-type")
        .selected_text(RichText::new(text).small().color(CREAM))
        .width(88.0)
        .show_ui(ui, |ui| {
            for (choice, label) in [
                (NoiseChoice::White, "WHITE"),
                (NoiseChoice::Pink, "PINK"),
                (NoiseChoice::Brown, "DARK"),
                (NoiseChoice::Digital, "DIGI"),
            ] {
                let on = current == choice;
                if ui
                    .selectable_label(on, RichText::new(label).small().color(if on { AMBER } else { CREAM }))
                    .clicked()
                {
                    setter.begin_set_parameter(param);
                    setter.set_parameter(param, choice);
                    setter.end_set_parameter(param);
                }
            }
        });
}

fn filter_picker(ui: &mut Ui, setter: &ParamSetter, param: &nih_plug::prelude::EnumParam<FilterChoice>) {
    let current = param.value();
    let text = match current {
        FilterChoice::Lowpass => "LP",
        FilterChoice::Bandpass => "BP",
        FilterChoice::Highpass => "HP",
    };
    egui::ComboBox::from_id_salt("sunder-filt-mode")
        .selected_text(RichText::new(text).small().color(CREAM))
        .width(64.0)
        .show_ui(ui, |ui| {
            for (choice, label) in [
                (FilterChoice::Lowpass, "LP"),
                (FilterChoice::Bandpass, "BP"),
                (FilterChoice::Highpass, "HP"),
            ] {
                let on = current == choice;
                if ui
                    .selectable_label(on, RichText::new(label).small().color(if on { AMBER } else { CREAM }))
                    .clicked()
                {
                    setter.begin_set_parameter(param);
                    setter.set_parameter(param, choice);
                    setter.end_set_parameter(param);
                }
            }
        });
}

fn latch(ui: &mut Ui, setter: &ParamSetter, param: &nih_plug::prelude::BoolParam, label: &str) {
    let on = param.value();
    ui.vertical(|ui| {
        ui.set_width(48.0);
        ui.add_space(6.0);
        let (rect, response) = ui.allocate_exact_size(vec2(44.0, 22.0), Sense::click());
        let painter = ui.painter();
        painter.rect_filled(rect, CornerRadius::same(11), INSET);
        painter.rect_stroke(rect, CornerRadius::same(11), Stroke::new(1.0, PANEL_EDGE), StrokeKind::Inside);
        let knob_x = if on { rect.right() - 11.0 } else { rect.left() + 11.0 };
        painter.circle_filled(
            pos2(knob_x, rect.center().y),
            8.0,
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
        ui.set_width(size + 2.0);
        ui.set_max_width(size + 2.0);
        ui.set_min_height(size + 28.0);
        ui.spacing_mut().item_spacing.y = 1.0;

        let (rect, mut response) = ui.allocate_exact_size(vec2(size, size), Sense::click_and_drag());
        let norm = param.modulated_normalized_value();
        interact_knob(ui, setter, param, &mut response, norm);
        paint_knob(ui, rect, norm, response.hovered() || response.dragged());

        ui.label(
            RichText::new(label)
                .font(FontId::proportional(9.5))
                .color(MUTED),
        );
        ui.label(
            RichText::new(pretty_value(param))
                .font(FontId::proportional(9.5))
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
        // Discrete MouseWheel events — smooth_scroll_delta lasts many frames and
        // was jumping int knobs (e.g. Voices 1→5) on a single notch.
        let mut steps = 0i32;
        ui.input(|i| {
            for event in &i.events {
                if let egui::Event::MouseWheel { delta, .. } = event {
                    if delta.y > 0.0 {
                        steps += 1;
                    } else if delta.y < 0.0 {
                        steps -= 1;
                    }
                }
            }
        });
        if steps != 0 {
            // At most one parameter step per frame (one physical notch).
            let steps = steps.signum();
            setter.begin_set_parameter(param);
            let value = param.modulated_plain_value();
            let value = if steps > 0 {
                param.next_step(value, false)
            } else {
                param.previous_step(value, false)
            };
            setter.set_parameter(param, value);
            setter.end_set_parameter(param);
            response.mark_changed();
            // Consume wheel so the parent ScrollArea does not move the page.
            ui.ctx().input_mut(|i| {
                i.smooth_scroll_delta.y = 0.0;
                i.raw_scroll_delta.y = 0.0;
            });
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
