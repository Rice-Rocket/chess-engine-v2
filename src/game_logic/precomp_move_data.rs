use bevy::prelude::*;

use super::{board::{WHITE_INDEX, BLACK_INDEX}, representation::coord_from_idx};


#[derive(Resource)]
pub struct PrecomputedMoveData {
    pub direction_offsets: [i32; 8],
    pub num_sqrs_to_edge: [[i32; 8]; 64],
    
    pub knight_moves: [Vec<u8>; 64],
    pub king_moves: [Vec<u8>; 64],
    pub pawn_attack_dirs: [[u8; 2]; 2],

    pub pawn_attacks_white: [Vec<i32>; 64],
    pub pawn_attacks_black: [Vec<i32>; 64],
    pub direction_lookup: [i32; 127],

    pub king_attack_bitboards: [u64; 64],
    pub knight_attack_bitboards: [u64; 64],
    pub pawn_attack_bitboards: [[u64; 2]; 64],
    
    pub rook_moves: [u64; 64],
    pub bishop_moves: [u64; 64],
    pub queen_moves: [u64; 64],

    pub manhattan_distance: [[u32; 64]; 64],
    pub king_distance: [[u32; 64]; 64],
    pub center_manhattan_distance: [u32; 64],
}

impl PrecomputedMoveData {
    pub fn num_rook_moves_to_sqr(&self, start_sqr: u32, target_sqr: u32) -> u32 {
        return self.manhattan_distance[start_sqr as usize][target_sqr as usize];
    }
    pub fn num_king_moves_to_sqr(&self, start_sqr: u32, target_sqr: u32) -> u32 {
        return self.king_distance[start_sqr as usize][target_sqr as usize];
    }
}


