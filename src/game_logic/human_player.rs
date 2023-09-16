use bevy::{prelude::*, window::PrimaryWindow};

use crate::ui::{board::{BoardUITransform, BoardUIResetPiecePosition, BoardSetSquareColor}, theme::{BoardTheme, SquareColorTypes}};

use super::{moves::Move, coord::Coord, representation::idx_from_coord, piece::is_color, board::{Board, MainBoard, BoardMakeMove}, player::Player};



#[derive(PartialEq)]
pub enum PlayerInputState {
    None,
    PieceSelected, 
    DraggingPiece,
}

#[derive(Component)]
pub struct HumanPlayer {
    pub current_state: PlayerInputState,
    selected_piece_sqr: Coord,
}

impl Default for HumanPlayer {
    fn default() -> Self {
        HumanPlayer {
            current_state: PlayerInputState::None,
            selected_piece_sqr: Coord::new(0, 0),
        }
    }
}


pub fn handle_player_input(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut HumanPlayer, &Player)>,
    buttons: Res<Input<MouseButton>>,
    board_transform: Res<BoardUITransform>,
    board_query: Query<&Board, With<MainBoard>>,
    mut reset_piece_position_evw: EventWriter<BoardUIResetPiecePosition>,
    mut make_move_evw: EventWriter<BoardMakeMove>,
    mut set_sqr_color_evw: EventWriter<BoardSetSquareColor>,
) {
    if let Some(mpos) = window_query.single().cursor_position() {
        if let Ok(board) = board_query.get_single() {
            for (mut player, player_data) in player_query.iter_mut() {
                if player_data.team != board.color_to_move { continue };
                if player.current_state == PlayerInputState::None {
                    handle_piece_selection(
                        &buttons,
                        &board_transform,
                        board,
                        &mut player,
                        mpos,
                        &mut set_sqr_color_evw,
                    );
                } else if player.current_state == PlayerInputState::DraggingPiece {
                    println!("Drag piece visual");
                    if buttons.just_released(MouseButton::Left) {
                        handle_piece_placement(
                            &mut player,
                            &board_transform,
                            &buttons,
                            &mut reset_piece_position_evw,
                            &board,
                            mpos,
                            &mut make_move_evw,
                            &mut set_sqr_color_evw,
                        );
                    }
                } else if player.current_state == PlayerInputState::PieceSelected {
                    if buttons.just_pressed(MouseButton::Left) {
                        handle_piece_placement(
                            &mut player,
                            &board_transform,
                            &buttons,
                            &mut reset_piece_position_evw,
                            &board,
                            mpos,
                            &mut make_move_evw,
                            &mut set_sqr_color_evw,
                        );
                    }
                }

                if buttons.just_pressed(MouseButton::Right) {
                    cancel_piece_selection(
                        &mut player,
                        &mut reset_piece_position_evw,
                        &mut set_sqr_color_evw,
                    )
                }
            }
        }
    }
}

pub fn handle_piece_selection(
    buttons: &Res<Input<MouseButton>>,
    board_transform: &Res<BoardUITransform>,
    board: &Board,
    player: &mut Mut<HumanPlayer>,
    mpos: Vec2,
    set_sqr_color_evw: &mut EventWriter<BoardSetSquareColor>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(piece_sqr) = board_transform.get_hovered_square(mpos) {
            player.selected_piece_sqr = piece_sqr;
            let idx = idx_from_coord(piece_sqr.file_idx, piece_sqr.rank_idx);
            if is_color(board.square[idx as usize], board.color_to_move) {
                println!("Highlight legal moves");
                set_sqr_color_evw.send(BoardSetSquareColor {
                    color: SquareColorTypes::Selected,
                    rank: player.selected_piece_sqr.rank_idx,
                    file: player.selected_piece_sqr.file_idx,
                });
                player.current_state = PlayerInputState::DraggingPiece;
            }
        }
    }
}

pub fn cancel_piece_selection(
    player: &mut Mut<HumanPlayer>,
    reset_piece_position_evw: &mut EventWriter<BoardUIResetPiecePosition>,
    set_sqr_color_evw: &mut EventWriter<BoardSetSquareColor>,
) {
    if player.current_state != PlayerInputState::None {
        player.current_state = PlayerInputState::None;
        set_sqr_color_evw.send(BoardSetSquareColor {
            color: SquareColorTypes::Normal,
            rank: player.selected_piece_sqr.rank_idx,
            file: player.selected_piece_sqr.file_idx,
        });
        reset_piece_position_evw.send(BoardUIResetPiecePosition {
            origin_file: player.selected_piece_sqr.file_idx,
            origin_rank: player.selected_piece_sqr.rank_idx,
        });
    }
}

pub fn handle_piece_placement(
    mut player: &mut Mut<HumanPlayer>,
    board_transform: &Res<BoardUITransform>,
    buttons: &Res<Input<MouseButton>>,
    mut reset_piece_position_evw: &mut EventWriter<BoardUIResetPiecePosition>,
    board: &Board,
    mpos: Vec2,
    mut make_move_evw: &mut EventWriter<BoardMakeMove>,
    mut set_sqr_color_evw: &mut EventWriter<BoardSetSquareColor>,
) {
    if let Some(target_sqr) = board_transform.get_hovered_square(mpos) {
        if target_sqr.is_eq(player.selected_piece_sqr) {
            reset_piece_position_evw.send(BoardUIResetPiecePosition {
                origin_file: target_sqr.file_idx,
                origin_rank: target_sqr.rank_idx,
            });
            if player.current_state == PlayerInputState::DraggingPiece {
                player.current_state = PlayerInputState::PieceSelected;
            } else {
                player.current_state = PlayerInputState::None;
                set_sqr_color_evw.send(BoardSetSquareColor {
                    color: SquareColorTypes::Normal,
                    rank: player.selected_piece_sqr.rank_idx,
                    file: player.selected_piece_sqr.file_idx,
                });
            }
        } else {
            let target_idx = idx_from_coord(target_sqr.file_idx, target_sqr.rank_idx);
            if is_color(board.square[target_idx as usize], board.color_to_move) && board.square[target_idx as usize] != 0 {
                cancel_piece_selection(&mut player, &mut reset_piece_position_evw, &mut set_sqr_color_evw);
                handle_piece_selection(
                    &buttons,
                    &board_transform,
                    board,
                    player,
                    mpos,
                    &mut set_sqr_color_evw,
                );
            } else {
                player_make_move(
                    Move::from_start_end(idx_from_coord(player.selected_piece_sqr.file_idx, player.selected_piece_sqr.rank_idx), target_idx), 
                    &mut make_move_evw
                );
                player.current_state = PlayerInputState::None;
            }
        }
    } else {
        cancel_piece_selection(&mut player, &mut reset_piece_position_evw, &mut set_sqr_color_evw);
    }
}

pub fn player_make_move(
    mov: Move,
    make_move_evw: &mut EventWriter<BoardMakeMove>,
) {
    make_move_evw.send(BoardMakeMove {
        mov, 
        in_search: false,
    });
}