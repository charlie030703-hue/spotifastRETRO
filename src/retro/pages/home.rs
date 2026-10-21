//! Retro Home & Browse view.
//!
//! Organized as catalog categories using the 8 Dell Ribbon tints,
//! with album covers proudly showcased on every card.

use egui::Ui;

use crate::api::models::{join_names, pick_image};
use crate::app::App;
use crate::model::{Action, Page};
use crate::retro::style::{
    fonts,
    primitives::{ribbon_card, section_eyebrow},
    tokens::*,
};

pub fn show(app: &mut App, ui: &mut Ui) {
    // 1. Top Section: Recent Discs
    section_eyebrow(ui, CatalogTint::Olive, "RECENTLY SPUN // QUICK CATALOG");

    let recents: Vec<(String, String, Option<String>, Page)> = app
        .recents
        .items
        .iter()
        .take(6)
        .map(|item| {
            let artists = join_names(item.track.artists.iter().map(|a| a.name.as_str()));
            let img = item
                .track
                .album
                .as_ref()
                .and_then(|a| pick_image(&a.images, 200))
                .map(str::to_string);
            let page = item
                .track
                .album
                .as_ref()
                .map(|a| Page::Album(a.id.clone()))
                .unwrap_or(Page::Home);
            (item.track.name.clone(), artists, img, page)
        })
        .collect();

    if recents.is_empty() {
        // Fallback placeholder tiles if recents haven't populated yet
        let defaults: [(&str, &str, CatalogTint); 3] = [
            ("Featured Mix Vol. 1", "Various Artists", CatalogTint::Salmon),
            ("Stereo Demonstration Disc", "High Fidelity Labs", CatalogTint::Peach),
            ("Top Hits 1996", "Chart Leaders", CatalogTint::Periwinkle),
        ];
        for &(name, artist, tint) in &defaults {
            ribbon_card(ui, &app.palette, tint, name, None, None, |ui| {
                ui.label(egui::RichText::new(artist).font(fonts::body()).color(INK_BLACK));
                ui.label(egui::RichText::new("HIGH QUALITY DIGITAL AUDIO").font(fonts::caption()).color(INK_BLACK));
            });
        }
    } else {
        for (idx, (name, artist, img, page)) in recents.into_iter().enumerate() {
            let card_tint = CatalogTint::ALL[idx % CatalogTint::ALL.len()];
            let (resp, _) = ribbon_card(
                ui,
                &app.palette,
                card_tint,
                &name,
                img.as_deref(),
                Some(app.backend.art()),
                |ui| {
                    ui.label(egui::RichText::new(format!("ARTIST: {artist}")).font(fonts::body()).color(INK_BLACK));
                    ui.label(egui::RichText::new("READY FOR HIGH FIDELITY PLAYBACK").font(fonts::caption()).color(INK_BLACK));
                },
            );
            if resp.clicked() {
                app.actions.push(Action::Open(page));
            }
        }
    }

    ui.add_space(SPACING_SECTION_SM);

    // 2. New Releases Section
    section_eyebrow(ui, CatalogTint::Salmon, "NEW RELEASES // HOT FROM THE PRESS");

    let new_albums: Vec<(String, String, String, Option<String>)> = app
        .home
        .new_releases
        .get()
        .map(|albums| {
            albums
                .iter()
                .take(4)
                .map(|album| {
                    let art = pick_image(&album.images, 200).map(str::to_string);
                    let artists = join_names(album.artists.iter().map(|a| a.name.as_str()));
                    (album.name.clone(), artists, album.id.clone(), art)
                })
                .collect()
        })
        .unwrap_or_default();

    if new_albums.is_empty() {
        ribbon_card(ui, &app.palette, CatalogTint::Lime, "Connecting to Spotify Broadcast...", None, None, |ui| {
            ui.label(egui::RichText::new("Catalog synchronization in progress.").font(fonts::body()).color(INK_BLACK));
        });
    } else {
        for (idx, (name, artist, id, img)) in new_albums.into_iter().enumerate() {
            let tint = CatalogTint::ALL[(idx + 2) % CatalogTint::ALL.len()];
            let (resp, _) = ribbon_card(
                ui,
                &app.palette,
                tint,
                &name,
                img.as_deref(),
                Some(app.backend.art()),
                |ui| {
                    ui.label(egui::RichText::new(format!("BY: {artist}")).font(fonts::body()).color(INK_BLACK));
                    ui.label(egui::RichText::new("FRESH PRESSING // 44.1 kHz 16-BIT").font(fonts::caption()).color(INK_BLACK));
                },
            );
            if resp.clicked() {
                app.actions.push(Action::Open(Page::Album(id)));
            }
        }
    }
}