impl Default for PrecomputedMoveData {
    fn default() -> Self {
        let mut pawn_attacks_white: [Vec<i32>; 64] = std::array::from_fn(|_| vec![]);
        let mut pawn_attacks_black: [Vec<i32>; 64] = std::array::from_fn(|_| vec![]);
        let mut num_sqrs_to_edge: [[i32; 8]; 64] = [[0; 8]; 64]; // index with [square][direction]
        let mut knight_moves: [Vec<u8>; 64] = std::array::from_fn(|_| vec![]);
        let mut king_moves: [Vec<u8>; 64] = std::array::from_fn(|_| vec![]);
        
        let mut rook_moves: [u64; 64] = [0; 64];
        let mut bishop_moves: [u64; 64] = [0; 64];
        let mut queen_moves: [u64; 64] = [0; 64];
        let pawn_attack_dirs: [[u8; 2]; 2] = [[4, 6], [7, 5]]; // index with [color index]
        
        let direction_offsets: [i32; 8] = [8, -8, -1, 1, 7, -7, 9, -9];
        let all_knight_jumps: [i32; 8] = [15, 17, -17, -15, 10, -6, 6, -10];
        let mut king_attack_bitboards: [u64; 64] = [0; 64];
        let mut knight_attack_bitboards: [u64; 64] = [0; 64];
        let mut pawn_attack_bitboards: [[u64; 2]; 64] = [[0; 2]; 64]; // index with [square][color index]

        for sqr_idx in 0..64 {
            let y = sqr_idx / 8;
            let x = sqr_idx - y * 8;

            let north = 7 - y;
            let south = y;
            let west = x;
            let east = 7 - x;
            num_sqrs_to_edge[sqr_idx as usize][0] = north;
            num_sqrs_to_edge[sqr_idx as usize][1] = south;
            num_sqrs_to_edge[sqr_idx as usize][2] = west;
            num_sqrs_to_edge[sqr_idx as usize][3] = east;
            num_sqrs_to_edge[sqr_idx as usize][4] = north.min(west);
            num_sqrs_to_edge[sqr_idx as usize][5] = south.min(east);
            num_sqrs_to_edge[sqr_idx as usize][6] = north.min(east);
            num_sqrs_to_edge[sqr_idx as usize][7] = south.min(west);

            let legal_knight_jumps = &mut knight_moves[sqr_idx as usize];
            let knight_bitboard = &mut knight_attack_bitboards[sqr_idx as usize];
            for knight_jump_delta in all_knight_jumps.iter() {
                let knight_jump_sqr = sqr_idx + knight_jump_delta;
                if knight_jump_sqr >= 0 && knight_jump_sqr < 64 {
                    let knight_sqr_y = knight_jump_sqr / 8;
                    let knight_sqr_x = knight_jump_sqr - knight_sqr_y * 8;
                    let max_coord_move_dst = (x - knight_sqr_x).abs().max((y - knight_sqr_y).abs());
                    if max_coord_move_dst == 2 {
                        legal_knight_jumps.push(knight_jump_sqr as u8);
                        *knight_bitboard |= 1u64 << knight_jump_sqr
                    }
                }
            }

            let legal_king_moves = &mut king_moves[sqr_idx as usize];
            let king_bitboard = &mut king_attack_bitboards[sqr_idx as usize];
            for king_move_delta in direction_offsets.iter() {
                let king_move_sqr = sqr_idx + king_move_delta;
                if king_move_sqr >= 0 && king_move_sqr < 64 {
                    let king_sqr_y = king_move_sqr / 8;
                    let king_sqr_x = king_move_sqr - king_sqr_y * 8;
                    let max_coord_move_dst = (x - king_sqr_x).abs().max((y - king_sqr_y).abs());
                    if max_coord_move_dst == 1 {
                        legal_king_moves.push(king_move_sqr as u8);
                        *king_bitboard |= 1u64 << king_move_sqr;
                    }
                }
            }

            let pawn_captures_white = &mut pawn_attacks_white[sqr_idx as usize];
            let pawn_captures_black = &mut pawn_attacks_black[sqr_idx as usize];
            let pawn_bitboard = &mut pawn_attack_bitboards[sqr_idx as usize];
            if x > 0 {
                if y < 7 {
                    pawn_captures_white.push(sqr_idx + 7);
                    pawn_bitboard[WHITE_INDEX as usize] |= 1u64 << (sqr_idx + 7);
                }
                if y > 0 {
                    pawn_captures_black.push(sqr_idx - 9);
                    pawn_bitboard[BLACK_INDEX as usize] |= 1u64 << (sqr_idx - 9);
                }
            }
            if x < 7 {
                if y < 7 {
                    pawn_captures_white.push(sqr_idx + 9);
                    pawn_bitboard[WHITE_INDEX as usize] |= 1u64 << (sqr_idx + 9);
                }
                if y > 0 {
                    pawn_captures_black.push(sqr_idx - 7);
                    pawn_bitboard[BLACK_INDEX as usize] |= 1u64 << (sqr_idx - 7);
                }
            }

            for direction_idx in 0..4 {
                let cur_dir_offset = direction_offsets[direction_idx];
                for n in 0..num_sqrs_to_edge[sqr_idx as usize][direction_idx] {
                    let target_sqr = sqr_idx + cur_dir_offset * (n + 1);
                    rook_moves[sqr_idx as usize] |= 1u64 << target_sqr;
                }
            }

            for direction_idx in 4..8 {
                let cur_dir_offset = direction_offsets[direction_idx];
                for n in 0..num_sqrs_to_edge[sqr_idx as usize][direction_idx] {
                    let target_sqr = sqr_idx + cur_dir_offset * (n + 1);
                    bishop_moves[sqr_idx as usize] |= 1u64 << target_sqr;
                }
            }

            queen_moves[sqr_idx as usize] = rook_moves[sqr_idx as usize] | bishop_moves[sqr_idx as usize];
        }

        let mut direction_lookup: [i32; 127] = [0; 127];
        for i in 0i32..127i32 {
            let offset = i - 63;
            let abs_offset = offset.abs();
            let mut abs_dir = 1;
            if abs_offset % 9 == 0 {
                abs_dir = 9;
            } else if abs_offset % 8 == 0 {
                abs_dir = 8;
            } else if abs_offset % 7 == 0 {
                abs_dir = 7;
            }

            direction_lookup[i as usize] = abs_dir * if offset >= 0 { if offset == 0 { 0 } else { 1 } } else { -1 };
        }

        let mut manhattan_distance: [[u32; 64]; 64] = [[0; 64]; 64];
        let mut king_distance: [[u32; 64]; 64] = [[0; 64]; 64];
        let mut center_manhattan_distance: [u32; 64] = [0; 64];
        for sqr_a in 0..64 {
            let coord_a = coord_from_idx(sqr_a);
            let file_center_dst = (3 - coord_a.file_idx as i32).max(coord_a.file_idx as i32 - 4) as u32;
            let rank_center_dst = (3 - coord_a.rank_idx as i32).max(coord_a.rank_idx as i32 - 4) as u32;
            center_manhattan_distance[sqr_a as usize] = file_center_dst + rank_center_dst;
            for sqr_b in 0..64 {
                let coord_b = coord_from_idx(sqr_b);
                let file_dst = (coord_a.file_idx as i32 - coord_b.file_idx as i32).abs();
                let rank_dst = (coord_a.rank_idx as i32 - coord_b.rank_idx as i32).abs();
                manhattan_distance[sqr_a as usize][sqr_b as usize] = (file_dst + rank_dst) as u32;
                king_distance[sqr_a as usize][sqr_b as usize] = file_dst.max(rank_dst) as u32;
            }
        };

        PrecomputedMoveData {
            direction_offsets,
            num_sqrs_to_edge,
            knight_moves,
            king_moves,
            pawn_attack_dirs,
            pawn_attacks_white,
            pawn_attacks_black,
            direction_lookup,
            king_attack_bitboards,
            knight_attack_bitboards,
            pawn_attack_bitboards,
            rook_moves,
            bishop_moves,
            queen_moves,
            manhattan_distance,
            king_distance,
            center_manhattan_distance,
        }
    }
}