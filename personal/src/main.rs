/*
#[derive(Debug)]
struct Node<T> {
    data: T,
    children: Vec<Node<T>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node { data: data, children: vec![] }
    }

    fn add_child(&mut self, child: Node<T>) {
        self.children.push(child);
    }
}
*/

use std::collections::HashMap;

#[derive(Debug)]
struct GameState {
    data: [i32; 2],
}

#[derive(Debug)]
struct Node {
    depth: i32,
    game_state: GameState,
    children: Vec<Node>,
}

impl Node {
    fn new(depth: i32, game_state: GameState) -> Node {
        Node {
            depth: depth,
            game_state: game_state,
            children: vec![],
        }
    }

    fn build(&mut self, target_depth: i32) {
        if self.depth == target_depth {
            return;
        }

        for _ in 0..2 {
            let gs = GameState {
                data: [self.depth + 1, self.depth + 2],
            };
            let mut node = Node::new(self.depth + 1, gs);

            node.build(target_depth);

            self.children.push(node);
        }
    }
}

type CardMoves = [i32; 4];

type CardMovesMap = HashMap<i32, CardMoves>;

fn main() {
    println!("Hello, world!");

    let mut node: Node = Node::new(0, GameState { data: [1, 2] });

    node.build(3);

    let mut card_moves_map: CardMovesMap = HashMap::new();

    let mut moves: [i32; 4] = [0; 4];

    let card_id: i32 = -2;

    card_moves_map.insert(-2, moves);

    let card_moves = card_moves_map.get(&card_id).unwrap();

    println!("{:?}", card_moves);
}
