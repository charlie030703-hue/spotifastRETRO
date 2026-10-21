//! Authentic Dell 1996 UI primitives from DESIGN-dell-1996.md.
//!
//! Enforces the period-accurate catalog-era visual constraints:
//! - {rounded.none} (0px) everywhere (only award seals get rounded.full)
//! - No soft shadows or atmospheric gradients: hard 1px/2px bevels and hairlines
//! - Flat color-block ribbon cards with right-edge artwork notches
//! - Arial Black / Helvetica Bold headers, Times New Roman body copy
//! - Dell Red strictly limited to the CTA play block and top banner time readout

use egui::{
    pos2, vec2, Align2, Color32, CornerRadius, Pos2, Rect,
    Response, Sense, Stroke, Ui, Vec2,
};

use super::fonts;
use super::tokens::*;
use crate::theme::{Icon, Palette};

/// Renders the literal black outer frame around the entire application window.
pub fn paint_page_frame(ui: &Ui, window_rect: Rect) {
    let painter = ui.painter();
    // 8px solid black frame
    painter.rect_stroke(
        window_rect.shrink(PAGE_FRAME_PAD / 2.0),
        CornerRadius::ZERO,
        Stroke::new(PAGE_FRAME_PAD, FRAME_INK),
        egui::StrokeKind::Middle,
    );
}

/// Section eyebrow banner in one of the 8 catalog tints (e.g. "DIMENSION DESKTOPS").
pub fn section_eyebrow(ui: &mut Ui, tint: CatalogTint, title: &str) -> Rect {
    let width = ui.available_width();
    let height = 48.0;
    let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());

    let painter = ui.painter();
    // Flat tint color fill, no border or radius
    painter.rect_filled(rect, CornerRadius::ZERO, tint.color());

    // Chunky display font in black ink
    painter.text(
        pos2(rect.min.x + SPACING_LG, rect.center().y),
        Align2::LEFT_CENTER,
        title.to_uppercase(),
        fonts::heading_1(),
        INK_BLACK,
    );

    ui.add_space(SPACING_M);
    rect
}

/// Album artwork with period-accurate hard bevel framing (0px radius).
pub fn bevel_art(
    ui: &mut Ui,
    palette: &Palette,
    url: Option<&str>,
    size: f32,
    fallback: Icon,
    art: Option<&crate::images::ArtLoader>,
) -> Rect {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());

    // Draw hard bevel drop-shadow (2px bottom-right)
    let shadow_rect = Rect::from_min_size(rect.min + vec2(BEVEL_PX, BEVEL_PX), rect.size());
    ui.painter()
        .rect_filled(shadow_rect, CornerRadius::ZERO, BEVEL_SHADOW);

    // Paint the cover image with sharp 0px corners
    crate::ui::widgets::paint_cover(ui, palette, url, rect, 0.0, fallback, art);

    // 1px hairline border around cover
    ui.painter()
        .rect_stroke(rect, CornerRadius::ZERO, Stroke::new(1.0, FRAME_INK), egui::StrokeKind::Inside);

    rect
}

/// The signature Dell 1996 Ribbon Card:
/// 1. Title bar (black/dark in dark mode, 1px bottom border, Helvetica Bold caps)
/// 2. Body (flat catalog tint fill, Times Roman 14, black ink, 1px black outline)
/// 3. Photo notch on right edge hanging above and below body with a 2px bevel drop-shadow
pub fn ribbon_card<R>(
    ui: &mut Ui,
    palette: &Palette,
    tint: CatalogTint,
    title: &str,
    cover_url: Option<&str>,
    art: Option<&crate::images::ArtLoader>,
    content: impl FnOnce(&mut Ui) -> R,
) -> (Response, R) {
    let total_width = ui.available_width();
    let body_height = 84.0;
    let card_height = RIBBON_TITLE_HEIGHT + body_height;

    let (outer_rect, response) = ui.allocate_exact_size(vec2(total_width, card_height), Sense::click());
    let painter = ui.painter();

    let title_rect = Rect::from_min_size(outer_rect.min, vec2(total_width, RIBBON_TITLE_HEIGHT));
    let body_rect = Rect::from_min_size(
        pos2(outer_rect.min.x, outer_rect.min.y + RIBBON_TITLE_HEIGHT),
        vec2(total_width, body_height),
    );

    // 1. Title Bar
    painter.rect_filled(title_rect, CornerRadius::ZERO, RIBBON_TITLE_BG);
    painter.rect_stroke(title_rect, CornerRadius::ZERO, Stroke::new(1.0, FRAME_INK), egui::StrokeKind::Inside);
    painter.text(
        pos2(title_rect.min.x + SPACING_MD, title_rect.center().y),
        Align2::LEFT_CENTER,
        title.to_uppercase(),
        fonts::heading_3(),
        RIBBON_TITLE_TEXT,
    );

    // 2. Body
    painter.rect_filled(body_rect, CornerRadius::ZERO, tint.color());
    painter.rect_stroke(body_rect, CornerRadius::ZERO, Stroke::new(1.0, FRAME_INK), egui::StrokeKind::Inside);

    // Right-edge artwork notch: sits in right 20-25% and hangs 10px above and below body
    let art_size = body_height + RIBBON_PHOTO_NOTCH_OVERHANG * 2.0;
    let art_rect = Rect::from_min_size(
        pos2(
            outer_rect.max.x - art_size - SPACING_LG,
            body_rect.min.y - RIBBON_PHOTO_NOTCH_OVERHANG,
        ),
        Vec2::splat(art_size),
    );

    // Art hard bevel shadow
    let shadow_rect = Rect::from_min_size(art_rect.min + vec2(BEVEL_PX, BEVEL_PX), art_rect.size());
    painter.rect_filled(shadow_rect, CornerRadius::ZERO, BEVEL_SHADOW);

    // Paint cover
    crate::ui::widgets::paint_cover(ui, palette, cover_url, art_rect, 0.0, Icon::Disc, art);
    painter.rect_stroke(art_rect, CornerRadius::ZERO, Stroke::new(1.0, FRAME_INK), egui::StrokeKind::Inside);

    // Inner text area (left of artwork)
    let text_area = Rect::from_min_max(
        body_rect.min + vec2(SPACING_LG, SPACING_MD),
        pos2(art_rect.min.x - SPACING_MD, body_rect.max.y - SPACING_MD),
    );

    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(text_area));
    let res = content(&mut child_ui);

    ui.add_space(SPACING_MD);
    (response, res)
}

