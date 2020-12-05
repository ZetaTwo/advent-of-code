pub fn add(a: i32, b: i32) -> i32 {
    internal_add(a, b)
}

fn internal_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_add() {
        assert_eq!(internal_add(2, 2), 4);
    }
}
