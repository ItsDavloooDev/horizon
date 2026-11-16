//! Lunar Client look, given to the engine's extension points at startup: the
//! windows' icon and the colours, sizes and fonts of the menus, HUD and overlays
//! with Lunar Client's signature dark obsidian, glowing cyan accents, and rounded pill aesthetics.
use horizon_client::menu_style::{
    Fonts, MenuStyle, OverlayColours, SettingsColours, Sizes, Surfaces,
};
use horizon_theme as theme;

/// The switch for the menus' look: `vanilla` gives them none, so the settings pages, the HUD
/// kit's tab strip and the overlays draw in the engine's `NEUTRAL` look.
const STYLE_VAR: &str = "HORIZON_MENU_STYLE";

/// Give the engine Lunar Client's look; call it once, before the first window opens.
pub(crate) fn give() {
    horizon_client::window::set_icon_png(theme::ICON_PNG);
    if std::env::var(STYLE_VAR).is_ok_and(|v| v.eq_ignore_ascii_case("vanilla")) {
        return;
    }
    // Lunar Client styling: Dark obsidian plates, glowing cyan accents, rounded pills
    let (blocks, dark) = (theme::lunar::LUNAR_BLOCKS, theme::lunar::LUNAR_DARK);
    horizon_client::menu_style::install(MenuStyle {
        settings: SettingsColours {
            page: blocks.page,
            ink: blocks.ink,
            ink_soft: blocks.ink_soft,
            muted: blocks.muted,
            outline: blocks.outline,
            accent: blocks.accent,
            face: blocks.face,
            // Lunar Cyan hover glow
            face_hover: theme::lunar::LUNAR_CYAN_HOVER,
            success: blocks.success,
            warn: blocks.warn,
            danger: blocks.danger,
        },
        overlay: OverlayColours {
            page: dark.page,
            backdrop_muted: 0x161c28,
            ink: dark.ink,
            ink_soft: dark.ink_soft,
            rule: dark.rule,
            accent: dark.accent,
            on_accent: dark.on_accent,
            lit: theme::lunar::LUNAR_CYAN,
            lit_edge: theme::lunar::LUNAR_CYAN_HOVER,
        },
        sizes: Sizes {
            caption: theme::CAPTION.px,
            label: theme::LABEL.px,
            ui: theme::UI.px,
            body: theme::BODY.px,
            title: theme::TITLE.px,
            display: theme::DISPLAY.px,
            micro: theme::MICRO.px,
            // Rounded corners for Lunar pill cards and buttons
            radius: 10.0,
            radius_plate: 14.0,
            control_large: 40.0,
        },
        fonts: Some(Fonts {
            regular: theme::fonts::GEIST_REGULAR,
            medium: theme::fonts::GEIST_MEDIUM,
            semibold: theme::fonts::GEIST_SEMIBOLD,
            mono: theme::fonts::GEIST_MONO,
        }),
        surfaces: Surfaces::Flat,
    });
}
