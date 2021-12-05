#![feature(test)]

fn main() {
    let mut input = parse_input();

    let reading_increases = count_increases(&input);
    println!("Reading Increases:\n{}", reading_increases);

    let window_increases = count_window_increases(&mut input);
    println!("Window Increases:\n{}", window_increases);
}

fn parse_input() -> Vec<u32> {
    include_str!("../input.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn count_increases(input: &Vec<u32>) -> u32 {
    let mut reading_increases = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            reading_increases += 1;
        }
    }
    reading_increases
}

fn calculate_cumulative_sums(input: &mut Vec<u32>) -> &mut Vec<u32> {
    input.iter_mut().fold(0, |acc, x| {
        *x += acc;
        *x
    });
    input
}

fn count_window_increases(input: &mut Vec<u32>) -> u32 {
    let cumulative_sums = calculate_cumulative_sums(input);
    let windows: Vec<u32> = cumulative_sums
        .iter()
        .skip(3)
        .zip(cumulative_sums.iter())
        .map(|(s0, s1)| s0 - s1)
        .collect();
    count_increases(&windows)
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Uniform, Rng};
    use test::Bencher;

    #[bench]
    fn bench_calculate_cumulative_sums(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(100, 5000);
        let mut input: Vec<u32> = (0..1000).map(|_| rng.sample(&range)).collect();

        b.iter(|| {
            calculate_cumulative_sums(&mut input);
        });
    }

    #[bench]
    fn bench_count_increases(b: &mut Bencher) {
        let input = parse_input();
        b.iter(|| count_increases(&input));
    }

    #[bench]
    fn bench_count_window_increases(b: &mut Bencher) {
        let input = parse_input();
        b.iter(|| {
            let mut input = input.to_vec();
            count_window_increases(&mut input)
        });
    }
}
