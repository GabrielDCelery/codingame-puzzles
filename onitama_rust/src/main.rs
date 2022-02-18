use std::cmp;
use std::collections::HashMap;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

static MIN_MAX_TREE_DEPTH: usize = 3;

static WHITE_PLAYER_STARTING_PIECES: i32 = 0b00000_00000_00000_00000_11111;
static BLACK_PLAYER_STARTING_PIECES: i32 = 0b11111_00000_00000_00000_00000;

static WHITE_PLAYER_SHRINE_MASK: i32 = 0b00000_00000_00000_00000_00100;
static BLACK_PLAYER_SHRINE_MASK: i32 = 0b00100_00000_00000_00000_00000;

static WIZARD_INDEX: usize = 0;
static WHITE_PLAYER_ID: usize = 0;
static BLACK_PLAYER_ID: usize = 1;
static STUDENTS_OFFSET: usize = 1;
static CARDS_OFFSET: usize = 5;
static NUM_OF_PIECES_PER_PLAYER: usize = 5;
static NUM_OF_CARDS_PER_PLAYER: usize = 2;
static NUM_OF_MOVES_PER_CARD: usize = 4;
static DEFAULT_CARD_ROTATION: i32 = 1;

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

type PlayerState = [i32; 9];

#[derive(Debug)]
struct PreCalculated {
    valid_moves_from_position_masks: HashMap<i32, i32>,
    board_bit_values_to_cells: HashMap<i32, String>,
    is_move_valid_from_position_with_card: HashMap<i32, HashMap<i32, [bool; 4]>>,
}

#[derive(Debug)]
struct GameState {
    player_states: [PlayerState; 2],
    middle_card: [i32; 2],
}

