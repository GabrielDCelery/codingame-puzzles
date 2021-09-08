use std::collections::HashMap;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Copy, Clone)]
enum TerrainType {
    Grassland,
    Water,
    Mountain,
    Swamp,
    Ravine,
    PointOfInterest,
}

impl TerrainType {
    fn convert_str_to_enum(string: &str) -> Result<TerrainType, ()> {
        return match string {
            "G" => Ok(TerrainType::Grassland),
            "W" => Ok(TerrainType::Water),
            "M" => Ok(TerrainType::Mountain),
            "S" => Ok(TerrainType::Swamp),
            "R" => Ok(TerrainType::Ravine),
            "I" => Ok(TerrainType::PointOfInterest),
            _ => Err(()),
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum PointOfInterest {
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

#[derive(Debug, Copy, Clone)]
struct Terrain {
    terrain_type: TerrainType,
}

type TerrainMatrix = Vec<Vec<Terrain>>;
type PointOfInterests = HashMap<PointOfInterest, [u8; 2]>;

#[derive(Debug)]
struct GameMap {
    width: usize,
    height: usize,
    num_of_point_of_interests: usize,
    terrain_matrix: TerrainMatrix,
    point_of_interests: PointOfInterests,
}

#[derive(Debug)]
struct GameState<'gm> {
    objective: PointOfInterest,
    map: &'gm GameMap,
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let game_inputs = input_line.split(" ").collect::<Vec<_>>();

    let map_width = parse_input!(game_inputs[0], usize);
    let map_height = parse_input!(game_inputs[1], usize);
    let num_of_point_of_interests = parse_input!(game_inputs[2], usize);

    let mut terrain_matrix = vec![
        vec![
            Terrain {
                terrain_type: TerrainType::Grassland
            };
            map_width
        ];
        map_height
    ];
    let mut point_of_interests = HashMap::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let objective_as_str = input_line.trim().to_string();
    let objective = PointOfInterest::convert_str_to_enum(&objective_as_str).unwrap();

    for h in 0..map_height {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();
        for w in 0..map_width {
            let terrain_type_as_str = line.chars().nth(w).unwrap().to_string();
            let terrain_type = TerrainType::convert_str_to_enum(&terrain_type_as_str).unwrap();
            terrain_matrix[h][w] = Terrain { terrain_type }
        }
    }

    for _ in 0..num_of_point_of_interests as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let game_inputs = input_line.split(" ").collect::<Vec<_>>();
        let point_of_interest = game_inputs[0].trim().to_string();
        let x = parse_input!(game_inputs[1], u8);
        let y = parse_input!(game_inputs[2], u8);

        point_of_interests.insert(
            PointOfInterest::convert_str_to_enum(&point_of_interest).unwrap(),
            [y, x],
        );
    }

    let game_map = GameMap {
        width: map_width,
        height: map_height,
        num_of_point_of_interests,
        terrain_matrix,
        point_of_interests,
    };

    let game_state = GameState {
        objective,
        map: &game_map,
    };

    println!("{:?}", game_state)
}
