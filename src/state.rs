#[derive(PartialEq)]
pub enum BlockStateType {
    Paragraph,
    ThematicBreak,
    ATXHeading(u8),
    SetextHeading,
    IndentedCodeBlock,
    FencedCodeBlock(char, usize),
    HTMLBlock(u8),
    // LinkDefinitions, // LinkReferenceDefinitions,
    BlockQuote,
    ListItem,
    List(char, usize, bool), // -, +, *, start, is_tight=true/false, type="ordered"/"bullet", delimiter="paren"/"period"
}

fn main() {
    let states: Vec<Box<dyn BlockState>> = vec![ParagraphState, ThematicBreakState];

    let text = String::new();
    if let Some(starting_state) = states
        .iter()
        .find(|state| state.should_start(text, ParagraphState))
    {
        starting_state.start(text);
    } else {
    }
}

pub trait BlockState {
    //  Determine starting conditions
    fn should_start(&self, text: String, state_type: BlockStateType) -> bool {}

    // determine if the condition will continue
    fn should_continue(&self, text: String, state_type: BlockStateType) -> bool {}

    fn start(&mut self, text: String);
    // fn finalize(&mut self);

    // fn continue_state(&mut self, text: String) {
    //     if self.should_continue(text) {
    //         self.start(text);
    //     } else {
    //         self.finalize();
    //     }
    // }
}

// Paragraph,
// ThematicBreak,
// ATXHeading(u8),
// SetextHeading,
// IndentedCodeBlock,
// FencedCodeBlock(char, usize),
// HTMLBlock(u8),
// // LinkDefinitions, // LinkReferenceDefinitions,
// BlockQuote,
// ListItem,
// List(char),

struct ParagraphState;
impl BlockState for ParagraphState {
    fn should_start(&self, text: String, state_type: BlockStateType) {
        false
    }

    fn should_continue(&self, text: String, state_type: BlockStateType) {
        false
    }

    fn start(&self, text: String) {}
}

#[derive(Debug)]
pub struct ParagraphToken {
    pub text: String,
}

impl ParagraphToken {
    pub fn new(text: String) -> Self {
        ParagraphToken { text }
    }
    pub fn set_inline_text(&mut self, new_text: String) {
        self.text = new_text;
    }
}

struct ThematicBreakState;
impl BlockState for ThematicBreakState {
    fn should_start(&self, text: String, state_type: BlockStateType) {
        false
    }

    fn should_continue(&self, text: String, state_type: BlockStateType) {
        false
    }

    fn start(&self, text: String) {}
}

#[derive(Debug)]
struct ThematicBreakToken;
impl ThematicBreakToken {
    pub fn new() -> Self {
        ThematicBreakToken
    }
}
