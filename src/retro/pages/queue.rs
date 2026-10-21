//! Playback Queue view in Dell 1996 style.

use egui::Ui;

use crate::app::App;
use crate::model::Action;
use crate::retro::style::{
    fonts,
    primitives::{self, section_eyebrow},
    tokens::*,
};

pub fn show(app: &mut App, ui: &mut Ui) {
    section_eyebrow(ui, CatalogTint::Steel, "PLAYBACK QUEUE // UPCOMING PROGRAM");

    ui.horizontal(|ui| {
        if primitives::button_secondary(ui, "CLEAR UPCOMING QUEUE").clicked() {
            app.actions.push(Action::ClearQueue);
        }
        ui.add_space(SPACING_SM);
        if primitives::button_secondary(ui, "REFRESH QUEUE").clicked() {
            app.actions.push(Action::RefreshQueue);
        }
    });

    ui.add_space(SPACING_LG);

    let items = app.queue.get().map(|q| q.queue.as_slice()).unwrap_or(&[]);
    if items.is_empty() {
        ui.label(egui::RichText::new("Queue is empty. Select songs or albums to add.").font(fonts::body()).color(INK_MUTED));
        return;
    }

    for (idx, song) in items.iter().enumerate() {
        let is_playing = idx == 0 && app.believed_playing();
        let dur_ms = song.duration_ms();
        let duration = format!("{}:{:02}", dur_ms / 60000, (dur_ms % 60000) / 1000);
        let resp = primitives::track_row(
            ui,
            idx + 1,
            song.name(),
            &song.subtitle(),
            "",
            &duration,
            is_playing,
        );

        if resp.clicked() {
            app.actions.push(Action::PlayUris {
                uris: vec![song.uri().to_string()],
                index: 0,
            });
        }
    }
}
