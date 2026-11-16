//! Lunar Client theme tokens and design system for Horizon.
//! Signature dark cosmic aesthetic, translucent acrylic panels, glowing cyan accents,
//! and modern typography inspired by Lunar Client (lunarclient.com).

use crate::{Blocks, Palette};

/// Primary Lunar Client branding constants.
pub const CLIENT_NAME: &str = "Lunar Client";
pub const CLIENT_SUBTITLE: &str = "Rust Edition";
pub const CLIENT_TAGLINE: &str = "High Performance Minecraft Client in Rust";
pub const VERSION_STRING: &str = "Lunar Client (Rust) v1.0.0";

/// Iconic Lunar Cyan accent color (`#17EAD9`).
pub const LUNAR_CYAN: u32 = 0x17ead9;
/// Bright cyan for hover states and glows (`#38efdf`).
pub const LUNAR_CYAN_HOVER: u32 = 0x38efdf;
/// Deep cyan for pressed states and active borders (`#0ea597`).
pub const LUNAR_CYAN_DARK: u32 = 0x0ea597;
/// Glowing Lunar Gold / Yellow (`#FBBF24`) used for crowns, ranks, and highlights.
pub const LUNAR_GOLD: u32 = 0xfbbf24;
/// Deep cosmic obsidian background (`#0b0e14`).
pub const LUNAR_BG: u32 = 0x0b0e14;
/// Translucent dark glass surface (`#121824`).
pub const LUNAR_SURFACE: u32 = 0x121824;

/// Lunar Client Dark Palette for overlays, HUD widgets, and transparent menus.
pub const LUNAR_DARK: Palette = Palette {
    page: 0x0b0e14,
    plate_soft: 0x131a26,
    plate: 0x1a2333,
    surface: 0x101622,
    rule: 0x1e2a3c,
    ink: 0xffffff,
    ink_soft: 0xd1d5db,
    muted: 0x94a3b8,
    accent: LUNAR_CYAN,
    on_accent: 0x06141d,
    danger: 0xf87171,
    success: 0x34d399,
    dark: true,
};

/// Lunar Client Blocks palette for panels, buttons, toggles, and settings shell.
pub const LUNAR_BLOCKS: Blocks = Blocks {
    page: 0x0b0e14,
    side: 0x080b10,
    panel: 0x121824,
    raised: 0x1a2333,
    button: 0x16202e,
    button_bevel: 0x243247,
    well: 0x090d14,
    bevel: 0x223045,
    lip: 0x06080e,
    outline: 0x040609,
    rule: 0x182232,
    ink: 0xffffff,
    ink_soft: 0xd1d5db,
    muted: 0x94a3b8,
    accent: LUNAR_CYAN,
    face: LUNAR_CYAN,
    face_bevel: LUNAR_CYAN_HOVER,
    face_lip: LUNAR_CYAN_DARK,
    on_face: 0x06141d,
    danger: 0xf87171,
    success: 0x34d399,
    warn: LUNAR_GOLD,
    dark: true,
};

/// Calculates a smooth chroma (rainbow RGB) color cycling across time.
/// `offset`: phase offset (0.0 to 1.0)
/// `speed`: cycles per second (typically 0.2 to 1.0)
/// `time_secs`: current continuous running time in seconds
#[inline]
pub fn chroma_rgb(offset: f32, speed: f32, time_secs: f32) -> (u8, u8, u8) {
    let hue = ((time_secs * speed + offset) % 1.0 + 1.0) % 1.0;
    let h6 = hue * 6.0;
    let x = (1.0 - (h6 % 2.0 - 1.0).abs()) * 255.0;
    let x_u8 = x as u8;
    match h6 as u32 {
        0 => (255, x_u8, 0),
        1 => (x_u8, 255, 0),
        2 => (0, 255, x_u8),
        3 => (0, x_u8, 255),
        4 => (x_u8, 0, 255),
        _ => (255, 0, x_u8),
    }
}

/// Returns a chroma color as `0xRRGGBB`.
#[inline]
pub fn chroma_u32(offset: f32, speed: f32, time_secs: f32) -> u32 {
    let (r, g, b) = chroma_rgb(offset, speed, time_secs);
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Returns a chroma color in 32-bit packed RGBA format for HUD rendering.
#[inline]
pub fn chroma_rgba(offset: f32, speed: f32, time_secs: f32) -> u32 {
    let (r, g, b) = chroma_rgb(offset, speed, time_secs);
    u32::from_ne_bytes([r, g, b, 255])
}

/// The Lunar Client crescent moon icon SVG.
pub const LUNAR_CRESCENT_SVG: &str = r#"<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
  <path d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C14.05 22 15.96 21.38 17.56 20.31C13.24 19.38 10.05 15.54 10.05 10.92C10.05 7.15 12.18 3.87 15.34 2.24C14.28 2.08 13.15 2 12 2Z" fill="#17EAD9"/>
</svg>"#;
