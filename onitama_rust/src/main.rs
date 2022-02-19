use std::cmp;
use std::collections::HashMap;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

static WHITE_PLAYER_SHRINE_MASK: i32 = 0b00000_00000_00000_00000_00100;
static BLACK_PLAYER_SHRINE_MASK: i32 = 0b00100_00000_00000_00000_00000;

static WHITE_EARLY_GAME_TARGET: i32 = 0b00000_00000_01110_01110_01110;
static WHITE_MID_GAME_TARGET: i32 = 0b00000_01110_01110_01110_00000;
static WHITE_END_GAME_TARGET: i32 = 0b01110_01110_01110_00000_00000;

static BLACK_EARLY_GAME_TARGET: i32 = 0b01110_01110_01110_00000_00000;
static BLACK_MID_GAME_TARGET: i32 = 0b00000_01110_01110_01110_00000;
static BLACK_END_GAME_TARGET: i32 = 0b00000_00000_01110_01110_01110;

static GAME_TARGETS: [[i32; 3]; 2] = [
    [
        WHITE_EARLY_GAME_TARGET,
        WHITE_MID_GAME_TARGET,
        WHITE_END_GAME_TARGET,
    ],
    [
        BLACK_EARLY_GAME_TARGET,
        BLACK_MID_GAME_TARGET,
        BLACK_END_GAME_TARGET,
    ],
];

static WIZARD_INDEX: usize = 4;
static WHITE_PLAYER_ID: usize = 0;
static BLACK_PLAYER_ID: usize = 1;
static STUDENTS_OFFSET: usize = 0;
static CARDS_OFFSET: usize = 5;
static NUM_OF_PIECES_PER_PLAYER: usize = 5;
static NUM_OF_CARDS_PER_PLAYER: usize = 2;
static NUM_OF_MOVES_PER_CARD: usize = 4;
static DEFAULT_CARD_ROTATION: i32 = 1;
static PLAYER_PIECES_POSITION_BITMAP_OFFSET: usize = 9;

static VALID_MOVES_FROM_POSITION_MASKS: [[i32; 2]; 25] = [
    [1, 0b00000_00000_00111_00111_00111],
    [2, 0b00000_00000_01111_01111_01111],
    [4, 0b00000_00000_11111_11111_11111],
    [8, 0b00000_00000_11110_11110_11110],
    [16, 0b00000_00000_11100_11100_11100],
    [32, 0b00000_00111_00111_00111_00111],
    [64, 0b00000_01111_01111_01111_01111],
    [128, 0b00000_11111_11111_11111_11111],
    [256, 0b00000_11110_11110_11110_11110],
    [512, 0b00000_11100_11100_11100_11100],
    [1024, 0b00111_00111_00111_00111_00111],
    [2048, 0b01111_01111_01111_01111_01111],
    [4096, 0b11111_11111_11111_11111_11111],
    [8192, 0b11110_11110_11110_11110_11110],
    [16384, 0b11100_11100_11100_11100_11100],
    [32768, 0b00111_00111_00111_00111_00000],
    [65536, 0b01111_01111_01111_01111_00000],
    [131072, 0b11111_11111_11111_11111_00000],
    [262144, 0b11110_11110_11110_11110_00000],
    [524288, 0b11100_11100_11100_11100_00000],
    [1048576, 0b00111_00111_00111_00000_00000],
    [2097152, 0b01111_01111_01111_00000_00000],
    [4194304, 0b11111_11111_11111_00000_00000],
    [8388608, 0b11110_11110_11110_00000_00000],
    [16777216, 0b11100_11100_11100_00000_00000],
];

static BOARD_BIT_VALUES_TO_CELLS: [(i32, &str); 25] = [
    (1, "A1"),
    (2, "B1"),
    (4, "C1"),
    (8, "D1"),
    (16, "E1"),
    (32, "A2"),
    (64, "B2"),
    (128, "C2"),
    (256, "D2"),
    (512, "E2"),
    (1024, "A3"),
    (2048, "B3"),
    (4096, "C3"),
    (8192, "D3"),
    (16384, "E3"),
    (32768, "A4"),
    (65536, "B4"),
    (131072, "C4"),
    (262144, "D4"),
    (524288, "E4"),
    (1048576, "A5"),
    (2097152, "B5"),
    (4194304, "C5"),
    (8388608, "D5"),
    (16777216, "E5"),
];

type CardMoves = [i32; 4];

