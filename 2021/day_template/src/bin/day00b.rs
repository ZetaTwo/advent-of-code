fn main() {
    let input = utils::get_parsed_lines_stdin::<u32>();

    match input {
        Err(e) => println!("Failed reading input: {:?}", e),
        Ok(values) => {
            println!("Test: {}", day00::add(values[0], values[1]));
        }
    }
}
