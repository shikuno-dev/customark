pub enum Node {
    // Document
    Document,
    // Container blocks
    // Leaf blocks
    ThematicBreak,
    Heading(HeadingNode),

    BlockQuote,
    HTMLBlock, // inlines
}

struct HeadingNode {
    id: u32,
    parent_id: u32,
    heading_level: u8,
    content: String,
}

impl Node {
    pub fn th(value: &str) -> Self {
        Node::ThematicBreak
    }
}
