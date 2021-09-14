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

#[derive(Debug, Copy, Clone, PartialEq)]
enum PlayerGoalStatus {
    Roaming,
    ObjectiveCompleted,
    RewardCollected,
}

#[derive(Debug, Copy, Clone)]
struct Coordinates {
    x: usize,
    y: usize,
}

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
    location: PointOfInterest,
    weapon: Weapon,
    mount: Mount,
    objective: PointOfInterest,
    goal_status: PlayerGoalStatus,
}

fn clone_player(player: &Player) -> Player {
    return Player {
        location: player.location,
        weapon: player.weapon,
        mount: player.mount,
        objective: player.objective,
        goal_status: player.goal_status,
    };
}

fn is_player_interested_moving_to_point_of_interest(
    next_point_of_interest: PointOfInterest,
    player: &Player,
    game_map: &GameMap,
) -> bool {
    if player.location == next_point_of_interest {
        return false;
    }

    return match next_point_of_interest {
        PointOfInterest::House => false,
        PointOfInterest::Castle => {
            return player.goal_status == PlayerGoalStatus::ObjectiveCompleted;
        }
        PointOfInterest::Princess => {
            return player.objective == PointOfInterest::Princess
                && player.goal_status == PlayerGoalStatus::Roaming;
        }
        /*
        PointOfInterest::Blacksmith => {
            return player.goal_status != PlayerGoalStatus::ObjectiveCompleted
                && player.weapon == Weapon::None
        }
        */
        _ => false,
    };
}

fn apply_point_of_interest_effect_on_player(
    player: &mut Player,
    point_of_interest: PointOfInterest,
) {
    player.location = point_of_interest;
    match point_of_interest {
        PointOfInterest::Castle => {
            player.goal_status = PlayerGoalStatus::RewardCollected;
        }
        PointOfInterest::Princess => {
            if player.objective != PointOfInterest::Princess {
                return;
            }
            player.goal_status = PlayerGoalStatus::ObjectiveCompleted;
        }
        PointOfInterest::Blacksmith => player.weapon = Weapon::Sword,
        PointOfInterest::Stable => player.mount = Mount::Horse,
        _ => (),
    };
}

#[derive(Debug)]
struct PlayerGoalPathTreeNode {
    player: Player,
    children: Vec<PlayerGoalPathTreeNode>,
}

impl PlayerGoalPathTreeNode {
    fn new(player: &Player, game_map: &GameMap) -> PlayerGoalPathTreeNode {
        let mut children: Vec<PlayerGoalPathTreeNode> = vec![];

        if player.goal_status != PlayerGoalStatus::RewardCollected {
            for next_point_of_interest in game_map.point_of_interest_map.keys() {
                if is_player_interested_moving_to_point_of_interest(
                    *next_point_of_interest,
                    &player,
                    &game_map,
                ) == false
                {
                    continue;
                }
                let mut cloned_player = clone_player(player);
                apply_point_of_interest_effect_on_player(
                    &mut cloned_player,
                    *next_point_of_interest,
                );
                children.push(PlayerGoalPathTreeNode::new(&cloned_player, &game_map));
            }
        }

        return PlayerGoalPathTreeNode {
            player: clone_player(player),
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
        location: PointOfInterest::House,
        weapon: Weapon::None,
        mount: Mount::None,
        objective: PointOfInterest::None,
        goal_status: PlayerGoalStatus::Roaming,
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
        let point_of_interest_as_str = game_inputs[0].trim().to_string();
        let x = parse_input!(game_inputs[1], usize);
        let y = parse_input!(game_inputs[2], usize);
        let point_of_interest =
            PointOfInterest::convert_str_to_enum(&point_of_interest_as_str).unwrap();
        point_of_interest_map.insert(point_of_interest, Coordinates { x, y });
    }
    //##################### READ GAME INPUT - END

    let game_map = GameMap {
        width: map_width,
        height: map_height,
        map_matrix,
        point_of_interest_map,
    };

    let root_node = PlayerGoalPathTreeNode::new(&player, &game_map);

    println!("{:?}", root_node);
}
