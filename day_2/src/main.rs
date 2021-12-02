#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn main() {
    let input = include_str!("../input.txt");

    let commands: Vec<_> = input.split("\n").filter(|l| !l.is_empty()).map(|l| {
        let parts: Vec<_> = l.split_whitespace().collect();
        let value: i32 = parts[1].parse().unwrap();
        match parts[0] {
            "forward" => Command::Forward(value),
            "down" => Command::Down(value),
            "up" => Command::Up(value),
            _ => panic!("unrecognised command")
        }
    }).collect();

    let (mut h, mut v) = (0, 0);

    for command in &commands {
        match command {
            Command::Forward(x) => { h += x; },
            Command::Down(x) => { v += x; },
            Command::Up(x) => { v -= x; },
        }
    }

    println!("Resulting position: ({}, {})", h, v);
}
