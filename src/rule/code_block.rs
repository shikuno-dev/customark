use super::BlockRule;
use crate::mdir::{
    token::{CodeBlockToken, Token},
    util::{count_indentation, is_blank_line},
};

// > Example 107: An indented code block cannot interrupt a paragraph,
// >              so there must be a blank line between a paragraph and a following indented code block.

pub struct IndentedCodeBlockRule;

impl BlockRule for IndentedCodeBlockRule {
    fn rule_type() -> String {
        format!("IndentedCodeBlock")
    }

    fn should_start(&self, text: &String, current_column: usize) -> bool {
        if count_indentation(text, current_column) > 4 {
            !is_blank_line(text)
        } else {
            false
        }
    }

    fn should_continue(&self, text: &String, current_column: usize) -> bool {
        !is_blank_line(text) && count_indentation(text, current_column) > 4
    }

    fn handle_start(&mut self, text: &String, current_column: usize) -> Box<dyn Token> {
        Box::new(CodeBlockToken::new(text, true))
    }

    fn handle_continuation(&mut self, text: &String, current_column: usize) -> Option<String> {
        None
    }
}

pub struct FencedCodeBlockRule;

impl BlockRule for FencedCodeBlockRule {
    fn rule_type() -> String {
        format!("FencedCodeBlock")
    }

    fn should_start(text: &String, current_column: usize) -> bool {
        if count_indentation(text, current_column) > 4 {
            !is_blank_line(text)
        } else {
            false
        }
    }

    fn should_continue(text: &String, current_column: usize) -> bool {
        !is_blank_line(text) && count_indentation(text, current_column) > 4
    }

    fn handle_start(&mut self, text: &String, current_column: usize) -> Box<dyn Token> {
        Box::new(CodeBlockToken::new(text, false))
    }

    fn handle_continuation(&mut self, text: &String, current_column: usize) -> Option<String> {
        None
    }
}
