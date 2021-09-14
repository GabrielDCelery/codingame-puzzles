use std::collections::HashMap;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Copy, Clone)]
enum Terrain {
    None,
    Grassland,
    Water,
    Mountain,
    Swamp,
    Ravine,
    PointOfInterest,
}

impl Terrain {
    fn convert_str_to_enum(string: &str) -> Result<Terrain, ()> {
        return match string {
            "G" => Ok(Terrain::Grassland),
            "W" => Ok(Terrain::Water),
            "M" => Ok(Terrain::Mountain),
            "S" => Ok(Terrain::Swamp),
            "R" => Ok(Terrain::Ravine),
            "I" => Ok(Terrain::PointOfInterest),
            _ => Err(()),
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum PointOfInterest {
    None,
    House,
    Castle,
    Blacksmith,
    Stable,
    Wizard,
    Princess,
    Dragon,
    Treasure,
}

impl PointOfInterest {
    fn convert_str_to_enum(string: &str) -> Result<PointOfInterest, ()> {
        return match string {
            "HOUSE" => Ok(PointOfInterest::House),
            "CASTLE" => Ok(PointOfInterest::Castle),
            "BLACKSMITH" => Ok(PointOfInterest::Blacksmith),
            "STABLE" => Ok(PointOfInterest::Stable),
            "WIZARD" => Ok(PointOfInterest::Wizard),
            "PRINCESS" => Ok(PointOfInterest::Princess),
            "DRAGON" => Ok(PointOfInterest::Dragon),
            "TREASURE" => Ok(PointOfInterest::Treasure),
            _ => Err(()),
        };
    }
}

type Coordinates = [u8; 2];

#[derive(Debug, Copy, Clone)]
struct MapCell {
    terrain: Terrain,
    point_of_interest: PointOfInterest,
}

type MapMatrix = Vec<Vec<MapCell>>;
type PointOfInterestMap = HashMap<PointOfInterest, Coordinates>;

#[derive(Debug)]
struct GameMap {
    width: usize,
    height: usize,
    map_matrix: MapMatrix,
    point_of_interest_map: PointOfInterestMap,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Weapon {
    None,
    Sword,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Mount {
    None,
    Horse,
}

#[derive(Debug)]
struct Player {
    weapon: Weapon,
    mount: Mount,
    objective: PointOfInterest,
    has_completed_objective: bool,
    has_finished_the_game: bool,
}

impl Player {
    fn clone(&self) -> Player {
        return Player {
            weapon: self.weapon,
            mount: self.mount,
            objective: self.objective,
            has_completed_objective: self.has_completed_objective,
            has_finished_the_game: self.has_finished_the_game,
        };
    }

    fn can_do(&self, point_of_interest: &PointOfInterest) -> bool {
        return match point_of_interest {
            PointOfInterest::House => false,
            PointOfInterest::Castle => {
                self.has_finished_the_game == false && self.has_completed_objective == true
            }
            PointOfInterest::Princess => {
                self.has_finished_the_game == false && self.has_completed_objective == false
            }
            PointOfInterest::Blacksmith => {
                self.has_finished_the_game == false && self.weapon == Weapon::None
            }
            _ => false,
        };
    }

    fn apply_point_of_interest_effect(&mut self, point_of_interest: &PointOfInterest) {
        match point_of_interest {
            PointOfInterest::Castle => self.has_finished_the_game = true,
            PointOfInterest::Princess => {
                if self.objective == PointOfInterest::Princess {
                    self.has_completed_objective = true
                }
            }
            PointOfInterest::Blacksmith => self.weapon = Weapon::Sword,
            PointOfInterest::Stable => self.mount = Mount::Horse,
            _ => (),
        };
    }

    fn apply_terrain_effect(&mut self, terrain: &Terrain) {
        match terrain {
            Terrain::Water => self.weapon = Weapon::None,
            _ => (),
        };
    }
}

#[derive(Debug)]
struct PlayerMovesTreeNode {
    player: Player,
    point_of_interest: PointOfInterest,
    children: Vec<PlayerMovesTreeNode>,
}

impl PlayerMovesTreeNode {
    fn new(
        player: &Player,
        point_of_interest: &PointOfInterest,
        available_point_of_interests: &Vec<PointOfInterest>,
    ) -> PlayerMovesTreeNode {
        let cloned_player = player.clone();
        let mut children: Vec<PlayerMovesTreeNode> = vec![];

        for chosen_point_of_interest in available_point_of_interests {
            let mut cloned_player = player.clone();
            if cloned_player.can_do(chosen_point_of_interest) {
                let mut cloned_point_of_interests = available_point_of_interests.clone();
                let index = available_point_of_interests
                    .iter()
                    .position(|&r| r == *chosen_point_of_interest)
                    .unwrap();
                cloned_point_of_interests.remove(index);
                cloned_player.apply_point_of_interest_effect(chosen_point_of_interest);
                children.push(PlayerMovesTreeNode::new(
                    &cloned_player,
                    &chosen_point_of_interest,
                    &cloned_point_of_interests,
                ))
            }
        }

        return PlayerMovesTreeNode {
            player: cloned_player,
            point_of_interest: *point_of_interest,
            children,
        };
    }
}

fn main() {
    //##################### READ GAME INPUT - START
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let game_inputs = input_line.split(" ").collect::<Vec<_>>();
    let map_width = parse_input!(game_inputs[0], usize);
    let map_height = parse_input!(game_inputs[1], usize);
    let num_of_point_of_interests = parse_input!(game_inputs[2], usize);
    //##################### READ GAME INPUT - END

    let mut player = Player {
        weapon: Weapon::None,
        mount: Mount::None,
        objective: PointOfInterest::None,
        has_completed_objective: false,
        has_finished_the_game: false,
    };

    let mut map_matrix = vec![
        vec![
            MapCell {
                terrain: Terrain::None,
                point_of_interest: PointOfInterest::None,
            };
            map_width
        ];
        map_height
    ];

    let mut point_of_interest_map = HashMap::new();

    //##################### READ GAME INPUT - START
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let point_of_interest_type_as_str = input_line.trim().to_string();
    player.objective =
        PointOfInterest::convert_str_to_enum(&point_of_interest_type_as_str).unwrap();
    for h in 0..map_height {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();
        for w in 0..map_width {
            let terrain_type_as_str = line.chars().nth(w).unwrap().to_string();
            let terrain = Terrain::convert_str_to_enum(&terrain_type_as_str).unwrap();
            map_matrix[h][w].terrain = terrain
        }
    }
    for _ in 0..num_of_point_of_interests as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let game_inputs = input_line.split(" ").collect::<Vec<_>>();
        let point_of_interest = game_inputs[0].trim().to_string();
        let x = parse_input!(game_inputs[1], u8);
        let y = parse_input!(game_inputs[2], u8);
        point_of_interest_map.insert(
            PointOfInterest::convert_str_to_enum(&point_of_interest).unwrap(),
            [y, x],
        );
    }
    //##################### READ GAME INPUT - END

    let available_point_of_interests: Vec<PointOfInterest> =
        point_of_interest_map.keys().cloned().collect();

    let game_map = GameMap {
        width: map_width,
        height: map_height,
        map_matrix,
        point_of_interest_map,
    };

    let root_node = PlayerMovesTreeNode::new(
        &player,
        &PointOfInterest::House,
        &available_point_of_interests,
    );

    println!("{:?}", root_node);
}
