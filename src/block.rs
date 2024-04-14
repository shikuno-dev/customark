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
    LinkDefinitions, // LinkReferenceDefinitions,
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
    fn initial_state(&self) -> Option<&BlockStateType> {
        self.stack.first()
    }
    fn current_block_state_type(&self) -> Option<&BlockStateType> {
        self.stack.last()
    }

    fn clear(&mut self) {
        self.stack.clear();
    }

    fn truncate_from(&mut self, position: usize) {
        if position < self.stack.len() {
            self.stack.truncate(position);
        }
    }

    // // fn find_position(&self, block_type: &BlockStateType) -> Option<usize> {
    // fn find_index(&self, block_type: &BlockStateType) -> Option<usize> {
    //     self.stack.iter().position(|t| t == block_type)
    // }

    fn contains(&self, block_type: &BlockStateType) -> bool {
        self.stack.contains(block_type)
    }
    fn find_position(&self, block_type: &BlockStateType) -> Option<usize> {
        self.stack.iter().position(|t| t == block_type)
    }
}

#[derive(Debug)]
struct LinkReferenceDefinition {
    label: String,
    destination: String,
    title: Option<String>,
}

impl LinkReferenceDefinition {
    fn label_matches(&self, target_label: &str) -> bool {
        self.label == target_label
    }
}
// If there are several matching definitions, the first one takes precedence:
// matching of labels is case-insensitive
fn has_matching_label(link_definitions: &[LinkReferenceDefinition], target_label: &str) -> bool {
    for definition in link_definitions {
        if definition.label_matches(target_label) {
            return true;
        }
    }
    false
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

    let mut consecutive_blank_lines_count: usize = 0;
    let mut can_continue_paragraph_text: bool = false; // if block_state.stack.last() == Some(&BlockStateType::Paragraph)
                                                       // can_continue_paragraph_text &&

    for (row, line) in normalized_text.lines().enumerate() {
        let mut column: usize = 0; // Required for expanding tab

        // let mut tokens: Vec<Token> = Vec::new();

        if is_blank_line(line) {
            if block_state.stack.is_empty() {
                continue;
            } else if block_state.stack.len() == 1 {
                // A sequence of non-blank lines that cannot be interpreted as other kinds of blocks forms a paragraph.
                if block_state.stack.last() == Some(&BlockStateType::Paragraph) {
                    // P Token is closed

                    // An indented code block cannot interrupt a paragraph. (This allows hanging indents and the like.) (Example 113)

                    block_state.clear();

                // A line consisting of optionally up to three spaces of indentation,
                // followed by a sequence of three or more matching -, _, or * characters,
                // each followed optionally by any number of spaces or tabs, forms a thematic break.
                } else if block_state.stack.last() == Some(&BlockStateType::ThematicBreak)
                    || block_state.stack.last() == Some(&BlockStateType::ATXHeading)
                    || block_state.stack.last() == Some(&BlockStateType::SetextHeading)
                {
                    block_state.clear();
                    // Thematic breaks do not need blank lines before or after (Example 57)
                    // ATX headings need not be separated from surrounding content by blank lines, and they can interrupt paragraphs (Example 77)
                    // A blank line is needed between a paragraph and a following setext heading,
                    // since otherwise the paragraph becomes part of the heading’s content: (Example 95)
                    // But in general a blank line is not required before or after setext headings: (Example 96)
                } else if block_state.stack.last() == Some(&BlockStateType::IndentedCodeBlock) {
                    consecutive_blank_lines_count += 1;
                } else if block_state.stack.last() == Some(&BlockStateType::FencedCodeBlock) {
                } else if block_state.stack.last() == Some(&BlockStateType::HTMLBlock) {
                    // type 1-5
                    consecutive_blank_lines_count += 1;
                    // type 6-7
                    // HTMLBlock Token is closed
                } else if block_state.stack.last() == Some(&BlockStateType::LinkDefinitions) {
                    block_state.clear();
                }
            } else if block_state.stack.len() > 1 {
                // list, quote
                if block_state.stack.first() == Some(&BlockStateType::BlockQuote) {
                    block_state.clear();
                }
                if block_state.stack.first() == Some(&BlockStateType::List) {
                    if let Some(position) = block_state.find_position(&BlockStateType::BlockQuote) {
                        // positoin of quote
                        block_state.truncate_from(position);
                    } else {
                        if block_state.stack.last() == Some(&BlockStateType::Paragraph)
                            || block_state.stack.last() == Some(&BlockStateType::ThematicBreak)
                            || block_state.stack.last() == Some(&BlockStateType::ATXHeading)
                            || block_state.stack.last() == Some(&BlockStateType::SetextHeading)
                        {
                            //
                        } else if block_state.stack.last()
                            == Some(&BlockStateType::IndentedCodeBlock)
                        {
                            consecutive_blank_lines_count += 1;
                        }
                    }
                }
            }
        } else {
            _tokenize(line.to_string(), &mut column, &mut block_state);
        }

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
            if position < 3 && line[0..position].chars().all(|c| c == ' ') {
                let spaces = calculate_spaces_until_next_tab_stop(position);
                // let spaces = calculate_spaces_until_next_tab_stop(position + column);
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
