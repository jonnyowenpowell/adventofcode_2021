fn main() {
    let input = include_str!("../input.txt");
    let readings: Vec<i32> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    let mut reading_increases = 0;
    for i in 1..readings.len() {
        if readings[i] > readings[i - 1] {
            reading_increases += 1;
        }
    }

    println!("Reading Increases:\n{}", reading_increases);

    let mut window_increases = 0;
    for i in 0..(readings.len() - 3) {
        let window_a: i32 = readings[i..i + 3].iter().sum();
        let window_b: i32 = readings[i + 1..i + 4].iter().sum();
        if window_a < window_b {
            window_increases += 1;
        }
    }

    println!("Window Increases:\n{}", window_increases);
}
