fn main() {
    let input = utils::get_parsed_lines_stdin::<u32>();

    match input {
        Err(e) => println!("Failed reading input: {:?}", e),
        Ok(_values) => {
            //println!("Test: {}", day03::add(values[0], values[1]));
        }
    }
}
