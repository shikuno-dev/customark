pub mod code_block;
pub mod hr;
pub mod list;

use crate::util::{
    tab::{calculate_spaces_until_next_tab_stop, expand_tabs},
    unicode::{is_blank_line, normalize_newlines},
};

fn count_indentation(markdown: &str) -> Vec<usize> {
    markdown
        .lines()
        .map(|line| line.chars().take_while(|&c| c == ' ').count())
        .collect()
}

#[derive(PartialEq)]
pub enum BlockStateType {
    Paragraph,
    ThematicBreak,
    ATXHeading,
    SetextHeading,
    IndentedCodeBlock,
    HTMLBlock,
    FencedCodeBlock,
    BlockQuote,
    ListItem,
    List,
}
struct BlockState {
    stack: Vec<BlockStateType>, // open blocks
}
impl BlockState {
    fn new() -> Self {
        BlockState { stack: Vec::new() }
    }

    fn open(&mut self, block_type: BlockStateType) {
        self.stack.push(block_type);
    }

    fn close(&mut self) {
        self.stack.pop();
    }

    fn current_block_state_type(&self) -> Option<&BlockStateType> {
        self.stack.last()
    }
}

enum TokenType {
    Paragraph,
    Headng,
    ThematicBreak,
    CodeBlock,
    HTMLBlock,
    BlockQuote,
    ListItem,
    List,
}

struct Token {}

pub fn tokenize(text: &str) {
    let normalized_text: String = normalize_newlines(text);
    let mut block_state: BlockState = BlockState::new();

    for (row, line) in normalized_text.lines().enumerate() {
        let mut column: usize = 0;

        if block_state.stack.is_empty() {
            println!("###");
        }
        _tokenize(line.to_string(), &mut column, &mut block_state);
        println!("{}", column);
    }
}

fn _tokenize(line: String, column: &mut usize, block_state: &mut BlockState) {
    // *column += 1;
    process_indentation(line, 0);

    // if (*column < 10) {
    //     _tokenize(line, column, block_state);
    // }
}

fn process_indentation(mut line: String, required_indent: usize) -> String {
    if required_indent == 0 {
        if let Some(position) = line.find('\t') {
            if (position > 3) {
                let spaces = calculate_spaces_until_next_tab_stop(position);
                let mut new_line = " ".repeat(spaces + position);
                new_line.push_str(&line[position + 1..]);
                return new_line;
            }
        } else {
            return line.to_string();
        }
    } else {
        // required_indentが0でない場合の処理
        println!("###");
        return line.to_string();
    }
    line
}
