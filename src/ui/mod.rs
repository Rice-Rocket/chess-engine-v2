use bevy::{prelude::*, window::PrimaryWindow};

pub mod board;
pub mod theme;
pub mod main_menu;
pub mod arrows;
pub mod ingame_menu;
pub mod text_input;

use bevy_egui::EguiPlugin;
use board::*;
use theme::*;
use main_menu::*;
use arrows::*;
use ingame_menu::*;
use text_input::*;

use crate::{AppState, state::AppMode};

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn finish_load_ui(
    mut commands: Commands,
    app_mode: Res<State<AppMode>>,
) {
    if app_mode.clone() != AppMode::GameHumanHuman {
        commands.insert_resource(NextState(Some(AppState::LoadAI)));
    } else {
        commands.insert_resource(NextState(Some(AppState::InGame)));
    }
}


pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::rgb_u8(40, 39, 40)))
            .init_resource::<BoardTheme>()
            .init_resource::<PieceTheme>()
            .init_resource::<BoardUITransform>()
            .init_resource::<BoardUI>()
            .init_resource::<CalcStatistics>()
            .init_resource::<DebugInfo>()
            .add_event::<BoardUIResetPiecePosition>()
            .add_event::<BoardSetSquareColor>()
            .add_event::<BoardResetSquareColors>()
            .add_event::<DebugPositionLoaded>()
            .add_plugins(TextInputPlugin)
            .add_plugins(EguiPlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_main_menu)
            .add_systems(Update, update_menu_buttons.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
        
            .add_systems(OnEnter(AppState::LoadUI), (
                init_board_ui_transform,
                init_piece_theme,
                spawn_board_ui,
                spawn_arrow_drawer,
                spawn_calc_stats,
                finish_load_ui,
            ).chain())

            .add_systems(OnEnter(AppState::LoadUI), (
                spawn_ai_vs_ai_menu,
            ).run_if(in_state(AppMode::GameAIAI)))

            .add_systems(Update, update_egui.run_if(in_state(AppState::InGame).and_then(in_state(AppMode::GameHumanHuman))))
            
            .add_systems(Update, (
                update_pieces,
                reset_piece_position,
                set_square_color,
                reset_square_colors,
                update_board_ui,
                update_arrows,
                drag_piece,
                update_board_ui_transform,
                update_board_ui_on_resize.after(update_board_ui_transform),
                update_menu_stats,
            ).run_if(in_state(AppState::InGame)))

            .add_systems(Update, reset_board_pieces.run_if(in_state(AppMode::GameHumanHuman).and_then(in_state(AppState::InGame))))
            
            .add_systems(OnEnter(AppState::GameOver), spawn_game_over_splash.run_if(in_state(AppMode::GameHumanHuman).or_else(in_state(AppMode::GameHumanAI))))
            .add_systems(OnExit(AppState::GameOver), (
                despawn_game_over_splash,
                reset_board_pieces,
            ));
    }
}