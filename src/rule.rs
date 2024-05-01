use super::token::Token;

pub mod block_quote;
pub mod code_block;
pub mod heading;
pub mod html_block;
pub mod list;
pub mod paragraph;
pub mod thematic_break;

//     Paragraph,
//     ThematicBreak,
//     ATXHeading(u8),
//     SetextHeading,
//     IndentedCodeBlock,
//     FencedCodeBlock(char, usize),
//     HTMLBlock(u8),
//     // LinkDefinitions, // LinkReferenceDefinitions,
//     BlockQuote,
//     ListItem,
//     List(char, usize, bool), // -, +, *, start, is_tight=true/false, type="ordered"/"bullet", delimiter="paren"/"period"

pub struct BlockRuleManager {
    pub rules: Vec<Box<dyn BlockRule>>,
}

impl BlockRuleManager {
    // fn new(rules: Vec<Box<dyn BlockRule>>) -> Self {
    //     let mut manager = BlockRuleManager { rules };
    pub fn new() -> Self {
        let mut manager = BlockRuleManager { rules: Vec::new() };
        manager
    }

    pub fn add(&mut self, rule: Box<dyn BlockRule>) {
        self.rules.push(rule);
    }

    pub fn last_rule(self) -> Option<Box<dyn BlockRule>> {
        self.rules.last()
    }

    pub fn remove_last(&mut self) {
        self.rules.pop();
    }

    pub fn first_rule(self) -> Option<Box<dyn BlockRule>> {
        self.rules.first()
    }

    pub fn remove_first_rule(&mut self) {
        if self.rules.get(0) {
            self.rules.remove(0)
        }
    }

    pub fn clear(&mut self) {
        self.rules.clear();
    }

    pub fn truncate_from(&mut self, position: usize) {
        if position < self.rules.len() {
            self.rules.truncate(position);
        }
    }
}

pub trait BlockRule {
    // fn new() -> Self;

    fn rule_type() -> String;
    // Method to determine the start condition of a block
    // current__column: Columns being processed in the line
    fn should_start(&self, text: &String, current_column: usize) -> bool;

    // Method to determine the continuation condition of a block
    fn should_continue(&self, text: &String, current_column: usize) -> bool;

    // Method to handle the start of the block
    fn handle_start(&mut self, text: &String, current_column: usize) -> Box<dyn Token>; // -> Vec<Box<dyn Token>>;

    // Method to handle the continuation of the block
    // Returns an optional string representing any additional text processed
    // or None if no additional text is processed
    fn handle_continuation(&mut self, text: &String, current_column: usize) -> Option<String>;

    // Method to handle the end of the block
    // fn finalize(&mut self);

    // Method to determine whether current token should be transformed into a different type of token.
    fn should_transform(&self, text: &String, current_column: usize) -> bool;

    // Method to convert the current token to a different type of token.
    fn handle_transform(&mut self, text: &String, current_column: usize) -> Box<dyn Token>;
}

pub fn apply_start_rule(
    block_rules: Vec<Box<dyn BlockRule>>,
    text: String,
    mut current_column: usize,
) {
    // if last_rule === ParagraphRule {
    //     // indented code block cannot interrupt a paragraph
    //     // container block
    // }

    // if IndentedCodeBlockState.should_start() {
    //     return IndentedCodeBlockState.start(text);
    // }

    if let Some(rule) = find_applicable_rule(&block_rules, &text, current_column) {
        rule.handle_start(&text, current_column);
        // if rule.should_continue(line) {
        //     rule.handle_continuation(line);
        // }
    }

    // Paragraph
    // ParagraphToken { text }
}

pub fn find_applicable_rule(
    block_rules: Vec<Box<dyn BlockRule>>,
    text: &String,
    current_column: usize,
) -> Option<Box<dyn BlockRule>> {
    for rule in block_rules {
        if rule.should_start(text, current_column) {
            return Some(rule);
        }
    }
    None
}

pub fn apply_continuance_rule(text: String) {}
