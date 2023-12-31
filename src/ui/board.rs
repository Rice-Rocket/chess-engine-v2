use bevy::{prelude::*, window::{PrimaryWindow, WindowResized}};

use crate::{
    board::moves::Move,
    board::coord::Coord,
    board::board::Board,
    game::manager::BoardMakeMove,
    board::piece::*,
    ui::*,
};

const PIECE_DEPTH: f32 = 0.1;
const PIECE_DRAG_DEPTH: f32 = 0.2;
const SIDE_PADDING: f32 = 50.0;

#[derive(Resource)]
pub struct BoardUI {
    pub show_legal_moves: bool,
    pub white_is_bottom: bool,
    pub last_made_move: Option<Move>,
    pub dragged_piece: Option<Coord>,
}

impl Default for BoardUI {
    fn default() -> Self {
        BoardUI {
            show_legal_moves: true,
            white_is_bottom: true,
            last_made_move: None,
            dragged_piece: None
        }
    }
}

#[derive(Resource)]
pub struct BoardUITransform {
    pub x_offset: f32,
    pub sqr_size: f32,
}

impl BoardUITransform {
    pub fn x_pos(&self, file: i8) -> f32 {
        file as f32 * self.sqr_size + self.x_offset + self.sqr_size / 2.0
    }
    pub fn y_pos(&self, rank: i8) -> f32 {
        rank as f32 * self.sqr_size + SIDE_PADDING + self.sqr_size / 2.0
    }
    pub fn pos_from_coord(&self, coord: Coord) -> Vec2 {
        Vec2::new(self.x_pos(coord.file()), self.y_pos(coord.rank()))
    }
    pub fn get_hovered_square(&self, mouse: Vec2) -> Option<Coord> {
        let file = ((mouse.x - self.x_offset) / self.sqr_size) as i8;
        let rank = ((mouse.y - SIDE_PADDING) / self.sqr_size) as i8;
        return match file >= 0 && file < 8 && rank >= 0 && rank < 8 {
            true => Some(Coord::new(file, 7 - rank)),
            false => None
        }
    }
}

impl Default for BoardUITransform {
    fn default() -> Self {
        BoardUITransform {
            x_offset: 0.0,
            sqr_size: 0.0,
        }
    }
}

#[derive(Component)]
pub struct BoardUISquare {
    pub color: SquareColorTypes,
    pub rank: i8,
    pub file: i8,
}

#[derive(Component)]
pub struct BoardUIPiece {
    pub piece: Piece,
    pub file: i8,
    pub rank: i8,
}

#[derive(Event)]
pub struct BoardUIResetPiecePosition {
    pub origin_file: i8,
    pub origin_rank: i8,
}

#[derive(Event)]
pub struct BoardSetSquareColor {
    pub color: SquareColorTypes,
    pub rank: i8,
    pub file: i8,
}

#[derive(Event)]
pub struct BoardResetSquareColors {
    pub color: Option<SquareColorTypes>
}

pub fn init_board_ui_transform(
    mut board_transform: ResMut<BoardUITransform>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    app_mode: Res<State<AppMode>>,
) {
    let window = window_query.get_single().unwrap();
    board_transform.sqr_size = (window.height() - SIDE_PADDING * 2.0) / 8.0;
    if app_mode.clone() == AppMode::GameAIAI {
        board_transform.x_offset = window.width() - window.height();
    } else {
        board_transform.x_offset = window.width() / 2.0 - window.height() / 2.0 + SIDE_PADDING;
    }
}

pub fn update_board_ui_transform(
    mut board_transform: ResMut<BoardUITransform>,
    mut window_resize_evr: EventReader<WindowResized>,
    app_mode: Res<State<AppMode>>,
) {
    for window_resize in window_resize_evr.iter() {
        board_transform.sqr_size = (window_resize.height - SIDE_PADDING * 2.0) / 8.0;
        if app_mode.clone() == AppMode::GameAIAI {
            board_transform.x_offset = window_resize.width - window_resize.height - SIDE_PADDING;
        } else {
            board_transform.x_offset = window_resize.width / 2.0 - window_resize.height / 2.0 + SIDE_PADDING;
        }
    }
}

