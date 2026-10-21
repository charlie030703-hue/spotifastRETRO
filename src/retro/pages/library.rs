//! Library page — Albums, Playlists, and Liked Songs.
//!
//! Organizes user's collection into period-accurate catalog listings
//! with prominent album art and crisp metadata cards.

use egui::Ui;

use crate::api::models::{join_names, pick_image};
use crate::app::App;
use crate::model::{Action, Page};
use crate::retro::style::{
    fonts,
    primitives::{self, ribbon_card, section_eyebrow},
    tokens::*,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LibraryTab {
    Albums,
    Playlists,
    LikedSongs,
}

pub fn show(app: &mut App, ui: &mut Ui, tab: LibraryTab) {
    let (eyebrow_title, eyebrow_tint) = match tab {
        LibraryTab::Albums => ("SAVED ALBUM ARCHIVE // CATALOG", CatalogTint::Peach),
        LibraryTab::Playlists => ("PERSONAL MIXTAPES & COMPILATIONS", CatalogTint::Periwinkle),
        LibraryTab::LikedSongs => ("FAVORITE TRACKS // LIKED SONGS", CatalogTint::Salmon),
    };

    section_eyebrow(ui, eyebrow_tint, eyebrow_title);

    // Tab switcher row (1996 button tabs)
    ui.horizontal(|ui| {
        if primitives::button_primary(ui, "ALBUMS").clicked() {
            app.actions.push(Action::Open(Page::Albums));
        }
        ui.add_space(SPACING_SM);

        if primitives::button_secondary(ui, "LIKED SONGS").clicked() {
            app.actions.push(Action::Open(Page::LikedSongs));
        }
    });

    ui.add_space(SPACING_LG);

    match tab {
        LibraryTab::Albums => {
            let albums = app.library.albums.items.clone();
            if albums.is_empty() {
                ui.label(egui::RichText::new("No albums saved in local catalog.").font(fonts::body()).color(INK_MUTED));
            } else {
                for (idx, item) in albums.iter().enumerate() {
                    let album = &item.album;
                    let art_url = pick_image(&album.images, 200).map(str::to_string);
                    let artists = join_names(album.artists.iter().map(|a| a.name.as_str()));
                    let card_tint = CatalogTint::ALL[idx % CatalogTint::ALL.len()];
                    let tracks_count = album.total_tracks.unwrap_or(0);
                    let release = album.release_date.as_deref().unwrap_or("Unknown");

                    let (resp, _) = ribbon_card(
                        ui,
                        &app.palette,
                        card_tint,
                        &album.name,
                        art_url.as_deref(),
                        Some(app.backend.art()),
                        |ui| {
                            ui.label(egui::RichText::new(format!("ARTIST: {artists}")).font(fonts::body()).color(INK_BLACK));
                            ui.label(
                                egui::RichText::new(format!("TRACKS: {tracks_count} // RELEASE: {release}"))
                                    .font(fonts::caption())
                                    .color(INK_BLACK),
                            );
                        },
                    );

                    if resp.clicked() {
                        app.actions.push(Action::Open(Page::Album(album.id.clone())));
                    }
                }
            }
        }
        LibraryTab::Playlists => {
            let playlists = app.library.playlists.get().cloned().unwrap_or_default();
            if playlists.is_empty() {
                ui.label(egui::RichText::new("No personal mixtapes found.").font(fonts::body()).color(INK_MUTED));
            } else {
                for (idx, pl) in playlists.iter().enumerate() {
                    let card_tint = CatalogTint::ALL[idx % CatalogTint::ALL.len()];
                    let art_url = pick_image(&pl.images, 200).map(str::to_string);
                    let curator = pl.owner.display_name.as_deref().unwrap_or("Spotify");
                    let total = pl.track_total();

                    let (resp, _) = ribbon_card(
                        ui,
                        &app.palette,
                        card_tint,
                        &pl.name,
                        art_url.as_deref(),
                        Some(app.backend.art()),
                        |ui| {
                            ui.label(egui::RichText::new(format!("CURATED BY: {curator}")).font(fonts::body()).color(INK_BLACK));
                            ui.label(egui::RichText::new(format!("TRACKS: {total}")).font(fonts::caption()).color(INK_BLACK));
                        },
                    );

                    if resp.clicked() {
                        app.actions.push(Action::Open(Page::Playlist(pl.id.clone())));
                    }
                }
            }
        }
        LibraryTab::LikedSongs => {
            let items = app.library.liked.items.clone();
            if items.is_empty() {
                ui.label(egui::RichText::new("No liked tracks in library yet.").font(fonts::body()).color(INK_MUTED));
            } else {
                primitives::track_table_header(ui);
                ui.add_space(2.0);
                for (idx, song) in items.iter().take(50).enumerate() {
                    let is_current = app.now_playing().as_ref().map(|n| n.uri.as_str()) == Some(song.track.uri.as_str());
                    let duration = format_duration(song.track.duration_ms);
                    let album_name = song.track.album.as_ref().map_or("", |a| a.name.as_str());
                    let resp = primitives::track_row(
                        ui,
                        idx + 1,
                        &song.track.name,
                        &join_names(song.track.artists.iter().map(|a| a.name.as_str())),
                        album_name,
                        &duration,
                        is_current,
                    );

                    if resp.clicked() {
                        app.actions.push(Action::PlayContext {
                            uri: "spotify:collection".into(),
                            offset_uri: Some(song.track.uri.clone()),
                            offset_index: Some(idx as u32),
                        });
                    }
                }
            }
        }
    }
}

fn format_duration(ms: u32) -> String {
    let s = ms / 1000;
    format!("{}:{:02}", s / 60, s % 60)
}
