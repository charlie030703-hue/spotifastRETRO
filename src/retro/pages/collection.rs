//! Collection pages: Album, Playlist, and Artist detail views.
//!
//! Features:
//! - Prominent 260px beveled album cover hero
//! - Dynamic catalog eyebrow tinted to the artwork
//! - The screen's singular Dell Red "PLAY FULL RECORD" CTA button
//! - Data table track listing with sharp borders and period-accurate typography

use egui::Ui;

use crate::api::models::{join_names, pick_image, ArtistRef};
use crate::app::App;
use crate::model::{Action, Page};
use crate::retro::style::{
    fonts,
    primitives::{self, bevel_art, cta_block_red, section_eyebrow},
    tint,
    tokens::*,
};
use crate::theme::Icon;

pub fn show_album(app: &mut App, ui: &mut Ui, album_id: &str) {
    let album_data = app.album_pages.get(album_id).and_then(|p| p.album.get()).cloned();

    let Some(album) = album_data else {
        ui.label(egui::RichText::new("Loading album from Spotify...").font(fonts::body()).color(INK_MUTED));
        return;
    };

    let cover_url = pick_image(&album.images, 300).map(str::to_string);
    let card_tint = tint::snap_artwork(&[], cover_url.as_deref());

    // 1. Eyebrow Header
    section_eyebrow(ui, card_tint, &format!("ALBUM ARCHIVE // {}", album.name));

    // 2. Hero Section: Cover + Metadata
    ui.horizontal(|ui| {
        bevel_art(
            ui,
            &app.palette,
            cover_url.as_deref(),
            240.0,
            Icon::Disc,
            Some(app.backend.art()),
        );

        ui.add_space(SPACING_LG);

        ui.vertical(|ui| {
            ui.label(egui::RichText::new(&album.name).font(fonts::heading_1()).color(INK));
            ui.add_space(SPACING_XS);

            let artists = join_names(album.artists.iter().map(|a| a.name.as_str()));
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("BY: ").font(fonts::button()).color(INK_MUTED));
                ui.label(egui::RichText::new(&artists).font(fonts::body()).color(PRIMARY));
            });

            ui.add_space(SPACING_XS);
            let release = album.release_date.as_deref().unwrap_or("Unknown");
            let total = album.total_tracks.unwrap_or(0);
            ui.label(
                egui::RichText::new(format!(
                    "RELEASE DATE: {release} // TOTAL TRACKS: {total}",
                ))
                .font(fonts::caption())
                .color(INK_MUTED),
            );

            ui.add_space(SPACING_MD);

            // Singular Dell Red CTA block
            if cta_block_red(ui, "PLAY FULL ALBUM", "Spin record from side 1 track 1").clicked() {
                app.actions.push(Action::PlayContext {
                    uri: album.uri.clone(),
                    offset_uri: None,
                    offset_index: Some(0),
                });
            }
        });
    });

    ui.add_space(SPACING_SECTION_SM);

    // 3. Track List Data Table
    ui.label(egui::RichText::new("TRACK LISTING // MASTER TAPES").font(fonts::heading_3()).color(INK));
    ui.add_space(SPACING_S);
    primitives::track_table_header(ui);
    ui.add_space(2.0);

    let tracks: Vec<crate::api::models::Track> = app
        .album_pages
        .get(album_id)
        .map(|p| p.tracks.items.clone())
        .filter(|t| !t.is_empty())
        .or_else(|| album.tracks.as_ref().map(|p| p.items.clone()))
        .unwrap_or_default();

    for (idx, track) in tracks.iter().enumerate() {
        let is_playing = app.now_playing().as_ref().map(|n| n.uri.as_str()) == Some(track.uri.as_str());
        let duration = format_duration(track.duration_ms);
        let resp = primitives::track_row(
            ui,
            idx + 1,
            &track.name,
            &artists_ref_str(&track.artists),
            &album.name,
            &duration,
            is_playing,
        );

        if resp.clicked() {
            app.actions.push(Action::PlayContext {
                uri: album.uri.clone(),
                offset_uri: Some(track.uri.clone()),
                offset_index: Some(idx as u32),
            });
        }
    }
}

pub fn show_playlist(app: &mut App, ui: &mut Ui, playlist_id: &str) {
    let playlist_data = app.playlist_pages.get(playlist_id).and_then(|p| p.playlist.get()).cloned();

    let Some(playlist) = playlist_data else {
        app.actions.push(Action::LoadMore(Page::Playlist(playlist_id.to_string())));
        ui.label(egui::RichText::new("Loading compilation from Spotify...").font(fonts::body()).color(INK_MUTED));
        return;
    };

    let cover_url = pick_image(&playlist.images, 300).map(str::to_string);
    let card_tint = tint::snap_artwork(&[], cover_url.as_deref());

    section_eyebrow(ui, card_tint, &format!("COMPILATION // {}", playlist.name));

    ui.horizontal(|ui| {
        bevel_art(
            ui,
            &app.palette,
            cover_url.as_deref(),
            240.0,
            Icon::Library,
            Some(app.backend.art()),
        );

        ui.add_space(SPACING_LG);

        ui.vertical(|ui| {
            ui.label(egui::RichText::new(&playlist.name).font(fonts::heading_1()).color(INK));
            ui.add_space(SPACING_XS);

            let curator = playlist.owner.display_name.as_deref().unwrap_or("Spotify");
            ui.label(
                egui::RichText::new(format!("CURATED BY: {curator}"))
                    .font(fonts::body())
                    .color(PRIMARY),
            );

            ui.add_space(SPACING_MD);

            if cta_block_red(ui, "PLAY COMPILATION", "Commence mixtape playback").clicked() {
                app.actions.push(Action::PlayContext {
                    uri: playlist.uri.clone(),
                    offset_uri: None,
                    offset_index: Some(0),
                });
            }
        });
    });

    ui.add_space(SPACING_SECTION_SM);

    ui.label(egui::RichText::new("TRACK LISTING").font(fonts::heading_3()).color(INK));
    ui.add_space(SPACING_S);
    primitives::track_table_header(ui);
    ui.add_space(2.0);

    let items = app
        .playlist_pages
        .get(playlist_id)
        .map(|p| p.items.items.clone())
        .unwrap_or_default();

    for (idx, item) in items.iter().enumerate() {
        let Some(playable) = item.item.as_ref().or(item.track.as_ref()) else { continue };
        let is_playing = app.now_playing().as_ref().map(|n| n.uri.as_str()) == Some(playable.uri());
        let duration = format_duration(playable.duration_ms());
        let resp = primitives::track_row(
            ui,
            idx + 1,
            playable.name(),
            &playable.subtitle(),
            &playlist.name,
            &duration,
            is_playing,
        );

        if resp.clicked() {
            app.actions.push(Action::PlayContext {
                uri: playlist.uri.clone(),
                offset_uri: Some(playable.uri().to_string()),
                offset_index: Some(idx as u32),
            });
        }
    }
}

fn artists_ref_str(artists: &[ArtistRef]) -> String {
    join_names(artists.iter().map(|a| a.name.as_str()))
}

fn format_duration(ms: u32) -> String {
    let s = ms / 1000;
    format!("{}:{:02}", s / 60, s % 60)
}
