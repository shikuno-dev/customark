use super::BlockRule;
use crate::mdir::{
    token::{ParagraphToken, Token},
    util::{count_indentation, is_blank_line},
};

// > A sequence of non-blank lines that cannot be interpreted as other kinds of blocks forms a paragraph.

// > Example 107: An indented code block cannot interrupt a paragraph,
// >              so there must be a blank line between a paragraph and a following indented code block.
pub struct ParagraphRule;

fn is_paragraph_rule(rule: &dyn BlockRule) -> bool {
    rule.downcast_ref::<ParagraphRule>().is_some()
}

impl BlockRule for ParagraphRule {
    fn rule_type() -> String {
        format!("Paragraph")
    }
    fn should_start(&self, text: &String, current_column: usize) -> bool {
        if count_indentation(text, current_column) < 4 {
            !is_blank_line(text)
        } else {
            false
        }
    }

    fn should_continue(&self, text: &String, current_column: usize) -> bool {
        !is_blank_line(text)
    }

    fn handle_start(&mut self, text: &String, current_column: usize) -> Box<dyn Token> {
        // vec![Box::new(ParagraphToken::new(text))]
        Box::new(ParagraphToken::new(text))
    }

    fn handle_continuation(&mut self, text: &String, current_column: usize) -> Option<String> {
        None
    }

    // fn finalize(&mut self) {}
}
