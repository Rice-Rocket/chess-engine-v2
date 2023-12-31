use bevy::prelude::*;
use super::{
    piece::Piece, piece_list::PieceList,
    moves::{self, Move}, zobrist::Zobrist, coord::Coord, game_state::GameState,
};
use crate::{fen, move_gen::{magics::MagicBitBoards, bitboard::bb::BitBoard}};
use crate::move_gen::bitboard::utils::*;


#[derive(Resource)]
pub struct Board {
    pub square: [Piece; 64],
    pub king_square: [Coord; 2],

    pub piece_bitboards: [BitBoard; Piece::MAX_PIECE_INDEX as usize + 1],
    pub color_bitboards: [BitBoard; 2],
    pub all_pieces_bitboard: BitBoard,
    pub friendly_orthogonal_sliders: BitBoard,
    pub friendly_diagonal_sliders: BitBoard,
    pub enemy_orthogonal_sliders: BitBoard,
    pub enemy_diagonal_sliders: BitBoard,

    pub total_pieces_no_pawns_kings: usize,
    all_pieces: Vec<PieceList>,
    game_state_history: Vec<GameState>,
    cached_in_check_val: bool,
    has_cached_in_check_val: bool,

    pub white_to_move: bool,
    pub move_color: u8,
    pub opponent_color: u8,
    pub move_color_idx: usize,
    pub opponent_color_idx: usize,
    pub repeat_position_history: Vec<u64>,

    pub plycount: usize,
    pub current_state: GameState, 
    pub move_log: Vec<Move>,
}

impl Default for Board {
    fn default() -> Self {
        let knights = vec![PieceList::new(10); 2];
        let pawns = vec![PieceList::new(8); 2];
        let rooks = vec![PieceList::new(10); 2];
        let bishops = vec![PieceList::new(10); 2];
        let queens = vec![PieceList::new(9); 2];
        let kings = vec![PieceList::new(1); 2];
        let empty_list = PieceList::new(0);
        Self {
            square: [Piece::NULL; 64],
            king_square: [Coord::NULL; 2],

            piece_bitboards: [BitBoard(0); Piece::MAX_PIECE_INDEX as usize + 1],
            color_bitboards: [BitBoard(0); 2],
            all_pieces_bitboard: BitBoard(0),
            friendly_orthogonal_sliders: BitBoard(0),
            friendly_diagonal_sliders: BitBoard(0),
            enemy_orthogonal_sliders: BitBoard(0),
            enemy_diagonal_sliders: BitBoard(0),

            total_pieces_no_pawns_kings: 0,
            all_pieces: vec![
                empty_list.clone(),
				pawns[Board::WHITE_INDEX].clone(),
				knights[Board::WHITE_INDEX].clone(),
				bishops[Board::WHITE_INDEX].clone(),
				rooks[Board::WHITE_INDEX].clone(),
				queens[Board::WHITE_INDEX].clone(),
                kings[Board::WHITE_INDEX].clone(),
				empty_list.clone(),
				pawns[Board::BLACK_INDEX].clone(),
				knights[Board::BLACK_INDEX].clone(),
				bishops[Board::BLACK_INDEX].clone(),
				rooks[Board::BLACK_INDEX].clone(),
				queens[Board::BLACK_INDEX].clone(),
                kings[Board::BLACK_INDEX].clone(),
            ],
            game_state_history: Vec::with_capacity(64),
            cached_in_check_val: false,
            has_cached_in_check_val: false,

            white_to_move: true,
            move_color: Piece::WHITE,
            opponent_color: Piece::BLACK,
            move_color_idx: Board::WHITE_INDEX,
            opponent_color_idx: Board::BLACK_INDEX,
            repeat_position_history: Vec::with_capacity(64),
            
            plycount: 0,
            current_state: GameState {
                captured_ptype: 0,
                en_passant_file: 0,
                castling_rights: 0,
                fifty_move_counter: 0,
                zobrist_key: 0,
            },
            move_log: Vec::new(),
        }
    }
}

