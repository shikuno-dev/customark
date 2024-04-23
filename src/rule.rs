pub trait BlockRule {
    // Method to determine the start condition of a block
    fn should_start(&self, text: &str) -> bool;

    // Method to determine the continuation condition of a block
    fn should_continue(&self, text: &str) -> bool;

    // Method to handle the start of the block
    fn handle_start(&mut self, text: &str);

    // Method to handle the continuation of the block
    // Returns an optional string representing any additional text processed
    // or None if no additional text is processed
    fn handle_continuation(&mut self, text: &str) -> Option<String>;

    // Method to handle the end of the block
    fn finalize(&mut self);
}

pub fn is_blank_line(line: &str) -> bool {
    line.chars()
        .all(|c| c == ' ' || c == '\t' || c == '\r' || c == '\n')
}
pub fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

pub fn calculate_spaces_until_next_tab_stop(position: usize) -> usize {
    let tab_stop_size = 4;
    tab_stop_size - (position % tab_stop_size)
}

pub fn trim_end_blank_line(&text: String) -> String {
    text.trim_end_matches(|c: char| c == ' ' || c == '\t' || c == '\n')
}
fn process_indentation(mut line: String, column: usize, required_indent: usize) -> String {
    if required_indent == 0 {
        if let Some(position) = line.find('\t') {
            if position < 3 && line[0..position].chars().all(|c| c == ' ') {
                let spaces = calculate_spaces_until_next_tab_stop(position);
                // let spaces = calculate_spaces_until_next_tab_stop(position + column);
                let mut new_line = " ".repeat(spaces + position);
                new_line.push_str(&line[position + 1..]);
                return new_line;
            }
        } else {
            return line;
        }
    } else {
        // list
        if required_indent > 4 {
            //
        }
        return line;
    }
    line
}

pub trait Token {
    fn is_leaf(&self) -> bool;
}

pub struct BlockState {
    pub stack: Vec<dyn BlockStateType>, // open blocks
}

pub fn apply_start_rule(
    text: String,
    // state_type: BlockStateType,
    block_state_types: Vec<dyn BlockStateType>,
    // column: usize,
) -> Token {
    // let column: usize = 0;
    // let mut tab_expanded_text = process_indentation(text, column, 0);

    if IndentedCodeBlockState.should_start() {
        return IndentedCodeBlockState.start(text);
    }

    if let Some(starting_state) = block_state_types
        .iter()
        .find(|state| state.should_start(text, ParagraphState))
    {
        return starting_state.start(text);
    }

    // Paragraph
    ParagraphToken { text }
}

pub fn apply_continuance_rule() {}

pub trait BlockRule {
    // Method to determine the start condition of a block
    fn should_start(&self, text: &str) -> bool;

    // Method to determine the continuation condition of a block
    fn should_continue(&self, text: &str) -> bool;

    // Method to handle the start of the block
    fn handle_start(&mut self, text: &str);

    // Method to handle the continuation of the block
    // Returns an optional string representing any additional text processed
    // or None if no additional text is processed
    fn handle_continuation(&mut self, text: &str) -> Option<String>;

    // Method to handle the end of the block
    fn finalize(&mut self);
}

struct ThematicBreakState;
struct ParagraphState;
impl BlockStateType for ParagraphState {
    fn should_start(&self, text: String) {
        true
    }

    fn should_continue(&self, text: String) {
        if is_blank_line(&text) {
            false
        } else {
            true
        }
    }

    fn start(&self, text: String) {}
}

#[derive(Debug)]
pub struct ParagraphToken {
    pub text: String,
}
impl Token for ParagraphToken {
    fn is_leaf(&self) -> bool {
        true
    }
}

impl ParagraphToken {
    pub fn new(text: String) -> Self {
        ParagraphToken { text }
    }
    pub fn set_inline_text(&mut self, new_text: String) {
        self.text = new_text;
    }
}

struct IndentedCodeBlockState;
impl BlockStateType for IndentedCodeBlockState {
    fn should_start(&self, text: String) {
        true
    }

    fn should_continue(&self, text: String) {
        if is_blank_line(&text) {
            false
        } else {
            true
        }
    }

    fn start(&self, text: String) {}
}

#[derive(Debug)]
pub struct CodeBlockToken {
    pub text: String,
}
impl Token for CodeBlockToken {
    fn is_leaf(&self) -> bool {
        true
    }
}

impl CodeBlockToken {
    pub fn new(text: String) -> Self {
        CodeBlockToken { text }
    }
    pub fn set_inline_text(&mut self, new_text: String) {
        self.text = new_text;
    }
    pub fn finalize(&self) {
        self.text = trim_end_blank_line(self.text);
    }
}

// struct ThematicBreakgRule;
