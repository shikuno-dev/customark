use crate::util::unicode::is_blank_line;

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
