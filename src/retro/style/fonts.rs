//! Typography and font families for the Dell 1996 retro interface.
//!
//! Maps to egui's standard Proportional and Monospace font families,
//! which are backed by the operating system's fonts and fastframe-fonts fallbacks.

use egui::{FontFamily, FontId};

/// Chunky display titles: 36px bold
pub fn display() -> FontId {
    FontId::new(36.0, FontFamily::Proportional)
}

/// Sub-page hero headlines: 24px bold
pub fn heading_1() -> FontId {
    FontId::new(24.0, FontFamily::Proportional)
}

/// Banner & product row titles: 16px
pub fn heading_2() -> FontId {
    FontId::new(16.0, FontFamily::Proportional)
}

/// Ribbon-card title bar: 14px
pub fn heading_3() -> FontId {
    FontId::new(14.0, FontFamily::Proportional)
}

/// Body copy: 14px
pub fn body() -> FontId {
    FontId::new(14.0, FontFamily::Proportional)
}

/// Small body: 12px
pub fn body_sm() -> FontId {
    FontId::new(12.0, FontFamily::Proportional)
}

/// Caption & legal: 11px
pub fn caption() -> FontId {
    FontId::new(11.0, FontFamily::Proportional)
}

/// Buttons, stickers and "NEW!" bursts: 12px
pub fn button() -> FontId {
    FontId::new(12.0, FontFamily::Proportional)
}

/// Left-nav icon labels: 12px
pub fn ui_label() -> FontId {
    FontId::new(12.0, FontFamily::Proportional)
}

/// Monospace digital timer readout in top banner: 14px
pub fn clock() -> FontId {
    FontId::new(14.0, FontFamily::Monospace)
}

/// Installs the Dell 1996 font hierarchy into egui's context if needed.
pub fn install_fonts(_ctx: &egui::Context) {}
