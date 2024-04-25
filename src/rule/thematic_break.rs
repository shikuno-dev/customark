use super::BlockRule;
use crate::mdir::{
    token::{ThematicBreakToken, Token},
    util::count_indentation,
};

pub struct ThematicBreakRule;

impl BlockRule for ThematicBreakRule {
    fn rule_type() -> String {
        format!("ThematicBreak")
    }

    fn should_start(&self, text: &String, current_column: usize) -> bool {
        let thematic_chars = ['-', '_', '*'];
        let mut char_count = 0;

        if count_indentation(text, current_column) < 4 {
            let mut last_char: Option<char> = None;

            for c in text.chars() {
                if thematic_chars.contains(&c) {
                    char_count += 1;

                    if let Some(last) = last_char {
                        if last != c {
                            //  Returns false if different from the immediately preceding character
                            return false;
                        }
                    }

                    last_char = Some(c);
                } else if c != ' ' && c != '\t' {
                    return false;
                }
            }

            if char_count < 3 {
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    fn should_continue(&self, text: &String, current_column: usize) -> bool {
        false
    }

    fn handle_start(&mut self, text: &String, current_column: usize) -> Box<dyn Token> {
        // vec![Box::new(ThematicBreakToken::new())]
        Box::new(ThematicBreakToken::new())
    }

    fn handle_continuation(&mut self, text: &String, current_column: usize) -> Option<String> {
        None
    }

    // fn finalize(&mut self) {
    //     self.text = trim_end_blank_line(self.text);
    // }
}
