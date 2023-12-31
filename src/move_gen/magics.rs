use bevy::prelude::*;

use crate::board::coord::Coord;

use super::bitboard::bb::BitBoard;


#[derive(Resource)]
pub struct MagicBitBoards {
    pub rook_mask: [BitBoard; 64],
    pub bishop_mask: [BitBoard; 64],
    pub rook_attacks: [Vec<BitBoard>; 64],
    pub bishop_attacks: [Vec<BitBoard>; 64],
    // pub rook_attacks: [Box<[u64]>; 64],
    // pub bishop_attacks: [Box<[u64]>; 64],
    // pub rook_attacks: [[u64; MagicBitBoards::MIN_ROOK_LOOKUP_SIZE]; 64],
    // pub bishop_attacks: [[u64; MagicBitBoards::MIN_BISHOP_LOOKUP_SIZE]; 64],
}

impl MagicBitBoards {
    pub const ROOK_SHIFTS: [u32; 64] = [52, 52, 52, 52, 52, 52, 52, 52, 53, 53, 53, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 53, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 52, 54, 53, 53, 53, 53, 54, 53, 52, 53, 54, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 53, 54, 53, 52, 53, 53, 53, 53, 53, 53, 52];
    pub const BISHOP_SHIFTS: [u32; 64] = [58, 60, 59, 59, 59, 59, 60, 58, 60, 59, 59, 59, 59, 59, 59, 60, 59, 59, 57, 57, 57, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 57, 57, 57, 59, 59, 60, 60, 59, 59, 59, 59, 60, 60, 58, 60, 59, 59, 59, 59, 59, 58];

    // 64 - shift_amount
    // pub const ROOK_N_BITS: [u32; 64] = [64 - 52, 64 - 52, 64 - 52, 64 - 52, 64 - 52, 64 - 52, 64 - 52, 64 - 52, 64 - 53, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 54, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 54, 64 - 54, 64 - 53, 64 - 52, 64 - 54, 64 - 53, 64 - 53, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 52, 64 - 53, 64 - 54, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 54, 64 - 54, 64 - 53, 64 - 53, 64 - 54, 64 - 53, 64 - 52, 64 - 53, 64 - 53, 64 - 53, 64 - 53, 64 - 53, 64 - 53, 64 - 52];
    // pub const BISHOP_N_BITS: [u32; 64] = [64 - 58, 64 - 60, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 60, 64 - 58, 64 - 60, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 60, 64 - 59, 64 - 59, 64 - 57, 64 - 57, 64 - 57, 64 - 57, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 57, 64 - 55, 64 - 55, 64 - 57, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 57, 64 - 55, 64 - 55, 64 - 57, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 57, 64 - 57, 64 - 57, 64 - 57, 64 - 59, 64 - 59, 64 - 60, 64 - 60, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 60, 64 - 60, 64 - 58, 64 - 60, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 59, 64 - 58];
    
    // 1 << n_bits
    // pub const ROOK_SIZES: [usize; 64] = [1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 52), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 52), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 52), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 54), 1 << (64 - 53), 1 << (64 - 52), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 53), 1 << (64 - 52)];
    // pub const BISHOP_SIZES: [usize; 64] = [1 << (64 - 58), 1 << (64 - 60), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 60), 1 << (64 - 58), 1 << (64 - 60), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 60), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 57), 1 << (64 - 57), 1 << (64 - 57), 1 << (64 - 57), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 57), 1 << (64 - 55), 1 << (64 - 55), 1 << (64 - 57), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 57), 1 << (64 - 55), 1 << (64 - 55), 1 << (64 - 57), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 57), 1 << (64 - 57), 1 << (64 - 57), 1 << (64 - 57), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 60), 1 << (64 - 60), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 60), 1 << (64 - 60), 1 << (64 - 58), 1 << (64 - 60), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 59), 1 << (64 - 58)];

    pub const MIN_ROOK_LOOKUP_SIZE: usize = 4096;
    pub const MIN_BISHOP_LOOKUP_SIZE: usize = 512;

