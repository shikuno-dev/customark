pub trait MarkRule {
    fn tokenize(&self, input: &str) -> Token;
    fn match_start_condition(&self, input: &str, state: &BlockStateType) -> bool;
    fn match_continuance_condition(&self, input: &str, state: &BlockStateType) -> bool;
    fn finalize(&self, token: Token) -> Token;
}

struct ThematicBreakgRule;

impl MarkRule for ThematicBreakgRule {
    fn tokenize(&self, _input: &str) -> Token {
        Token::ThematicBreak(ThematicBreak::new())
    }

    fn match_start_condition(&self, input: &str, state: &BlockStateType) -> bool {
        let thematic_chars = ['-', '_', '*'];

        let mut char_count = 0;
        let mut last_char: Option<char> = None;
        for c in input.chars() {
            if thematic_chars.contains(&c) {
                char_count += 1;

                if let Some(last) = last_char {
                    if last != c {
                        return false;
                    }
                }
                last_char = Some(c);
            } else if c != ' ' && c != '\t' {
                return false;
            }
        }

        if char_count < 3 {
            return false;
        }

        true
    }

    fn match_continuance_condition(&self, _input: &str, state: &BlockStateType) -> bool {
        false
    }
    fn finalize(&self, token: Token) -> Token {
        token
    }
}
