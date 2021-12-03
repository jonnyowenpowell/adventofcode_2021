fn main() {
    let input = include_str!("../input.txt");

    let bit_length = input.lines().next().unwrap().len() as u32;
    let input_numbers: Vec<_> = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect();
    let number_count = input_numbers.len();

    let (mut gamma, mut epsilon) = (0, 0);
    for i in 0..bit_length {
        let is_gamma_bit_set =
            input_numbers
                .iter()
                .fold(0, |c, n| if n & 2_u32.pow(i) > 0 { c + 1 } else { c })
                > number_count / 2;
        let inc = 2_u32.pow(i);
        if is_gamma_bit_set {
            gamma += inc;
        } else {
            epsilon += inc;
        }
    }

    println!("Gamma: {:b}", gamma);
    println!("Epsilon: {:b}", epsilon);
    println!("Power Consumption: {}", gamma * epsilon);

    let mut oxygen_rating_candidates = input_numbers.to_vec();
    let mut oxygen_rating: Option<u32> = Option::None;
    for i in 0..bit_length {
        let mask = 2_u32.pow(bit_length - (i + 1));
        let is_bit_commonly_set =
            oxygen_rating_candidates
                .iter()
                .fold(0, |c, n| if n & mask > 0 { c + 1 } else { c }) as f32
                >= oxygen_rating_candidates.len() as f32 / 2.0;
        oxygen_rating_candidates = oxygen_rating_candidates
            .into_iter()
            .filter(|&n| n & mask == if is_bit_commonly_set { mask } else { 0 })
            .collect();
        if oxygen_rating_candidates.len() == 1 {
            oxygen_rating = Some(*oxygen_rating_candidates.first().unwrap());
            break;
        }
    }

    let mut co2_rating_candidates = input_numbers.to_vec();
    let mut co2_rating: Option<u32> = Option::None;
    for i in 0..bit_length {
        let mask = 2_u32.pow(bit_length - (i + 1));
        let is_bit_uncommonly_set =
            (co2_rating_candidates
                .iter()
                .fold(0, |c, n| if n & mask > 0 { c + 1 } else { c }) as f32)
                < (co2_rating_candidates.len() as f32 / 2.0);
        co2_rating_candidates = co2_rating_candidates
            .into_iter()
            .filter(|&n| n & mask == if is_bit_uncommonly_set { mask } else { 0 })
            .collect();
        if co2_rating_candidates.len() == 1 {
            co2_rating = Some(*co2_rating_candidates.first().unwrap());
            break;
        }
    }

    println!("Oxygen Generator Rating: {:b}", oxygen_rating.unwrap());
    println!("CO2 Scrubber Rating: {:b}", co2_rating.unwrap());
    println!(
        "Life Support Rating: {}",
        oxygen_rating.unwrap() * co2_rating.unwrap()
    );
}
