//! Retro Search & Query Catalog view.
//!
//! Provides a period-accurate 1996 search query interface
//! rendering hits as ribbon cards and track data rows.

use egui::{vec2, Ui};

use crate::api::models::{join_names, pick_image};
use crate::app::App;
use crate::model::{Action, Page};
use crate::retro::style::{
    fonts,
    primitives::{self, ribbon_card, section_eyebrow},
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

    let Some(results) = app.search.results.get() else {
        ui.label(egui::RichText::new("Enter artist, song, or album keyword above.").font(fonts::body()).color(INK_MUTED));
        return;
    };

    // 1. Album Hits
    if let Some(albums) = results.albums.as_ref().filter(|a| !a.items.is_empty()) {
        ui.label(egui::RichText::new("ALBUM MATCHES").font(fonts::heading_3()).color(INK));
        ui.add_space(SPACING_S);

        for (idx, album) in albums.items.iter().take(4).enumerate() {
            let card_tint = CatalogTint::ALL[idx % CatalogTint::ALL.len()];
            let cover_url = pick_image(&album.images, 200).map(str::to_string);
            let artists = join_names(album.artists.iter().map(|a| a.name.as_str()));
            let total = album.total_tracks.unwrap_or(0);

            let (resp, _) = ribbon_card(
                ui,
                &app.palette,
                card_tint,
                &album.name,
                cover_url.as_deref(),
                Some(app.backend.art()),
                |ui| {
                    ui.label(egui::RichText::new(format!("ARTIST: {artists}")).font(fonts::body()).color(INK_BLACK));
                    ui.label(egui::RichText::new(format!("TRACKS: {total}")).font(fonts::caption()).color(INK_BLACK));
                },
            );

            if resp.clicked() {
                app.actions.push(Action::Open(Page::Album(album.id.clone())));
            }
        }
        ui.add_space(SPACING_SECTION_SM);
    }

    // 2. Track Hits
    if let Some(tracks) = results.tracks.as_ref().filter(|t| !t.items.is_empty()) {
        ui.label(egui::RichText::new("TRACK MATCHES").font(fonts::heading_3()).color(INK));
        ui.add_space(SPACING_S);

        for (idx, track) in tracks.items.iter().take(15).enumerate() {
            let is_playing = app.now_playing().as_ref().map(|n| n.uri.as_str()) == Some(track.uri.as_str());
            let duration = format_duration(track.duration_ms);
            let album_name = track.album.as_ref().map_or("", |a| a.name.as_str());
            let resp = primitives::track_row(
                ui,
                idx + 1,
                &track.name,
                &join_names(track.artists.iter().map(|a| a.name.as_str())),
                album_name,
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
