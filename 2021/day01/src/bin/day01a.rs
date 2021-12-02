fn main() {
    let input = utils::get_parsed_lines_stdin::<u32>();

    match input {
        Err(e) => println!("Failed reading input: {:?}", e),
        Ok(values) => {
            let num_increases = day01::count_increases(&values);
            println!("Number of increases {}", num_increases)
        }
    }
}
