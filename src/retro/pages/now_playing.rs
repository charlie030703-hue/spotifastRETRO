//! Now Playing page — Artwork-Driven Hero view.
//!
//! Makes the album artwork the undisputed visual star of the application:
//! - Giant, beveled album cover hero (up to 380x380) with 0px radius and hard drop shadow
//! - Eyebrow section banner dynamically snapped to the artwork's catalog tint
//! - Dell Ribbon Card detailing song title, artist, album and playback position
//! - The screen's singular Dell Red CTA panel ("PLAY / PAUSE RECORD")
//! - Authentic 1996 transport controls and segmented track progress

use egui::{pos2, vec2, Color32, CornerRadius, Rect, Sense, Stroke, Ui};

use crate::app::App;
use crate::model::{Action, Page};
use crate::retro::style::{
    fonts,
    primitives::{self, bevel_art, cta_block_red, section_eyebrow, text_link},
    tint, tokens::*,
};
use crate::theme::Icon;

pub fn show(app: &mut App, ui: &mut Ui) {
    let now = app.now_playing();

    let (title, artist, album_name, cover_url, duration_ms, position_ms, is_playing) = match now.as_ref() {
        Some(now) => (
            now.title.clone(),
            now.subtitle.clone(),
            now.album_id.clone().unwrap_or_else(|| "Unknown Album".into()),
            now.art_url.clone(),
            now.duration_ms,
            now.position_ms,
            now.playing,
        ),
        None => (
            "No Record Loaded".into(),
            "Select an album, artist, or playlist".into(),
            "Spotifast Turntable Ready".into(),
            None,
            0,
            0,
            false,
        ),
    };

    // Calculate catalogue tint from album artwork
    let palette_colors = app.now_playing_palette();
    let current_tint = tint::snap_artwork(palette_colors.unwrap_or(&[]), cover_url.as_deref());

    // 1. Eyebrow Banner
    let eyebrow_text = if is_playing {
        format!("NOW SPINNING // {}", current_tint.name().to_uppercase())
    } else {
        "PHONOGRAPH READY".into()
    };
    section_eyebrow(ui, current_tint, &eyebrow_text);

    ui.add_space(SPACING_LG);

    // 2. Large Central Artwork Hero with Spinning CD Disc
    ui.vertical_centered(|ui| {
        let hero_size = 310.0;
        let art_loader = app.backend.art();

        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - (hero_size + 140.0)).max(0.0) / 2.0);

            // Album cover sleeve
            let art_rect = bevel_art(
                ui,
                &app.palette,
                cover_url.as_deref(),
                hero_size,
                Icon::Disc,
                Some(art_loader),
            );

            // Spinning Compact Disc partially peeking out from the right of the sleeve!
            let cd_center = pos2(art_rect.max.x + 40.0, art_rect.center().y);
            let cd_radius = hero_size * 0.44;
            let cd_angle = if is_playing {
                (position_ms as f32) * 0.003
            } else {
                0.0
            };
            primitives::draw_cd(ui.painter(), cd_center, cd_radius, cd_angle);

            // Optional "NEW!" burst sticker on recent releases
            if is_playing {
                primitives::new_burst_sticker(ui, pos2(art_rect.max.x - 16.0, art_rect.min.y + 16.0));
            }
        });

        ui.add_space(SPACING_XL);

        // 3. Track Details Ribbon Card
        primitives::ribbon_card(
            ui,
            &app.palette,
            current_tint,
            &title,
            cover_url.as_deref(),
            Some(art_loader),
            |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("ARTIST: ").font(fonts::button()).color(INK_BLACK));
                    if text_link(ui, &artist, true).clicked() {
                        if let Some(now) = now.as_ref() {
                            if let Some(artist_id) = now.artists.first().and_then(|a| a.id.as_ref()) {
                                app.actions.push(Action::Open(Page::Artist(artist_id.clone())));
                            }
                        }
                    }
                });

                ui.add_space(SPACING_XS);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("ALBUM: ").font(fonts::button()).color(INK_BLACK));
                    ui.label(egui::RichText::new(&album_name).font(fonts::body_sm()).color(INK_BLACK));
                });

                ui.add_space(SPACING_XS);

                let pos_str = format_ms(position_ms);
                let dur_str = format_ms(duration_ms);
                ui.label(
                    egui::RichText::new(format!("POSITION: {pos_str} OF {dur_str} (COMPACT DISC AUDIO // 44.1 kHz)"))
                        .font(fonts::caption())
                        .color(INK_BLACK),
                );
            },
        );

        ui.add_space(SPACING_LG);

        // 4. The singular Dell Red CTA block
        let (cta_title, cta_subtitle) = if is_playing {
            ("PAUSE RECORD PLAYBACK", "Halt compact disc laser pickup at current track")
        } else {
            ("SPIN COMPACT DISC // PLAY", "Commence 16-bit digital playback through sound card")
        };

        if cta_block_red(ui, cta_title, cta_subtitle).clicked() {
            app.actions.push(Action::TogglePlay);
        }

        ui.add_space(SPACING_LG);

        // 5. Chunky 1996 Transport Button Row (centered, perfectly uniform)
        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 410.0).max(0.0) / 2.0);
            ui.spacing_mut().item_spacing.x = 8.0;

            if primitives::nav_button(ui, "|< PREV", false, 74.0, 28.0).clicked() {
                app.actions.push(Action::Previous);
            }

            let play_label = if is_playing { "|| PAUSE" } else { "> PLAY" };
            if primitives::nav_button(ui, play_label, is_playing, 78.0, 28.0).clicked() {
                app.actions.push(Action::TogglePlay);
            }

            if primitives::nav_button(ui, "NEXT >|", false, 74.0, 28.0).clicked() {
                app.actions.push(Action::Next);
            }

            let shuffle_active = now.as_ref().map_or(false, |n| n.shuffle);
            if primitives::nav_button(ui, "SHUFFLE", shuffle_active, 78.0, 28.0).clicked() {
                app.actions.push(Action::ToggleShuffle);
            }

            let repeat_active = now.as_ref().map_or(false, |n| n.repeat != crate::player::RepeatMode::Off);
            if primitives::nav_button(ui, "REPEAT", repeat_active, 74.0, 28.0).clicked() {
                app.actions.push(Action::CycleRepeat);
            }
        });

        ui.add_space(SPACING_LG);

        // 6. Segmented Progress Bar (1996 catalog style)
        paint_segmented_progress(ui, position_ms, duration_ms);
    });
}

