use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use interception_calculator;
use interception_calculator::parser::ParseResult;

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut parser = interception_calculator::parser::Parser::new();
    loop {
        let readline = rl.readline("\n$ ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                let input = line.trim().to_lowercase();
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
                if let ParseResult::Matched = parser.parse_about(input.as_str()) {
                    continue;
                }
                if let ParseResult::Matched = parser.parse_help(input.as_str()) {
                    continue;
                }
                println!("未知指令. 输入问号查看帮助.");
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("出现错误: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
