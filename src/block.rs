pub mod code_block;
pub mod hr;
pub mod list;

use crate::util::{
    tab::{calculate_spaces_until_next_tab_stop, expand_tabs},
    unicode::is_blank_line,
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

// normalize_newlines --> lines
//  let lines: Vec<&str> = normalized_text.split('\n').collect();
pub fn tokenize(lines: Vec<&str>) {
    let mut block_state: BlockState = BlockState::new();
    let mut tokens: Vec<Token> = Vec::new();

    // let mut current_token = Token {};
    let mut consecutive_blank_lines_count: usize = 0;
    let mut current_content = String::new();

    // block_state.block_stack.len();

    for (row, line) in lines.iter().enumerate() {
        let mut column = 0;

        if block_state.stack.len() == 0 {
            // for (column, char) in line.chars().enumerate() {}
            let mut indent_count: usize = 0;

            if !is_blank_line(line) {
                let mut chars = line.chars();
                while column < line.chars().count() {
                    let ch = chars.nth(column).unwrap(); // Access character at column
                    if column == 0 {
                        match ch {
                            '\t' => {
                                // indented_code_block
                                block_state.open(BlockStateType::IndentedCodeBlock);
                                current_content.push_str(&line[1..]);
                                continue;
                            }
                            ' ' => {
                                indent_count += 1;
                                //
                                //
                                // Handle spaces based on your specific logic
                                // (e.g., indent or not based on context)
                            }
                            _ => {
                                // Handle other characters as needed
                            }
                        }
                        column += 1;
                    }
                }
            }
        } else if block_state.stack.len() == 1 {
            if let Some(last_element) = block_state.stack.last() {
                if *last_element == BlockStateType::IndentedCodeBlock {
                    if is_blank_line(line) {
                        consecutive_blank_lines_count += 1;
                    } else {
                    }
                }
            }
        } else { //  block_state.stack.len() > 1
        }

        // for (column, word) in line.split_whitespace().enumerate() {
        //     println!("  Word {}: {}", column + 1, word);
        // }
    }
}

fn count_indent(line: &str, n: usize) -> usize {
    // Count the number of leading spaces or tabs
    let mut indent_count = 0;
    let mut position = 1;

    for c in line.chars() {
        if c == ' ' {
            indent_count += 1;
        } else if c == '\t' {
            indent_count += calculate_spaces_until_next_tab_stop(position);
        } else {
            break;
        }
        position += 1;
    }

    indent_count
}
