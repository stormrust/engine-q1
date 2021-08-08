use std::{cell::RefCell, rc::Rc};

use crate::{ParseError, ParserState};

#[derive(Debug)]
pub enum LineResult {
    Success(String),
    Error(String, ParseError),
}

fn chomp_newline(s: &str) -> &str {
    if let Some(s) = s.strip_suffix('\n') {
        s
    } else {
        s
    }
}

pub fn process_script(parser_state: &Rc<RefCell<ParserState>>, script_text: &str) -> LineResult {
    if script_text.trim() == "" {
        LineResult::Success(script_text.to_string());
    } else {
        let line = chomp_newline(script_text);
        /*
                if let Some(failure) = err {
                    return LineResult::Error(line.to_string(), failure.into());
                }
        */
    }
    LineResult::Success("ok done with process_script".to_string())
}
