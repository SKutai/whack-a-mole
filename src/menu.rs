//! Systems that build and tear down the main-menu screen.
//!
//! # Bevy UI in a nutshell
//!
//! Bevy renders UI using a tree of entities that each carry a [`Node`]
//! component (the flexbox layout container) plus optional visual components
//! such as [`BackgroundColor`] or [`Text`].  Child entities are attached via
//! Bevy's parent-child hierarchy so that despawning the root with
//! `despawn_recursive` automatically cleans up the whole tree.
//!
//! # Systems in this module
//!
//! | System                  | Schedule                          | Purpose |
//! |-------------------------|-----------------------------------|---------|
//! | [`spawn_menu`]          | `OnEnter(GameState::Menu)`        | Build the menu UI hierarchy. |
//! | [`despawn_menu`]        | `OnExit(GameState::Menu)`         | Remove every entity tagged with [`MenuRoot`]. |
//! | [`handle_start_button`] | `Update` while in `Menu` state    | React to button interactions and start a new round. |

use bevy::prelude::*;

use crate::components::{HighScoreText, MenuRoot, StartButton};
use crate::constants::{
    BUTTON_COLOR, BUTTON_HOVER_COLOR, BUTTON_PRESS_COLOR, GAME_DURATION, MOLE_SPAWN_INTERVAL,
};
use crate::resources::{GameState, GameTimer, MoleSpawnTimer, Score, HighScore};

/// Spawns the main-menu UI: a centred "Start" button and a high-score label.
///
/// `high_score` is injected automatically by Bevy because it is declared as
/// `Res<HighScore>` – any system can request any resource this way.
///
/// The entire menu lives under a single root entity tagged with [`MenuRoot`]
/// so that [`despawn_menu`] can remove everything in one recursive call.
pub fn spawn_menu(mut commands: Commands, high_score: Res<HighScore>) {
    commands
        .spawn((
            // A full-screen flex container that centres its children vertically
            // and horizontally.
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            MenuRoot,
        ))
        .with_children(|parent| {
            // ── Start button ─────────────────────────────────────────────────
            parent
                .spawn((
                    Button,
                    StartButton,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(BUTTON_COLOR),
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(10.0)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Start"),
                        TextFont {
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // ── High-score label ─────────────────────────────────────────────
            parent.spawn((
                Text::new(format!("High Score: {}", high_score.0)),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                HighScoreText,
            ));
        });
}

/// Recursively despawns the entire menu hierarchy.
///
/// Bevy schedules this on `OnExit(GameState::Menu)`, so it runs automatically
/// the moment the state transitions away from the menu.
pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles mouse interaction with the Start button.
///
/// Bevy's `Interaction` component is automatically updated by the UI backend
/// every frame.  The `Changed<Interaction>` filter means this system only runs
/// when the interaction state actually changes, keeping it efficient.
///
/// On press:
/// * resets [`Score`] to `0`,
/// * inserts fresh [`GameTimer`] and [`MoleSpawnTimer`] resources,
/// * triggers the transition to [`GameState::Playing`].
pub fn handle_start_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(BUTTON_PRESS_COLOR);
                score.0 = 0;
                commands.insert_resource(GameTimer(Timer::from_seconds(
                    GAME_DURATION,
                    TimerMode::Once,
                )));
                commands.insert_resource(MoleSpawnTimer(Timer::from_seconds(
                    MOLE_SPAWN_INTERVAL,
                    TimerMode::Repeating,
                )));
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(BUTTON_HOVER_COLOR);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(BUTTON_COLOR);
            }
        }
    }
}
