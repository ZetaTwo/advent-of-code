use utils;

fn main() {
    let input = utils::get_integer_lines::<u32>();

    match input {
        Err(e) => println!("Failed reading input: {:?}", e),
        Ok(values) => {
            match day01::find_term(&values, 2020) {
                Some(term) => {
                    let other = 2020 - term;
                    let product = term * other;
                    let sum = term + other;
                    println!("Found {} + {} = {}, {} * {} = {}", term, other, sum, term, other, product);
                }
                None => println!("Failed to find pair")
            }
        }
    }
}
