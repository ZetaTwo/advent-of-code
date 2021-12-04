pub fn count_bits<'a, I, J, const N: usize>(entries: I) -> [u32; N]
where
    I: IntoIterator<Item = &'a J>,
    J: AsRef<str> + 'a,
{
    let mut vals = [0; N];
    for bits in entries.into_iter() {
        for (i, bit) in bits.as_ref().chars().enumerate() {
            if bit == '1' {
                vals[i] += 1;
            }
        }
    }
    vals
}

pub fn calculate_power<const N: usize>(counts: &[u32; N], total: usize) -> u32 {
    let mut gamma = 0;
    for (i, count) in counts.iter().enumerate().take(N) {
        if *count > total as u32 - *count {
            gamma |= 1 << (N - i - 1);
        }
    }
    let epsilon = gamma ^ ((1 << N) - 1);
    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn test_count_bits() {
        assert_eq!(
            count_bits::<_, _, 5>(&vec!["11111", "00000", "11100"]),
            [2, 2, 2, 1, 1]
        );
        assert_eq!(
            count_bits::<_, _, 5>(&vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010"
            ]),
            [7, 5, 8, 7, 5]
        );
    }

    #[test]
    fn test_power() {
        let bits = &vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let counts = count_bits::<_, _, 5>(bits);
        let power = calculate_power::<5>(&counts, bits.len());
        assert_eq!(power, 198);
    }

    #[test]
    fn test_day00a() {
        let file = File::open("../input/3.in").expect("Failed to open test data");
        let reader = BufReader::new(file);
        let input = utils::get_parsed_lines::<String, _>(reader).expect("Failed to read test data");

        const N: usize = 12;
        let res = count_bits::<_, _, N>(&input);
        let power = calculate_power::<N>(&res, input.len());

        assert_eq!(power, 3882564);
    }

    #[test]
    fn test_day00b() {
        let file = File::open("../input/3.in").expect("Failed to open test data");
        let reader = BufReader::new(file);
        let _input = utils::get_parsed_lines::<String, _>(reader).expect("Failed to read test data");

        let res = 2; //add(input[0], 1);

        assert_eq!(0 * res, 0);
    }
}
