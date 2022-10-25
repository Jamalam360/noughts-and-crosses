use bevy::{prelude::*, window::PresentMode};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Noughts and Crosses"),
            width: 600.,
            height: 600.,
            present_mode: PresentMode::AutoVsync,
            resizable: false,
            ..default()
        })
        .insert_resource(State {
            turn: String::from("X"),
            winner: false,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_current_player)
        .add_system(check_mouse_input)
        .add_system(check_winner)
        .run();
}

#[derive(Component)]
struct PlayerDisplay;

#[derive(Debug, Component)]
struct Position {
    row: isize,
    col: isize,
}

struct State {
    turn: String,
    winner: bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn().insert_bundle(Camera2dBundle::default());

    let text_style = TextStyle {
        font: asset_server.load("FiraSans-Bold.ttf"),
        font_size: 120.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;

    for row in -1..2 {
        for col in -1..2 {
            commands
                .spawn()
                .insert_bundle(Text2dBundle {
                    text: Text::from_section("-", text_style.clone())
                        .with_alignment(text_alignment),
                    transform: Transform::from_xyz((row * 120) as f32, (col * 120) as f32, 0.),
                    ..default()
                })
                .insert(Position { row, col });
        }
    }

    let info_text_style = TextStyle {
        font: asset_server.load("FiraSans-Bold.ttf"),
        font_size: 60.0,
        color: Color::GREEN,
    };

    commands
        .spawn()
        .insert_bundle(Text2dBundle {
            text: Text::from_section("", info_text_style.clone()).with_alignment(text_alignment),
            transform: Transform::from_xyz(0., 260., 0.),
            ..default()
        })
        .insert(PlayerDisplay);
}

fn update_current_player(
    state: Res<State>,
    mut query: Query<&mut Text, (With<Text>, With<PlayerDisplay>)>,
) {
    for mut text in &mut query {
        let str;

        if state.winner {
            str = state.turn.to_owned() + " wins!";
        } else {
            if state.turn == "X" {
                str = String::from("Player 1's Turn (X)");
            } else {
                str = String::from("Player 2's Turn (O)");
            }
        }

        set_text(&mut text, &str);
    }
}

fn check_mouse_input(
    mouse_button_input: Res<Input<MouseButton>>,
    mut windows: ResMut<Windows>,
    mut state: ResMut<State>,
    mut query: Query<(&mut Text, &mut Position), (With<Text>, With<Position>)>,
) {
    if state.winner {
        return;
    };

    let window = windows.primary_mut();

    if mouse_button_input.just_released(MouseButton::Left) {
        let pos_opt = window.cursor_position();

        if pos_opt.is_some() {
            let pos = pos_opt.unwrap();

            for row in -1..2 {
                for col in -1..2 {
                    let x_pos_min = (300 + row * 120 - 60) as f32;
                    let x_pos_max = (300 + row * 120 + 60) as f32;
                    let y_pos_min = (300 + col * 120 - 60) as f32;
                    let y_pos_max = (300 + col * 120 + 60) as f32;

                    if pos.x > x_pos_min
                        && pos.x < x_pos_max
                        && pos.y > y_pos_min
                        && pos.y < y_pos_max
                    {
                        for mut i in &mut query {
                            if i.1.row == row && i.1.col == col {
                                if i.0.sections[0].value == "-" {
                                    set_text(&mut i.0, &state.turn);
                                    state.turn = get_opposite_turn(&state.turn);
                                    break;
                                }
                            }
                        }

                        break;
                    }
                }
            }
        }
    }
}

fn check_winner(
    mut state: ResMut<State>,
    query: Query<(&mut Text, &mut Position), (With<Text>, With<Position>)>,
) {
    if state.winner {
        return;
    }

    let mut board = [["-", "-", "-"], ["-", "-", "-"], ["-", "-", "-"]];

    for i in query.iter() {
        board[2 - (i.1.col + 1) as usize][(i.1.row + 1) as usize] = &i.0.sections[0].value;
    }

    let mut cols: [[&str; 3]; 3] = [["-"; 3]; 3];

    for col_number in 0..3 {
        cols[col_number] = [
            board[0][col_number],
            board[1][col_number],
            board[2][col_number],
        ];
    }

    let mut diagonal_0 = [["-"; 3]; 3];
    let mut diagonal_1 = [["-"; 3]; 3];

    diagonal_0[0] = [board[0][0], board[1][1], board[2][2]];
    diagonal_1[0] = [board[0][2], board[1][1], board[2][0]];

    for line in [board, cols, diagonal_0, diagonal_1].concat() {
        if line[0] == line[1] && line[1] == line[2] && line[0] != "-" {
            state.winner = true;
            state.turn = get_opposite_turn(&state.turn);
            break;
        }
    }
}

fn get_opposite_turn(turn: &String) -> String {
    if turn == "X" {
        String::from("O")
    } else {
        String::from("X")
    }
}

fn set_text(text: &mut Mut<Text>, value: &String) {
    text.sections[0].value = value.to_owned();
}