impl Board {
    pub const WHITE_INDEX: usize = 0;
    pub const BLACK_INDEX: usize = 1;

    pub fn get_piece_list(&self, piece: Piece) -> &PieceList {
        return &self.all_pieces[piece.color_index() * 7 + piece.piece_type() as usize];
    }
    pub fn get_piece_list_mut(&mut self, piece: Piece) -> &mut PieceList {
        return &mut self.all_pieces[piece.color_index() * 7 + piece.piece_type() as usize];
    }
}

#[derive(Event)]
pub struct BoardUnmakeMove {
    pub mov: moves::Move,
    pub in_search: bool,
}

// #[derive(Event)]
// pub struct BoardLoadPosition {
//     fen_str: Option<String>,
// }


impl Board {
    pub fn make_move(&mut self, mov: Move, in_search: bool, zobrist: &Zobrist) {
        // print_board(self.square);

        let start_sqr = mov.start();
        let target_sqr = mov.target();
        let move_flag = mov.move_flag();
        let is_promotion: bool = mov.is_promotion();
        let is_en_passant: bool = move_flag == Move::EN_PASSANT_CAPTURE;
    
        let moved_piece = self.square[start_sqr.index()];
        let moved_ptype = moved_piece.piece_type();
        let captured_piece = if is_en_passant { Piece::new(Piece::PAWN | self.opponent_color) } else { self.square[target_sqr.index()] };
        let captured_ptype = captured_piece.piece_type();

        let prev_castle_state = self.current_state.castling_rights;
        let prev_en_passant_file = self.current_state.en_passant_file;
        let mut new_zobrist_key = self.current_state.zobrist_key;
        let mut new_castling_rights = self.current_state.castling_rights;
        let mut new_en_passant_file = 0;

        self.move_piece(moved_piece, start_sqr, target_sqr);

        if captured_ptype != Piece::NONE {
            let mut capture_sqr = target_sqr;
            if is_en_passant {
                capture_sqr = target_sqr + if self.white_to_move { -8 } else { 8 };
                self.square[capture_sqr.index()] = Piece::NULL;
            }
            if captured_ptype != Piece::PAWN {
                self.total_pieces_no_pawns_kings -= 1;
            }

            self.get_piece_list_mut(captured_piece).remove_piece(capture_sqr);
            self.piece_bitboards[captured_piece.index()].toggle_square(capture_sqr.square());
            self.color_bitboards[self.opponent_color_idx].toggle_square(capture_sqr.square());
            new_zobrist_key ^= zobrist.pieces_array[capture_sqr.index()][captured_piece.index()];
        }

        if moved_ptype == Piece::KING {
            self.king_square[self.move_color_idx] = target_sqr;
            new_castling_rights &= if self.white_to_move { 0b1100 } else { 0b0011 };
            if move_flag == Move::CASTLING {
                let rook_piece = Piece::new(Piece::ROOK | self.move_color);
                let kingside = target_sqr == Coord::G1 || target_sqr == Coord::G8;
                let castling_rook_from = if kingside { target_sqr + 1 } else { target_sqr - 2 };
                let castling_rook_to = if kingside { target_sqr - 1 } else { target_sqr + 1 };

                self.piece_bitboards[rook_piece.index()].toggle_squares(castling_rook_from.square(), castling_rook_to.square());
                self.color_bitboards[self.move_color_idx].toggle_squares(castling_rook_from.square(), castling_rook_to.square());
                self.get_piece_list_mut(rook_piece).move_piece(castling_rook_from, castling_rook_to);
                self.square[castling_rook_from.index()] = Piece::NULL;
                self.square[castling_rook_to.index()] = rook_piece;
                
                new_zobrist_key ^= zobrist.pieces_array[castling_rook_from.index()][rook_piece.index()];
                new_zobrist_key ^= zobrist.pieces_array[castling_rook_to.index()][rook_piece.index()];
            }
        }
        if is_promotion {
            self.total_pieces_no_pawns_kings += 1;
            let prom_ptype = match move_flag {
                Move::QUEEN_PROMOTION => Piece::QUEEN,
                Move::ROOK_PROMOTION => Piece::ROOK,
                Move::KNIGHT_PROMOTION => Piece::KNIGHT,
                Move::BISHOP_PROMOTION => Piece::BISHOP,
                _ => Piece::NONE,
            };
            let prom_piece = Piece::new(prom_ptype | self.move_color);
            self.piece_bitboards[moved_piece.index()].toggle_square(target_sqr.square());
            self.piece_bitboards[prom_piece.index()].toggle_square(target_sqr.square());
            self.get_piece_list_mut(moved_piece).remove_piece(target_sqr);
            self.get_piece_list_mut(prom_piece).add_piece(target_sqr);
            self.square[target_sqr.index()] = prom_piece;
        }

        if move_flag == Move::PAWN_TWO_FORWARD {
            let file = start_sqr.file() + 1;
            new_en_passant_file = file;
            new_zobrist_key ^= zobrist.en_passant_file[file as usize];
        }

        if prev_castle_state != 0 {
            if target_sqr == Coord::H1 || start_sqr == Coord::H1 {
                new_castling_rights &= GameState::CLEAR_WHITE_KINGSIDE_MASK;
            } else if target_sqr == Coord::A1 || start_sqr == Coord::A1 {
                new_castling_rights &= GameState::CLEAR_WHITE_QUEENSIDE_MASK;
            } else if target_sqr == Coord::H8 || start_sqr == Coord::H8 {
                new_castling_rights &= GameState::CLEAR_BLACK_KINGSIDE_MASK;
            } else if target_sqr == Coord::A8 || start_sqr == Coord::A8 {
                new_castling_rights &= GameState::CLEAR_BLACK_QUEENSIDE_MASK;
            }
        }

        new_zobrist_key ^= zobrist.side_to_move;
        new_zobrist_key ^= zobrist.pieces_array[start_sqr.index()][moved_piece.index()];
        new_zobrist_key ^= zobrist.pieces_array[target_sqr.index()][self.square[target_sqr.index()].index()];
        new_zobrist_key ^= zobrist.en_passant_file[prev_en_passant_file as usize];

        if new_castling_rights != prev_castle_state {
            new_zobrist_key ^= zobrist.castling_rights[prev_castle_state as usize];
            new_zobrist_key ^= zobrist.castling_rights[new_castling_rights as usize];
        }

        self.white_to_move = !self.white_to_move;
        self.move_color = if self.white_to_move { Piece::WHITE } else { Piece::BLACK };
        self.opponent_color = if self.white_to_move { Piece::BLACK } else { Piece::WHITE };
        self.move_color_idx = 1 - self.move_color_idx;
        self.opponent_color_idx = 1 - self.opponent_color_idx;
        self.plycount += 1;
        let mut new_fifty_move_counter = self.current_state.fifty_move_counter + 1;

        self.all_pieces_bitboard = self.color_bitboards[Board::WHITE_INDEX] | self.color_bitboards[Board::BLACK_INDEX];
        self.update_slider_bitboards();

        if moved_ptype == Piece::PAWN || captured_ptype != Piece::NONE {
            if !in_search {
                self.repeat_position_history.clear();
            }
            new_fifty_move_counter = 0;
        }

        let new_state = GameState {
            captured_ptype, 
            en_passant_file: new_en_passant_file,
            castling_rights: new_castling_rights,
            fifty_move_counter: new_fifty_move_counter,
            zobrist_key: new_zobrist_key,
        };
        self.game_state_history.push(new_state);
        self.current_state = new_state;
        self.has_cached_in_check_val = false;
        if !in_search {
            self.repeat_position_history.push(new_state.zobrist_key);
            self.move_log.push(mov);
        }
    }
    
