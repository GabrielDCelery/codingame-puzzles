use std::io;
use std::slice::Iter;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Clone)]
struct Coordinates {
    x: i8,
    y: i8,
}

impl Coordinates {
    fn new(x: i8, y: i8) -> Coordinates {
        return Coordinates { x, y };
    }

    fn add_vector(&mut self, vector: &Vector) {
        self.x += vector.x;
        self.y += vector.y;
    }
}

#[derive(Debug)]
struct Vector {
    x: i8,
    y: i8,
}

impl Vector {
    fn to_enum(&self) -> Direction {
        return match self {
            Vector { x: 0, y: -1 } => Direction::UP,
            Vector { x: 0, y: 1 } => Direction::DOWN,
            Vector { x: -1, y: 0 } => Direction::LEFT,
            Vector { x: 1, y: 0 } => Direction::RIGHT,
            _ => panic!("Not handled direction vector"),
        };
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

static DIRECTIONS: [Direction; 4] = [
    Direction::UP,
    Direction::DOWN,
    Direction::LEFT,
    Direction::RIGHT,
];

impl Direction {
    fn to_str(&self) -> &'static str {
        return match self {
            Direction::UP => "UP",
            Direction::DOWN => "DOWN",
            Direction::LEFT => "LEFT",
            Direction::RIGHT => "RIGHT",
        };
    }

    fn to_vector(&self) -> Vector {
        return match self {
            Direction::UP => Vector { x: 0, y: -1 },
            Direction::DOWN => Vector { x: 0, y: 1 },
            Direction::LEFT => Vector { x: -1, y: 0 },
            Direction::RIGHT => Vector { x: 1, y: 0 },
            _ => panic!("Not handled direction"),
        };
    }

    fn iterator() -> Iter<'static, Direction> {
        return DIRECTIONS.iter();
    }
}

trait Criterion {
    fn new() -> Self;
    fn is_true(&self, local_area: &LocalArea) -> bool;
}

struct HasEnemyNextToMe {}

impl Criterion for HasEnemyNextToMe {
    fn new() -> HasEnemyNextToMe {
        return HasEnemyNextToMe {};
    }

    fn is_true(&self, local_area: &LocalArea) -> bool {
        for direction in Direction::iterator() {
            let vector = direction.to_vector();
            let mut coordinates = Coordinates::new(2, 2);
            coordinates.add_vector(&vector);
            if local_area.is_enemy_at_coords(&coordinates) {
                return true;
            };
        }
        return false;
    }
}

#[derive(Debug)]
struct LocalArea {
    id: usize,
    grid: [[i8; 5]; 5],
}

impl LocalArea {
    fn new() -> LocalArea {
        return LocalArea {
            id: 0,
            grid: [[0; 5]; 5],
        };
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn set_local_area_cell(&mut self, coordinates: &Coordinates, value: i8) {
        self.grid[coordinates.y as usize][coordinates.x as usize] = value;
    }

    fn get_robot_health_at_coords(&self, coordinates: &Coordinates) -> i8 {
        return self.grid[coordinates.y as usize][coordinates.x as usize];
    }

    fn is_enemy_at_coords(&self, coordinates: &Coordinates) -> bool {
        return self.get_robot_health_at_coords(coordinates) < 0;
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_robots = parse_input!(input_line, i32);

        let mut local_area_list: Vec<LocalArea> = vec![];

        for i in 0..number_of_robots as usize {
            let mut local_area = LocalArea::new();
            local_area.set_id(i);

            for j in 0..5 as usize {
                let mut inputs = String::new();
                io::stdin().read_line(&mut inputs).unwrap();
                let y = j as i8;
                let mut x = 0 as i8;
                for k in inputs.split_whitespace() {
                    let cell = parse_input!(k, i8);
                    local_area.set_local_area_cell(&Coordinates::new(x, y), cell);
                    x += 1;
                }
            }
            local_area_list.insert(local_area.id, local_area);
        }

        eprintln!("{:?}", local_area_list);

        for i in 0..number_of_robots as usize {
            let local_area = local_area_list.get(i).unwrap();

            let criterion = HasEnemyNextToMe::new();

            criterion.is_true(&local_area);
            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");

            // GUARD | MOVE (LEFT/RIGHT/UP/DOWN) | ATTACK (LEFT/RIGHT/UP/DOWN) | SELFDESTRUCTION <message>
            println!("SELFDESTRUCTION");
        }
    }
}
