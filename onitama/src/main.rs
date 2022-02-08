use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

static EMPTY_BOARD: i32 = 0b00000_00000_00000_00000_00000;

struct Card {
    card_id: i32,
}

#[derive(Debug)]
struct Player {
    king: i32,
    students: [i32; 4],
    cards: [i32; 2],
}

#[derive(Debug)]
struct GameState {
    players: [Player; 2],
    middle_card: i32,
}

fn apply_move(p: i32, m: i32) -> i32 {
    return p << m;
}

fn is_valid_move(result: i32) -> bool {
    return result > EMPTY_BOARD; // did not move off the board
}

fn shift_bits(to_shift: i32, shift_by: i32) -> i32 {
    if shift_by > 0 {
        return to_shift << shift_by;
    }
    if shift_by < 0 {
        return to_shift >> (-1 * shift_by);
    }
    return to_shift;
}
/*
fn get_mask_for_reachable_cells_from_position(position: i32) -> i32 {
    let mut reachable_cells: i32 = position;
    for x in [-2, -1, 0, 1, 2] {
        for y in [-10, -5, 0, 5, 10] {
            let shift_by = x + y;
            reachable_cells = reachable_cells | shift_bits(position, shift_by);
        }
    }
    return reachable_cells;
}
*/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let player_id = parse_input!(input_line, i32);

    // game loop
    loop {
        let mut game_state = GameState {
            players: [
                Player {
                    king: 0,
                    students: [0, 0, 0, 0],
                    cards: [0, 0],
                },
                Player {
                    king: 0,
                    students: [0, 0, 0, 0],
                    cards: [0, 0],
                },
            ],
            middle_card: 0,
        };
        let mut b_student_index = 0;
        let mut w_student_index = 0;

        for i in 0..5 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let board = input_line.trim_matches('\n').to_string();
            for k in 0..5 as usize {
                let cell = board.chars().nth(k).unwrap();
                let cell_index = (24 - (i * 5 + k)) as u32;
                let base: i32 = 2;
                match cell {
                    'w' => {
                        game_state.players[0].students[w_student_index] = base.pow(cell_index);
                        w_student_index += 1;
                    }
                    'W' => {
                        game_state.players[0].king = base.pow(cell_index);
                    }
                    'b' => {
                        game_state.players[1].students[b_student_index] = base.pow(cell_index);
                        b_student_index += 1;
                    }
                    'B' => {
                        game_state.players[1].king = base.pow(cell_index);
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

            match owner {
                0 => {
                    game_state.players[0].cards[w_card_index] = card_id;
                    w_card_index += 1;
                }
                1 => {
                    game_state.players[1].cards[b_card_index] = card_id;
                    b_card_index += 1;
                }
                -1 => game_state.middle_card = card_id,
                _ => {}
            }
        }
        println!("{:?}", game_state);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let action_count = parse_input!(input_line, i32);
        for i in 0..action_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let card_id = parse_input!(inputs[0], i32);
            // let move = inputs[1].trim().to_string();
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("1 A1B2 moving the student");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
