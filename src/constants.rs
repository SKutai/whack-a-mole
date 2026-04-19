//! Shared constants used throughout the game.
//!
//! Centralising magic numbers here makes it easy to tweak game-feel without
//! hunting through system code.

use bevy::prelude::*;

/// Radius of a mole sprite in pixels.
pub const MOLE_RADIUS: f32 = 35.0;

/// Total duration of a single round, in seconds.
pub const GAME_DURATION: f32 = 30.0;

/// How long a mole stays on screen before disappearing on its own, in seconds.
pub const MOLE_LIFETIME: f32 = 1.0;

/// How often a new mole is spawned during a round, in seconds.
pub const MOLE_SPAWN_INTERVAL: f32 = 1.0;

/// Grass-green background that fills the window.
pub const BACKGROUND_COLOR: Color = Color::srgb(0.18, 0.65, 0.18);

/// Brown colour used to draw each mole sprite.
pub const MOLE_COLOR: Color = Color::srgb(0.55, 0.27, 0.07);

/// Default (idle) colour of the Start button.
pub const BUTTON_COLOR: Color = Color::srgb(0.25, 0.75, 0.25);

/// Colour the Start button turns when the cursor hovers over it.
pub const BUTTON_HOVER_COLOR: Color = Color::srgb(0.35, 0.85, 0.35);

/// Colour the Start button turns while it is being pressed.
pub const BUTTON_PRESS_COLOR: Color = Color::srgb(0.15, 0.55, 0.15);
