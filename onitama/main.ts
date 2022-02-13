/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
declare const readline: any;

const BOARD: number = 0b11111_00_11111_00_11111_00_11111_00_11111;

type GameState = number[];

type CardMoves = number[];

type Cards = { [index: string]: CardMoves };

const WHITE_PLAYER_ID: number = 0;
const BLACK_PLAYER_ID: number = 1;
const WHITE_PLAYER_OFFSET: number = 0;
const BLACK_PLAYER_OFFSET: number = 9;
const WIZARD_OFFSET: number = 0;
const STUDENTS_OFFSET: number = 1;
const NUM_OF_WIZARD: number = 1;
const NUM_OF_STUDENTS: number = 4;
const CARDS_OFFSET: number = 5;
const NUM_OF_CARDS_PER_PLAYER: number = 2;
const MIDDLE_CARD_OFFSET: number = 18;

type MinMaxNode = {
  score: number;
  depth: number;
  chosen_card_id: number;
  game_state: GameState;
  child_nodes: MinMaxNode[];
};

const shift_bits = (to_shift: number, shift_by: number): number => {
  if (shift_by > 0) {
    return to_shift << shift_by;
  }
  if (shift_by < 0) {
    return to_shift >> (-1 * shift_by);
  }
  return to_shift;
};

const apply_move_to_game_state = (
  game_state: GameState,
  cards: Cards,
  piece_local_index: number,
  card_local_index: number,
  card_move_local_index: number,
  player_id: number
): number => {
  let player_global_offset =
    player_id === WHITE_PLAYER_ID ? WHITE_PLAYER_OFFSET : BLACK_PLAYER_OFFSET;

  const after_move = shift_bits(
    player_global_offset + piece_local_index,
    cards[player_global_offset + CARDS_OFFSET + card_local_index][
      card_move_local_index
    ]
  );
};

const is_valid_game_state = (game_state: GameState): boolean => {};

const build_min_max_tree = (
  node: MinMaxNode,
  cards: Cards,
  target_depth: number,
  maximizing_player_id: number
) => {
  if (node.depth === target_depth) {
    return;
  }
  let player_global_offset =
    maximizing_player_id === WHITE_PLAYER_ID
      ? WHITE_PLAYER_OFFSET
      : BLACK_PLAYER_OFFSET;

  for (
    let card_local_index = 0;
    card_local_index < NUM_OF_CARDS_PER_PLAYER;
    card_local_index++
  ) {
    const card_global_index =
      player_global_offset + CARDS_OFFSET + card_local_index;
    let card_id = node.game_state[card_global_index];
    let card_moves = cards[card_id];
    card_moves.forEach((card_move) => {
      [0, 1, 2, 3, 4].forEach((moving_piece_local_index) => {
        const moving_piece_global_index =
          player_global_offset + moving_piece_local_index;
        const moving_piece_board_position_before_move =
          node.game_state[moving_piece_global_index];
        const moving_piece_board_position_after_move = shift_bits(
          moving_piece_board_position_before_move,
          card_move
        );

        // check if piece moves off the board
        if ((moving_piece_board_position_after_move & BOARD) <= 0) {
          return;
        }

        for (
          let other_piece_local_index = 0, iMax = 5;
          other_piece_local_index < iMax;
          other_piece_local_index++
        ) {
          const other_piece_global_index =
            player_global_offset + other_piece_local_index;
          if (other_piece_global_index === moving_piece_global_index) {
            continue;
          }
          const other_piece_board_position =
            node.game_state[other_piece_global_index];
          // check if we moved on our own piece
          if (
            (other_piece_board_position &
              moving_piece_board_position_after_move) !==
            0
          ) {
            return;
          }
        }

        // check if piece steps on other own piece
      });
    });
  }
};

const apply_move = (to_move: number, move_by: number): number => {
  return shift_bits(to_move, move_by) & BOARD;
};

