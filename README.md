<p align="center">
  <img src="crates/horizon-theme/icons/glass/lunar-crescent.svg" width="96" alt="Lunar Client Rust Logo">
</p>

<h1 align="center">🌙 Lunar Client (Rust Edition)</h1>

<p align="center">
  <b>The aesthetic, features, and fluidity of <a href="https://www.lunarclient.com/">Lunar Client</a>, built 100% in pure Rust on Horizon.</b><br>
  Instant loading, extreme FPS, native wgpu pipeline, and zero Java garbage collection pauses.
</p>

<p align="center">
  <a href="#-lunar-client-features"><b>Features</b></a> ·
  <a href="#-aesthetic--theming"><b>Aesthetics</b></a> ·
  <a href="#-hud-mods--presets"><b>HUD Mods</b></a> ·
  <a href="#-quick-start"><b>Quick Start</b></a> ·
  <a href="#-credits--upstream"><b>Upstream Horizon</b></a>
</p>

---

## ✨ Aesthetic & Theming

- **Dark Cosmic Glass UI:** Deep obsidian palettes (`#0B0E14`), translucent acrylic panels, and smooth pill-radius containers.
- **Iconic Lunar Cyan:** Electric `#17EAD9` glow on focused buttons, slider tracks, active toggles, and selection boxes.
- **Lunar Crescent Badge:** High-resolution 8×8 glowing crescent moon mark rendered beside player nametags and in the player list.
- **Polished Main Menu:** Custom Lunar Client header banner, live status badge (`● Online • Ready • Anti-Cheat Active`), and glowing hover outlines on buttons.
- **Chroma RGB Rainbow:** Dynamic real-time color cycling across HUD modules, text highlights, and custom crosshairs.

---

## 🎮 Lunar Client Features

### 📊 HUD Mods (Right Shift to Edit)
- **Keystrokes:** WASD, LMB, RMB, and Space keys with live CPS counters, rounded key tiles, press highlight animations, and acrylic borders.
- **CPS Counter:** Independent Left and Right click tracking in the last second.
- **FPS Display:** Real-time frame performance monitoring.
- **Armor & Equipment Status:** Durability percentage, remaining uses, and active equipment slot view.
- **Potion Effects HUD:** Active effect icons, durations, and amplifier tags.
- **Coordinates & Direction:** Precise block coordinates, facing direction, and biome indicators.
- **ToggleSprint & ToggleSneak:** On-screen status indicator (`[Sprinting (Toggled)]`, `[Sneaking (Toggled)]`).
- **Combo Counter & Reach Display:** Hits in succession and attack distance measurements.
- **Clock & Speedometer:** Local system clock and real-time movement velocity in blocks per second.

### ⚔️ PvP & Mechanics
- **1.8.9 Legacy Combat Feel:** Classic 1.8 sword blocking animation, legacy attack swing feel, and 1.8 hit sounds.
- **Custom Crosshair:** Configurable shapes (cross, dot, circle, T), custom gap, thickness, outline, and dynamic entity hover colors.
- **Custom Hit Color:** Customize the red damage flash tint to any color (Cyan, Purple, Gold, or Chroma).
- **Smooth Zoom:** Smooth FOV zoom (OptiFine/Lunar style) with mouse wheel zoom adjustments.
- **Freelook / 360° Perspective:** Free camera rotation around your character without altering movement or aim direction.
- **Quality of Life:** Lower fire overlay, clear water, clear lava, compact chat with timestamps, and in-game chat search (Ctrl+F).

### 👗 Lunar Cosmetics
- Native support for **Lunar Capes**, **Animated Wings**, **Bandanas**, **Hats**, **Masks**, **Pets**, and **Emotes** on the avatar skeleton.

---

## 🚀 Quick Start

### Requirements
- **Minecraft: Java Edition:** A legal copy (version 26.3). Assets and models are built from your local installation.
- **Rust Toolchain:** 2021 edition (Rust 1.85+ recommended).

### Running
```bash
# Clone the repository
git clone https://github.com/ItsDavloooDev/horizon.git
cd horizon

# Build with the fast development profile
cargo build --profile fast -p horizon-client

# Launch into the game
cargo run --profile fast -p horizon-client
```

*In-game:* Press **Right Shift** anytime to access the Lunar HUD Layout Editor and configure your widgets!

---

## 👥 Credits & Upstream

- Fork of the exceptional **[horizon-rust/horizon](https://github.com/horizon-rust/horizon)** project, originally founded by [Ludovic Marie](https://lu4ovic.com) and the Horizon Team.
- Inspired by the aesthetics, user experience, and mod suite of **[Lunar Client](https://www.lunarclient.com/)**.
- Licensed under [GNU LGPL v3.0 or later](LICENSE).
- *Disclaimer: Not an official Minecraft product. Not approved by or associated with Mojang, Microsoft, or Moonsworth.*
