//! Retro Lyrics view.
//!
//! Renders track lyrics with Times Roman body type, highlighting
//! the current sung line in bold Dell Yellow (#fcc20f) or Dell Red (#e91d2a).

use egui::Ui;

use crate::app::App;
use crate::model::Loadable;
use crate::retro::style::{
    fonts,
    primitives::section_eyebrow,
    tokens::*,
};

pub fn show(app: &mut App, ui: &mut Ui) {
    section_eyebrow(ui, CatalogTint::Peach, "SONG LYRICS // TRANSCRIPTION");

    let now = app.now_playing();
    let Some(now) = now else {
        ui.label(egui::RichText::new("Nothing playing.").font(fonts::body()).color(INK_MUTED));
        return;
    };

    let lyrics = match &app.lyrics {
        Loadable::NotLoaded | Loadable::Loading => {
            ui.label(egui::RichText::new("Retrieving synchronized lyric sheet...").font(fonts::body()).color(INK_MUTED));
            return;
        }
        Loadable::Failed(err) => {
            ui.label(egui::RichText::new(format!("Transcription unavailable: {err}")).font(fonts::body()).color(PRIMARY));
            return;
        }
        Loadable::Loaded(None) => {
            ui.label(egui::RichText::new("No lyric sheet on file for this master.").font(fonts::body()).color(INK_MUTED));
            return;
        }
        Loadable::Loaded(Some(lyrics)) if lyrics.instrumental => {
            ui.label(egui::RichText::new("Instrumental track. No vocal lyrics recorded.").font(fonts::body()).color(INK_MUTED));
            return;
        }
        Loadable::Loaded(Some(lyrics)) => lyrics.clone(),
    };

    let active_idx = lyrics.active_line(now.position_ms);

    ui.vertical_centered(|ui| {
        for (idx, line) in lyrics.lines.iter().enumerate() {
            let is_active = Some(idx) == active_idx;
            if is_active {
                ui.label(
                    egui::RichText::new(&line.text)
                        .font(fonts::heading_2())
                        .color(YELLOW_STICKER),
                );
            } else {
                ui.label(
                    egui::RichText::new(&line.text)
                        .font(fonts::body())
                        .color(INK_MUTED),
                );
            }
            ui.add_space(SPACING_S);
        }
    });
}
