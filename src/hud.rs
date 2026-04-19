//! Systems that manage the in-game HUD (heads-up display).
//!
//! The HUD consists of two text nodes positioned absolutely on the screen:
//!
//! * **Time** – top-left corner, shows seconds remaining.
//! * **Score** – top-right corner, shows moles whacked this round.
//!
//! Both nodes are tagged with [`HudRoot`] so they can be cleaned up in bulk
//! when the round ends.
//!
//! # Systems in this module
//!
//! | System         | Schedule                           | Purpose |
//! |----------------|------------------------------------|---------|
//! | [`spawn_hud`]  | `OnEnter(GameState::Playing)`      | Create the timer and score text nodes. |
//! | [`despawn_hud`]| `OnExit(GameState::Playing)`       | Remove every entity tagged with [`HudRoot`]. |
//! | [`update_hud`] | `Update` while in `Playing` state  | Refresh text content each frame. |

use bevy::prelude::*;

use crate::components::{HudRoot, ScoreText, TimerText};
use crate::constants::GAME_DURATION;
use crate::resources::{GameTimer, Score};

/// Spawns the timer (top-left) and score (top-right) text nodes.
///
/// `PositionType::Absolute` takes the nodes out of the normal flex flow so
/// they can be pinned to screen corners with `top` / `left` / `right` offsets.
pub fn spawn_hud(mut commands: Commands) {
    // Timer text – top-left corner
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(16.0),
                left: Val::Px(16.0),
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!("Time: {:.0}", GAME_DURATION)),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TimerText,
            ));
        });

    // Score text – top-right corner
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(16.0),
                right: Val::Px(16.0),
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Score: 0"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                ScoreText,
            ));
        });
}

/// Recursively despawns all HUD root entities and their children.
pub fn despawn_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Refreshes the timer and score text every frame.
///
/// The two queries use mutual exclusion filters (`Without<ScoreText>` and
/// `Without<TimerText>`) to satisfy Bevy's borrow checker: a single mutable
/// `Query<&mut Text>` that could match both text nodes would require two
/// simultaneous mutable borrows of the same component type, which Bevy
/// disallows.  The exclusion filters split them into non-overlapping sets.
pub fn update_hud(
    game_timer: Res<GameTimer>,
    score: Res<Score>,
    mut timer_query: Query<&mut Text, (With<TimerText>, Without<ScoreText>)>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<TimerText>)>,
) {
    let remaining = game_timer.0.remaining_secs().ceil() as u32;
    for mut text in &mut timer_query {
        **text = format!("Time: {}", remaining);
    }
    for mut text in &mut score_query {
        **text = format!("Score: {}", score.0);
    }
}
