//! Artwork tint snapping.
//!
//! Maps an artwork's dominant color to the closest of the 8 Dell 1996 catalogue
//! ribbon tints using CIELAB ΔE color distance. This keeps the color palette
//! strictly closed and period-accurate while ensuring every album/playlist ribbon
//! harmonizes directly with its cover art.

use egui::Color32;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use super::tokens::CatalogTint;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Lab {
    l: f32,
    a: f32,
    b: f32,
}

impl Lab {
    fn from_rgb([r, g, b]: [u8; 3]) -> Self {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let pivot = |c: f32| -> f32 {
            if c > 0.04045 {
                ((c + 0.055) / 1.055).powf(2.4)
            } else {
                c / 12.92
            }
        };

        let r_lin = pivot(r);
        let g_lin = pivot(g);
        let b_lin = pivot(b);

        let x = (r_lin * 0.4124564 + g_lin * 0.3575761 + b_lin * 0.1804375) * 100.0;
        let y = (r_lin * 0.2126729 + g_lin * 0.7151522 + b_lin * 0.0721750) * 100.0;
        let z = (r_lin * 0.0193339 + g_lin * 0.1191920 + b_lin * 0.9503041) * 100.0;

        // D65 standard illuminant
        let fx = pivot_xyz(x / 95.047);
        let fy = pivot_xyz(y / 100.000);
        let fz = pivot_xyz(z / 108.883);

        Self {
            l: (116.0 * fy) - 16.0,
            a: 500.0 * (fx - fy),
            b: 200.0 * (fy - fz),
        }
    }

    fn distance_squared(&self, other: &Self) -> f32 {
        let dl = self.l - other.l;
        let da = self.a - other.a;
        let db = self.b - other.b;
        dl * dl + da * da + db * db
    }
}

fn pivot_xyz(v: f32) -> f32 {
    if v > 0.008856 {
        v.cbrt()
    } else {
        (7.787 * v) + (16.0 / 116.0)
    }
}

/// Precomputed Lab values for each catalog tint.
struct TintLabs {
    table: [(CatalogTint, Lab); 8],
}

impl TintLabs {
    fn new() -> Self {
        let table = [
            (CatalogTint::Olive, Lab::from_rgb([0x8e, 0x8a, 0x25])),
            (CatalogTint::Sage, Lab::from_rgb([0xb3, 0xbd, 0x95])),
            (CatalogTint::Salmon, Lab::from_rgb([0xd7, 0x7a, 0x7a])),
            (CatalogTint::Peach, Lab::from_rgb([0xe6, 0x91, 0x5d])),
            (CatalogTint::Lime, Lab::from_rgb([0xc0, 0xd4, 0xa7])),
            (CatalogTint::Sky, Lab::from_rgb([0x9a, 0xb6, 0xc8])),
            (CatalogTint::Steel, Lab::from_rgb([0xa5, 0xb8, 0xc0])),
            (CatalogTint::Periwinkle, Lab::from_rgb([0x8c, 0x9a, 0xe0])),
        ];
        Self { table }
    }

    fn nearest(&self, rgb: [u8; 3]) -> CatalogTint {
        let target = Lab::from_rgb(rgb);
        let mut best_tint = self.table[0].0;
        let mut min_dist = f32::MAX;

        for (tint, lab) in &self.table {
            let dist = target.distance_squared(lab);
            if dist < min_dist {
                min_dist = dist;
                best_tint = *tint;
            }
        }
        best_tint
    }
}

fn tint_labs() -> &'static TintLabs {
    static LABS: OnceLock<TintLabs> = OnceLock::new();
    LABS.get_or_init(TintLabs::new)
}

/// Global tint cache for artwork URLs to prevent redundant distance calculations.
static TINT_CACHE: OnceLock<Mutex<HashMap<String, CatalogTint>>> = OnceLock::new();

fn tint_cache() -> &'static Mutex<HashMap<String, CatalogTint>> {
    TINT_CACHE.get_or_init(|| Mutex::new(HashMap::with_capacity(128)))
}

/// Snaps an RGB color to the closest 1996 catalogue ribbon tint.
pub fn snap_color(rgb: [u8; 3]) -> CatalogTint {
    tint_labs().nearest(rgb)
}

/// Returns the nearest catalogue tint for a Color32.
pub fn snap_color32(color: Color32) -> CatalogTint {
    snap_color([color.r(), color.g(), color.b()])
}

/// Determines the best catalogue tint for artwork by analyzing its dominant colors,
/// caching by URL if provided.
pub fn snap_artwork(palette_colors: &[[u8; 3]], url: Option<&str>) -> CatalogTint {
    if let Some(url) = url {
        if let Ok(cache) = tint_cache().lock() {
            if let Some(tint) = cache.get(url) {
                return *tint;
            }
        }
    }

    // Pick the most chromatic dominant color if available, fallback to first color or Steel
    let best_rgb = palette_colors
        .iter()
        .copied()
        .find(|&[r, g, b]| {
            let max = r.max(g).max(b) as f32;
            let min = r.min(g).min(b) as f32;
            (max - min) > 20.0 // Has noticeable saturation
        })
        .or_else(|| palette_colors.first().copied())
        .unwrap_or([0xa5, 0xb8, 0xc0]); // Steel fallback

    let tint = snap_color(best_rgb);

    if let Some(url) = url {
        if let Ok(mut cache) = tint_cache().lock() {
            cache.insert(url.to_string(), tint);
        }
    }

    tint
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_colors_match_themselves() {
        assert_eq!(snap_color([0x8e, 0x8a, 0x25]), CatalogTint::Olive);
        assert_eq!(snap_color([0xb3, 0xbd, 0x95]), CatalogTint::Sage);
        assert_eq!(snap_color([0xd7, 0x7a, 0x7a]), CatalogTint::Salmon);
        assert_eq!(snap_color([0xe6, 0x91, 0x5d]), CatalogTint::Peach);
        assert_eq!(snap_color([0xc0, 0xd4, 0xa7]), CatalogTint::Lime);
        assert_eq!(snap_color([0x9a, 0xb6, 0xc8]), CatalogTint::Sky);
        assert_eq!(snap_color([0xa5, 0xb8, 0xc0]), CatalogTint::Steel);
        assert_eq!(snap_color([0x8c, 0x9a, 0xe0]), CatalogTint::Periwinkle);
    }

    #[test]
    fn warm_reddish_snaps_to_salmon_or_peach() {
        let warm_pink = [220, 110, 110];
        assert_eq!(snap_color(warm_pink), CatalogTint::Salmon);

        let warm_orange = [235, 140, 80];
        assert_eq!(snap_color(warm_orange), CatalogTint::Peach);
    }

    #[test]
    fn blue_violet_snaps_to_periwinkle() {
        let indigo = [130, 140, 230];
        assert_eq!(snap_color(indigo), CatalogTint::Periwinkle);
    }
}
