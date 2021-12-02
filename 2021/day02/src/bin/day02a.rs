use day02::ApplyCommand;

fn main() {
    let input = utils::get_parsed_lines_stdin::<day02::Command>();

    match input {
        Err(e) => println!("Failed reading input: {:?}", e),
        Ok(commands) => {
            let res = day02::State::run_commands(&commands);
            println!("Depth * Horizontal = {}", res.depth * res.horizontal);
        }
    }
}