type CardMovesMap = HashMap<i32, CardMoves>;

type PlayerState = [i32; 10];

#[derive(Debug)]
struct PreCalculated {
    board_bit_values_to_cells: HashMap<i32, String>,
    positions_after_card_move_map: HashMap<i32, HashMap<i32, Vec<i32>>>,
    positions_after_rotated_card_move_map: HashMap<i32, HashMap<i32, Vec<i32>>>,
}

#[derive(Debug)]
struct GameState {
    players: [PlayerState; 2],
    middle_card: [i32; 2],
}

impl GameState {
    fn clone(&self) -> GameState {
        return GameState {
            players: [self.players[0].clone(), self.players[1].clone()],
            middle_card: self.middle_card.clone(),
        };
    }

    fn get_player_card(&self, player_id: usize, card_index: usize) -> (i32, i32) {
        let player_card_index = CARDS_OFFSET + card_index * 2;
        return (
            self.players[player_id][player_card_index],
            self.players[player_id][player_card_index + 1],
        );
    }

    fn get_middle_card(&self) -> (i32, i32) {
        return (self.middle_card[0], self.middle_card[1]);
    }

    fn set_player_piece_position(
        &mut self,
        player_id: usize,
        piece_index: usize,
        piece_position: i32,
    ) {
        self.players[player_id][piece_index] = piece_position;
    }

    fn set_player_card(
        &mut self,
        player_id: usize,
        card_index: usize,
        card_id: i32,
        card_rotation: i32,
    ) {
        let player_card_index = CARDS_OFFSET + card_index * 2;
        self.players[player_id][player_card_index] = card_id;
        self.players[player_id][player_card_index + 1] = card_rotation;
    }

    fn set_middle_card(&mut self, card_id: i32, card_rotation: i32) {
        self.middle_card[0] = card_id;
        self.middle_card[1] = card_rotation;
    }

    fn re_clculate_player_pieces_bitmap(&mut self) {
        for player_id in 0..2 {
            let mut player_pieces_bitmap = 0;
            for player_piece_index in 0..NUM_OF_PIECES_PER_PLAYER {
                player_pieces_bitmap =
                    player_pieces_bitmap | self.players[player_id][player_piece_index]
            }
            self.players[player_id][PLAYER_PIECES_POSITION_BITMAP_OFFSET] = player_pieces_bitmap;
        }
    }

    fn is_game_finished(&self) -> bool {
        let white_wizard_position = self.players[WHITE_PLAYER_ID][WIZARD_INDEX];
        let black_wizard_position = self.players[BLACK_PLAYER_ID][WIZARD_INDEX];
        return (white_wizard_position == 0)
            || ((white_wizard_position & BLACK_PLAYER_SHRINE_MASK) > 0)
            || (black_wizard_position == 0)
            || ((black_wizard_position & WHITE_PLAYER_SHRINE_MASK) > 0);
    }

    fn get_num_of_player_pieces(&self, player_id: usize) -> i32 {
        let mut num_of_player_pieces = 0;
        for piece_index in 0..NUM_OF_PIECES_PER_PLAYER {
            let piece_position = self.players[player_id][piece_index];
            if piece_position > 0 {
                num_of_player_pieces += 1;
            }
        }
        return num_of_player_pieces;
    }

    fn get_game_score_for_maximizing_player(&self, maximizing_player_id: usize) -> i32 {
        let score = self.get_game_state_score();
        if maximizing_player_id == WHITE_PLAYER_ID {
            return score;
        }
        return -1 * score;
    }

