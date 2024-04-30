use super::BlockRule;
use crate::mdir::{
    token::{HeadingToken, Token},
    util::{count_indentation, is_blank_line},
};

pub struct AtxCodeBlockRule;

impl BlockRule for AtxCodeBlockRule {
    fn rule_type() -> String {
        format!("AtxCodeBlockRule")
    }

    fn should_start(&self, text: &String, current_column: usize) -> bool {
        if count_indentation(text, current_column) < 4 {
            let trimmed = text.trim_start();
            if let Some(first_char) = trimmed.chars().next() {
                let num_hashes = trimmed.chars().take_while(|&c| c == '#').count();
                if num_hashes > 0 && num_hashes <= 6 {
                    if trimmed.len() == num_hashes {
                        return true;
                    } else if first_char == ' ' || first_char == '\t' {
                        return true;
                    }
                }
            }
            false
        } else {
            false
        }
    }

    fn should_continue(&self, text: &String, current_column: usize) -> bool {
        !is_blank_line(text) && count_indentation(text, current_column) > 4
    }

    fn handle_start(&mut self, text: &String, current_column: usize) -> Box<dyn Token> {
        Box::new(HeadingToken::new(text, true))
    }

    fn handle_continuation(&mut self, text: &String, current_column: usize) -> Option<String> {
        None
    }
}
