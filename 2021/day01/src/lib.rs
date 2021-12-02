use itermore::IterMore;

pub fn count_increases<T, I>(values: &I) -> usize
where
    T: Ord + Clone,
    for<'a> &'a I: IntoIterator<Item = &'a T>,
{
    values
        .into_iter()
        .windows::<2>()
        .filter(|vals| vals[1] > vals[0])
        .count()
}

pub fn count_increases4<T, I>(values: &I) -> usize
where
    T: Ord + Clone,
    for<'a> &'a I: IntoIterator<Item = &'a T>,
{
    values
        .into_iter()
        .windows::<4>()
        .filter(|vals| vals[3] > vals[0])
        .count()
}

pub fn sum_windows3<T, I>(values: &I) -> Vec<T>
where
    for<'a> T: Copy + std::iter::Sum<&'a T>,
    for<'a> &'a I: IntoIterator<Item = &'a T>,
{
    sum_windows::<T, I, 3>(values)
}

pub fn sum_windows<T, I, const N: usize>(values: &I) -> Vec<T>
where
    for<'a> T: Copy + std::iter::Sum<&'a T>,
    for<'a> &'a I: IntoIterator<Item = &'a T>,
{
    values
        .into_iter()
        .windows::<N>()
        .map(|vals| vals.iter().copied().sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn test_count_increases() {
        assert_eq!(
            count_increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }

    #[test]
    fn test_sum_windows() {
        assert_eq!(sum_windows3(&[1, 2, 3, 4]), &[6, 9]);
    }

    #[test]
    fn test_count_sum_windows() {
        assert_eq!(
            count_increases(&sum_windows3(&[
                199, 200, 208, 210, 200, 207, 240, 269, 260, 263
            ])),
            5
        );
    }

    #[test]
    fn test_count_increases4() {
        assert_eq!(
            count_increases4(&sum_windows3(&[
                199, 200, 208, 210, 200, 207, 240, 269, 260, 263
            ])),
            5
        );
    }

    #[test]
    fn test_day01a() {
        let file = File::open("../input/1.in").expect("Failed to open test data");
        let reader = BufReader::new(file);
        let input = utils::get_parsed_lines::<u32, _>(reader).expect("Failed to read test data");
        
        let num_increases = count_increases(&input);

        assert_eq!(num_increases, 1288);
    }

    #[test]
    fn test_day01b() {
        let file = File::open("../input/1.in").expect("Failed to open test data");
        let reader = BufReader::new(file);
        let input = utils::get_parsed_lines::<u32, _>(reader).expect("Failed to read test data");
        
        let windows = sum_windows3(&input);
        let num_increases = count_increases(&windows);

        let num_increases2 = count_increases4(&input);

        assert_eq!(num_increases, 1311);
        assert_eq!(num_increases2, 1311);
    }
}
