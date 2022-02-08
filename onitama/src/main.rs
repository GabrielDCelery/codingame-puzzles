use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

static BOARD: i64 = 0b11111_00_11111_00_11111_00_11111_00_11111;

type GameState = [i64; 20];

type CardMoves = [i64; 4];

type Cards = [CardMoves; 16];

static WHITE_PLAYER_ID: usize = 0;
static BLACK_PLAYER_ID: usize = 1;
static WHITE_PLAYER_OFFSET: usize = 0;
static BLACK_PLAYER_OFFSET: usize = 9;
static WIZARD_OFFSET: usize = 0;
static STUDENTS_OFFSET: usize = 1;
static NUM_OF_STUDENTS: usize = 4;
static CARDS_OFFSET: usize = 5;
static NUM_OF_CARDS: usize = 2;
static MIDDLE_CARD_OFFSET: usize = 18;

struct MinMaxNode {
    score: i64,
    depth: i64,
    chosen_card_id: i64,
    game_state: GameState,
    child_nodes: Vec<MinMaxNode>,
}

impl MinMaxNode {
    pub fn expand(&mut self) {
        self.child_nodes = vec![Node::new(), Node::new()];
    }

    pub fn is_leaf(&self) -> bool {
        self.game_state.len() == 0
    }

    fn expand_leaf_and_inc(&mut self) {
        if self.is_leaf() {
            return self.expand();
        }

        let index = 0;
        self.child_nodes[index].expand_leaf_and_inc();
        self.data += 1
    }
}

fn build_min_max_tree(
    tree: &Vec<MinMaxNode>,
    node_global_index: usize,
    cards: &Cards,
    target_depth: i64,
    maximizing_player_id: usize,
) {
    let node = &tree[node_global_index];
    if node.depth == target_depth {
        return;
    }
    let player_offset = if maximizing_player_id == WHITE_PLAYER_ID {
        WHITE_PLAYER_OFFSET
    } else {
        BLACK_PLAYER_OFFSET
    };
    let cards_offset = player_offset + CARDS_OFFSET;
    let wizard_offset = player_offset + WIZARD_OFFSET;
    let students_offset = player_offset + STUDENTS_OFFSET;
    for i in 0..NUM_OF_CARDS {
        let card_id = node.game_state[cards_offset + i];
        let card_moves = cards[card_id as usize];
        for card_move in card_moves {
            // let cloned_game_state = node.game_state.clone();
            let wizard = node.game_state[wizard_offset];
            let wizard_after_move = apply_move(wizard, card_move);
            if wizard_after_move != 0 {
                let mut cloned_game_state = node.game_state.clone();
                cloned_game_state[wizard_offset] = wizard_after_move;
                let child_node_wizard = MinMaxNode {
                    score: 0,
                    depth: node.depth + 1,
                    chosen_card_id: card_id,
                    game_state: cloned_game_state,
                    child_nodes: Vec::new(),
                };
                tree.push(child_node_wizard);
                build_min_max_tree(&tree, 1, &cards, target_depth, maximizing_player_id);
            }

            for student_index in 0..NUM_OF_STUDENTS {
                let student = node.game_state[students_offset + i];
                let student_after_move = apply_move(student, card_move);
                if student_after_move != 0 {
                    let mut cloned_game_state = node.game_state.clone();
                    cloned_game_state[students_offset + i] = student_after_move;
                    let child_node_student = MinMaxNode {
                        score: 0,
                        depth: node.depth + 1,
                        chosen_card_id: card_id,
                        game_state: cloned_game_state,
                        child_nodes: Vec::new(),
                    };
                    tree.push(child_node_student);
                    build_min_max_tree(&tree, 1, &cards, target_depth, maximizing_player_id);
                }
            }
        }
    }
}

fn shift_bits(to_shift: i64, shift_by: i64) -> i64 {
    if shift_by > 0 {
        return to_shift << shift_by;
    }
    if shift_by < 0 {
        return to_shift >> (-1 * shift_by);
    }
    return to_shift;
}

fn apply_move(to_move: i64, move_by: i64) -> i64 {
    return shift_bits(to_move, move_by) & BOARD;
}