fn paint_segmented_progress(ui: &mut Ui, position_ms: u32, duration_ms: u32) {
    let width = 460.0_f32.min(ui.available_width() - 40.0);
    let height = 18.0;
    let (rect, _response) = ui.allocate_exact_size(vec2(width, height), Sense::click_and_drag());
    let painter = ui.painter();

    // Background container with 1px border
    painter.rect_filled(rect, CornerRadius::ZERO, FRAME_INK);
    painter.rect_stroke(rect, CornerRadius::ZERO, Stroke::new(1.0, BEVEL_HIGHLIGHT), egui::StrokeKind::Inside);

    let progress = if duration_ms > 0 {
        (position_ms as f32 / duration_ms as f32).clamp(0.0, 1.0)
    } else {
        0.0
    };

    // Draw chunky progress segments (each 8px wide with 2px gap)
    let segment_w = 8.0;
    let gap = 2.0;
    let total_segments = ((width - 4.0) / (segment_w + gap)).floor() as usize;
    let active_segments = (total_segments as f32 * progress).round() as usize;

    for i in 0..total_segments {
        let x = rect.min.x + 2.0 + (i as f32 * (segment_w + gap));
        let seg_rect = Rect::from_min_size(pos2(x, rect.min.y + 2.0), vec2(segment_w, height - 4.0));
        let color = if i < active_segments {
            // First 70% yellow, next 20% peach, top 10% red
            let pct = i as f32 / total_segments as f32;
            if pct > 0.88 {
                PRIMARY
            } else if pct > 0.65 {
                CatalogTint::Peach.color()
            } else {
                YELLOW_STICKER
            }
        } else {
            Color32::from_rgb(0x1a, 0x1a, 0x1a)
        };
        painter.rect_filled(seg_rect, CornerRadius::ZERO, color);
    }
}

fn format_ms(ms: u32) -> String {
    let total_secs = ms / 1000;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("{:02}:{:02}", mins, secs)
}