    pub fn unmake_move(&mut self, mov: Move, in_search: bool) {
        self.white_to_move = !self.white_to_move;
        self.move_color = if self.white_to_move { Piece::WHITE } else { Piece::BLACK };
        self.opponent_color = if self.white_to_move { Piece::BLACK } else { Piece::WHITE };
        self.move_color_idx = 1 - self.move_color_idx;
        self.opponent_color_idx = 1 - self.opponent_color_idx;
        let undoing_white_move = self.white_to_move;

        let move_from = mov.start();
        let move_to = mov.target();
        let move_flag = mov.move_flag();

        let undoing_en_passant = move_flag == Move::EN_PASSANT_CAPTURE;
        let undoing_promotion = mov.is_promotion();
        let undoing_capture = self.current_state.captured_ptype != Piece::NONE;

        let moved_piece = if undoing_promotion { Piece::new(Piece::PAWN | self.move_color) } else { self.square[move_to.index()] };
        let moved_ptype = moved_piece.piece_type();
        let captured_ptype = self.current_state.captured_ptype;

        if undoing_promotion {
            let promoted_piece = self.square[move_to.index()];
            let pawn_piece = Piece::new(Piece::PAWN | self.move_color);
            self.total_pieces_no_pawns_kings -= 1;

            self.get_piece_list_mut(promoted_piece).remove_piece(move_to);
            self.get_piece_list_mut(moved_piece).add_piece(move_to);
            self.piece_bitboards[promoted_piece.index()].toggle_square(move_to.square());
            self.piece_bitboards[pawn_piece.index()].toggle_square(move_to.square());
        }

        self.move_piece(moved_piece, move_to, move_from);

        if undoing_capture {
            let mut capture_square = move_to;
            let captured_piece = Piece::new(captured_ptype | self.opponent_color);

            if undoing_en_passant {
                capture_square = move_to + (if undoing_white_move { -8 } else { 8 });
            }
            if captured_ptype != Piece::PAWN {
                self.total_pieces_no_pawns_kings += 1;
            }

            self.piece_bitboards[captured_piece.index()].toggle_square(capture_square.square());
            self.color_bitboards[self.opponent_color_idx].toggle_square(capture_square.square());
            self.get_piece_list_mut(captured_piece).add_piece(capture_square);
            self.square[capture_square.index()] = captured_piece;
        }

        if moved_ptype == Piece::KING {
            self.king_square[self.move_color_idx] = move_from;
            if move_flag == Move::CASTLING {
                let rook_piece = Piece::new(Piece::ROOK | self.move_color);
                let kingside = move_to == Coord::G1 || move_to == Coord::G8;
                let rook_square_before_castling = if kingside { move_to + 1 } else { move_to - 2 };
                let rook_square_after_castling = if kingside { move_to - 1 } else { move_to + 1 };

                self.piece_bitboards[rook_piece.index()].toggle_squares(rook_square_after_castling.square(), rook_square_before_castling.square());
                self.color_bitboards[self.move_color_idx].toggle_squares(rook_square_after_castling.square(), rook_square_before_castling.square());
                self.square[rook_square_after_castling.index()] = Piece::NULL;
                self.square[rook_square_before_castling.index()] = rook_piece;
                self.get_piece_list_mut(rook_piece).move_piece(rook_square_after_castling, rook_square_before_castling);
            }
        }

        self.all_pieces_bitboard = self.color_bitboards[Board::WHITE_INDEX] | self.color_bitboards[Board::BLACK_INDEX];
        self.update_slider_bitboards();
        
        if !in_search && self.repeat_position_history.len() > 0 {
            self.repeat_position_history.pop();
        }
        if !in_search {
            self.move_log.remove(self.move_log.len() - 1);
        }

        self.game_state_history.pop();
        self.current_state = self.game_state_history[self.game_state_history.len() - 1];
        self.plycount -= 1;
        self.has_cached_in_check_val = false;
    }