    fn get_game_state_score(&self) -> i32 {
        let white_wizard_position = self.players[WHITE_PLAYER_ID][WIZARD_INDEX];
        if white_wizard_position == 0 {
            return -100000;
        }
        if (white_wizard_position & BLACK_PLAYER_SHRINE_MASK) > 0 {
            return 100000;
        }
        let black_wizard_position = self.players[BLACK_PLAYER_ID][WIZARD_INDEX];
        if black_wizard_position == 0 {
            return 100000;
        }
        if (black_wizard_position & WHITE_PLAYER_SHRINE_MASK) > 0 {
            return -100000;
        }
        let num_of_white_pieces = self.get_num_of_player_pieces(WHITE_PLAYER_ID);
        let num_of_black_pieces = self.get_num_of_player_pieces(BLACK_PLAYER_ID);

        let points_from_num_of_pieces = num_of_white_pieces * 100 - num_of_black_pieces * 100;

        let num_of_total_pieces = num_of_white_pieces + num_of_black_pieces;

        let game_target_index = match num_of_total_pieces {
            10 => 0,
            9 => 0,
            8 => 1,
            7 => 1,
            6 => 1,
            5 => 1,
            4 => 2,
            3 => 2,
            2 => 2,
            1 => 2,
            0 => 2,
            _ => 0,
        };
        let white_player_target_mask = GAME_TARGETS[WHITE_PLAYER_ID][game_target_index];
        let black_player_target_mask = GAME_TARGETS[BLACK_PLAYER_ID][game_target_index];

        let white_player_pieces_bitmap =
            self.players[WHITE_PLAYER_ID][PLAYER_PIECES_POSITION_BITMAP_OFFSET];
        let black_player_pieces_bitmap =
            self.players[BLACK_PLAYER_ID][PLAYER_PIECES_POSITION_BITMAP_OFFSET];

        let num_of_white_pieces_matching_mask =
            (white_player_pieces_bitmap & white_player_target_mask).count_ones() as i32;
        let num_of_black_pieces_matching_mask =
            (black_player_pieces_bitmap & black_player_target_mask).count_ones() as i32;

        let points_from_pieces_in_preferred_position =
            num_of_white_pieces_matching_mask * 10 - num_of_black_pieces_matching_mask * 10;

        return points_from_num_of_pieces + points_from_pieces_in_preferred_position;
    }
}

#[derive(Debug)]
struct MinMaxNode {
    depth: usize,
    root_player_id: usize,
    current_player_id: usize,
    score: i32,
    command: String,
    game_state: GameState,
    child_nodes: Vec<MinMaxNode>,
}

impl MinMaxNode {
    fn new(
        depth: usize,
        root_player_id: usize,
        current_player_id: usize,
        score: i32,
        command: String,
        game_state: GameState,
    ) -> MinMaxNode {
        return MinMaxNode {
            root_player_id,
            current_player_id,
            depth,
            game_state,
            score,
            command,
            child_nodes: vec![],
        };
    }

