//! Core gameplay systems: mole spawning, lifetime ticking, click detection,
//! and the round timer.
//!
//! # How moles work
//!
//! 1. [`spawn_mole`] fires every [`crate::constants::MOLE_SPAWN_INTERVAL`]
//!    seconds (tracked by [`MoleSpawnTimer`]) and creates a brown circle sprite
//!    at a random position within the window bounds.
//! 2. Each mole carries a [`Mole`] component with its own countdown timer.
//!    [`tick_mole_lifetimes`] advances every mole's timer and despawns those
//!    whose timer has finished.
//! 3. [`handle_mole_click`] converts the cursor position from Bevy's top-left
//!    screen-space into world-space (where `(0, 0)` is the screen centre) and
//!    checks whether the click landed within [`crate::constants::MOLE_RADIUS`]
//!    pixels of any live mole.
//!
//! # Round lifecycle
//!
//! [`tick_game_timer`] counts down the round and, when the timer finishes,
//! updates [`HighScore`] if necessary and transitions back to
//! [`GameState::Menu`].
//!
//! [`despawn_all_moles`] is scheduled on `OnExit(GameState::Playing)` to
//! clean up any moles still on screen when the round ends.
//!
//! # Systems in this module
//!
//! | System                  | Schedule                          | Purpose |
//! |-------------------------|-----------------------------------|---------|
//! | [`tick_game_timer`]     | `Update` while `Playing`          | Advance round timer; return to menu on finish. |
//! | [`spawn_mole`]          | `Update` while `Playing`          | Spawn a new mole on each spawn-timer tick. |
//! | [`tick_mole_lifetimes`] | `Update` while `Playing`          | Despawn moles whose lifetime has expired. |
//! | [`handle_mole_click`]   | `Update` while `Playing`          | Detect left-clicks on moles and increment score. |
//! | [`despawn_all_moles`]   | `OnExit(GameState::Playing)`      | Remove all remaining moles when the round ends. |

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::components::Mole;
use crate::constants::{MOLE_COLOR, MOLE_LIFETIME, MOLE_RADIUS};
use crate::resources::{GameState, GameTimer, HighScore, MoleSpawnTimer, Score};

/// Advances the round countdown and transitions back to the menu when time
/// runs out.
///
/// If the player's score beats the stored high score, the high score is
/// updated before the transition.
pub fn tick_game_timer(
    mut game_timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
) {
    // `time.delta()` is the duration of the last frame – Bevy provides this
    // via the built-in `Time` resource so systems stay frame-rate independent.
    game_timer.0.tick(time.delta());
    if game_timer.0.just_finished() {
        if score.0 > high_score.0 {
            high_score.0 = score.0;
        }
        next_state.set(GameState::Menu);
    }
}

/// Spawns a new mole at a random position whenever the spawn timer fires.
///
/// The mole is a coloured [`Sprite`] (a 2-D textured quad) paired with a
/// [`Transform`] that positions it in world space and a [`Mole`] component
/// that carries its lifetime timer.
pub fn spawn_mole(
    mut commands: Commands,
    mut spawn_timer: ResMut<MoleSpawnTimer>,
    time: Res<Time>,
    game_timer: Res<GameTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Don't spawn new moles if the round has already finished (the timer can
    // fire one last time on the same frame the game ends).
    if game_timer.0.finished() {
        return;
    }

    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.just_finished() {
        let window = window_query.single();
        // Keep moles fully inside the window by subtracting the radius from
        // each half-extent.
        let half_w = window.width() / 2.0 - MOLE_RADIUS;
        let half_h = window.height() / 2.0 - MOLE_RADIUS;

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-half_w..half_w);
        let y = rng.gen_range(-half_h..half_h);

        commands.spawn((
            Sprite {
                color: MOLE_COLOR,
                // `custom_size` overrides the sprite's natural size (which
                // would normally match the texture dimensions).
                custom_size: Some(Vec2::splat(MOLE_RADIUS * 2.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Mole {
                lifetime: Timer::from_seconds(MOLE_LIFETIME, TimerMode::Once),
            },
        ));
    }
}

/// Ticks every live mole's lifetime timer and despawns any that have expired.
pub fn tick_mole_lifetimes(
    mut commands: Commands,
    mut moles: Query<(Entity, &mut Mole)>,
    time: Res<Time>,
) {
    for (entity, mut mole) in &mut moles {
        mole.lifetime.tick(time.delta());
        if mole.lifetime.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Detects left mouse-button clicks and checks whether they hit a mole.
///
/// Bevy's cursor position uses a top-left origin (y increases downward),
/// while world-space sprites use a centre origin (y increases upward).  The
/// conversion below maps between the two coordinate systems.
///
/// Only the first mole hit per click is awarded (the `break` prevents
/// double-counting overlapping moles).
pub fn handle_mole_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    moles: Query<(Entity, &Transform), With<Mole>>,
    mut score: ResMut<Score>,
    game_timer: Res<GameTimer>,
) {
    if game_timer.0.finished() {
        return;
    }
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let window = window_query.single();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Convert cursor position (top-left origin) to world space (centre origin).
    let world_x = cursor_pos.x - window.width() / 2.0;
    let world_y = -(cursor_pos.y - window.height() / 2.0);

    for (entity, transform) in &moles {
        let mole_pos = transform.translation.truncate();
        let click_pos = Vec2::new(world_x, world_y);
        if mole_pos.distance(click_pos) <= MOLE_RADIUS {
            commands.entity(entity).despawn();
            score.0 += 1;
            break;
        }
    }
}

/// Despawns every mole entity still on screen.
///
/// Scheduled on `OnExit(GameState::Playing)` to ensure no stale mole sprites
/// bleed through to the menu screen.
pub fn despawn_all_moles(mut commands: Commands, moles: Query<Entity, With<Mole>>) {
    for entity in &moles {
        commands.entity(entity).despawn();
    }
}