const calc_bit_shift_for_card = (dx: number, dy: number): number => {
  let total_shift: number = 0;
  switch (dx) {
    case -2: {
      total_shift += 2;
      break;
    }
    case -1: {
      total_shift += 1;
      break;
    }
    case 1: {
      total_shift += -1;
      break;
    }
    case 2: {
      total_shift += -2;
      break;
    }
    default: {
    }
  }
  switch (dy) {
    case -2: {
      total_shift += -14;
      break;
    }
    case -1: {
      total_shift += -7;
      break;
    }
    case 1: {
      total_shift += 7;
      break;
    }
    case 2: {
      total_shift += 14;
      break;
    }
    default: {
    }
  }

  return total_shift;
};

const playerId: number = parseInt(readline());

// game loop
while (true) {
  let game_state: GameState = new Array(20).fill(0);
  let cards: Cards = {};
  let b_student_index = 0;
  let w_student_index = 0;

  for (let i = 0; i < 5; i++) {
    const board: string = readline();
    const cells = board.split("");
    cells.forEach((cell, k) => {
      let cell_index = 32 - (i * 7 + k);
      switch (cell) {
        case "w": {
          let index = WHITE_PLAYER_OFFSET + STUDENTS_OFFSET + w_student_index;
          game_state[index] = Math.pow(2, cell_index);
          w_student_index += 1;
          break;
        }
        case "W": {
          let index = WHITE_PLAYER_OFFSET + WIZARD_OFFSET;
          game_state[index] = Math.pow(2, cell_index);
          break;
        }
        case "b": {
          let index = BLACK_PLAYER_OFFSET + STUDENTS_OFFSET + b_student_index;
          game_state[index] = Math.pow(2, cell_index);
          b_student_index += 1;
          break;
        }
        case "B": {
          let index = BLACK_PLAYER_OFFSET + WIZARD_OFFSET;
          game_state[index] = Math.pow(2, cell_index);
          break;
        }
        default: {
        }
      }
    });
  }

  let b_card_index = 0;
  let w_card_index = 0;

  for (let i = 0; i < 5; i++) {
    var inputs: string[] = readline().split(" ");
    const owner: number = parseInt(inputs[0]);
    const cardId: number = parseInt(inputs[1]);
    const dx1: number = parseInt(inputs[2]);
    const dy1: number = parseInt(inputs[3]);
    const dx2: number = parseInt(inputs[4]);
    const dy2: number = parseInt(inputs[5]);
    const dx3: number = parseInt(inputs[6]);
    const dy3: number = parseInt(inputs[7]);
    const dx4: number = parseInt(inputs[8]);
    const dy4: number = parseInt(inputs[9]);

    cards[cardId] = [
      calc_bit_shift_for_card(dx1, dy1),
      calc_bit_shift_for_card(dx2, dy2),
      calc_bit_shift_for_card(dx3, dy3),
      calc_bit_shift_for_card(dx4, dy4),
    ];

    switch (owner) {
      case 0: {
        let index = WHITE_PLAYER_OFFSET + CARDS_OFFSET + w_card_index;
        game_state[index] = cardId;
        game_state[index + 1] = 0;
        w_card_index += 2;
        break;
      }
      case 1: {
        let index = BLACK_PLAYER_OFFSET + CARDS_OFFSET + b_card_index;
        game_state[index] = cardId;
        game_state[index + 1] = 0;
        b_card_index += 2;
        break;
      }
      case -1: {
        let index = MIDDLE_CARD_OFFSET;
        game_state[index] = cardId;
        game_state[index + 1] = 0;
        break;
      }
      default: {
      }
    }
  }

  const actionCount: number = parseInt(readline());
  for (let i = 0; i < actionCount; i++) {
    var inputs: string[] = readline().split(" ");
    const cardId: number = parseInt(inputs[0]);
    const move: string = inputs[1];
  }

  const rootNode: MinMaxNode = {
    score: 0,
    depth: 0,
    chosen_card_id: 0,
    game_state: game_state,
    child_nodes: [],
  };

  build_min_max_tree(rootNode, cards, 1, 0);

  console.error(JSON.stringify(rootNode));

  // Write an action using console.log()
  // To debug: console.error('Debug messages...');

  console.log("1 A1B2 moving the student");
}