    fn build(&mut self, pre_calculated: &PreCalculated, target_depth: usize) {
        if self.depth == target_depth || self.game_state.is_game_finished() {
            return;
        }
        let (middle_card_id, middle_card_rotation) = self.game_state.get_middle_card();
        for piece_index in 0..NUM_OF_PIECES_PER_PLAYER {
            let piece_position_before_move =
                self.game_state.players[self.current_player_id][piece_index];
            if piece_position_before_move == 0 {
                continue;
            }
            for card_index in 0..NUM_OF_CARDS_PER_PLAYER {
                let (card_id, card_rotation) = self
                    .game_state
                    .get_player_card(self.current_player_id, card_index);

                let piece_positions_after_move;

                if card_rotation == 1 {
                    piece_positions_after_move = pre_calculated
                        .positions_after_card_move_map
                        .get(&piece_position_before_move)
                        .unwrap()
                        .get(&card_id)
                        .unwrap();
                } else {
                    piece_positions_after_move = pre_calculated
                        .positions_after_rotated_card_move_map
                        .get(&piece_position_before_move)
                        .unwrap()
                        .get(&card_id)
                        .unwrap();
                }

                for piece_position_after_move_ in piece_positions_after_move.iter() {
                    let piece_position_after_move = *piece_position_after_move_;
                    let own_pieces_bitmap = self.game_state.players[self.current_player_id]
                        [PLAYER_PIECES_POSITION_BITMAP_OFFSET];
                    let is_moving_on_own_piece =
                        (piece_position_after_move & own_pieces_bitmap) > 0;
                    if is_moving_on_own_piece {
                        continue;
                    }

                    let mut cloned_game_state = self.game_state.clone();

                    cloned_game_state.set_player_piece_position(
                        self.current_player_id,
                        piece_index,
                        piece_position_after_move,
                    );

                    cloned_game_state.set_player_card(
                        self.current_player_id,
                        card_index,
                        middle_card_id,
                        middle_card_rotation,
                    );
                    cloned_game_state.set_middle_card(card_id, -1 * card_rotation);
                    let opponent_id = get_opponent_id(self.current_player_id);
                    let opponent_pieces_bitmap =
                        self.game_state.players[opponent_id][PLAYER_PIECES_POSITION_BITMAP_OFFSET];

                    let is_capturing_opponent_piece =
                        (piece_position_after_move & opponent_pieces_bitmap) > 0;
                    if is_capturing_opponent_piece {
                        for opponent_piece_index in 0..NUM_OF_PIECES_PER_PLAYER {
                            let opponent_piece_position =
                                self.game_state.players[opponent_id][opponent_piece_index];
                            if opponent_piece_position == 0 {
                                continue;
                            }
                            if (piece_position_after_move & opponent_piece_position) != 0 {
                                cloned_game_state.set_player_piece_position(
                                    opponent_id,
                                    opponent_piece_index,
                                    0,
                                );
                                break;
                            }
                        }
                    }

                    cloned_game_state.re_clculate_player_pieces_bitmap();

                    let piece_position_on_board_before_move = pre_calculated
                        .board_bit_values_to_cells
                        .get(&piece_position_before_move)
                        .unwrap();

                    let piece_position_on_board_after_move = pre_calculated
                        .board_bit_values_to_cells
                        .get(&piece_position_after_move)
                        .unwrap();

                    let command = card_id.to_string()
                        + &" "
                        + &piece_position_on_board_before_move
                        + &piece_position_on_board_after_move;

                    let mut child_node = MinMaxNode::new(
                        self.depth + 1,
                        self.root_player_id,
                        get_opponent_id(self.current_player_id),
                        0,
                        command,
                        cloned_game_state,
                    );

                    child_node.build(&pre_calculated, target_depth);

                    self.child_nodes.push(child_node);
                }
            }
        }
    }
    fn score_min_max_tree(
        &mut self,
        depth: usize,
        alpha: i32,
        beta: i32,
        is_maximizing_player: bool,
    ) -> i32 {
        if depth == 0 || self.game_state.is_game_finished() {
            let score = self
                .game_state
                .get_game_score_for_maximizing_player(self.root_player_id);
            self.score = score;
            return score;
        }
        if is_maximizing_player {
            let mut max_eval = -100000000;
            let mut max_alpha = alpha;
            for child_node in self.child_nodes.iter_mut() {
                let node_eval = child_node.score_min_max_tree(depth - 1, max_alpha, beta, false);
                max_eval = cmp::max(max_eval, node_eval);
                max_alpha = cmp::max(max_alpha, node_eval);
                if beta <= max_alpha {
                    break;
                }
            }
            self.score = max_eval;
            return max_eval;
        }
        let mut min_eval = 100000000;
        let mut min_beta = beta;
        for child_node in self.child_nodes.iter_mut() {
            let node_eval = child_node.score_min_max_tree(depth - 1, alpha, min_beta, true);
            min_eval = cmp::min(min_eval, node_eval);
            min_beta = cmp::min(min_beta, node_eval);
            if min_beta <= alpha {
                break;
            }
        }
        self.score = min_eval;
        return min_eval;
    }
    fn get_next_command(&self) -> (String, i32) {
        let mut max_score = -100000000;
        let mut next_command = "".to_string();
        for child_node in &self.child_nodes {
            if child_node.score > max_score {
                max_score = child_node.score;
                next_command = child_node.command.clone();
            }
        }
        return (next_command, max_score);
    }
}

fn coordinates_to_bitwise_shift(x: i32, y: i32) -> i32 {
    return x + y * 5;
}

fn shift_position(position: i32, shift_by: i32) -> i32 {
    if shift_by > 0 {
        return position << shift_by;
    }
    if shift_by < 0 {
        return position >> (-1 * shift_by);
    }
    return position;
}

fn get_opponent_id(player_id: usize) -> usize {
    if player_id == WHITE_PLAYER_ID {
        return BLACK_PLAYER_ID;
    }
    return WHITE_PLAYER_ID;
}

