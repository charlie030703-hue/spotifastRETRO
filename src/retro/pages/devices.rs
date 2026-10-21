//! Retro Spotify Connect & Hardware Output devices view.

use egui::Ui;

use crate::app::App;
use crate::model::Action;
use crate::retro::style::{
    fonts,
    primitives::{self, ribbon_card, section_eyebrow},
    tokens::*,
};

pub fn show(app: &mut App, ui: &mut Ui) {
    section_eyebrow(ui, CatalogTint::Olive, "SOUND OUTPUT // CONNECT HARDWARE");

    ui.horizontal(|ui| {
        if primitives::button_secondary(ui, "REFRESH BROADCAST DEVICES").clicked() {
            app.actions.push(Action::RefreshDevices);
        }
    });

    ui.add_space(SPACING_LG);

    // 1. This Computer (Local Sound Card)
    let this_comp_name = format!("{} (Sound Blaster 16 / Host)", app.settings.device_name);
    let (resp, _) = ribbon_card(
        ui,
        &app.palette,
        CatalogTint::Lime,
        &this_comp_name,
        None,
        None,
        |ui| {
            ui.label(egui::RichText::new("INTERNAL AUDIO ADAPTER // DMA CHANNEL 1").font(fonts::body()).color(INK_BLACK));
            ui.label(egui::RichText::new("LOCAL LOW-LATENCY OUTPUT READY").font(fonts::caption()).color(INK_BLACK));
        },
    );

    if resp.clicked() {
        app.actions.push(Action::SetAudioDevice(None));
    }

    ui.add_space(SPACING_MD);

    // 2. Discovered Network Receivers / Spotify Connect
    let devices = app.devices.clone();
    if devices.is_empty() {
        ui.label(egui::RichText::new("No external Spotify Connect appliances found on LAN.").font(fonts::body()).color(INK_MUTED));
        return;
    }

    for (idx, dev) in devices.iter().enumerate() {
        let tint = CatalogTint::ALL[(idx + 3) % CatalogTint::ALL.len()];
        let status = if dev.is_active {
            "ACTIVE TARGET // AUDIO STREAMING"
        } else {
            "AVAILABLE VIA NETWORK BROADCAST"
        };

        let (resp, _) = ribbon_card(
            ui,
            &app.palette,
            tint,
            &dev.name,
            None,
            None,
            |ui| {
                ui.label(egui::RichText::new(format!("TYPE: {} // VOLUME: {}%", dev.kind, dev.volume_percent.unwrap_or(100))).font(fonts::body()).color(INK_BLACK));
                ui.label(egui::RichText::new(status).font(fonts::caption()).color(INK_BLACK));
            },
        );

        if resp.clicked() {
            if let Some(id) = &dev.id {
                app.actions.push(Action::Transfer(id.clone()));
            }
        }
    }
}
