#![feature(test)]

fn main() {
    let mut readings = parse_input();

    let reading_increases = count_increases(&readings);
    println!("Reading increases: {}", reading_increases);

    let window_increases = count_window_increases(&mut readings);
    println!("Window increases: {}", window_increases);
}

fn parse_input() -> Vec<u32> {
    include_str!("../input.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn count_increases(values: &Vec<u32>) -> u32 {
    let mut increases = 0;
    for i in 1..values.len() {
        if values[i] > values[i - 1] {
            increases += 1;
        }
    }
    increases
}

fn calculate_cumulative_sums(values: &mut Vec<u32>) -> &mut Vec<u32> {
    values.iter_mut().fold(0, |acc, x| {
        *x += acc;
        *x
    });
    values
}

fn count_window_increases(readings: &mut Vec<u32>) -> u32 {
    let cumulative_sums = calculate_cumulative_sums(readings);
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
