use std::collections::HashMap;
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

    fn get_manhattan_distance(&self, other: &Coordinates) -> i8 {
        return (other.x - self.x).abs() + (other.y - self.y).abs();
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

#[derive(Debug)]
struct Robot {
    id: usize,
    is_friendly: bool,
    health: i8,
    coordinates: Coordinates,
}

#[derive(Debug)]
struct LocalArea {
    id: usize,
    grid: [[i8; 5]; 5],
    robots: Vec<Robot>,
}

impl LocalArea {
    fn new(id: usize) -> LocalArea {
        return LocalArea {
            id,
            grid: [[0; 5]; 5],
            robots: vec![],
        };
    }

    fn set_local_area_cell(&mut self, coordinates: &Coordinates, value: i8) {
        self.grid[coordinates.y as usize][coordinates.x as usize] = value;
        if value == 0 {
            return;
        }
        let id = self.robots.len();
        let is_friendly = value > 0;
        let health = value.abs();
        self.robots.insert(
            id,
            Robot {
                id,
                is_friendly,
                health,
                coordinates: coordinates.clone(),
            },
        )
    }

    fn get_robot_health_at_coords(&self, coordinates: &Coordinates) -> i8 {
        return self.grid[coordinates.y as usize][coordinates.x as usize];
    }

    fn is_enemy_at_coords(&self, coordinates: &Coordinates) -> bool {
        return self.get_robot_health_at_coords(coordinates) < 0;
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ConditionName {
    DefaultTrue,
    AmIAlone,
}

trait Condition {
    fn is_true(&self, local_area: &LocalArea) -> bool;
}

type ConditionDictionary = HashMap<ConditionName, Box<dyn Condition>>;

struct DefaultTrue {}

impl Condition for DefaultTrue {
    fn is_true(&self, _: &LocalArea) -> bool {
        return true;
    }
}

struct AmIAlone {}

impl Condition for AmIAlone {
    fn is_true(&self, local_area: &LocalArea) -> bool {
        let friendly: Vec<&Robot> = local_area
            .robots
            .iter()
            .filter(|v| v.is_friendly == true)
            .collect();
        return friendly.len() == 1;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum ActionName {
    SeekFriends,
    GuardMySelf,
}

trait Action {
    fn get_command(&self, local_area: &LocalArea) -> String;
}

type ActionDictionary = HashMap<ActionName, Box<dyn Action>>;

struct SeekFriends {}

impl Action for SeekFriends {
    fn get_command(&self, _: &LocalArea) -> String {
        return "MOVE UP".to_string();
    }
}

struct GuardMySelf {}

impl Action for GuardMySelf {
    fn get_command(&self, _: &LocalArea) -> String {
        return "GUARD".to_string();
    }
}

struct ActionConfiguration {
    conditions: Vec<ConditionName>,
    action_name: ActionName,
}

impl ActionConfiguration {
    fn new(action_name: ActionName, conditions: Vec<ConditionName>) -> ActionConfiguration {
        return ActionConfiguration {
            action_name,
            conditions,
        };
    }

    fn is_matching_preconditions(
        &self,
        local_area: &LocalArea,
        condition_dictionary: &ConditionDictionary,
    ) -> bool {
        for precondition in self.conditions.iter() {
            let precondition_trait = condition_dictionary.get(&precondition).unwrap();
            let is_true = precondition_trait.is_true(&local_area);
            if is_true == false {
                return false;
            }
        }
        return true;
    }
}

struct GameAI<'a> {
    condition_dictionary: &'a ConditionDictionary,
    action_configurations: &'a Vec<ActionConfiguration>,
    action_dictionary: &'a ActionDictionary,
}

impl GameAI<'_> {
    fn pick_action(&self, local_area: &LocalArea) -> ActionName {
        for action_configuration in self.action_configurations.iter() {
            let is_matching = action_configuration
                .is_matching_preconditions(&local_area, self.condition_dictionary);
            if is_matching {
                return action_configuration.action_name;
            }
        }
        panic!("Could not find action matching conditions")
    }

    fn get_command_for_robot(&self, local_area: &LocalArea) -> String {
        let action_name = self.pick_action(&local_area);
        let action = self.action_dictionary.get(&action_name).unwrap();
        return action.get_command(&local_area);
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut condition_dictionary: ConditionDictionary = HashMap::new();

    condition_dictionary.insert(ConditionName::AmIAlone, Box::new(AmIAlone {}));
    condition_dictionary.insert(ConditionName::DefaultTrue, Box::new(DefaultTrue {}));

    let mut action_dictionary: ActionDictionary = HashMap::new();

    action_dictionary.insert(ActionName::SeekFriends, Box::new(SeekFriends {}));
    action_dictionary.insert(ActionName::GuardMySelf, Box::new(GuardMySelf {}));

    let action_configurations: Vec<ActionConfiguration> = vec![
        ActionConfiguration::new(ActionName::SeekFriends, vec![ConditionName::AmIAlone]),
        ActionConfiguration::new(ActionName::GuardMySelf, vec![ConditionName::DefaultTrue]),
    ];

    let game_ai = GameAI {
        condition_dictionary: &condition_dictionary,
        action_dictionary: &action_dictionary,
        action_configurations: &action_configurations,
    };

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_robots = parse_input!(input_line, i32);

        let mut local_area_list: Vec<LocalArea> = vec![];

        for i in 0..number_of_robots as usize {
            let mut local_area = LocalArea::new(i);

            for j in 0..5 as usize {
                let mut inputs = String::new();
                io::stdin().read_line(&mut inputs).unwrap();
                let y = j as i8;
                let mut x = 0 as i8;
                for k in inputs.split_whitespace() {
                    let value = parse_input!(k, i8);
                    local_area.set_local_area_cell(&Coordinates::new(x, y), value);
                    x += 1;
                }
            }
            local_area_list.insert(local_area.id, local_area);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        // GUARD | MOVE (LEFT/RIGHT/UP/DOWN) | ATTACK (LEFT/RIGHT/UP/DOWN) | SELFDESTRUCTION <message>

        for i in 0..number_of_robots as usize {
            let local_area = local_area_list.get(i).unwrap();
            let command = game_ai.get_command_for_robot(&local_area);

            println!("{}", command);
        }
    }
}
