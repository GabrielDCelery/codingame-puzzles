use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug)]
struct Robot {
    id: usize,
    local_area: [[i8; 5]; 5],
}

impl Robot {
    fn new() -> Robot {
        return Robot {
            id: 0,
            local_area: [[0; 5]; 5],
        };
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn set_local_area_cell(&mut self, x: usize, y: usize, value: i8) {
        self.local_area[y][x] = value;
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

        for robot_id in 0..number_of_robots as usize {
            let mut robot = Robot::new();
            robot.set_id(robot_id);

            for j in 0..5 as usize {
                let mut inputs = String::new();
                io::stdin().read_line(&mut inputs).unwrap();
                let y: usize = 5 - j;
                for k in inputs.split_whitespace() {
                    let x: usize = k.parse().unwrap();
                    let cell = parse_input!(k, i8);
                    robot.set_local_area_cell(y, x, cell);
                }
            }
            eprintln!("{:?}", robot);
        }
        for robot_id in 0..number_of_robots as usize {
            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");

            // GUARD | MOVE (LEFT/RIGHT/UP/DOWN) | ATTACK (LEFT/RIGHT/UP/DOWN) | SELFDESTRUCTION <message>
            println!("SELFDESTRUCTION");
        }
    }
}
