//! Design tokens from the Dell 1996 design reference (DESIGN-dell-1996.md)
//! adapted for the Spotifast Retro dark-mode interface.

use egui::Color32;

// ─── Colors ───

/// Dell Red signature accent (#e91d2a).
/// Per design rules: strictly reserved for the page's singular play CTA block
/// and the top banner's elapsed time readout. Never used as general decoration.
pub const PRIMARY: Color32 = Color32::from_rgb(0xe9, 0x1d, 0x2a);

/// Contrast text on Dell Red (#ffffff).
pub const ON_PRIMARY: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);

/// Pure black frame (#000000).
/// Used for the 8px page frame border, top banner background, and dark hairline borders.
pub const FRAME_INK: Color32 = Color32::from_rgb(0x00, 0x00, 0x00);

/// CRT off-black canvas (#1c1c1c).
/// Gives contrast against the literal 8px pure-black outer frame.
pub const CANVAS: Color32 = Color32::from_rgb(0x1c, 0x1c, 0x1c);

/// Surface fill (#252525) for panels, cards, and input backgrounds.
pub const SURFACE: Color32 = Color32::from_rgb(0x25, 0x25, 0x25);

/// Surface hover state (#323232).
pub const SURFACE_HOVER: Color32 = Color32::from_rgb(0x32, 0x32, 0x32);

/// Surface active / depressed state (#181818).
pub const SURFACE_ACTIVE: Color32 = Color32::from_rgb(0x18, 0x18, 0x18);

/// Text color on dark surfaces (#f2f2f2).
pub const INK: Color32 = Color32::from_rgb(0xf2, 0xf2, 0xf2);

/// Muted text color (#a4a4a4) for secondary metadata and timestamps.
pub const INK_MUTED: Color32 = Color32::from_rgb(0xa4, 0xa4, 0xa4);

/// Black ink (#000000) used inside the tinted ribbon card bodies.
/// Ribbon tints are mid-tones, preserving original 1996 catalog readability.
pub const INK_BLACK: Color32 = Color32::from_rgb(0x00, 0x00, 0x00);

/// Yellow sticker (#fcc20f) for the "BUY a DELL" / "PLAY a RECORD" badge and "NEW!" bursts.
pub const YELLOW_STICKER: Color32 = Color32::from_rgb(0xfc, 0xc2, 0x0f);

/// Purple stripe accent (#6a26a4) inside the sticker chrome.
pub const PURPLE_STRIPE: Color32 = Color32::from_rgb(0x6a, 0x26, 0xa4);

/// Link blue on dark background (#8c9ae0 - periwinkle tint for optimal WCAG contrast).
pub const LINK: Color32 = Color32::from_rgb(0x8c, 0x9a, 0xe0);

/// Link blue inside ribbon card body (#0000ee - classic Netscape 3.x Mosaic link).
pub const LINK_RIBBON: Color32 = Color32::from_rgb(0x00, 0x00, 0xee);

/// Hard bevel highlight on dark surfaces (#555555).
pub const BEVEL_HIGHLIGHT: Color32 = Color32::from_rgb(0x55, 0x55, 0x55);

/// Hard bevel shadow (#000000).
pub const BEVEL_SHADOW: Color32 = Color32::from_rgb(0x00, 0x00, 0x00);

/// Ribbon-card title bar background in dark mode (#0a0a0a).
pub const RIBBON_TITLE_BG: Color32 = Color32::from_rgb(0x0a, 0x0a, 0x0a);

/// Ribbon-card title bar text (#ffffff).
pub const RIBBON_TITLE_TEXT: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);

// ─── The 8 Ribbon-Card Catalogue Tints ───

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CatalogTint {
    Olive,
    Sage,
    Salmon,
    Peach,
    Lime,
    Sky,
    Steel,
    Periwinkle,
}

impl CatalogTint {
    pub const ALL: [CatalogTint; 8] = [
        CatalogTint::Olive,
        CatalogTint::Sage,
        CatalogTint::Salmon,
        CatalogTint::Peach,
        CatalogTint::Lime,
        CatalogTint::Sky,
        CatalogTint::Steel,
        CatalogTint::Periwinkle,
    ];

    /// Returns the exact RGB Color32 for this tint.
    pub const fn color(&self) -> Color32 {
        match self {
            CatalogTint::Olive => Color32::from_rgb(0x8e, 0x8a, 0x25),
            CatalogTint::Sage => Color32::from_rgb(0xb3, 0xbd, 0x95),
            CatalogTint::Salmon => Color32::from_rgb(0xd7, 0x7a, 0x7a),
            CatalogTint::Peach => Color32::from_rgb(0xe6, 0x91, 0x5d),
            CatalogTint::Lime => Color32::from_rgb(0xc0, 0xd4, 0xa7),
            CatalogTint::Sky => Color32::from_rgb(0x9a, 0xb6, 0xc8),
            CatalogTint::Steel => Color32::from_rgb(0xa5, 0xb8, 0xc0),
            CatalogTint::Periwinkle => Color32::from_rgb(0x8c, 0x9a, 0xe0),
        }
    }

    /// Associated product line or mood name from 1996 catalog.
    pub const fn name(&self) -> &'static str {
        match self {
            CatalogTint::Olive => "Dimension",
            CatalogTint::Sage => "Latitude",
            CatalogTint::Salmon => "OptiPlex GX",
            CatalogTint::Peach => "Dimension XPS",
            CatalogTint::Lime => "OptiPlex Gs",
            CatalogTint::Sky => "Dellware",
            CatalogTint::Steel => "Precision",
            CatalogTint::Periwinkle => "PowerEdge",
        }
    }
}

// ─── Spacing Scale (4px base) ───

pub const PAGE_FRAME_PAD: f32 = 8.0;
pub const SPACING_XXS: f32 = 2.0;
pub const SPACING_XS: f32 = 4.0;
pub const SPACING_S: f32 = 6.0;
pub const SPACING_SM: f32 = 8.0;
pub const SPACING_M: f32 = 10.0;
pub const SPACING_MD: f32 = 12.0;
pub const SPACING_LG: f32 = 16.0;
pub const SPACING_XL: f32 = 20.0;
pub const SPACING_XXL: f32 = 24.0;
pub const SPACING_SECTION_SM: f32 = 32.0;
pub const SPACING_SECTION: f32 = 40.0;
pub const SPACING_SECTION_LG: f32 = 48.0;

// ─── Component Sizing ───

pub const TOP_BANNER_HEIGHT: f32 = 44.0;
pub const LEFT_NAV_WIDTH: f32 = 240.0;
pub const FOOTER_PLAYER_HEIGHT: f32 = 72.0;
pub const RIBBON_TITLE_HEIGHT: f32 = 26.0;
pub const RIBBON_PHOTO_NOTCH_OVERHANG: f32 = 10.0;
pub const BEVEL_PX: f32 = 2.0;