fn get_num_of_estimated_moves_for_player(
    game_state: &GameState,
    pre_calculated: &PreCalculated,
    player_id: usize,
) -> usize {
    let mut num_of_possible_moves = 0;
    for piece_index in 0..NUM_OF_PIECES_PER_PLAYER {
        let piece_position_before_move = game_state.players[player_id][piece_index];
        if piece_position_before_move == 0 {
            continue;
        }
        for card_index in 0..NUM_OF_CARDS_PER_PLAYER {
            let (card_id, card_rotation) = game_state.get_player_card(player_id, card_index);

            let piece_positions_after_move;

            if card_rotation == 1 {
                piece_positions_after_move = pre_calculated
                    .positions_after_card_move_map
                    .get(&piece_position_before_move)
                    .unwrap()
                    .get(&card_id)
                    .unwrap()
            } else {
                piece_positions_after_move = pre_calculated
                    .positions_after_rotated_card_move_map
                    .get(&piece_position_before_move)
                    .unwrap()
                    .get(&card_id)
                    .unwrap()
            }

            for piece_position_after_move_ in piece_positions_after_move.iter() {
                let piece_position_after_move = *piece_position_after_move_;
                let own_pieces_bitmap =
                    game_state.players[player_id][PLAYER_PIECES_POSITION_BITMAP_OFFSET];
                let is_moving_on_own_piece = (piece_position_after_move & own_pieces_bitmap) > 0;
                if is_moving_on_own_piece {
                    continue;
                }
                num_of_possible_moves += 1;
            }
        }
    }
    return num_of_possible_moves;
}

