use utils;

fn main() {
    let input = utils::get_integer_lines::<u32>();

    match input {
        Err(e) => println!("Failed reading input: {:?}", e),
        Ok(values) => {
            let total_weight: u32 = values.into_iter().map(day01::module_fuel_req).sum();
            println!("Total weight: {}", total_weight);
        }
    }
}