    pub fn make_null_move(&mut self, zobrist: &Res<Zobrist>) {
        self.white_to_move = !self.white_to_move;
        self.plycount += 1;
        
        let mut new_zobrist_key = self.current_state.zobrist_key;
        new_zobrist_key ^= zobrist.side_to_move;
        new_zobrist_key ^= zobrist.en_passant_file[self.current_state.en_passant_file as usize];

        let new_state = GameState {
            captured_ptype: Piece::NONE,
            en_passant_file: 0,
            castling_rights: self.current_state.castling_rights,
            fifty_move_counter: self.current_state.fifty_move_counter + 1,
            zobrist_key: new_zobrist_key,
        };
        self.current_state = new_state;
        self.game_state_history.push(new_state);
        self.update_slider_bitboards();
        self.has_cached_in_check_val = true;
        self.cached_in_check_val = false;
    }

    pub fn unmake_null_move(&mut self) {
        self.white_to_move = !self.white_to_move;
        self.plycount -= 1;
        self.game_state_history.pop();
        self.current_state = self.game_state_history[self.game_state_history.len() - 1];
        self.update_slider_bitboards();
        self.has_cached_in_check_val = true;
        self.cached_in_check_val = false;
    }
    
    pub fn load_position(&mut self, fen_str: Option<String>, zobrist: &mut Zobrist) {
        *self = Self::default();
        let loaded_pos = match fen_str {
            Some(str) => fen::position_from_fen(str),
            None => fen::position_from_fen(String::from(fen::START_FEN))
        };

        for sqr_idx in 0i8..64i8 {
            let sqr = Coord::from_idx(sqr_idx);
            let piece = Piece::new(loaded_pos.squares[sqr.index()]);
            let ptype = piece.piece_type();
            let color_idx = if piece.is_color(Piece::WHITE) { Board::WHITE_INDEX } else { Board::BLACK_INDEX };
            self.square[sqr.index()] = piece;
    
            if ptype != Piece::NONE {
                self.piece_bitboards[piece.index()].set_square(sqr_idx);
                self.color_bitboards[color_idx].set_square(sqr_idx);
                if ptype == Piece::KING {
                    self.king_square[color_idx] = sqr;
                }
                self.get_piece_list_mut(piece).add_piece(sqr);
                self.total_pieces_no_pawns_kings += if ptype == Piece::KING || ptype == Piece::PAWN { 0 } else { 1 };
            }
        }
    
        self.white_to_move = loaded_pos.white_to_move;
        self.move_color = if self.white_to_move { Piece::WHITE } else { Piece::BLACK };
        self.opponent_color = if self.white_to_move { Piece::BLACK } else { Piece::WHITE };
        self.move_color_idx = if self.white_to_move { 0 } else { 1 };
        self.opponent_color_idx = 1 - self.move_color_idx;

        self.all_pieces_bitboard = self.color_bitboards[Board::WHITE_INDEX] | self.color_bitboards[Board::BLACK_INDEX];
        self.update_slider_bitboards();
    
        let white_castle = (if loaded_pos.white_castle_kingside { 1 << 0 } else { 0 }) | (if loaded_pos.white_castle_queenside { 1 << 1 } else { 0 });
        let black_castle = (if loaded_pos.black_castle_kingside { 1 << 2 } else { 0 }) | (if loaded_pos.black_castle_queenside { 1 << 3 } else { 0 });
        let castling_rights = white_castle | black_castle;

        self.plycount = (loaded_pos.move_count as usize - 1) * 2 + (if self.white_to_move { 0 } else { 1 });
        self.current_state = GameState {
            captured_ptype: Piece::NONE,
            en_passant_file: loaded_pos.ep_file,
            castling_rights,
            fifty_move_counter: loaded_pos.fifty_move_ply_count,
            zobrist_key: 0
        };
        let zobrist_key = zobrist.calc_zobrist_key(self);
        self.current_state.zobrist_key = zobrist_key;
        self.repeat_position_history.push(zobrist_key);
        self.game_state_history.push(self.current_state);
    }

