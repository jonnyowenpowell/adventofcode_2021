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

    println!("Result:\n{}", reading_increases);
}
