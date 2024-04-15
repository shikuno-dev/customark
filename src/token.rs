#[derive(Debug)]
pub enum Token {
    Paragraph(ParagraphToken),
    Heading(HeadingToken),
    ThematicBreak,
    CodeBlock(CodeBlockToken),
    HTMLBlock(HTMLBlockToken),
    BlockQuote(BlockQuoteToken),
    List(ListToken),
    ListItem(ListItemToken),
}

impl Token {
    pub fn paragraph(inline_text: String) -> Self {
        Token::Paragraph(ParagraphToken::new(inline_text))
    }

    pub fn heading(inline_text: String, heading_level: u8) -> Self {
        Token::Heading(HeadingToken::new(inline_text, heading_level))
    }

    pub fn thematic_break() -> Self {
        Token::ThematicBreak
    }

    pub fn code_block(text: String, info_string: String) -> Self {
        Token::CodeBlock(CodeBlockToken::new(text, info_string))
    }

    pub fn html_block(text: String) -> Self {
        Token::HTMLBlock(HTMLBlockToken::new(text))
    }

    pub fn block_quote(len: usize) -> Self {
        Token::BlockQuote(BlockQuoteToken::new(len))
    }

    pub fn list(list_type: ListType, start: usize, tight: bool, len: usize) -> Self {
        Token::List(ListToken::new(list_type, start, tight, len))
    }

    pub fn list_item(len: usize) -> Self {
        Token::ListItem(ListItemToken::new(len))
    }

    pub fn convert_to_heading(&mut self, heading_level: u8) -> Option<Self> {
        match self {
            Token::Paragraph(paragraph_token) => Some(Token::Heading(HeadingToken {
                inline_text: paragraph_token.inline_text.clone(),
                heading_level,
            })),
            _ => None, // Non-Paragraph Tokens cannot be converted.
        }
    }

    // Link reference definitions
    // link label, link destination, link title(optional)
    // pub fn convert_to_Link_reference_definitions(&mut self, heading_level: u8) {
    //     match self {
    //         Token::Paragraph(paragraph_token) => {}
    //         _ => {} // Non-Paragraph Tokens cannot be converted.
    //     }
    // }
}

#[derive(Debug)]
pub struct ParagraphToken {
    pub inline_text: String,
}

impl ParagraphToken {
    pub fn new(inline_text: String) -> Self {
        ParagraphToken { inline_text }
    }
    pub fn set_inline_text(&mut self, new_inline_text: String) {
        self.inline_text = new_inline_text;
    }
}

#[derive(Debug)]
pub struct HeadingToken {
    pub inline_text: String,
    pub heading_level: u8,
}

impl HeadingToken {
    pub fn new(inline_text: String, heading_level: u8) -> Self {
        HeadingToken {
            inline_text,
            heading_level,
        }
    }
    pub fn set_inline_text(&mut self, new_inline_text: String) {
        self.inline_text = new_inline_text;
    }
}
#[derive(Debug)]
pub struct CodeBlockToken {
    pub text: String,
    pub info_string: String,
}
impl CodeBlockToken {
    pub fn new(text: String, info_string: String) -> Self {
        CodeBlockToken { text, info_string }
    }
    pub fn set_inline_text(&mut self, new_text: String) {
        self.text = new_text;
    }
}

#[derive(Debug)]
pub struct HTMLBlockToken {
    pub text: String,
}

impl HTMLBlockToken {
    pub fn new(text: String) -> Self {
        HTMLBlockToken { text }
    }
    pub fn set_inline_text(&mut self, new_text: String) {
        self.text = new_text;
    }
}

#[derive(Debug)]
pub struct BlockQuoteToken {
    pub len: usize,
}

impl BlockQuoteToken {
    pub fn new(len: usize) -> Self {
        BlockQuoteToken { len }
    }

    pub fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }
}

#[derive(Debug)]
pub struct ListItemToken {
    pub len: usize,
}

impl ListItemToken {
    pub fn new(len: usize) -> Self {
        ListItemToken { len }
    }

    pub fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }
}

#[derive(Debug)]
pub enum ListType {
    Ordered,
    // -, +, or *
    BulletMinus,
    BulletPlus,
    BulletAsterisk,
}

#[derive(Debug)]
pub struct ListToken {
    pub list_type: ListType,
    pub start: usize,
    pub tight: bool,
    pub len: usize,
}

impl ListToken {
    pub fn new(list_type: ListType, start: usize, tight: bool, len: usize) -> Self {
        ListToken {
            list_type,
            start,
            tight,
            len,
        }
    }

    pub fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }
}
