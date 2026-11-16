//! Lunar Client mark as the game draws it before a Lunar player's name:
//! a glowing Lunar Crescent Moon in 8×8 texels with signature Lunar cyan.

/// The mark's rectangles in its 8×8 square: `(x, y, width, height, ARGB)`.
/// Renders a sharp, elegant crescent moon silhouette in Lunar Cyan (`#17EAD9`).
pub const MARK: [(u8, u8, u8, u8, u32); 8] = [
    (3, 0, 3, 1, 0xFF17_EAD9), // Top curve
    (2, 1, 2, 1, 0xFF38_EFDF), // Upper outer curve (highlight)
    (1, 2, 2, 1, 0xFFFFFFFF), // Mid-upper bright shine
    (1, 3, 2, 1, 0xFF17_EAD9), // Center crest
    (1, 4, 2, 1, 0xFF17_EAD9), // Center crest
    (1, 5, 2, 1, 0xFF17_EAD9), // Mid-lower crest
    (2, 6, 2, 1, 0xFF0E_A597), // Lower curve
    (3, 7, 3, 1, 0xFF0E_A597), // Bottom curve
];

/// The mark as 8×8 RGBA texels, rows from the top.
pub fn texels() -> [[u8; 4]; 64] {
    let mut out = [[0; 4]; 64];
    for (x, y, w, h, argb) in MARK {
        let [a, r, g, b] = argb.to_be_bytes();
        for yy in y..y + h {
            for xx in x..x + w {
                out[usize::from(yy) * 8 + usize::from(xx)] = [r, g, b, a];
            }
        }
    }
    out
}
