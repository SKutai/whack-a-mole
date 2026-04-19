use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

// ── Game states ──────────────────────────────────────────────────────────────

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
}

// ── Resources ────────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
struct HighScore(u32);

#[derive(Resource)]
struct GameTimer(Timer);

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Resource)]
struct MoleSpawnTimer(Timer);

// ── Components ───────────────────────────────────────────────────────────────

#[derive(Component)]
struct Mole {
    lifetime: Timer,
}

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct HighScoreText;

#[derive(Component)]
struct TimerText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct MenuRoot;

#[derive(Component)]
struct HudRoot;

// ── Constants ────────────────────────────────────────────────────────────────

const MOLE_RADIUS: f32 = 35.0;
const GAME_DURATION: f32 = 30.0;
const MOLE_LIFETIME: f32 = 1.0;
const MOLE_SPAWN_INTERVAL: f32 = 1.0;
const BACKGROUND_COLOR: Color = Color::srgb(0.18, 0.65, 0.18);
const MOLE_COLOR: Color = Color::srgb(0.55, 0.27, 0.07);
const BUTTON_COLOR: Color = Color::srgb(0.25, 0.75, 0.25);
const BUTTON_HOVER_COLOR: Color = Color::srgb(0.35, 0.85, 0.35);
const BUTTON_PRESS_COLOR: Color = Color::srgb(0.15, 0.55, 0.15);

// ── Entry point ──────────────────────────────────────────────────────────────

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
        // Systems that always run
        .add_systems(Startup, setup_camera)
        // Menu systems
        .add_systems(OnEnter(GameState::Menu), spawn_menu)
        .add_systems(OnExit(GameState::Menu), despawn_menu)
        .add_systems(
            Update,
            (handle_start_button,).run_if(in_state(GameState::Menu)),
        )
        // Playing systems
        .add_systems(OnEnter(GameState::Playing), spawn_hud)
        .add_systems(OnExit(GameState::Playing), (despawn_hud, despawn_all_moles))
        .add_systems(
            Update,
            (
                tick_game_timer,
                spawn_mole,
                tick_mole_lifetimes,
                handle_mole_click,
                update_hud,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

// ── Startup ──────────────────────────────────────────────────────────────────

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// ── Menu ─────────────────────────────────────────────────────────────────────

fn spawn_menu(mut commands: Commands, high_score: Res<HighScore>) {
    commands
        .spawn((
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
            // Start button
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

            // High score label
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

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_start_button(
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

// ── HUD ──────────────────────────────────────────────────────────────────────

fn spawn_hud(mut commands: Commands) {
    // Timer text – top-left
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

    // Score text – top-right
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

fn despawn_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_hud(
    game_timer: Res<GameTimer>,
    score: Res<Score>,
    mut timer_query: Query<&mut Text, (With<TimerText>, Without<ScoreText>)>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<TimerText>)>,
) {
    let remaining = (game_timer.0.remaining_secs()).ceil() as u32;
    for mut text in &mut timer_query {
        **text = format!("Time: {}", remaining);
    }
    for mut text in &mut score_query {
        **text = format!("Score: {}", score.0);
    }
}

// ── Game logic ────────────────────────────────────────────────────────────────

fn tick_game_timer(
    mut game_timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
) {
    game_timer.0.tick(time.delta());
    if game_timer.0.just_finished() {
        if score.0 > high_score.0 {
            high_score.0 = score.0;
        }
        next_state.set(GameState::Menu);
    }
}

fn spawn_mole(
    mut commands: Commands,
    mut spawn_timer: ResMut<MoleSpawnTimer>,
    time: Res<Time>,
    game_timer: Res<GameTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if game_timer.0.finished() {
        return;
    }
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.just_finished() {
        let window = window_query.single();
        let half_w = window.width() / 2.0 - MOLE_RADIUS;
        let half_h = window.height() / 2.0 - MOLE_RADIUS;

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-half_w..half_w);
        let y = rng.gen_range(-half_h..half_h);

        commands.spawn((
            Sprite {
                color: MOLE_COLOR,
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

fn tick_mole_lifetimes(
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

fn handle_mole_click(
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

    // Convert cursor position (top-left origin) to world space (center origin)
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

fn despawn_all_moles(mut commands: Commands, moles: Query<Entity, With<Mole>>) {
    for entity in &moles {
        commands.entity(entity).despawn();
    }
}
