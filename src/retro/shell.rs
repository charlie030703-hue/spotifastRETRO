//! Dell 1996 Retro Application Shell.
//!
//! Orchestrates the outer 8px black frame, the black top banner with digital
//! time readout and "PLAY a RECORD" tab, the 2x4 icon navigation rail with
//! prominent album artwork, the central view container, and the footer transport bar.

use egui::{
    pos2, vec2, Align, Align2, CornerRadius, Rect, Sense, Stroke, Ui,
};

use crate::app::App;
use crate::model::{Action, Page};
use crate::retro::pages;
use crate::retro::style::{
    fonts,
    primitives::{self, bevel_art, paint_page_frame, play_a_record_sticker},
    tokens::*,
};
use crate::theme::Icon;

pub fn show(app: &mut App, ui: &mut Ui) {
    let window_rect = ui.max_rect();

    // 1. Paint literal 8px black border around window viewport
    paint_page_frame(ui, window_rect);

    // The usable canvas area inside the 8px frame
    let canvas_rect = window_rect.shrink(PAGE_FRAME_PAD);

    // Handle window dragging via top banner
    let banner_rect = Rect::from_min_size(canvas_rect.min, vec2(canvas_rect.width(), TOP_BANNER_HEIGHT));
    fastframe_titlebar::drag(ui, banner_rect);

    // 2. Render Top Banner
    render_top_banner(app, ui, banner_rect);

    // 3. Bottom Player Bar
    let player_rect = Rect::from_min_size(
        pos2(canvas_rect.min.x, canvas_rect.max.y - FOOTER_PLAYER_HEIGHT),
        vec2(canvas_rect.width(), FOOTER_PLAYER_HEIGHT),
    );
    pages::player::show(app, ui, player_rect);

    // 4. Left Navigation Rail
    let content_top = banner_rect.max.y;
    let content_bottom = player_rect.min.y;
    let content_height = content_bottom - content_top;

    let left_rail_rect = Rect::from_min_size(
        pos2(canvas_rect.min.x, content_top),
        vec2(LEFT_NAV_WIDTH, content_height),
    );
    render_left_rail(app, ui, left_rail_rect);

    // 5. Central View Container
    let central_rect = Rect::from_min_max(
        pos2(left_rail_rect.max.x, content_top),
        pos2(canvas_rect.max.x, content_bottom),
    );
    render_central_view(app, ui, central_rect);

    // Handle window resize edges
    fastframe_titlebar::resize(ui);
}

fn render_top_banner(app: &mut App, ui: &mut Ui, rect: Rect) {
    {
        let painter = ui.painter();
        painter.rect_filled(rect, CornerRadius::ZERO, FRAME_INK);
        painter.line_segment([rect.left_bottom(), rect.right_bottom()], Stroke::new(1.0, BEVEL_HIGHLIGHT));

        // Wordmark: SPOTIFAST RETRO
        painter.text(
            pos2(rect.min.x + SPACING_LG, rect.center().y),
            Align2::LEFT_CENTER,
            "SPOTIFAST RETRO",
            fonts::heading_1(),
            INK,
        );

        // Tagline in Helvetica caps
        painter.text(
            pos2(rect.min.x + 230.0, rect.center().y + 1.0),
            Align2::LEFT_CENTER,
            "BUILD YOUR OWN PLAYLIST. ONLINE.",
            fonts::button(),
            INK_MUTED,
        );
    }

    // Center/Right: "PLAY a RECORD" yellow button
    let sticker_rect = Rect::from_center_size(pos2(rect.center().x + 60.0, rect.center().y), vec2(110.0, 26.0));
    let mut sticker_ui = ui.new_child(egui::UiBuilder::new().max_rect(sticker_rect));
    if play_a_record_sticker(&mut sticker_ui).clicked() {
        app.actions.push(Action::Open(Page::Search));
    }

    // Elapsed / Total Time Readout in Dell Red (#e91d2a)
    let now = app.now_playing();
    let (pos_ms, dur_ms) = now.as_ref().map_or((0, 0), |n| (n.position_ms, n.duration_ms));
    let time_str = if dur_ms > 0 {
        format!("{}:{:02} / {}:{:02}", pos_ms / 60000, (pos_ms % 60000) / 1000, dur_ms / 60000, (dur_ms % 60000) / 1000)
    } else {
        "1-800-SPOTIFAST".into()
    };

    let time_rect = Rect::from_min_size(pos2(rect.max.x - 280.0, rect.min.y + 10.0), vec2(150.0, 24.0));
    let painter = ui.painter();
    painter.rect_filled(time_rect, CornerRadius::ZERO, FRAME_INK);
    painter.text(
        pos2(time_rect.center().x, time_rect.center().y),
        Align2::CENTER_CENTER,
        time_str,
        fonts::clock(),
        PRIMARY,
    );

    // Window Caption Controls
    let controls_rect = Rect::from_min_size(pos2(rect.max.x - 110.0, rect.min.y + 8.0), vec2(100.0, 28.0));
    let mut controls_ui = ui.new_child(egui::UiBuilder::new().max_rect(controls_rect));
    render_caption_buttons(app, &mut controls_ui);
}

