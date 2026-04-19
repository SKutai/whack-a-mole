//! # Whack-a-Mole
//!
//! A simple arcade game built with [Bevy](https://bevyengine.org/).
//!
//! This file is the entry point.  All game logic lives in the sub-modules
//! below; `main` does nothing but wire everything together into a Bevy [`App`].
//!
//! ## Module layout
//!
//! | Module           | Contents |
//! |------------------|----------|
//! [`constants`]      | Shared magic numbers (radius, timings, colours). |
//! [`components`]     | ECS [`Component`] definitions. |
//! [`resources`]      | ECS [`Resource`] definitions and the [`GameState`] machine. |
//! [`menu`]           | Main-menu UI systems. |
//! [`hud`]            | In-game HUD systems. |
//! [`game`]           | Mole spawning, lifetime ticking, click detection, round timer. |
//!
//! [`GameState`]: resources::GameState

mod components;
mod constants;
mod game;
mod hud;
mod menu;
mod resources;

use bevy::prelude::*;

use constants::BACKGROUND_COLOR;
use resources::{GameState, HighScore, Score};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Whack-a-Mole".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_resource::<HighScore>()
        .init_resource::<Score>()
        // ── Startup ──────────────────────────────────────────────────────────
        .add_systems(Startup, setup_camera)
        // ── Menu ─────────────────────────────────────────────────────────────
        .add_systems(OnEnter(GameState::Menu), menu::spawn_menu)
        .add_systems(OnExit(GameState::Menu), menu::despawn_menu)
        .add_systems(
            Update,
            menu::handle_start_button.run_if(in_state(GameState::Menu)),
        )
        // ── Playing ──────────────────────────────────────────────────────────
        .add_systems(OnEnter(GameState::Playing), hud::spawn_hud)
        .add_systems(
            OnExit(GameState::Playing),
            (hud::despawn_hud, game::despawn_all_moles),
        )
        .add_systems(
            Update,
            (
                game::tick_game_timer,
                game::spawn_mole,
                game::tick_mole_lifetimes,
                game::handle_mole_click,
                hud::update_hud,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

/// Spawns the 2-D camera used to render sprites and UI.
///
/// [`Camera2d`] is a Bevy bundle that sets up an orthographic camera looking
/// down the z-axis.  Without it nothing would be drawn.
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