fn calc_bit_shift_for_card(dx: i64, dy: i64) -> i64 {
    let mut total_shift: i64 = 0;
    match dx {
        -2 => total_shift += 2,
        -1 => total_shift += 1,
        0 => total_shift += 0,
        1 => total_shift += -1,
        2 => total_shift += -2,
        _ => {}
    }
    match dy {
        -2 => total_shift += -10,
        -1 => total_shift += -5,
        0 => total_shift += 0,
        1 => total_shift += 5,
        2 => total_shift += 10,
        _ => {}
    }
    return total_shift;
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let player_id = parse_input!(input_line, i64);

    // game loop
    loop {
        let mut game_state: GameState = [0; 20];
        let mut cards: Cards = [[0, 0, 0, 0]; 16];
        let mut b_student_index = 0;
        let mut w_student_index = 0;

        for i in 0..5 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let board = input_line.trim_matches('\n').to_string();
            for k in 0..5 as usize {
                let cell = board.chars().nth(k).unwrap();
                let cell_index = (32 - (i * 7 + k)) as u32;
                let base: i64 = 2;
                match cell {
                    'w' => {
                        let index = WHITE_PLAYER_OFFSET + STUDENTS_OFFSET + w_student_index;
                        game_state[index] = base.pow(cell_index);
                        w_student_index += 1;
                    }
                    'W' => {
                        let index = WHITE_PLAYER_OFFSET + WIZARD_OFFSET;
                        game_state[index] = base.pow(cell_index);
                    }
                    'b' => {
                        let index = BLACK_PLAYER_OFFSET + STUDENTS_OFFSET + b_student_index;
                        game_state[index] = base.pow(cell_index);
                        b_student_index += 1;
                    }
                    'B' => {
                        let index = BLACK_PLAYER_OFFSET + WIZARD_OFFSET;
                        game_state[index] = base.pow(cell_index);
                    }
                    _ => {}
                }
            }
        }

        let mut b_card_index = 0;
        let mut w_card_index = 0;

        for i in 0..5 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let owner = parse_input!(inputs[0], i64);
            let card_id = parse_input!(inputs[1], i64);
            let dx_1 = parse_input!(inputs[2], i64);
            let dy_1 = parse_input!(inputs[3], i64);
            let dx_2 = parse_input!(inputs[4], i64);
            let dy_2 = parse_input!(inputs[5], i64);
            let dx_3 = parse_input!(inputs[6], i64);
            let dy_3 = parse_input!(inputs[7], i64);
            let dx_4 = parse_input!(inputs[8], i64);
            let dy_4 = parse_input!(inputs[9], i64);

            cards[card_id as usize] = [
                calc_bit_shift_for_card(dx_1, dy_1),
                calc_bit_shift_for_card(dx_2, dy_2),
                calc_bit_shift_for_card(dx_3, dy_3),
                calc_bit_shift_for_card(dx_4, dy_4),
            ];

            match owner {
                0 => {
                    let index = WHITE_PLAYER_OFFSET + CARDS_OFFSET + w_card_index;
                    game_state[index] = card_id;
                    game_state[index + 1] = 0;
                    w_card_index += 2;
                }
                1 => {
                    let index = BLACK_PLAYER_OFFSET + CARDS_OFFSET + b_card_index;
                    game_state[index] = card_id;
                    game_state[index + 1] = 0;
                    b_card_index += 2;
                }
                -1 => {
                    let index = MIDDLE_CARD_OFFSET;
                    game_state[index] = card_id;
                    game_state[index + 1] = 0;
                }
                _ => {}
            }
        }

        println!("{:?}", cards);

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let action_count = parse_input!(input_line, i64);
        for _i in 0..action_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let _card_id = parse_input!(inputs[0], i64);
            // let move = inputs[1].trim().to_string();
        }

        let mut min_max_tree = Vec::new();

        min_max_tree.push(MinMaxNode {
            score: 0,
            depth: 0,
            chosen_card_id: 0,
            game_state: game_state.clone(),
            child_nodes: Vec::new(),
        });

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