impl GameState {
    fn clone(&self) -> GameState {
        return GameState {
            player_states: [self.player_states[0].clone(), self.player_states[1].clone()],
            middle_card: self.middle_card.clone(),
        };
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

    fn build(
        &mut self,
        card_moves_map: &CardMovesMap,
        pre_calculated: &PreCalculated,
        target_depth: usize,
    ) {
        if self.depth == target_depth || is_game_finished(&self.game_state) {
            return;
        }
        for piece_index in 0..NUM_OF_PIECES_PER_PLAYER {
            let piece_position = self.game_state.player_states[self.current_player_id][piece_index];
            if piece_position == 0 {
                continue;
            }
            for card_index in 0..NUM_OF_CARDS_PER_PLAYER {
                for card_move_index in 0..NUM_OF_MOVES_PER_CARD {
                    if !is_player_move_valid(
                        &self.game_state,
                        &card_moves_map,
                        &pre_calculated,
                        self.current_player_id,
                        piece_index,
                        card_index,
                        card_move_index,
                    ) {
                        continue;
                    }

                    let mut cloned_game_state = self.game_state.clone();

                    apply_player_moving_piece_to_game_state(
                        &mut cloned_game_state,
                        &card_moves_map,
                        &pre_calculated,
                        self.current_player_id,
                        piece_index,
                        card_index,
                        card_move_index,
                    );

                    let (card_id, card_rotation) =
                        get_player_card(&self.game_state, self.current_player_id, card_index);

                    let piece_position_on_baord_before_move = pre_calculated
                        .board_bit_values_to_cells
                        .get(&piece_position)
                        .unwrap();

                    let piece_position_on_baord_after_move = get_piece_position_on_baord(
                        &cloned_game_state,
                        &pre_calculated,
                        self.current_player_id,
                        piece_index,
                    );

                    let command = card_id.to_string()
                        + &" "
                        + &piece_position_on_baord_before_move
                        + &piece_position_on_baord_after_move;

                    let mut child_node = MinMaxNode::new(
                        self.depth + 1,
                        self.root_player_id,
                        get_opponent_id(self.current_player_id),
                        0,
                        command,
                        cloned_game_state,
                    );

                    child_node.build(&card_moves_map, &pre_calculated, MIN_MAX_TREE_DEPTH);

                    self.child_nodes.push(child_node);
                }

                /*

                let mut cloned_game_state = self.game_state.clone();

                apply_player_passing_to_game_state(
                    &mut cloned_game_state,
                    self.current_player_id,
                    card_index,
                );

                let (card_id, card_rotation) =
                    get_player_card(&self.game_state, self.current_player_id, card_index);

                let command = card_id.to_string() + &" PASS";

                let mut child_node = MinMaxNode::new(
                    self.depth + 1,
                    self.root_player_id,
                    get_opponent_id(self.current_player_id),
                    0,
                    command,
                    cloned_game_state,
                );

                child_node.build(&card_moves_map, &pre_calculated, MIN_MAX_TREE_DEPTH);

                self.child_nodes.push(child_node);
                */
            }
        }
    }

    fn score_min_max_tree(
        &mut self,
        depth: usize,
        alpha: i32,
        beta: i32,
        is_min_maxing_player: bool,
    ) -> i32 {
        if depth == 0 || is_game_finished(&self.game_state) {
            let score = get_game_score_for_maximizing_player(&self.game_state, self.root_player_id);
            self.score = score;
            return score;
        }
        if is_min_maxing_player {
            let mut max_eval = -100000000;
            let mut max_alpha = alpha;
            for child_node in &self.child_nodes {
                let node_eval = score_min_max_tree(&child_node, depth - 1, alpha, beta, false);
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
        for child_node in &self.child_nodes {
            let node_eval = score_min_max_tree(&child_node, depth - 1, alpha, min_beta, true);
            min_eval = cmp::min(min_eval, node_eval);
            min_beta = cmp::min(min_beta, node_eval);
            if min_beta <= alpha {
                break;
            }
        }
        self.score = min_eval;
        return min_eval;
    }

    fn get_next_command(&self) -> String {
        let mut max_score = -10000;
        let mut next_command = "".to_string();
        for child_node in &self.child_nodes {
            if child_node.score >= max_score {
                max_score = child_node.score;
                next_command = child_node.command.clone();
            }
        }
        return next_command;
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

fn set_player_piece_position(
    game_state: &mut GameState,
    player_id: usize,
    piece_index: usize,
    new_position: i32,
) {
    game_state.player_states[player_id][piece_index] = new_position;
}

fn set_player_card(
    game_state: &mut GameState,
    player_id: usize,
    card_index: usize,
    card_id: i32,
    card_rotation: i32,
) {
    let player_card_index = CARDS_OFFSET + card_index * 2;
    game_state.player_states[player_id][player_card_index] = card_id;
    game_state.player_states[player_id][player_card_index + 1] = card_rotation;
}

fn set_middle_card(game_state: &mut GameState, card_id: i32, card_rotation: i32) {
    game_state.middle_card[0] = card_id;
    game_state.middle_card[1] = card_rotation;
}

fn is_game_finished(game_state: &GameState) -> bool {
    let white_wizard_position = game_state.player_states[WHITE_PLAYER_ID][WIZARD_INDEX];
    let black_wizard_position = game_state.player_states[BLACK_PLAYER_ID][WIZARD_INDEX];

    return (white_wizard_position == 0)
        || ((white_wizard_position & BLACK_PLAYER_SHRINE_MASK) > 0)
        || (black_wizard_position == 0)
        || ((black_wizard_position & WHITE_PLAYER_SHRINE_MASK) > 0);
}

fn get_player_piece_position(game_state: &GameState, player_id: usize, piece_index: usize) -> i32 {
    return game_state.player_states[player_id][piece_index];
}

fn get_player_card(game_state: &GameState, player_id: usize, card_index: usize) -> (i32, i32) {
    let player_card_index = CARDS_OFFSET + card_index * 2;
    return (
        game_state.player_states[player_id][player_card_index],
        game_state.player_states[player_id][player_card_index + 1],
    );
}

fn get_middle_card(game_state: &GameState) -> (i32, i32) {
    return (game_state.middle_card[0], game_state.middle_card[1]);
}

fn get_opponent_id(player_id: usize) -> usize {
    if player_id == WHITE_PLAYER_ID {
        return BLACK_PLAYER_ID;
    }
    return WHITE_PLAYER_ID;
}

fn get_piece_position_on_baord(
    game_state: &GameState,
    pre_calculated: &PreCalculated,
    player_id: usize,
    piece_index: usize,
) -> String {
    let piece_position = get_player_piece_position(&game_state, player_id, piece_index);
    let boar_cell = pre_calculated
        .board_bit_values_to_cells
        .get(&piece_position)
        .unwrap();
    return boar_cell.to_string();
}

fn is_player_move_valid(
    game_state: &GameState,
    card_moves_map: &CardMovesMap,
    pre_calculated: &PreCalculated,
    player_id: usize,
    piece_index: usize,
    card_index: usize,
    card_move_index: usize,
) -> bool {
    let (card_id, card_rotation) = get_player_card(&game_state, player_id, card_index);
    let card_moves = card_moves_map.get(&card_id).unwrap();
    let card_move = card_moves[card_move_index];
    if card_move == 0 {
        return false;
    }
    let piece_position_before_move = get_player_piece_position(&game_state, player_id, piece_index);
    let shift_by = card_rotation * card_move;
    let piece_valid_move_mask = pre_calculated
        .valid_moves_from_position_masks
        .get(&piece_position_before_move)
        .unwrap();
    let piece_position_after_move =
        shift_position(piece_position_before_move, shift_by) & piece_valid_move_mask;
    if piece_position_after_move == 0 {
        return false;
    }
    for other_piece_index in 0..NUM_OF_PIECES_PER_PLAYER {
        if piece_index == other_piece_index {
            continue;
        }
        let other_own_piece_position =
            get_player_piece_position(&game_state, player_id, other_piece_index);
        if other_own_piece_position == 0 {
            continue;
        }
        if (piece_position_after_move & other_own_piece_position) > 0 {
            return false;
        }
    }
    return true;
}

fn get_game_score_for_maximizing_player(
    game_state: &GameState,
    maximizing_player_id: usize,
) -> i32 {
    let score = get_game_state_score(&game_state);
    if maximizing_player_id == WHITE_PLAYER_ID {
        return score;
    }
    return -1 * score;
}

fn get_game_state_score(game_state: &GameState) -> i32 {
    let white_wizard_position = game_state.player_states[WHITE_PLAYER_ID][WIZARD_INDEX];
    if white_wizard_position == 0 {
        return -100;
    }
    if (white_wizard_position & BLACK_PLAYER_SHRINE_MASK) > 0 {
        return 100;
    }
    let black_wizard_position = game_state.player_states[BLACK_PLAYER_ID][WIZARD_INDEX];
    if black_wizard_position == 0 {
        return 100;
    }
    if (black_wizard_position & WHITE_PLAYER_SHRINE_MASK) > 0 {
        return -100;
    }
    let mut num_of_white_pieces = 0;
    for piece_index in 0..NUM_OF_PIECES_PER_PLAYER {
        let piece_position = game_state.player_states[WHITE_PLAYER_ID][piece_index];
        if piece_position > 0 {
            num_of_white_pieces += 1;
        }
    }
    let mut num_of_black_pieces = 0;
    for piece_index in 0..NUM_OF_PIECES_PER_PLAYER {
        let piece_position = game_state.player_states[BLACK_PLAYER_ID][piece_index];
        if piece_position > 0 {
            num_of_black_pieces += 1;
        }
    }
    let white_piece_advantage = num_of_white_pieces - num_of_black_pieces;
    return match white_piece_advantage {
        4 => 80,
        3 => 60,
        2 => 40,
        1 => 20,
        0 => 0,
        -1 => -10,
        -2 => -20,
        -3 => -30,
        -4 => -40,
        _ => 0,
    };
}

fn apply_player_moving_piece_to_game_state(
    game_state: &mut GameState,
    card_moves_map: &CardMovesMap,
    pre_calculated: &PreCalculated,
    player_id: usize,
    piece_index: usize,
    card_index: usize,
    card_move_index: usize,
) {
    let piece_position_before_move = get_player_piece_position(&game_state, player_id, piece_index);
    let (card_id, card_rotation) = get_player_card(&game_state, player_id, card_index);
    let card_moves = card_moves_map.get(&card_id).unwrap();
    let card_move = card_moves[card_move_index];
    let shift_by = card_rotation * card_move;
    let piece_valid_move_mask = pre_calculated
        .valid_moves_from_position_masks
        .get(&piece_position_before_move)
        .unwrap();
    let piece_position_after_move =
        shift_position(piece_position_before_move, shift_by) & piece_valid_move_mask;

    set_player_piece_position(
        game_state,
        player_id,
        piece_index,
        piece_position_after_move,
    );

    let (middle_card_id, middle_card_rotation) = get_middle_card(&game_state);

    set_player_card(
        game_state,
        player_id,
        card_index,
        middle_card_id,
        middle_card_rotation,
    );

    set_middle_card(game_state, card_id, -1 * card_rotation);

    let opponent_id = get_opponent_id(player_id);

    for opponent_piece_index in 0..NUM_OF_PIECES_PER_PLAYER {
        let opponent_piece_position =
            get_player_piece_position(&game_state, opponent_id, opponent_piece_index);
        if opponent_piece_position == 0 {
            continue;
        }
        if (piece_position_after_move & opponent_piece_position) != 0 {
            set_player_piece_position(game_state, opponent_id, opponent_piece_index, 0);
            return;
        }
    }
}

fn apply_player_passing_to_game_state(
    game_state: &mut GameState,
    player_id: usize,
    card_index: usize,
) {
    let (card_id, card_rotation) = get_player_card(&game_state, player_id, card_index);

    let (middle_card_id, middle_card_rotation) = get_middle_card(&game_state);

    set_player_card(
        game_state,
        player_id,
        card_index,
        middle_card_id,
        middle_card_rotation,
    );

    set_middle_card(game_state, card_id, -1 * card_rotation);
}

fn score_min_max_tree(
    min_max_node: &MinMaxNode,
    depth: usize,
    alpha: i32,
    beta: i32,
    is_min_maxing_player: bool,
) -> i32 {
    if depth == 0 || is_game_finished(&min_max_node.game_state) {
        let score = get_game_score_for_maximizing_player(
            &min_max_node.game_state,
            min_max_node.root_player_id,
        );
        return score;
    }
    if is_min_maxing_player {
        let mut max_eval = -100000000;
        let mut max_alpha = alpha;
        for child_node in &min_max_node.child_nodes {
            let node_eval = score_min_max_tree(&child_node, depth - 1, alpha, beta, false);
            max_eval = cmp::max(max_eval, node_eval);
            max_alpha = cmp::max(max_alpha, node_eval);
            if beta <= max_alpha {
                break;
            }
        }
        return max_eval;
    }

    let mut min_eval = 100000000;
    let mut min_beta = beta;
    for child_node in &min_max_node.child_nodes {
        let node_eval = score_min_max_tree(&child_node, depth - 1, alpha, min_beta, true);
        min_eval = cmp::min(min_eval, node_eval);
        min_beta = cmp::min(min_beta, node_eval);
        if min_beta <= alpha {
            break;
        }
    }
    return min_eval;
}

fn main() {
    let mut pre_calculated: PreCalculated = PreCalculated {
        valid_moves_from_position_masks: HashMap::new(),
        board_bit_values_to_cells: HashMap::new(),
        is_move_valid_from_position_with_card: HashMap::new(),
    };
    for valid_moves_from_position_mask in VALID_MOVES_FROM_POSITION_MASKS.iter() {
        let [position, mask] = valid_moves_from_position_mask;
        pre_calculated
            .valid_moves_from_position_masks
            .insert(*position, *mask);
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
            player_states: [[0; 9]; 2],
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
                        game_state.player_states[WHITE_PLAYER_ID][WIZARD_INDEX] = piece_position;
                    }
                    'w' => {
                        game_state.player_states[WHITE_PLAYER_ID]
                            [STUDENTS_OFFSET + w_student_index] = piece_position;
                        w_student_index += 1;
                    }
                    'B' => {
                        game_state.player_states[BLACK_PLAYER_ID][WIZARD_INDEX] = piece_position;
                    }
                    'b' => {
                        game_state.player_states[BLACK_PLAYER_ID]
                            [STUDENTS_OFFSET + b_student_index] = piece_position;
                        b_student_index += 1;
                    }
                    _ => {}
                }
            }
        }

        let mut w_card_index: usize = 0;
        let mut b_card_index: usize = 0;

        for _ in 0..5 as usize {
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

            let mut moves: [i32; 4] = [0; 4];

            moves[0] = coordinates_to_bitwise_shift(dx_1, dy_1);
            moves[1] = coordinates_to_bitwise_shift(dx_2, dy_2);
            moves[2] = coordinates_to_bitwise_shift(dx_3, dy_3);
            moves[3] = coordinates_to_bitwise_shift(dx_4, dy_4);
            card_moves_map.insert(card_id, moves);

            match owner {
                0 => {
                    let player_card_id_index = CARDS_OFFSET + w_card_index * 2;
                    let player_card_rotation_index = player_card_id_index + 1;
                    game_state.player_states[WHITE_PLAYER_ID][player_card_id_index] = card_id;
                    game_state.player_states[WHITE_PLAYER_ID][player_card_rotation_index] =
                        DEFAULT_CARD_ROTATION;
                    w_card_index += 1;
                }
                1 => {
                    let player_card_id_index = CARDS_OFFSET + w_card_index * 2;
                    let player_card_rotation_index = player_card_id_index + 1;
                    game_state.player_states[BLACK_PLAYER_ID][player_card_id_index] = card_id;
                    game_state.player_states[BLACK_PLAYER_ID][player_card_rotation_index] =
                        DEFAULT_CARD_ROTATION;
                    b_card_index += 1;
                }
                -1 => {
                    game_state.middle_card[0] = card_id;
                    game_state.middle_card[1] = DEFAULT_CARD_ROTATION;
                }
                _ => {}
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

        let mut root_node = MinMaxNode::new(
            0,
            root_player_id,
            root_player_id,
            0,
            "".to_string(),
            game_state,
        );
        root_node.build(&card_moves_map, &pre_calculated, MIN_MAX_TREE_DEPTH);
        root_node.score_min_max_tree(MIN_MAX_TREE_DEPTH, -100000000, 100000000, true);
        let command = root_node.get_next_command();
        println!("{}", command);
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
