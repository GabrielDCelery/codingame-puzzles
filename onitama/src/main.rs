use std::cmp;
use std::collections::HashMap;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

static MIN_MAX_TREE_DEPTH: usize = 4;

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

type CardMoves = [i32; 4];

type CardMovesMap = HashMap<i32, CardMoves>;

type PlayerState = [i32; 9];

#[derive(Debug)]
struct BoardConfigs {
    piece_move_masks: HashMap<i32, i32>,
    board_column_masks: HashMap<String, i32>,
    board_row_masks: HashMap<String, i32>,
    board_column_letters: [String; 5],
    board_row_letters: [String; 5],
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
        board_configs: &BoardConfigs,
        target_depth: usize,
    ) {
        if self.depth == target_depth || is_game_finished(&self.game_state) {
            return;
        }

        for piece_index in 0..NUM_OF_PIECES_PER_PLAYER {
            if !does_player_piece_exist(&self.game_state, self.current_player_id, piece_index) {
                return;
            }
            for card_index in 0..NUM_OF_CARDS_PER_PLAYER {
                for card_move_index in 0..NUM_OF_MOVES_PER_CARD {
                    if !is_player_move_valid(
                        &self.game_state,
                        &card_moves_map,
                        &board_configs,
                        self.current_player_id,
                        piece_index,
                        card_index,
                        card_move_index,
                    ) {
                        continue;
                    }
                    let piece_position_on_baord_before_move = get_piece_position_on_baord(
                        &self.game_state,
                        &board_configs,
                        self.current_player_id,
                        piece_index,
                    );

                    let mut cloned_game_state = self.game_state.clone();

                    apply_player_moving_piece_to_game_state(
                        &mut cloned_game_state,
                        &card_moves_map,
                        &board_configs,
                        self.current_player_id,
                        piece_index,
                        card_index,
                        card_move_index,
                    );

                    let (card_id, card_rotation) =
                        get_player_card(&self.game_state, self.current_player_id, card_index);

                    let piece_position_on_baord_after_move = get_piece_position_on_baord(
                        &cloned_game_state,
                        &board_configs,
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

                    child_node.build(&card_moves_map, &board_configs, MIN_MAX_TREE_DEPTH);

                    self.child_nodes.push(child_node);
                }

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

                child_node.build(&card_moves_map, &board_configs, MIN_MAX_TREE_DEPTH);

                self.child_nodes.push(child_node);
            }
        }
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

fn init_player_wizard(game_state: &mut GameState, player_id: usize, x: i32, y: i32) {
    set_player_piece_position(
        game_state,
        player_id,
        WIZARD_INDEX,
        shift_position(1, coordinates_to_bitwise_shift(x, y)),
    );
}

fn init_player_student(
    game_state: &mut GameState,
    player_id: usize,
    student_index: usize,
    x: i32,
    y: i32,
) {
    set_player_piece_position(
        game_state,
        player_id,
        STUDENTS_OFFSET + student_index,
        shift_position(1, coordinates_to_bitwise_shift(x, y)),
    );
}

fn init_player_card(game_state: &mut GameState, player_id: usize, card_index: usize, card_id: i32) {
    let player_card_index = CARDS_OFFSET + card_index * 2;
    game_state.player_states[player_id][player_card_index] = card_id;
    game_state.player_states[player_id][player_card_index + 1] = 1;
}

fn init_middle_card(game_state: &mut GameState, card_id: i32) {
    game_state.middle_card[0] = card_id;
    game_state.middle_card[1] = 1;
}

fn is_game_finished(game_state: &GameState) -> bool {
    let white_wizard_position = game_state.player_states[WHITE_PLAYER_ID][WIZARD_INDEX];
    let black_wizard_position = game_state.player_states[BLACK_PLAYER_ID][WIZARD_INDEX];

    let is_finished = white_wizard_position == 0
        || (white_wizard_position & BLACK_PLAYER_SHRINE_MASK) > 0
        || black_wizard_position == 0
        || (black_wizard_position & WHITE_PLAYER_SHRINE_MASK) > 0;
    return is_finished;
}

fn does_player_piece_exist(game_state: &GameState, player_id: usize, piece_index: usize) -> bool {
    return get_player_piece_position(&game_state, player_id, piece_index) > 0;
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

fn get_matching_column_for_piece_position(
    board_configs: &BoardConfigs,
    piece_position: i32,
) -> String {
    for col in board_configs.board_column_letters.iter() {
        let mask = board_configs.board_column_masks.get(col).unwrap();
        let matching = (mask & piece_position) > 0;
        if matching {
            return col.clone();
        }
    }
    return "".to_string();
}

fn get_matching_row_for_piece_position(
    board_configs: &BoardConfigs,
    piece_position: i32,
) -> String {
    for row in board_configs.board_row_letters.iter() {
        let mask = board_configs.board_row_masks.get(row).unwrap();
        let matching = (mask & piece_position) > 0;
        if matching {
            return row.clone();
        }
    }
    return "".to_string();
}

fn get_piece_position_on_baord(
    game_state: &GameState,
    board_configs: &BoardConfigs,
    player_id: usize,
    piece_index: usize,
) -> String {
    let piece_position = get_player_piece_position(&game_state, player_id, piece_index);
    let matching_column = get_matching_column_for_piece_position(&board_configs, piece_position);
    let matching_row = get_matching_row_for_piece_position(&board_configs, piece_position);
    let concatenated = matching_column.clone() + &matching_row;
    return concatenated;
}

fn is_player_move_valid(
    game_state: &GameState,
    card_moves_map: &CardMovesMap,
    board_configs: &BoardConfigs,
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
    let piece_valid_move_mask = board_configs
        .piece_move_masks
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
        return -10;
    }
    if (white_wizard_position & BLACK_PLAYER_SHRINE_MASK) > 0 {
        return 10;
    }
    let black_wizard_position = game_state.player_states[BLACK_PLAYER_ID][WIZARD_INDEX];
    if black_wizard_position == 0 {
        return 10;
    }
    if (black_wizard_position & WHITE_PLAYER_SHRINE_MASK) > 0 {
        return -10;
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
        4 => 8,
        3 => 6,
        2 => 4,
        1 => 2,
        0 => 6,
        -1 => -1,
        -2 => -2,
        -3 => -3,
        -4 => -4,
        _ => 0,
    };
    /*
        const whiteWizardPosition = getPlayerPiecePosition(gameState, WHITE_PLAYER_ID, WIZARD_INDEX);
        if (whiteWizardPosition === 0) {
            return -1;
        }
        if ((whiteWizardPosition & BLACK_PLAYER_SHRINE_MASK) > 0) {
            return 1;
        }
        const blackWizardPosition = getPlayerPiecePosition(gameState, BLACK_PLAYER_ID, WIZARD_INDEX);
        if (blackWizardPosition === 0) {
            return 1;
        }
        if ((blackWizardPosition & WHITE_PLAYER_SHRINE_MASK) > 0) {
            return -1;
        }
        const numOfWhitePieces = [...Array(NUM_OF_PIECES_PER_PLAYER)].filter((_, pieceIndex) => {
            const position = getPlayerPiecePosition(gameState, WHITE_PLAYER_ID, pieceIndex);
            return position > 0;
        }).length;
        const numOfBlackPieces = [...Array(NUM_OF_PIECES_PER_PLAYER)].filter((_, pieceIndex) => {
            const position = getPlayerPiecePosition(gameState, BLACK_PLAYER_ID, pieceIndex);
            return position > 0;
        }).length;
        const whitePieceAdvantage = numOfWhitePieces - numOfBlackPieces;
        switch (whitePieceAdvantage) {
            case 4: {
                return 0.8;
            }
            case 3: {
                return 0.6;
            }
            case 2: {
                return 0.4;
            }
            case 1: {
                return 0.2;
            }
            case 0: {
                return 0;
            }
            case -1: {
                return -0.2;
            }
            case -2: {
                return -0.4;
            }
            case -3: {
                return -0.6;
            }
            case -4: {
                return -0.8;
            }
            default: {
            }
        }
        return 0;

    if (score === 0) {
        return 0;
    }
    return maximizingPlayerID === WHITE_PLAYER_ID ? score : -1 * score;
    */
}

fn apply_player_moving_piece_to_game_state(
    game_state: &mut GameState,
    card_moves_map: &CardMovesMap,
    board_configs: &BoardConfigs,
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
    let piece_valid_move_mask = board_configs
        .piece_move_masks
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
        return get_game_score_for_maximizing_player(
            &min_max_node.game_state,
            min_max_node.root_player_id,
        );
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
    let mut board_configs: BoardConfigs = BoardConfigs {
        piece_move_masks: HashMap::new(),
        board_column_masks: HashMap::new(),
        board_row_masks: HashMap::new(),
        board_column_letters: [
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ],
        board_row_letters: [
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
        ],
    };
    let board_masks_configs: [[i32; 2]; 25] = [
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
    for board_masks_config in board_masks_configs.iter() {
        board_configs
            .piece_move_masks
            .insert(board_masks_config[0], board_masks_config[1]);
    }

    board_configs
        .board_column_masks
        .insert("A".to_string(), 0b00001_00001_00001_00001_00001);
    board_configs
        .board_column_masks
        .insert("B".to_string(), 0b00010_00010_00010_00010_00010);
    board_configs
        .board_column_masks
        .insert("C".to_string(), 0b00100_00100_00100_00100_00100);
    board_configs
        .board_column_masks
        .insert("D".to_string(), 0b01000_01000_01000_01000_01000);
    board_configs
        .board_column_masks
        .insert("E".to_string(), 0b10000_10000_10000_10000_10000);

    board_configs
        .board_row_masks
        .insert("1".to_string(), 0b00000_00000_00000_00000_11111);
    board_configs
        .board_row_masks
        .insert("2".to_string(), 0b00000_00000_00000_11111_00000);
    board_configs
        .board_row_masks
        .insert("3".to_string(), 0b00000_00000_11111_00000_00000);
    board_configs
        .board_row_masks
        .insert("4".to_string(), 0b00000_11111_00000_00000_00000);
    board_configs
        .board_row_masks
        .insert("5".to_string(), 0b11111_00000_00000_00000_00000);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let player_id = parse_input!(input_line, usize);

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
                match cell {
                    'W' => {
                        init_player_wizard(&mut game_state, WHITE_PLAYER_ID, x, y);
                    }
                    'w' => {
                        init_player_student(
                            &mut game_state,
                            WHITE_PLAYER_ID,
                            w_student_index,
                            x,
                            y,
                        );
                        w_student_index += 1;
                    }
                    'B' => {
                        init_player_wizard(&mut game_state, BLACK_PLAYER_ID, x, y);
                    }
                    'b' => {
                        init_player_student(
                            &mut game_state,
                            BLACK_PLAYER_ID,
                            b_student_index,
                            x,
                            y,
                        );
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
                    init_player_card(&mut game_state, WHITE_PLAYER_ID, w_card_index, card_id);
                    w_card_index += 1;
                }
                1 => {
                    init_player_card(&mut game_state, BLACK_PLAYER_ID, b_card_index, card_id);
                    b_card_index += 1;
                }
                -1 => {
                    init_middle_card(&mut game_state, card_id);
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

        let mut root_node = MinMaxNode::new(0, player_id, player_id, 0, "".to_string(), game_state);

        root_node.build(&card_moves_map, &board_configs, MIN_MAX_TREE_DEPTH);
        let score = score_min_max_tree(&root_node, MIN_MAX_TREE_DEPTH, -100000000, 100000000, true);

        println!("{:?}", score);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("1 A1B2 moving the student");
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