fn main() {
    let mut valid_moves_from_position_masks: HashMap<i32, i32> = HashMap::new();
    let mut pre_calculated: PreCalculated = PreCalculated {
        board_bit_values_to_cells: HashMap::new(),
        positions_after_card_move_map: HashMap::new(),
        positions_after_rotated_card_move_map: HashMap::new(),
    };
    for valid_moves_from_position_mask in VALID_MOVES_FROM_POSITION_MASKS.iter() {
        let [position, mask] = valid_moves_from_position_mask;
        valid_moves_from_position_masks.insert(*position, *mask);
    }
    for board_bit_value_to_cell in BOARD_BIT_VALUES_TO_CELLS.iter() {
        let (position, cell) = board_bit_value_to_cell;
        pre_calculated
            .board_bit_values_to_cells
            .insert(*position, cell.to_string());
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let root_player_id = parse_input!(input_line, usize);

    // game loop
    loop {
        let mut game_state: GameState = GameState {
            players: [[0; 10]; 2],
            middle_card: [0; 2],
        };
        let mut card_moves_map: CardMovesMap = HashMap::new();

        let mut b_student_index = 0;
        let mut w_student_index = 0;

        for i in 0..5 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let board = input_line.trim_matches('\n').to_string();
            let y: i32 = 4 - i as i32;
            for k in 0..5 as usize {
                let cell = board.chars().nth(k).unwrap();
                let x = k as i32;
                let piece_position = shift_position(1, coordinates_to_bitwise_shift(x, y));
                match cell {
                    'W' => {
                        game_state.set_player_piece_position(
                            WHITE_PLAYER_ID,
                            WIZARD_INDEX,
                            piece_position,
                        );
                    }
                    'w' => {
                        game_state.set_player_piece_position(
                            WHITE_PLAYER_ID,
                            STUDENTS_OFFSET + w_student_index,
                            piece_position,
                        );
                        w_student_index += 1;
                    }
                    'B' => {
                        game_state.set_player_piece_position(
                            BLACK_PLAYER_ID,
                            WIZARD_INDEX,
                            piece_position,
                        );
                    }
                    'b' => {
                        game_state.set_player_piece_position(
                            BLACK_PLAYER_ID,
                            STUDENTS_OFFSET + b_student_index,
                            piece_position,
                        );
                        b_student_index += 1;
                    }
                    _ => {}
                }
            }
        }

        let mut w_card_index: usize = 0;
        let mut b_card_index: usize = 0;

        let mut card_ids: Vec<i32> = Vec::new();

        for i in 0..5 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let owner = parse_input!(inputs[0], i32);
            let card_id = parse_input!(inputs[1], i32);
            let dx_1 = parse_input!(inputs[2], i32);
            let dy_1 = parse_input!(inputs[3], i32);
            let dx_2 = parse_input!(inputs[4], i32);
            let dy_2 = parse_input!(inputs[5], i32);
            let dx_3 = parse_input!(inputs[6], i32);
            let dy_3 = parse_input!(inputs[7], i32);
            let dx_4 = parse_input!(inputs[8], i32);
            let dy_4 = parse_input!(inputs[9], i32);

            card_ids.insert(i, card_id);

            let mut moves: [i32; 4] = [0; 4];

            moves[0] = coordinates_to_bitwise_shift(dx_1, dy_1);
            moves[1] = coordinates_to_bitwise_shift(dx_2, dy_2);
            moves[2] = coordinates_to_bitwise_shift(dx_3, dy_3);
            moves[3] = coordinates_to_bitwise_shift(dx_4, dy_4);
            card_moves_map.insert(card_id, moves);

            match owner {
                0 => {
                    game_state.set_player_card(
                        WHITE_PLAYER_ID,
                        w_card_index,
                        card_id,
                        DEFAULT_CARD_ROTATION,
                    );
                    w_card_index += 1;
                }
                1 => {
                    game_state.set_player_card(
                        BLACK_PLAYER_ID,
                        b_card_index,
                        card_id,
                        DEFAULT_CARD_ROTATION,
                    );
                    b_card_index += 1;
                }
                -1 => {
                    game_state.set_middle_card(card_id, DEFAULT_CARD_ROTATION);
                }
                _ => {}
            }
        }
        for y in 0..5 {
            for x in 0..5 {
                let piece_position_before_move =
                    shift_position(1, coordinates_to_bitwise_shift(x, y));
                let piece_valid_move_mask = valid_moves_from_position_masks
                    .get(&piece_position_before_move)
                    .unwrap();
                let mut card_moves_map_for_position = HashMap::new();
                for card_id in card_ids.clone() {
                    let mut card_moves: Vec<i32> = Vec::new();
                    for card_move_index in 0..NUM_OF_MOVES_PER_CARD {
                        let shift_by = 1 * (card_moves_map.get(&card_id).unwrap()[card_move_index]);
                        if shift_by == 0 {
                            continue;
                        }
                        let piece_position_after_move =
                            shift_position(piece_position_before_move, shift_by)
                                & piece_valid_move_mask;
                        if piece_position_after_move == 0 {
                            continue;
                        }
                        card_moves.push(piece_position_after_move);
                    }
                    card_moves_map_for_position.insert(card_id, card_moves);
                }
                pre_calculated
                    .positions_after_card_move_map
                    .insert(piece_position_before_move, card_moves_map_for_position);

                let mut card_moves_map_for_position_rotated = HashMap::new();

                for card_id in card_ids.clone() {
                    let mut card_moves: Vec<i32> = Vec::new();
                    for card_move_index in 0..NUM_OF_MOVES_PER_CARD {
                        let shift_by =
                            -1 * (card_moves_map.get(&card_id).unwrap()[card_move_index]);
                        if shift_by == 0 {
                            continue;
                        }
                        let piece_position_after_move =
                            shift_position(piece_position_before_move, shift_by)
                                & piece_valid_move_mask;
                        if piece_position_after_move == 0 {
                            continue;
                        }
                        card_moves.push(piece_position_after_move);
                    }
                    card_moves_map_for_position_rotated.insert(card_id, card_moves);
                }
                pre_calculated.positions_after_rotated_card_move_map.insert(
                    piece_position_before_move,
                    card_moves_map_for_position_rotated,
                );
            }
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let action_count = parse_input!(input_line, i32);
        for _ in 0..action_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let _card_id = parse_input!(inputs[0], i32);
            let _move = inputs[1].trim().to_string();
        }

        game_state.re_clculate_player_pieces_bitmap();
        let num_of_possible_moves_for_white =
            get_num_of_estimated_moves_for_player(&game_state, &pre_calculated, WHITE_PLAYER_ID);
        let num_of_possible_moves_for_black =
            get_num_of_estimated_moves_for_player(&game_state, &pre_calculated, BLACK_PLAYER_ID);
        let num_of_possible_moves_in_total =
            num_of_possible_moves_for_white + num_of_possible_moves_for_black;

        let mut root_node = MinMaxNode::new(
            0,
            root_player_id,
            root_player_id,
            0,
            "".to_string(),
            game_state,
        );

        let mut target_depth: usize = 0;
        if num_of_possible_moves_in_total >= 39 {
            target_depth = 3;
        }
        if num_of_possible_moves_in_total >= 19 && num_of_possible_moves_in_total < 39 {
            target_depth = 4;
        }
        if num_of_possible_moves_in_total >= 11 && num_of_possible_moves_in_total < 19 {
            target_depth = 5;
        }
        if num_of_possible_moves_in_total < 11 {
            target_depth = 6;
        }

        root_node.build(&pre_calculated, target_depth);
        root_node.score_min_max_tree(target_depth, -100000000, 100000000, true);
        let (command, score) = root_node.get_next_command();

        println!(
            "{} s: {} d: {} wm: {} bm: {}",
            command,
            score,
            target_depth,
            num_of_possible_moves_for_white,
            num_of_possible_moves_for_black
        );
    }
}
