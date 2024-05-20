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

        unimplemented!()
    }
}
