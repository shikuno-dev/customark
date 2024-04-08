pub enum Node {
    // Document
    Document,
    // Leaf blocks
    Paragraph,
    ThematicBreak,
    Heading(HeadingNode),
    CodeBlock,
    // Container blocks
    BlockQuote,
    ListItem,
    List,
}

pub struct HeadingNode {
    pub id: u32,
    pub parent_id: u32,
    pub heading_level: u8,
    pub content: String,
}

impl Node {
    pub fn th(value: &str) -> Self {
        Node::ThematicBreak
    }
}