    pub fn in_check(&mut self, magic: &MagicBitBoards, bbutils: &BitBoardUtils) -> bool {
        if self.has_cached_in_check_val {
            return self.cached_in_check_val;
        }
        self.cached_in_check_val = self.get_in_check_state(magic, bbutils);
        self.has_cached_in_check_val = true;
        return self.cached_in_check_val;
    }
    fn get_in_check_state(&self, magic: &MagicBitBoards, bbutils: &BitBoardUtils) -> bool {
        let king_sqr = self.king_square[self.move_color_idx];
        let blockers = self.all_pieces_bitboard;

        if self.enemy_orthogonal_sliders.0 != 0 {
            let rook_attacks = magic.get_rook_attacks(king_sqr, blockers);
            if (rook_attacks & self.enemy_orthogonal_sliders).0 != 0 {
                return true;
            }
        }
        if self.enemy_diagonal_sliders.0 != 0 {
            let bishop_attacks = magic.get_bishop_attacks(king_sqr, blockers);
            if (bishop_attacks & self.enemy_diagonal_sliders).0 != 0 {
                return true;
            }
        }

        let enemy_knights = self.piece_bitboards[Piece::new(Piece::KNIGHT | self.opponent_color).index()];
        if (bbutils.knight_attacks[king_sqr.index()] & enemy_knights).0 != 0 {
            return true;
        }

        let enemy_pawns = self.piece_bitboards[Piece::new(Piece::PAWN | self.opponent_color).index()];
        let pawn_attack_mask = if self.white_to_move { bbutils.white_pawn_attacks[king_sqr.index()] } else { bbutils.black_pawn_attacks[king_sqr.index()] };
        if (pawn_attack_mask & enemy_pawns).0 != 0 {
            return true;
        }

        return false;
    }

