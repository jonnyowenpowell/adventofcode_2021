fn main() {
    let commands = parse_input();

    let position = calculate_position(&commands);
    println!("Resulting position: ({}, {})", position.0, position.1);

    let adjusted_position = calculate_adjusted_position(&commands);
    println!(
        "Resulting position: ({}, {})",
        adjusted_position.0, adjusted_position.1
    );
}

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse_input() -> Vec<Command> {
    include_str!("../input.txt")
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            let value: u32 = parts[1].parse().unwrap();
            match parts[0] {
                "forward" => Command::Forward(value),
                "down" => Command::Down(value),
                "up" => Command::Up(value),
                _ => panic!("unrecognised command"),
            }
        })
        .collect()
}

fn calculate_position(commands: &Vec<Command>) -> (u32, u32) {
    let (mut h, mut v) = (0, 0);
    for command in commands {
        match command {
            Command::Forward(x) => {
                h += x;
            }
            Command::Down(x) => {
                v += x;
            }
            Command::Up(x) => {
                v -= x;
            }
        }
    }
    (h, v)
}

fn calculate_adjusted_position(commands: &Vec<Command>) -> (u32, u32) {
    let (mut aim, mut h, mut v) = (0, 0, 0);
    for command in commands {
        match command {
            Command::Forward(x) => {
                h += x;
                v += aim * x;
            }
            Command::Down(x) => {
                aim += x;
            }
            Command::Up(x) => {
                aim -= x;
            }
        }
    }
    (h, v)
}
