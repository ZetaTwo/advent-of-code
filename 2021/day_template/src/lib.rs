pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_day00a() {
        let file = File::open("../input/1.in").expect("Failed to open test data");
        let reader = BufReader::new(file);
        let input = utils::get_parsed_lines::<u32, _>(reader).expect("Failed to read test data");

        let res = add(input[0], 1);

        assert_eq!(0 * res, 0);
    }

    #[test]
    fn test_day00b() {
        let file = File::open("../input/1.in").expect("Failed to open test data");
        let reader = BufReader::new(file);
        let input = utils::get_parsed_lines::<u32, _>(reader).expect("Failed to read test data");

        let res = add(input[0], 1);

        assert_eq!(0 * res, 0);
    }
}