    fn move_piece(&mut self, piece: Piece, start: Coord, target: Coord) {
        self.piece_bitboards[piece.index()].toggle_squares(start.square(), target.square());
        self.color_bitboards[piece.color_index()].toggle_squares(start.square(), target.square());

        self.get_piece_list_mut(piece).move_piece(start, target);
        self.square[start.index()] = Piece::NULL;
        self.square[target.index()] = piece;
    }
    fn update_slider_bitboards(&mut self) {
        let friendly_rook = Piece::new(Piece::ROOK | self.move_color);
        let friendly_queen = Piece::new(Piece::QUEEN | self.move_color);
        let friendly_bishop = Piece::new(Piece::BISHOP | self.move_color);
        self.friendly_orthogonal_sliders = self.piece_bitboards[friendly_rook.index()] | self.piece_bitboards[friendly_queen.index()];
        self.friendly_diagonal_sliders = self.piece_bitboards[friendly_bishop.index()] | self.piece_bitboards[friendly_queen.index()];

        let enemy_rook = Piece::new(Piece::ROOK | self.opponent_color);
        let enemy_queen = Piece::new(Piece::QUEEN | self.opponent_color);
        let enemy_bishop = Piece::new(Piece::BISHOP | self.opponent_color);
        self.enemy_orthogonal_sliders = self.piece_bitboards[enemy_rook.index()] | self.piece_bitboards[enemy_queen.index()];
        self.enemy_diagonal_sliders = self.piece_bitboards[enemy_bishop.index()] | self.piece_bitboards[enemy_queen.index()];
    }
}

// fn print_board(squares: [Piece; 64]) {
//     for y in 0..8 {
//         let mut string = String::new();
//         for x in 0..8 {
//             string += &format!("{:?}", squares[y * 8 + x]);
//         }
//         println!("{}", string);
//     }
// }

pub fn spawn_main_board(
    mut commands: Commands,
    mut zobrist: ResMut<Zobrist>,
) {
    let mut board = Board::default();

    board.load_position(None, &mut zobrist);
    commands.insert_resource(board);
}
