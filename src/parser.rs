use crate::link::LinkReferenceDefinition;

pub enum BreakType {
    SoftLineBreak, // softbreak
    HardLineBreak, // Hard line breaks
}
pub trait Node: std::fmt::Debug {}

pub struct Config<'a> {
    break_type: BreakType,
    block_nodes: Vec<Box<dyn Node>>,
    inline_nodes: Vec<Box<dyn Node>>,
    p_name: &'a str, // "paragraph"
}

pub struct Parser<'a> {
    pub config: Config<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(config: Config) -> Self {
        Parser { config }
    }
    fn parse(&self, text: &str) -> Box<dyn Node> {
        // self.config.block_nodes;
        let normalized_text: String = text.replace("\r\n", "\n").replace('\r', "\n");

        let mut root_node: Box<dyn Node> = Box::new(RootNode::default());
        let mut parent_node: &Box<dyn Node> = &root_node;

        let mut end_with_paragraph: bool = false;
        let mut depth: usize = 0;

        root_node
    }
}

#[derive(Debug)]
pub struct InlineParser {}

pub fn parse_inline(
    mut parent_node: &Box<dyn Node>,
    link_definitions: Vec<LinkReferenceDefinition>,
    inline_nodes: Vec<Box<dyn Node>>,
) {
}
