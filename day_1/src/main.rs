fn main() {
    let input = include_str!("../input.txt");
    let readings: Vec<i32> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    println!("Input:\n{:?}", readings);
}