fn render_caption_buttons(app: &mut App, ui: &mut Ui) {
    ui.horizontal(|ui| {
        if primitives::button_secondary(ui, "_").clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }
        ui.add_space(2.0);
        if primitives::button_secondary(ui, "[]").clicked() {
            let maxed = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!maxed));
        }
        ui.add_space(2.0);
        if primitives::button_primary(ui, "X").clicked() {
            app.actions.push(Action::CloseWindow);
        }
    });
}

fn render_left_rail(app: &mut App, ui: &mut Ui, rect: Rect) {
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::ZERO, CANVAS);
    painter.line_segment([rect.right_top(), rect.right_bottom()], Stroke::new(1.0, BEVEL_HIGHLIGHT));

    let mut rail_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(SPACING_MD, SPACING_MD)))
            .layout(egui::Layout::top_down(Align::Center)),
    );

    // 2x4 Navigation Tile Grid
    let grid_items = [
        ("HOME", Page::Home),
        ("FIND", Page::Search),
        ("ALBUMS", Page::Albums),
        ("LIKED", Page::LikedSongs),
        ("QUEUE", Page::Queue),
        ("LYRICS", Page::TopSongs), // Links to Now Playing / Lyrics hero
    ];

    for chunk in grid_items.chunks(2) {
        rail_ui.horizontal(|ui| {
            for (label, target_page) in chunk {
                let is_active = app.page() == target_page;
                let btn = if is_active {
                    primitives::button_primary(ui, label)
                } else {
                    primitives::button_secondary(ui, label)
                };
                if btn.clicked() {
                    app.actions.push(Action::Open(target_page.clone()));
                }
                ui.add_space(SPACING_XS);
            }
        });
        rail_ui.add_space(SPACING_S);
    }

    // Hardware & Visuals Actions
    rail_ui.horizontal(|ui| {
        if primitives::button_secondary(ui, "DEVICES").clicked() {
            app.actions.push(Action::RefreshDevices);
        }
        ui.add_space(SPACING_XS);
        if primitives::button_secondary(ui, "WINAMP").clicked() {
            app.actions.push(Action::ToggleWinampWindow);
        }
    });

    rail_ui.add_space(SPACING_LG);

    // Prominent Now Playing Album Art in Left Rail
    let now = app.now_playing();
    let (cover_url, title, artist) = match now.as_ref() {
        Some(n) => (n.art_url.as_deref(), n.title.as_str(), n.subtitle.as_str()),
        None => (None, "Phonograph Idle", "Spotifast Sound Card"),
    };

    let art_size = 190.0;
    let art_resp = bevel_art(
        &mut rail_ui,
        &app.palette,
        cover_url,
        art_size,
        Icon::Disc,
        Some(app.backend.art()),
    );

    // Clicking the cover opens the Now Playing Hero page
    let click_area = rail_ui.interact(art_resp, egui::Id::new("left-art-click"), Sense::click());
    if click_area.clicked() {
        app.actions.push(Action::Open(Page::TopSongs));
    }

    rail_ui.add_space(SPACING_MD);

    rail_ui.label(egui::RichText::new(title.to_uppercase()).font(fonts::heading_3()).color(INK));
    rail_ui.label(egui::RichText::new(artist).font(fonts::caption()).color(INK_MUTED));
}

fn render_central_view(app: &mut App, ui: &mut Ui, rect: Rect) {
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::ZERO, CANVAS);

    let mut central_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(Align::Min)),
    );

    egui::ScrollArea::vertical()
        .id_salt("retro-central-scroll")
        .auto_shrink([false, false])
        .show(&mut central_ui, |ui| {
            ui.add_space(SPACING_LG);
            let page = app.page().clone();

            match page {
                Page::Home => pages::home::show(app, ui),
                Page::Search => pages::search::show(app, ui),
                Page::Albums | Page::Artists => pages::library::show(app, ui, pages::library::LibraryTab::Albums),
                Page::LikedSongs => pages::library::show(app, ui, pages::library::LibraryTab::LikedSongs),
                Page::Album(ref id) => pages::collection::show_album(app, ui, id),
                Page::Playlist(ref id) => pages::collection::show_playlist(app, ui, id),
                Page::Queue => pages::queue::show(app, ui),
                Page::TopSongs => pages::now_playing::show(app, ui),
                Page::Settings => crate::ui::settings::show(app, ui), // Keep settings reachable
                _ => pages::home::show(app, ui),
            }

            ui.add_space(SPACING_XXL);
        });
}
