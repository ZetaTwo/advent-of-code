use std::collections::HashSet;

pub fn find_term<'a, I>(vals: I, target: u32) -> Option<&'a u32>
where
    I: IntoIterator<Item = &'a u32>,
{
    let mut seen = HashSet::<u32>::new();
    for item in vals.into_iter() {
        let rem = target - item;
        if seen.contains(&rem) {
            return Some(item);
        }
        seen.insert(*item);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    #[test]
    fn test_module_fuel_req() {
        assert_eq!(find_term(&[1, 2, 3], 10), None);
        assert_eq!(find_term(&[1721, 979, 366, 299, 675, 1456], 2020), Some(&299u32));
    }

    #[test]
    fn test_day01a() {
        let file = File::open("../input/1.in").expect("Failed to open test data");
        let reader = BufReader::new(file);

        let input = reader
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.parse::<u32>())
            .filter_map(Result::ok);

        let target = 2020u32;
        let term = find_term(input, target).expect("No pair found");
        let other = target - term;
        let product = term*other;

        assert_eq!(product, 471019);
    }

    /*#[test]
    fn test_day01b() {
        let file = File::open("../input/1.in").expect("Failed to open test data");
        let reader = BufReader::new(file);

        let total_weight: u32 = reader
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.parse::<u32>())
            .filter_map(Result::ok)
            .map(module_total_fuel_req)
            .sum();

        assert_eq!(total_weight, 4985145);
    }*/
}