/// The singular Dell Red CTA block ("PLAY THIS RECORD").
/// Exactly one per screen maximum to preserve high urgency.
pub fn cta_block_red(ui: &mut Ui, title: &str, subtitle: &str) -> Response {
    let width = ui.available_width();
    let height = 54.0;
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());

    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::ZERO, PRIMARY);
    painter.rect_stroke(rect, CornerRadius::ZERO, Stroke::new(1.0, FRAME_INK), egui::StrokeKind::Inside);

    painter.text(
        pos2(rect.min.x + SPACING_LG, rect.min.y + 16.0),
        Align2::LEFT_CENTER,
        title.to_uppercase(),
        fonts::heading_2(),
        ON_PRIMARY,
    );

    painter.text(
        pos2(rect.min.x + SPACING_LG, rect.min.y + 36.0),
        Align2::LEFT_CENTER,
        subtitle,
        fonts::body_sm(),
        Color32::from_rgb(0xff, 0xf0, 0xee),
    );

    ui.add_space(SPACING_MD);
    response
}

/// Yellow "PLAY a RECORD" / "BUY a DELL" sticker with purple accent stripe.
pub fn play_a_record_sticker(ui: &mut Ui) -> Response {
    let size = vec2(110.0, 26.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let painter = ui.painter();

    // Yellow background
    painter.rect_filled(rect, CornerRadius::ZERO, YELLOW_STICKER);
    painter.rect_stroke(rect, CornerRadius::ZERO, Stroke::new(1.0, FRAME_INK), egui::StrokeKind::Inside);

    // Purple stripe behind the lowercase "a"
    let stripe_rect = Rect::from_min_size(pos2(rect.min.x + 40.0, rect.min.y + 2.0), vec2(14.0, 22.0));
    painter.rect_filled(stripe_rect, CornerRadius::ZERO, PURPLE_STRIPE);

    painter.text(
        pos2(rect.min.x + 6.0, rect.center().y),
        Align2::LEFT_CENTER,
        "PLAY",
        fonts::button(),
        INK_BLACK,
    );

    painter.text(
        pos2(stripe_rect.center().x, stripe_rect.center().y),
        Align2::CENTER_CENTER,
        "a",
        fonts::button(),
        ON_PRIMARY,
    );

    painter.text(
        pos2(rect.min.x + 58.0, rect.center().y),
        Align2::LEFT_CENTER,
        "RECORD",
        fonts::button(),
        INK_BLACK,
    );

    response
}

/// Angled yellow "NEW!" burst sticker.
pub fn new_burst_sticker(ui: &Ui, pos: Pos2) {
    let painter = ui.painter();
    let size = vec2(44.0, 20.0);
    let rect = Rect::from_center_size(pos, size);

    // Hard bevel drop-shadow
    painter.rect_filled(
        Rect::from_min_size(rect.min + vec2(1.5, 1.5), size),
        CornerRadius::ZERO,
        BEVEL_SHADOW,
    );

    painter.rect_filled(rect, CornerRadius::ZERO, YELLOW_STICKER);
    painter.rect_stroke(rect, CornerRadius::ZERO, Stroke::new(1.0, FRAME_INK), egui::StrokeKind::Inside);

    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        "NEW!",
        fonts::button(),
        INK_BLACK,
    );
}

