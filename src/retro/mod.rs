//! Spotifast Retro interface module.
//!
//! Rebuilds the user interface in the Dell 1996 design aesthetic:
//! - Literal 8px black page frame
//! - Flat color-block ribbon cards with right-notched artwork
//! - Prominent album covers as the primary visual element
//! - 8 catalogue tints mapped to artwork dominant colors
//! - Strictly 0px corner radii and hard bevel depth

pub mod pages;
pub mod shell;
pub mod style;

use crate::app::App;
use crate::backend::AuthStatus;

/// Primary render entrypoint for the Dell 1996 Retro interface.
pub fn show(app: &mut App, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();

    // Check auth status
    let signed_in = app.is_connected() && app.user.is_some();
    let connecting = matches!(app.auth, AuthStatus::Connecting | AuthStatus::Starting)
        || (app.is_connected() && app.user.is_none());

    if !signed_in {
        crate::ui::login::show(app, ui, connecting);
        return;
    }

    // Render Retro shell & active page
    shell::show(app, ui);

    // Keep devices popup and background dialogs active
    crate::ui::devices::popup(app, &ctx);
    crate::ui::dialogs::show(app, &ctx);
}