    pub const ROOK_MAGICS: [u64; 64] = [468374916371625120, 18428729537625841661, 2531023729696186408, 6093370314119450896, 13830552789156493815, 16134110446239088507, 12677615322350354425, 5404321144167858432, 2111097758984580, 18428720740584907710, 17293734603602787839, 4938760079889530922, 7699325603589095390, 9078693890218258431, 578149610753690728, 9496543503900033792, 1155209038552629657, 9224076274589515780, 1835781998207181184, 509120063316431138, 16634043024132535807, 18446673631917146111, 9623686630121410312, 4648737361302392899, 738591182849868645, 1732936432546219272, 2400543327507449856, 5188164365601475096, 10414575345181196316, 1162492212166789136, 9396848738060210946, 622413200109881612, 7998357718131801918, 7719627227008073923, 16181433497662382080, 18441958655457754079, 1267153596645440, 18446726464209379263, 1214021438038606600, 4650128814733526084, 9656144899867951104, 18444421868610287615, 3695311799139303489, 10597006226145476632, 18436046904206950398, 18446726472933277663, 3458977943764860944, 39125045590687766, 9227453435446560384, 6476955465732358656, 1270314852531077632, 2882448553461416064, 11547238928203796481, 1856618300822323264, 2573991788166144, 4936544992551831040, 13690941749405253631, 15852669863439351807, 18302628748190527413, 12682135449552027479, 13830554446930287982, 18302628782487371519, 7924083509981736956, 4734295326018586370];
    pub const BISHOP_MAGICS: [u64; 64] = [16509839532542417919, 14391803910955204223, 1848771770702627364, 347925068195328958, 5189277761285652493, 3750937732777063343, 18429848470517967340, 17870072066711748607, 16715520087474960373, 2459353627279607168, 7061705824611107232, 8089129053103260512, 7414579821471224013, 9520647030890121554, 17142940634164625405, 9187037984654475102, 4933695867036173873, 3035992416931960321, 15052160563071165696, 5876081268917084809, 1153484746652717320, 6365855841584713735, 2463646859659644933, 1453259901463176960, 9808859429721908488, 2829141021535244552, 576619101540319252, 5804014844877275314, 4774660099383771136, 328785038479458864, 2360590652863023124, 569550314443282, 17563974527758635567, 11698101887533589556, 5764964460729992192, 6953579832080335136, 1318441160687747328, 8090717009753444376, 16751172641200572929, 5558033503209157252, 17100156536247493656, 7899286223048400564, 4845135427956654145, 2368485888099072, 2399033289953272320, 6976678428284034058, 3134241565013966284, 8661609558376259840, 17275805361393991679, 15391050065516657151, 11529206229534274423, 9876416274250600448, 16432792402597134585, 11975705497012863580, 11457135419348969979, 9763749252098620046, 16960553411078512574, 15563877356819111679, 14994736884583272463, 9441297368950544394, 14537646123432199168, 9888547162215157388, 18140215579194907366, 18374682062228545019];

    pub fn get_slider_attacks(&self, square: Coord, blockers: BitBoard, ortho: bool) -> BitBoard {
        if ortho { self.get_rook_attacks(square, blockers) } else { self.get_bishop_attacks(square, blockers) }
    }
    pub fn get_rook_attacks(&self, square: Coord, blockers: BitBoard) -> BitBoard {
        let key = ((blockers & self.rook_mask[square.index()]).0 as u128 * Self::ROOK_MAGICS[square.index()] as u128) as u64 >> Self::ROOK_SHIFTS[square.index()];
        return self.rook_attacks[square.index()][key as usize];
    }
    pub fn get_bishop_attacks(&self, square: Coord, blockers: BitBoard) -> BitBoard {
        let key = ((blockers & self.bishop_mask[square.index()]).0 as u128 * Self::BISHOP_MAGICS[square.index()] as u128) as u64 >> Self::BISHOP_SHIFTS[square.index()];
        return self.bishop_attacks[square.index()][key as usize];
    }

    pub fn create_blocker_bitboards(move_mask: BitBoard) -> Vec<BitBoard> {
        let mut move_sqr_idx = Vec::new();
        for i in 0..64 {
            if ((move_mask >> i) & 1).0 == 1 {
                move_sqr_idx.push(i);
            }
        }
        let n_patterns = 1 << move_sqr_idx.len();
        let mut blocker_bitboards = vec![BitBoard(0); n_patterns];

        for pattern_idx in 0..n_patterns {
            for bit_idx in 0..move_sqr_idx.len() {
                let bit = BitBoard(((pattern_idx >> bit_idx) & 1) as u64);
                blocker_bitboards[pattern_idx] |= bit << move_sqr_idx[bit_idx];
            };
        };
        return blocker_bitboards
    }
    pub fn create_movement_mask(start_coord: Coord, ortho: bool) -> BitBoard {
        let mut mask = BitBoard(0);
        let directions = if ortho { Coord::ROOK_DIRECTIONS } else { Coord::BISHOP_DIRECTIONS };

        for dir in directions {
            for dst in 1..8 {
                let coord = start_coord + dir * dst;
                let next_coord = start_coord + dir * (dst + 1);
                if next_coord.is_valid() {
                    mask.set_square(coord.square());
                } else { break; }
            }
        }
        return mask;
    }
    pub fn legal_move_bitboard_from_blockers(start_sqr: Coord, blockers: BitBoard, ortho: bool) -> BitBoard {
        let mut bitboard = BitBoard(0);
        let directions = if ortho { Coord::ROOK_DIRECTIONS } else { Coord::BISHOP_DIRECTIONS };

        for dir in directions {
            for dst in 1..8 {
                let coord = start_sqr + dir * dst;
                if coord.is_valid() {
                    bitboard.set_square(coord.square());
                    if blockers.contains_square(coord.square()) {
                        break;
                    }
                } else { break; }
            }
        }
        return bitboard;
    }
    
