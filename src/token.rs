// #[derive(Debug)]
// pub enum Token {
//     Paragraph(Paragraph),
//     Heading(Heading),
//     ThematicBreak,
//     CodeBlock(CodeBlock),
//     HTMLBlock(HTMLBlock),
//     BlockQuote(BlockQuote),
//     List(List),
//     ListItem(ListItem),
// }
pub trait Token {
    fn is_leaf(&self) -> bool;
    fn children(&self) -> Option<Vec<Box<dyn Token>>>;

    fn render(&self) -> String;
    fn update(&mut self, text: String);
    fn finalize(&mut self);
}

// impl Token {
//     pub fn paragraph(inline_text: String) -> Self {
//         Token::Paragraph(Paragraph::new(inline_text))
//     }

//     pub fn heading(inline_text: String, heading_level: u8) -> Self {
//         Token::Heading(Heading::new(inline_text, heading_level))
//     }

//     pub fn thematic_break() -> Self {
//         Token::ThematicBreak
//     }

//     pub fn code_block(text: String, info_string: String) -> Self {
//         Token::CodeBlock(CodeBlock::new(text, info_string))
//     }

//     pub fn html_block(text: String) -> Self {
//         Token::HTMLBlock(HTMLBlock::new(text))
//     }

//     pub fn block_quote(len: usize) -> Self {
//         Token::BlockQuote(BlockQuote::new(len))
//     }

//     pub fn list(list_type: ListType, start: usize, tight: bool, len: usize) -> Self {
//         Token::List(List::new(list_type, start, tight, len))
//     }

//     pub fn list_item(len: usize) -> Self {
//         Token::ListItem(ListItem::new(len))
//     }

//     pub fn convert_to_heading(&mut self, heading_level: u8) -> Option<Self> {
//         match self {
//             Token::Paragraph(paragraph_token) => Some(Token::Heading(Heading {
//                 inline_text: paragraph_token.inline_text.clone(),
//                 heading_level,
//             })),
//             _ => None, // Non-Paragraph Tokens cannot be converted.
//         }
//     }

//     // Link reference definitions
//     // link label, link destination, link title(optional)
//     // pub fn convert_to_Link_reference_definitions(&mut self, heading_level: u8) {
//     //     match self {
//     //         Token::Paragraph(paragraph_token) => {}
//     //         _ => {} // Non-Paragraph Tokens cannot be converted.
//     //     }
//     // }
// }

// #[derive(Debug)]
// pub struct DocumentToken {
//     children: Vec<Box<dyn Token>>,
// }

// impl DocumentToken {
//     pub fn new() -> Self {
//         DocumentToken {
//             children: Vec::new(),
//         }
//     }
// }
// impl Token for DocumentToken {
//     fn is_leaf(&self) -> bool {
//         false
//     }
//     fn render(&self) -> String {
//         // let mut result = format!("{}:\n", self.name);
//         let mut result = String::new();
//         for child in &self.children {
//             result.push_str(&format!("{}\n", child.render()));
//         }
//         result
//     }
// }

// #[derive(Debug)]
// pub struct Paragraph {
//     pub inline_text: String,
// }

// impl Paragraph {
//     pub fn new(inline_text: String) -> Self {
//         Paragraph { inline_text }
//     }
//     pub fn set_inline_text(&mut self, new_inline_text: String) {
//         self.inline_text = new_inline_text;
//     }
// }

// #[derive(Debug)]
// pub struct Heading {
//     pub inline_text: String,
//     pub heading_level: u8,
// }

// impl Heading {
//     pub fn new(inline_text: String, heading_level: u8) -> Self {
//         Heading {
//             inline_text,
//             heading_level,
//         }
//     }
//     pub fn set_inline_text(&mut self, new_inline_text: String) {
//         self.inline_text = new_inline_text;
//     }
// }
// #[derive(Debug)]
// pub struct CodeBlock {
//     pub text: String,
//     pub info_string: String,
// }
// impl CodeBlock {
//     pub fn new(text: String, info_string: String) -> Self {
//         CodeBlock { text, info_string }
//     }
//     pub fn set_inline_text(&mut self, new_text: String) {
//         self.text = new_text;
//     }
// }

// #[derive(Debug)]
// pub struct HTMLBlock {
//     pub text: String,
// }

// impl HTMLBlock {
//     pub fn new(text: String) -> Self {
//         HTMLBlock { text }
//     }
//     pub fn set_inline_text(&mut self, new_text: String) {
//         self.text = new_text;
//     }
// }

// #[derive(Debug)]
// pub struct BlockQuote {
//     pub len: usize,
// }

// impl BlockQuote {
//     pub fn new(len: usize) -> Self {
//         BlockQuote { len }
//     }

//     pub fn set_len(&mut self, new_len: usize) {
//         self.len = new_len;
//     }
// }

// #[derive(Debug)]
// pub struct ListItem {
//     pub len: usize,
// }

// impl ListItem {
//     pub fn new(len: usize) -> Self {
//         ListItem { len }
//     }

//     pub fn set_len(&mut self, new_len: usize) {
//         self.len = new_len;
//     }
// }

// #[derive(Debug)]
// pub enum ListType {
//     Ordered,
//     // -, +, or *
//     BulletMinus,
//     BulletPlus,
//     BulletAsterisk,
// }

// #[derive(Debug)]
// pub struct List {
//     pub list_type: ListType,
//     pub start: usize,
//     pub tight: bool,
//     pub len: usize,
// }

// impl List {
//     pub fn new(list_type: ListType, start: usize, tight: bool, len: usize) -> Self {
//         List {
//             list_type,
//             start,
//             tight,
//             len,
//         }
//     }

//     pub fn set_len(&mut self, new_len: usize) {
//         self.len = new_len;
//     }
// }