pub fn spawn_board_ui(
    mut commands: Commands,
    board_theme: Res<BoardTheme>,
    board_transform: Res<BoardUITransform>,
    piece_theme: Res<PieceTheme>,
    board: Res<Board>,
) {
    for rank in 0..8 {
        for file in 0..8 {
            let sqr = Coord::new(file, rank);
            let sqr_component = BoardUISquare {
                color: SquareColorTypes::Normal,
                rank, file
            };
            let x_pos = board_transform.x_pos(file);
            let y_pos = board_transform.y_pos(rank);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: if (rank + file) % 2 == 0 { board_theme.light_squares.normal } else { board_theme.dark_squares.normal },
                        custom_size: Some(Vec2::new(board_transform.sqr_size, board_transform.sqr_size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                    ..default()
                },
                sqr_component
            ));

            let piece_component = BoardUIPiece {
                piece: board.square[sqr.index()],
                rank, file
            };
            if let Some(sprite) = piece_theme.get_piece_sprite(piece_component.piece) {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x_pos, y_pos, PIECE_DEPTH),
                        texture: sprite,
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    piece_component
                ));
            } // else {
            //     commands.spawn((
            //         SpriteBundle {
            //             transform: Transform::from_xyz(x_pos, y_pos + PIECE_DEPTH, 1.0),
            //             visibility: Visibility::Hidden,
            //             ..default()
            //         },
            //         piece_component
            //     ));
            // }
        }
    }
}

pub fn reset_board_pieces(
    mut commands: Commands,
    pieces_query: Query<Entity, With<BoardUIPiece>>,
    board_transform: Res<BoardUITransform>,
    board: Res<Board>,
    piece_theme: Res<PieceTheme>,
    debug_pos_loaded_evr: EventReader<DebugPositionLoaded>,
    app_mode: Res<State<AppMode>>,
) {
    if app_mode.clone() != AppMode::GameAIAI && debug_pos_loaded_evr.len() == 0 {
        return;
    };

    for piece in pieces_query.iter() {
        commands.entity(piece).despawn();
    }
    for rank in 0..8 {
        for file in 0..8 {
            let sqr = Coord::new(file, rank);
            let x_pos = board_transform.x_pos(file);
            let y_pos = board_transform.y_pos(rank);
            let piece_component = BoardUIPiece {
                piece: board.square[sqr.index()],
                rank, file
            };
            if let Some(sprite) = piece_theme.get_piece_sprite(piece_component.piece) {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x_pos, y_pos, PIECE_DEPTH),
                        texture: sprite,
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    piece_component
                ));
            }
        }
    }
}

