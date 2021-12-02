use itermore::IterMore;

pub fn count_increases<T>(values: impl IntoIterator<Item = T>) -> usize
where
    T: Ord + Clone,
{
    values
        .into_iter()
        .windows::<2>()
        .filter(|vals| vals[1] > vals[0])
        .count()
}

pub fn sum_windows3<'a, T: 'a, I>(values: I) -> Vec<T>
where
    //T: std::iter::Sum<&'a T> + Clone + Copy,
    T: std::iter::Sum + Clone + Copy,
    I: IntoIterator<Item = T>,
{
    sum_windows::<'a, T, I, 3>(values)
}

pub fn sum_windows<'a, T: 'a, I, const N: usize>(values: I) -> Vec<T>
where
    //T: std::iter::Sum<&'a T> + Clone + Copy,
    T: std::iter::Sum + Clone + Copy,
    I: IntoIterator<Item = T>,
{
    values
        .into_iter()
        .windows::<N>()
        .map(|vals| vals.iter().cloned().sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    #[test]
    fn test_count_increases() {
        assert_eq!(
            count_increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }

    #[test]
    fn test_sum_windows() {
        assert_eq!(sum_windows3(vec![1, 2, 3, 4]), &[6, 9]);
    }

    #[test]
    fn test_count_sum_windows() {
        assert_eq!(
            count_increases(sum_windows3(vec![
                199, 200, 208, 210, 200, 207, 240, 269, 260, 263
            ])),
            5
        );
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

        let num_increases = count_increases(input);

        assert_eq!(num_increases, 1288);
    }

    #[test]
    fn test_day01b() {
        let file = File::open("../input/1.in").expect("Failed to open test data");
        let reader = BufReader::new(file);

        let input = reader
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.parse::<u32>())
            .filter_map(Result::ok);

        let windows = sum_windows3(input);
        let num_increases = count_increases(windows);

        assert_eq!(num_increases, 1311);
    }
}
