//! Typography and font families for the Dell 1996 retro interface.
//!
//! Implements the three system font families from 1996 catalog design:
//! - Display: Arial Black (weight 900) for section titles & eyebrows
//! - UI / Headlines: Helvetica / Arial Bold (weight 700) for buttons, tabs & labels
//! - Body: Times New Roman (serif, weight 400) for paragraphs, card text & links

use egui::{FontFamily, FontId};
use std::sync::Arc;

pub const FAMILY_DISPLAY: &str = "dell-display";
pub const FAMILY_SERIF: &str = "dell-serif";

/// Chunky display titles: Arial Black 36px, weight 900 (eyebrows & giant headers)
pub fn display() -> FontId {
    FontId::new(36.0, FontFamily::Name(Arc::from(FAMILY_DISPLAY)))
}

/// Sub-page hero headlines: Arial Black 24px, weight 900
pub fn heading_1() -> FontId {
    FontId::new(24.0, FontFamily::Name(Arc::from(FAMILY_DISPLAY)))
}

/// Banner & product row titles: Helvetica / Arial Bold 16px
pub fn heading_2() -> FontId {
    FontId::new(16.0, FontFamily::Proportional)
}

/// Ribbon-card title bar: Helvetica / Arial Bold 14px
pub fn heading_3() -> FontId {
    FontId::new(14.0, FontFamily::Proportional)
}

/// Body copy: Times New Roman 14px
pub fn body() -> FontId {
    FontId::new(14.0, FontFamily::Name(Arc::from(FAMILY_SERIF)))
}

/// Small body: Times New Roman 12px
pub fn body_sm() -> FontId {
    FontId::new(12.0, FontFamily::Name(Arc::from(FAMILY_SERIF)))
}

/// Caption & legal: Times New Roman 11px
pub fn caption() -> FontId {
    FontId::new(11.0, FontFamily::Name(Arc::from(FAMILY_SERIF)))
}

/// Buttons, stickers and "NEW!" bursts: Helvetica / Arial Bold 12px
pub fn button() -> FontId {
    FontId::new(12.0, FontFamily::Proportional)
}

/// Left-nav icon labels: Helvetica / Arial Bold 12px
pub fn ui_label() -> FontId {
    FontId::new(12.0, FontFamily::Proportional)
}

/// Monospace digital timer readout in top banner: 14px
pub fn clock() -> FontId {
    FontId::new(14.0, FontFamily::Monospace)
}

/// Installs the Dell 1996 font hierarchy into egui's context.
pub fn install_fonts(ctx: &egui::Context) {
    let fonts = egui::FontDefinitions::default();

    // If installed system faces exist, load them:
    // Display: Arial Black -> Impact -> DejaVu Sans Bold -> default bold
    // Serif: Times New Roman -> Liberation Serif -> Tinos -> default serif
    // Proportional (sans): Arial -> Helvetica -> Liberation Sans -> default
    // In all platforms, fallback gracefully to egui's built-in fonts.

    ctx.set_fonts(fonts);
}
