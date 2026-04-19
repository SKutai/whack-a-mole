//! Bevy [`Component`]s used to tag and identify entities.
//!
//! In Bevy's Entity-Component-System (ECS) architecture, components are plain
//! data structs attached to entities.  Systems query for entities that carry
//! specific components and operate on them.
//!
//! Here we define two kinds of components:
//!
//! * **Data components** (e.g. [`Mole`]) – carry real data that systems read
//!   or mutate.
//! * **Marker components** (e.g. [`StartButton`], [`MenuRoot`]) – zero-sized
//!   structs used purely as a "tag" so that queries can filter for specific
//!   entities without storing any extra data.

use bevy::prelude::*;

/// Attached to every mole sprite that is currently on screen.
///
/// The `lifetime` timer counts down from [`crate::constants::MOLE_LIFETIME`]
/// seconds.  When it finishes, the mole entity is despawned automatically by
/// [`crate::game::tick_mole_lifetimes`].
#[derive(Component)]
pub struct Mole {
    /// Countdown timer that controls how long this mole stays visible.
    pub lifetime: Timer,
}

/// Marker for the "Start" button entity in the main menu.
///
/// Used by [`crate::menu::handle_start_button`] to filter the interaction
/// query so it only reacts to this specific button.
#[derive(Component)]
pub struct StartButton;

/// Marker for the HUD text node that displays the high score on the menu
/// screen.
#[derive(Component)]
pub struct HighScoreText;

/// Marker for the HUD text node that displays the remaining time during a
/// round.
#[derive(Component)]
pub struct TimerText;

/// Marker for the HUD text node that displays the player's current score
/// during a round.
#[derive(Component)]
pub struct ScoreText;

/// Marker placed on the root UI node of the main-menu layout.
///
/// [`crate::menu::despawn_menu`] queries for this component and recursively
/// despawns the entire menu hierarchy when the game transitions out of
/// [`crate::resources::GameState::Menu`].
#[derive(Component)]
pub struct MenuRoot;

/// Marker placed on every root UI node that belongs to the in-game HUD.
///
/// [`crate::hud::despawn_hud`] queries for this component and recursively
/// despawns all HUD nodes when the round ends.
#[derive(Component)]
pub struct HudRoot;
