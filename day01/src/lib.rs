pub fn module_fuel_req(weight: u32) -> u32 {
    let weight_third = weight / 3;
    if weight_third >= 2 {
        weight_third - 2
    } else {
        0
    }
}

pub fn module_total_fuel_req(part_weight: u32) -> u32 {
    let fuel_parts = std::iter::successors(Some(part_weight), |weight| {
        if weight > &0 {
            Some(module_fuel_req(*weight))
        } else {
            None
        }
    });

    let total_fuel_weight: u32 = fuel_parts.skip(1).sum();
    total_fuel_weight
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    #[test]
    fn test_module_fuel_req() {
        assert_eq!(module_fuel_req(12), 2);
        assert_eq!(module_fuel_req(14), 2);
        assert_eq!(module_fuel_req(1969), 654);
        assert_eq!(module_fuel_req(100756), 33583);
    }

    #[test]
    fn test_module_fuel_req2() {
        assert_eq!(module_total_fuel_req(14), 2);
        assert_eq!(module_total_fuel_req(1969), 966);
        assert_eq!(module_total_fuel_req(100756), 50346);
    }

    #[test]
    fn test_day01a() {
        let file = File::open("../input/1.in").expect("Failed to open test data");
        let reader = BufReader::new(file);

        let total_weight: u32 = reader
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.parse::<u32>())
            .filter_map(Result::ok)
            .map(module_fuel_req)
            .sum();

        assert_eq!(total_weight, 3325347);
    }

    #[test]
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
    }
}
