pub mod list;

use crate::util::{
    tab::{calculate_spaces_until_next_tab_stop, expand_tabs},
    unicode::is_blank_line,
};

enum BlockStateType {
    // Idle,
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

struct Token {}

// normalize_newlines --> lines
//  let lines: Vec<&str> = normalized_text.split('\n').collect();
pub fn tokenize(lines: Vec<&str>) {
    let block_state: BlockState = BlockState::new();
    let tokens: Vec<Token> = Vec::new();

    let mut current_token = Token {};

    // block_state.block_stack.len();

    for (row, line) in lines.iter().enumerate() {
        let mut column = 0;

        if (is_blank_line(&line)) {}

        // if( block_state ){
        // expand_tabs(n)
        // }

        if block_state.stack.len() == 0 {
            while column < line.len() {
                if column == 0 {
                    let word = &line[column..column];
                    if word == "\u{0009}" {
                    } else if word == "\u{0020}" {
                    }
                }
            }
        } else if block_state.stack.len() == 1 {
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

pub fn is_thematic_breaks(line: &str) -> bool {
    // Count the number of leading spaces or tabs
    let mut indent_count = 0;
    for c in line.chars() {
        if c == ' ' || c == '\t' {
            indent_count += 1;
        } else {
            break;
        }
    }

    // 先頭の空白またはタブの数が3未満の場合はテーマ区切りではない
    if indent_count >= 3 {
        // 先頭の空白またはタブを除去した行を取得
        let line_trimmed = &line[indent_count..];

        // テーマ区切りの文字を定義
        let thematic_chars = ['-', '_', '*'];

        // テーマ区切りの文字で構成されているかを確認
        let mut char_count = 0;
        let mut last_char: Option<char> = None;
        for c in line_trimmed.chars() {
            if thematic_chars.contains(&c) {
                char_count += 1;
                last_char = Some(c);
            } else if c != ' ' && c != '\t' {
                // テーマ区切りの文字以外が見つかった場合はテーマ区切りではない
                return false;
            }
        }

        // 3つ以上のテーマ区切りの文字があるかどうかを確認
        if char_count >= 3 {
            // テーマ区切りの最後の文字を取得
            if let Some(last_char) = last_char {
                // テーマ区切りの最後の文字以降にスペースまたはタブがあるかを確認
                for c in line_trimmed.chars().rev() {
                    if c != ' ' && c != '\t' {
                        return false;
                    } else if c != last_char {
                        break;
                    }
                }

                return true;
            }
        }
    }

    false
}
