//! Bevy [`Resource`]s and the [`GameState`] state machine.
//!
//! # Resources vs Components
//!
//! While components live *on* entities, resources are **global singletons**
//! stored directly in the [`bevy::prelude::World`].  Any system can request a
//! resource as a parameter (via [`Res`] for read-only access or [`ResMut`] for
//! mutable access) and Bevy automatically injects it.
//!
//! # State machines
//!
//! [`GameState`] is registered with `App::init_state` in `main`.  Bevy then
//! exposes schedule hooks – `OnEnter`, `OnExit`, and `Update` filtered by
//! `in_state` – that let you run different systems depending on which state is
//! currently active.

use bevy::prelude::*;

// ── Game state ────────────────────────────────────────────────────────────────

/// The top-level state machine for the game.
///
/// Bevy drives transitions between states when a system calls
/// `next_state.set(...)`.  All `OnEnter` / `OnExit` schedules fire
/// automatically on the next frame after the transition is requested.
///
/// | State     | Description                                          |
/// |-----------|------------------------------------------------------|
/// | `Menu`    | The title / high-score screen shown between rounds.  |
/// | `Playing` | An active round where moles appear and the timer counts down. |
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    /// Shown at startup and after each round finishes.
    #[default]
    Menu,
    /// Active during a round.
    Playing,
}

// ── Resources ─────────────────────────────────────────────────────────────────

/// The best score achieved across all rounds in the current session.
///
/// Persists for the lifetime of the app because it is initialised with
/// `init_resource` rather than inserted / removed per round.
#[derive(Resource, Default)]
pub struct HighScore(pub u32);

/// Countdown timer for the current round.
///
/// Inserted when the player presses Start; not present while in the menu.
/// [`crate::game::tick_game_timer`] ticks this every frame and triggers the
/// transition back to [`GameState::Menu`] when it finishes.
#[derive(Resource)]
pub struct GameTimer(pub Timer);

/// The player's score for the current round.
///
/// Reset to `0` each time a new round starts.
#[derive(Resource, Default)]
pub struct Score(pub u32);

/// Repeating timer that controls how often a new mole is spawned.
///
/// Inserted alongside [`GameTimer`] when a round begins.
/// [`crate::game::spawn_mole`] ticks it and spawns a mole each time it
/// completes a cycle.
#[derive(Resource)]
pub struct MoleSpawnTimer(pub Timer);