pub fn update_pieces(
    mut commands: Commands,
    mut make_move_evr: EventReader<BoardMakeMove>,
    board_transform: Res<BoardUITransform>,
    // board: Res<Board>,
    piece_theme: Res<PieceTheme>,
    mut pieces_query: Query<(&mut BoardUIPiece, Entity, &mut Transform)>,
) {
    for make_move_event in make_move_evr.iter() {
        let mov = make_move_event.mov;
        let start = mov.start();
        let target = mov.target();
        let captured_sqr = if mov.move_flag() == Move::EN_PASSANT_CAPTURE {
            Coord::new(target.file(), start.rank()) } else { target };
        let (mut found_start, mut found_end) = (false, false);
        for (mut piece, piece_entity, mut transform) in pieces_query.iter_mut() {
            let piece_coord = Coord::new(piece.file, piece.rank);
            if piece_coord == start {
                let x_pos = board_transform.x_pos(target.file());
                let y_pos = board_transform.y_pos(target.rank());
                if !mov.is_promotion() {
                    piece.file = target.file();
                    piece.rank = target.rank();
                    transform.translation = Vec3::new(x_pos, y_pos, 0.0);
                } else {
                    commands.entity(piece_entity).despawn();
                    let piece_component = BoardUIPiece {
                        piece: Piece::new(mov.promotion_ptype() | piece.piece.color()),
                        rank: target.rank(), 
                        file: target.file()
                    };
                    if let Some(sprite) = piece_theme.get_piece_sprite(piece_component.piece) {
                        commands.spawn((
                            SpriteBundle {
                                transform: Transform::from_xyz(x_pos, y_pos + PIECE_DEPTH, 1.0),
                                texture: sprite,
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            piece_component
                        ));
                    }
                }
                found_start = true;
            } else if piece_coord == captured_sqr {
                commands.entity(piece_entity).despawn();
                found_end = true;
            }
            if found_start && found_end { break; }
        }

        if mov.move_flag() == Move::CASTLING {
            let kingside = target == Coord::G1 || target == Coord::G8;
            let castling_rook_from = if kingside { target + 1 } else { target - 2 };
            let castling_rook_to = if kingside { target - 1 } else { target + 1 };
            for (mut piece, _, mut transform) in pieces_query.iter_mut() {
                let piece_coord = Coord::new(piece.file, piece.rank);
                if piece_coord == castling_rook_from {
                    let x_pos = board_transform.x_pos(castling_rook_to.file());
                    let y_pos = board_transform.y_pos(castling_rook_to.rank());
                    piece.file = castling_rook_to.file();
                    piece.rank = castling_rook_to.rank();
                    transform.translation = Vec3::new(x_pos, y_pos, 0.0);
                    break;
                }
            }
        }
    }
}

pub fn update_board_ui(
    mut board_ui: ResMut<BoardUI>,
    mut set_square_evw: EventWriter<BoardSetSquareColor>,
    mut make_move_evr: EventReader<BoardMakeMove>,
) {
    for make_move_event in make_move_evr.iter() {
        if let Some(last_move) = board_ui.last_made_move {
            let last_move_start_coord = last_move.start();
            let last_move_end_coord = last_move.target();
            set_square_evw.send(BoardSetSquareColor {
                color: SquareColorTypes::Normal,
                rank: last_move_start_coord.rank(),
                file: last_move_start_coord.file(),
            });
            set_square_evw.send(BoardSetSquareColor {
                color: SquareColorTypes::Normal,
                rank: last_move_end_coord.rank(),
                file: last_move_end_coord.file(),
            });
        }
        let move_start_coord = make_move_event.mov.start();
        let move_end_coord = make_move_event.mov.target();
        set_square_evw.send(BoardSetSquareColor {
            color: SquareColorTypes::MoveFromHighlight,
            rank: move_start_coord.rank(),
            file: move_start_coord.file(),
        });
        set_square_evw.send(BoardSetSquareColor {
            color: SquareColorTypes::MoveToHighlight,
            rank: move_end_coord.rank(),
            file: move_end_coord.file(),
        });
        board_ui.last_made_move = Some(make_move_event.mov);
    };
}

pub fn update_board_ui_on_resize(
    board_transform: Res<BoardUITransform>,
    mut squares_query: Query<(&mut Transform, &mut Sprite, &BoardUISquare), Without<BoardUIPiece>>,
    mut pieces_query: Query<(&mut Transform, &BoardUIPiece), Without<BoardUISquare>>,
    mut window_resize_evr: EventReader<WindowResized>,
) {
    for _window_resize_event in window_resize_evr.iter() {
        for (mut transform, mut sprite, square) in squares_query.iter_mut() {
            let x_pos = board_transform.x_pos(square.file);
            let y_pos = board_transform.y_pos(square.rank);
            sprite.custom_size = Some(Vec2::new(board_transform.sqr_size, board_transform.sqr_size));
            transform.translation = Vec3::new(x_pos, y_pos, 0.0);
        }
        for (mut transform, piece) in pieces_query.iter_mut() {
            let x_pos = board_transform.x_pos(piece.file);
            let y_pos = board_transform.y_pos(piece.rank);
            transform.translation = Vec3::new(x_pos, y_pos, PIECE_DEPTH);
        }
    }
}

pub fn reset_piece_position(
    mut reset_position_evr: EventReader<BoardUIResetPiecePosition>,
    mut pieces_query: Query<(&BoardUIPiece, &mut Transform)>,
    board_transform: Res<BoardUITransform>,
) {
    for event in reset_position_evr.iter() {
        let rank = event.origin_rank;
        let file = event.origin_file;
        for (piece, mut transform) in pieces_query.iter_mut() {
            if piece.file == file && piece.rank == rank {
                let x_pos = board_transform.x_pos(file);
                let y_pos = board_transform.y_pos(rank);
                transform.translation = Vec3::new(x_pos, y_pos, 0.0);
            }
        }
    }
}

pub fn set_square_color(
    mut square_query: Query<(&mut Sprite, &mut BoardUISquare)>,
    mut set_square_evr: EventReader<BoardSetSquareColor>,
    board_theme: Res<theme::BoardTheme>,
) {
    for event in set_square_evr.iter() {
        for (mut sprite, mut square) in square_query.iter_mut() {
            if square.file == event.file && square.rank == event.rank {
                let color = match (event.color, (square.rank + square.file) % 2 == 0) {
                    (SquareColorTypes::Normal, true) => board_theme.light_squares.normal,
                    (SquareColorTypes::Normal, false) => board_theme.dark_squares.normal,
                    (SquareColorTypes::Legal, true) => board_theme.light_squares.legal,
                    (SquareColorTypes::Legal, false) => board_theme.dark_squares.legal,
                    (SquareColorTypes::Selected, true) => board_theme.light_squares.selected,
                    (SquareColorTypes::Selected, false) => board_theme.dark_squares.selected,
                    (SquareColorTypes::MoveFromHighlight, true) => board_theme.light_squares.move_from_highlight,
                    (SquareColorTypes::MoveFromHighlight, false) => board_theme.dark_squares.move_from_highlight,
                    (SquareColorTypes::MoveToHighlight, true) => board_theme.light_squares.move_to_highlight,
                    (SquareColorTypes::MoveToHighlight, false) => board_theme.dark_squares.move_to_highlight,
                };
                square.color = event.color;
                sprite.color = color;
            }
        }
    }
}

pub fn reset_square_colors(
    mut square_query: Query<(&mut Sprite, &mut BoardUISquare)>,
    mut reset_square_evr: EventReader<BoardResetSquareColors>,
    board_theme: Res<theme::BoardTheme>,
) {
    for event in reset_square_evr.iter() {
        for (mut sprite, mut square) in square_query.iter_mut() {
            if let Some(reset_color) = event.color {
                if reset_color == square.color {
                    let color = match (square.rank + square.file) % 2 == 0 {
                        true => board_theme.light_squares.normal,
                        false => board_theme.dark_squares.normal
                    };
                    square.color = SquareColorTypes::Normal;
                    sprite.color = color;
                }
            } else if square.color != SquareColorTypes::MoveFromHighlight && square.color != SquareColorTypes::MoveToHighlight {
                let color = match (square.rank + square.file) % 2 == 0 {
                    true => board_theme.light_squares.normal,
                    false => board_theme.dark_squares.normal
                };
                square.color = SquareColorTypes::Normal;
                sprite.color = color;
            }
        }
    }
}

pub fn drag_piece(
    window_query: Query<&Window, With<PrimaryWindow>>,
    board_ui: Res<BoardUI>,
    mut pieces_query: Query<(&BoardUIPiece, &mut Transform)>
) {
    if let Some(mpos) = window_query.single().cursor_position() {
        if let Some(piece_sqr) = board_ui.dragged_piece {
            for (piece, mut transform) in pieces_query.iter_mut() {
                if piece.rank == piece_sqr.rank() && piece.file == piece_sqr.file() {
                    transform.translation = Vec3::new(mpos.x, window_query.single().height() - mpos.y, PIECE_DRAG_DEPTH);
                }
            }
        }
    }
}