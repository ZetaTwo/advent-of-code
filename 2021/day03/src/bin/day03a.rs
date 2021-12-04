fn main() {
    let input = utils::get_parsed_lines_stdin::<String>();

    match input {
        Err(e) => println!("Failed reading input: {:?}", e),
        Ok(values) => {
            const N: usize = 12;
            let res = day03::count_bits::<_, _, N>(&values);
            let power = day03::calculate_power::<N>(&res, values.len());

            println!("Power: {}", power);
        }
    }
}