    // fn create_rook_table(square: Coord, magic: u64, left_shift: u32) -> [u64; MagicBitBoards::MIN_ROOK_LOOKUP_SIZE] {
    //     let mut table: [u64; MagicBitBoards::MIN_ROOK_LOOKUP_SIZE];
    //     let move_mask = Self::create_movement_mask(square, true);
    //     let blockers = Self::create_blocker_bitboards(move_mask);

    //     for pattern in blockers {
    //         let index = (pattern * magic) >> left_shift;
    //         let moves = Self::legal_move_bitboard_from_blockers(square, pattern, true);
    //         table[index as usize] = moves;
    //     };
    //     return table;
    // }
    // fn create_bishop_table(square: Coord, magic: u64, left_shift: u32) -> [u64; MagicBitBoards::MIN_BISHOP_LOOKUP_SIZE] {
    //     let mut table: [u64; MagicBitBoards::MIN_BISHOP_LOOKUP_SIZE];
    //     let move_mask = Self::create_movement_mask(square, false);
    //     let blockers = Self::create_blocker_bitboards(move_mask);

    //     for pattern in blockers {
    //         let index = (pattern * magic) >> left_shift;
    //         let moves = Self::legal_move_bitboard_from_blockers(square, pattern, false);
    //         table[index as usize] = moves;
    //     };
    //     return table;
    // }
    fn create_table(square: Coord, ortho: bool, magic: u64, left_shift: u32) -> Vec<BitBoard> {
        let n_bits = 64 - left_shift;
        let lookup_size = 1 << n_bits;
        let mut table = vec![BitBoard(0); lookup_size];
        let move_mask = Self::create_movement_mask(square, ortho);
        let blockers = Self::create_blocker_bitboards(move_mask);

        for pattern in blockers {
            let index = (pattern.0 as u128 * magic as u128) as u64 >> left_shift;
            let moves = Self::legal_move_bitboard_from_blockers(square, pattern, ortho);
            table[index as usize] = moves;
        };
        return table;
    }
}

impl Default for MagicBitBoards {
    fn default() -> Self {
        let mut rook_mask: [BitBoard; 64] = [BitBoard(0); 64];
        let mut bishop_mask: [BitBoard; 64] = [BitBoard(0); 64];

        for sqr_idx in 0..64 {
            let sqr = Coord::from_idx(sqr_idx);
            rook_mask[sqr.index()] = Self::create_movement_mask(sqr, true);
            bishop_mask[sqr.index()] = Self::create_movement_mask(sqr, false);
        };

        const EMPTY_VEC: Vec<BitBoard> = Vec::new();
        let mut rook_attacks: [Vec<BitBoard>; 64] = [EMPTY_VEC; 64];
        let mut bishop_attacks: [Vec<BitBoard>; 64] = [EMPTY_VEC; 64];

        for i in 0..64 {
            let sqr = Coord::from_idx(i);
            rook_attacks[sqr.index()] = Self::create_table(sqr, true, Self::ROOK_MAGICS[sqr.index()], Self::ROOK_SHIFTS[sqr.index()]);
            bishop_attacks[sqr.index()] = Self::create_table(sqr, false, Self::BISHOP_MAGICS[sqr.index()], Self::BISHOP_SHIFTS[sqr.index()]);
        };

        MagicBitBoards {
            rook_mask,
            bishop_mask,
            rook_attacks,
            bishop_attacks,
        }
    }
}

pub fn spawn_magic_bitboards(
    mut commands: Commands,
) {
    commands.insert_resource(MagicBitBoards::default());
}