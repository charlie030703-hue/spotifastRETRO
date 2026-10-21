//! Retro Footer Player bar.
//!
//! Fixed 72px bar at the bottom of the window:
//! - Pure black fill with 1px top border
//! - Left: Beveled artwork thumbnail (50x50, 0px radius), track title, artist link
//! - Center: Square transport buttons (PREV, PLAY/PAUSE, NEXT) and segmented track progress
//! - Right: Chunky master volume fader and output mode

use egui::{pos2, vec2, Align, Align2, Color32, CornerRadius, Rect, Stroke, Ui, Vec2};

use crate::app::App;
use crate::model::Action;
use crate::retro::style::{
    fonts,
    primitives,
    tokens::*,
};
use crate::theme::Icon;

pub fn show(app: &mut App, ui: &mut Ui, rect: Rect) {
    // Pure black background with 1px top border
    ui.painter().rect_filled(rect, CornerRadius::ZERO, FRAME_INK);
    ui.painter().line_segment(
        [rect.left_top(), rect.right_top()],
        Stroke::new(1.0, BEVEL_HIGHLIGHT),
    );

    let now = app.now_playing();
    let is_playing = now.as_ref().map_or(false, |n| n.playing);

    // ─── 1. Left Section: Now Playing Cover + Title (width ~260px) ───
    let left_rect = Rect::from_min_size(rect.min + vec2(SPACING_LG, 10.0), vec2(280.0, 52.0));
    let cover_size = 50.0;
    let cover_rect = Rect::from_min_size(left_rect.min, Vec2::splat(cover_size));

    let (title, artist, cover_url) = match now.as_ref() {
        Some(n) => (n.title.as_str(), n.subtitle.as_str(), n.art_url.as_deref()),
        None => ("NOT PLAYING", "IDLE PHONOGRAPH", None),
    };

    crate::ui::widgets::paint_cover(
        ui,
        &app.palette,
        cover_url,
        cover_rect,
        0.0,
        Icon::Disc,
        Some(app.backend.art()),
    );
    ui.painter().rect_stroke(cover_rect, CornerRadius::ZERO, Stroke::new(1.0, FRAME_INK), egui::StrokeKind::Inside);

    // Title and artist text (clipped so they never overlap the center)
    let max_text_w = 210.0;
    let title_pos = pos2(cover_rect.max.x + SPACING_M, left_rect.min.y + 8.0);
    let title_clip = ui.painter().with_clip_rect(Rect::from_min_size(title_pos, vec2(max_text_w, 18.0)));
    title_clip.text(
        title_pos,
        Align2::LEFT_TOP,
        title.to_uppercase(),
        fonts::heading_3(),
        INK,
    );

    let artist_pos = pos2(cover_rect.max.x + SPACING_M, left_rect.min.y + 26.0);
    let artist_clip = ui.painter().with_clip_rect(Rect::from_min_size(artist_pos, vec2(max_text_w, 16.0)));
    artist_clip.text(
        artist_pos,
        Align2::LEFT_TOP,
        artist,
        fonts::caption(),
        INK_MUTED,
    );

    // ─── 2. Center Section: Transport Controls + Progress ───
    let center_w = 420.0_f32.min(rect.width() - 560.0).max(240.0);
    let center_rect = Rect::from_center_size(
        pos2(rect.center().x, rect.center().y),
        vec2(center_w, 56.0),
    );

    let mut center_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(center_rect)
            .layout(egui::Layout::top_down(Align::Center)),
    );

    center_ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;

        if primitives::nav_button(ui, "|<", false, 36.0, 24.0).clicked() {
            app.actions.push(Action::Previous);
        }

        let play_label = if is_playing { "|| PAUSE" } else { "> PLAY" };
        if primitives::nav_button(ui, play_label, is_playing, 76.0, 24.0).clicked() {
            app.actions.push(Action::TogglePlay);
        }

        if primitives::nav_button(ui, ">|", false, 36.0, 24.0).clicked() {
            app.actions.push(Action::Next);
        }
    });

    center_ui.add_space(SPACING_XS);

    // Progress bar in center
    let (pos_ms, dur_ms) = now.as_ref().map_or((0, 0), |n| (n.position_ms, n.duration_ms));
    let progress_w = center_w - 20.0;
    let progress_rect = Rect::from_min_size(
        pos2(center_rect.center().x - progress_w / 2.0, center_rect.min.y + 36.0),
        vec2(progress_w, 10.0),
    );

    ui.painter().rect_filled(progress_rect, CornerRadius::ZERO, Color32::from_rgb(0x11, 0x11, 0x11));
    ui.painter().rect_stroke(progress_rect, CornerRadius::ZERO, Stroke::new(1.0, BEVEL_HIGHLIGHT), egui::StrokeKind::Inside);

    if dur_ms > 0 {
        let pct = (pos_ms as f32 / dur_ms as f32).clamp(0.0, 1.0);
        let filled_rect = Rect::from_min_size(progress_rect.min, vec2(progress_w * pct, 10.0));
        ui.painter().rect_filled(filled_rect, CornerRadius::ZERO, YELLOW_STICKER);
    }

    // ─── 3. Right Section: Master Volume ───
    let right_rect = Rect::from_min_size(
        pos2(rect.max.x - 220.0, rect.min.y + 16.0),
        vec2(200.0, 40.0),
    );

    let mut right_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(right_rect)
            .layout(egui::Layout::right_to_left(Align::Center)),
    );

    let mut vol = app.settings.volume as f32;
    right_ui.horizontal(|ui| {
        let slider = ui.add_sized(
            [90.0, 18.0],
            egui::Slider::new(&mut vol, 0.0..=100.0).show_value(false),
        );
        if slider.changed() {
            app.actions.push(Action::SetVolume(vol as u8));
        }

        ui.label(egui::RichText::new("VOL:").font(fonts::button()).color(INK_MUTED));
    });
}
