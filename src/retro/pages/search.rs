//! Retro Search & Query Catalog view.
//!
//! Provides a period-accurate 1996 search query interface
//! rendering hits as ribbon cards and track data rows.

use egui::{pos2, vec2, Align, Align2, Color32, CornerRadius, Rect, Sense, Stroke, Ui, Vec2};

use crate::api::models::{join_names, pick_image};
use crate::app::App;
use crate::model::{Action, Page};
use crate::retro::style::{
    fonts,
    primitives::{self, ribbon_card, section_eyebrow, text_link},
    tint,
    tokens::*,
};

pub fn show(app: &mut App, ui: &mut Ui) {
    section_eyebrow(ui, CatalogTint::Sky, "ONLINE SEARCH // GLOBAL REPERTOIRE");

    // Search Form Box (1996 HTML 3.2 Form Box style)
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("QUERY:").font(fonts::button()).color(INK));

        let mut query = app.search.query.clone();
        let edit = ui.add(
            egui::TextEdit::singleline(&mut query)
                .font(fonts::body())
                .text_color(INK_BLACK)
                .margin(vec2(8.0, 6.0))
                .desired_width(320.0),
        );

        if edit.changed() {
            app.actions.push(Action::Search(query));
        }

        if primitives::button_primary(ui, "EXECUTE QUERY").clicked() {
            app.actions.push(Action::Search(app.search.query.clone()));
        }
    });

    ui.add_space(SPACING_LG);

    let results = app.search.results.clone();
    let Some(results) = results else {
        ui.label(egui::RichText::new("Enter artist, song, or album keyword above.").font(fonts::body()).color(INK_MUTED));
        return;
    };

    // 1. Album Hits
    if !results.albums.is_empty() {
        ui.label(egui::RichText::new("ALBUM MATCHES").font(fonts::heading_3()).color(INK));
        ui.add_space(SPACING_S);

        for (idx, album) in results.albums.iter().take(4).enumerate() {
            let card_tint = CatalogTint::ALL[idx % CatalogTint::ALL.len()];
            let cover_url = pick_image(&album.images, 200).map(str::to_string);
            let artists = join_names(album.artists.iter().map(|a| a.name.as_str()));

            let (resp, _) = ribbon_card(
                ui,
                &app.palette,
                card_tint,
                &album.name,
                cover_url.as_deref(),
                Some(app.backend.art()),
                |ui| {
                    ui.label(egui::RichText::new(format!("ARTIST: {artists}")).font(fonts::body()).color(INK_BLACK));
                    ui.label(egui::RichText::new(format!("TRACKS: {}", album.total_tracks)).font(fonts::caption()).color(INK_BLACK));
                },
            );

            if resp.clicked() {
                app.actions.push(Action::Open(Page::Album(album.id.clone())));
            }
        }
        ui.add_space(SPACING_SECTION_SM);
    }

    // 2. Track Hits
    if !results.tracks.is_empty() {
        ui.label(egui::RichText::new("TRACK MATCHES").font(fonts::heading_3()).color(INK));
        ui.add_space(SPACING_S);

        for (idx, track) in results.tracks.iter().take(15).enumerate() {
            let is_playing = app.playing_track_uri().as_deref() == Some(track.uri.as_str());
            let duration = format_duration(track.duration_ms);
            let resp = primitives::track_row(
                ui,
                idx + 1,
                &track.name,
                &join_names(track.artists.iter().map(|a| a.name.as_str())),
                &track.album.name,
                &duration,
                is_playing,
            );

            if resp.clicked() {
                app.actions.push(Action::PlayUris {
                    uris: vec![track.uri.clone()],
                    index: 0,
                });
            }
        }
    }
}

fn format_duration(ms: u32) -> String {
    let s = ms / 1000;
    format!("{}:{:02}", s / 60, s % 60)
}
