use crate::link::LinkReferenceDefinition;

pub enum BreakType {
    SoftLineBreak, // softbreak
    HardLineBreak, // Hard line breaks
}
pub trait Node: std::fmt::Debug {}

#[derive(Debug)]
pub enum NodeType<'a> {
    Root,
    BlockContainer { name: &'a str },
    BlockLeaf { name: &'a str },
    InlineContainer { name: &'a str },
    InlineLeaf { name: &'a str },
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct RootNode<'a> {
    node_type: NodeType<'a>,
    location: Location,
    children: Vec<Box<dyn Node>>,
    is_open: bool,
    depth: usize,
}

impl<'a> Default for RootNode<'a> {
    fn default() -> Self {
        RootNode {
            node_type: NodeType::Root,
            location: Location::default(),
            children: Vec::new(),
            is_open: true,
            depth: 0,
        }
    }
}

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
        loop {
            if let Some(last_child) = get_last_open_child(parent_node.children()) {}
        }

        root_node
    }
}

fn get_last_open_child(children: Option<&Vec<Box<dyn Node>>>) -> Option<&Box<dyn Node>> {
    match children {
        Some(node_vec) => {
            if let Some(last_node) = node_vec.last() {
                if last_node.is_open() {
                    return Some(last_node);
                }
            }
            None
        }
        None => None,
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
