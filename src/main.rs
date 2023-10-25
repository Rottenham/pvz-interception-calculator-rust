#[cfg(feature = "en")]
use pvz_interception_calculator::lang::en::*;

#[cfg(feature = "zh")]
use pvz_interception_calculator::lang::zh::*;

fn main() -> rustyline::Result<()> {
    let mut rustyline = rustyline::DefaultEditor::new()?;
    let mut parser = pvz_interception_calculator::parser::Parser::default();
    loop {
        match rustyline.readline("\n$ ") {
            Ok(line) => {
                rustyline.add_history_entry(line.as_str()).unwrap();
                let input = line.trim().to_lowercase();
                use pvz_interception_calculator::parser::ParseResult;
                if let ParseResult::Matched = parser.parse_scene(input.as_str()) {
                    continue;
                }
                if let ParseResult::Matched = parser.parse_wave(input.as_str()) {
                    continue;
                }
                if let ParseResult::Matched = parser.parse_delay(input.as_str()) {
                    continue;
                }
                if let ParseResult::Matched = parser.parse_doom(input.as_str()) {
                    continue;
                }
                if let ParseResult::Matched = parser.parse_hit_or_nohit(input.as_str()) {
                    continue;
                }
                if let ParseResult::Matched = parser.parse_find_max_delay(input.as_str()) {
                    continue;
                }
                if let ParseResult::Matched = parser.parse_garg_x_range_of_imp_x(input.as_str()) {
                    continue;
                }
                if let ParseResult::Matched = parser.parse_about(input.as_str()) {
                    continue;
                }
                if let ParseResult::Matched = parser.parse_help(input.as_str()) {
                    continue;
                }
                println!("{UNKNOWN_COMMAND}");
            }
            Err(rustyline::error::ReadlineError::Interrupted)
            | Err(rustyline::error::ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("{ERROR}: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
