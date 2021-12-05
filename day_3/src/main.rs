fn main() {
    let (numbers, bit_length) = parse_input();

    let power_consumption = calculate_power_consumption(&numbers, bit_length);
    println!("Power consumption: {}", power_consumption);

    let life_support_rating = calculate_life_support_rating(&numbers, bit_length);
    println!("Life Support rating: {}", life_support_rating);
}

fn parse_input() -> (Vec<u32>, u32) {
    let input = include_str!("../input.txt");

    let bit_length = input.lines().next().unwrap().len() as u32;
    (
        input
            .lines()
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect(),
        bit_length,
    )
}

fn count_non_zero_masked(input: &Vec<u32>, mask: u32) -> u32 {
    input
        .iter()
        .fold(0, |c, n| if n & mask > 0 { c + 1 } else { c })
}

fn calculate_power_consumption(numbers: &Vec<u32>, bit_length: u32) -> u32 {
    let number_count = numbers.len() as u32;

    let (mut gamma, mut epsilon) = (0, 0);
    for i in 0..bit_length {
        let mask = 2_u32.pow(i);
        let is_bit_set = (2 * count_non_zero_masked(&numbers, mask)) > number_count;
        if is_bit_set {
            gamma += mask;
        } else {
            epsilon += mask;
        }
    }
    gamma * epsilon
}

fn select_subsystem_rating(
    numbers: &Vec<u32>,
    bit_length: u32,
    bit_condition: fn(bool, u32) -> u32,
) -> u32 {
    let mut candidates = numbers.to_vec();
    let mut rating: Option<u32> = Option::None;
    for i in 0..bit_length {
        let mask = 2_u32.pow(bit_length - (i + 1));
        let is_bit_commonly_set =
            2 * count_non_zero_masked(&candidates, mask) >= candidates.len() as u32;
        candidates = candidates
            .into_iter()
            .filter(|&n| n & mask == bit_condition(is_bit_commonly_set, mask))
            .collect();
        if candidates.len() == 1 {
            rating = Some(*candidates.first().unwrap());
            break;
        }
    }
    rating.expect("subsystem rating not found")
}

fn calculate_life_support_rating(numbers: &Vec<u32>, bit_length: u32) -> u32 {
    let oxygen_rating =
        select_subsystem_rating(
            &numbers,
            bit_length,
            |is_set, mask| if is_set { mask } else { 0 },
        );

    let co2_rating = select_subsystem_rating(
        &numbers,
        bit_length,
        |is_set, mask| if is_set { 0 } else { mask },
    );

    oxygen_rating * co2_rating
}
