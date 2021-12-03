fn main() {
    let input = include_str!("../input.txt");
    let lines: Vec<_> = input.split("\n").filter(|l| !l.is_empty()).collect();

    let bit_length = lines[0].len();
    let gamma_bit_counts = vec![0 as isize; bit_length];

    let gamma_bit_counts = lines.iter().fold(gamma_bit_counts, |bits, l| {
        let count_delta: Vec<isize> = l
            .as_bytes()
            .iter()
            .map(|c| if c == &('0' as u8) { -1 } else { 1 })
            .collect();

        bits.iter()
            .enumerate()
            .map(|(i, c)| c + count_delta[i])
            .collect()
    });

    println!("Bit counts:\n{:?}", gamma_bit_counts);
}