/// Round red award / certified seal ("TOP 10").
pub fn cert_seal(ui: &mut Ui, text: &str) -> Response {
    let size = 52.0;
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let painter = ui.painter();

    // Outer circle
    painter.circle_filled(rect.center(), size / 2.0, PRIMARY);
    painter.circle_stroke(rect.center(), size / 2.0, Stroke::new(2.0, FRAME_INK));

    // Inner ring
    painter.circle_stroke(rect.center(), size / 2.0 - 4.0, Stroke::new(1.0, ON_PRIMARY));

    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        text,
        fonts::button(),
        ON_PRIMARY,
    );

    response
}

/// Classic 1996 button primary: pure black filled, 1px border, white Helvetica Bold uppercase.
pub fn button_primary(ui: &mut Ui, label: &str) -> Response {
    let padding = vec2(SPACING_LG, SPACING_S);
    let galley = ui.painter().layout_no_wrap(
        label.to_uppercase(),
        fonts::button(),
        ON_PRIMARY,
    );
    let size = galley.size() + padding * 2.0;
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    let painter = ui.painter();
    let fill = if response.is_pointer_button_down_on() {
        SURFACE_ACTIVE
    } else {
        FRAME_INK
    };

    painter.rect_filled(rect, CornerRadius::ZERO, fill);
    painter.rect_stroke(rect, CornerRadius::ZERO, Stroke::new(1.0, FRAME_INK), egui::StrokeKind::Inside);
    painter.galley(rect.min + padding, galley, ON_PRIMARY);

    response
}

/// Classic 1996 button secondary: white/light filled, 1px black border, black label.
pub fn button_secondary(ui: &mut Ui, label: &str) -> Response {
    let padding = vec2(SPACING_LG, SPACING_S);
    let galley = ui.painter().layout_no_wrap(
        label.to_uppercase(),
        fonts::button(),
        INK_BLACK,
    );
    let size = galley.size() + padding * 2.0;
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::ZERO, Color32::WHITE);
    painter.rect_stroke(rect, CornerRadius::ZERO, Stroke::new(1.0, FRAME_INK), egui::StrokeKind::Inside);
    painter.galley(rect.min + padding, galley, INK_BLACK);

    response
}

/// Classic Mosaic / Netscape underlined link in Times Roman.
pub fn text_link(ui: &mut Ui, text: &str, in_ribbon: bool) -> Response {
    let color = if in_ribbon { LINK_RIBBON } else { LINK };
    let galley = ui.painter().layout_no_wrap(text.to_string(), fonts::body(), color);
    let (rect, response) = ui.allocate_exact_size(galley.size(), Sense::click());

    let painter = ui.painter();
    painter.galley(rect.min, galley, color);
    // Draw period-accurate underline
    painter.line_segment(
        [
            pos2(rect.min.x, rect.max.y),
            pos2(rect.max.x, rect.max.y),
        ],
        Stroke::new(1.0, color),
    );

    response
}

/// Track list data table row.
pub fn track_row(
    ui: &mut Ui,
    number: usize,
    title: &str,
    artist: &str,
    album: &str,
    duration: &str,
    is_playing: bool,
) -> Response {
    let width = ui.available_width();
    let height = 28.0;
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());
    let painter = ui.painter();

    let bg = if is_playing {
        SURFACE_HOVER
    } else if response.hovered() {
        SURFACE
    } else {
        Color32::TRANSPARENT
    };

    if bg != Color32::TRANSPARENT {
        painter.rect_filled(rect, CornerRadius::ZERO, bg);
    }

    // 1px bottom border
    painter.line_segment(
        [pos2(rect.min.x, rect.max.y), pos2(rect.max.x, rect.max.y)],
        Stroke::new(1.0, Color32::from_rgb(0x30, 0x30, 0x30)),
    );

    let text_color = if is_playing { PRIMARY } else { INK };

    // Track number
    painter.text(
        pos2(rect.min.x + SPACING_SM, rect.center().y),
        Align2::LEFT_CENTER,
        format!("{:02}", number),
        fonts::caption(),
        INK_MUTED,
    );

    // Title
    painter.text(
        pos2(rect.min.x + 40.0, rect.center().y),
        Align2::LEFT_CENTER,
        title,
        fonts::body_sm(),
        text_color,
    );

    // Artist
    painter.text(
        pos2(rect.min.x + width * 0.45, rect.center().y),
        Align2::LEFT_CENTER,
        artist,
        fonts::body_sm(),
        INK_MUTED,
    );

    // Album
    painter.text(
        pos2(rect.min.x + width * 0.72, rect.center().y),
        Align2::LEFT_CENTER,
        album,
        fonts::body_sm(),
        INK_MUTED,
    );

    // Duration
    painter.text(
        pos2(rect.max.x - SPACING_MD, rect.center().y),
        Align2::RIGHT_CENTER,
        duration,
        fonts::caption(),
        INK_MUTED,
    );

    response
}
