#[derive(Debug)]
pub enum NodeType {
    Root,
    BlockLeaf { name: String },
    BlockContainer { name: String },
    InlineLeaf { name: String },
    InlineContainer { name: String },
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

pub trait Node: std::fmt::Debug {
    fn add_child(&mut self, child: Box<dyn Node>);

    fn children(&self) -> Option<&Vec<Box<dyn Node>>>;

    fn deepth(&self) -> usize;

    // Return the type of the token.
    fn token_type(&self) -> &NodeType;

    // Return the location of the token in the source text.
    fn location(&self) -> &Location;

    // Render the token into a string representation.
    fn render(&self) -> String;

    // Method to determine the start condition of a block
    fn should_start(&self, text: &str, current_column: usize) -> bool;

    // Method to determine the continuation condition of a block
    #[allow(unused_variables)]
    fn should_continue(&self, text: &str, current_column: usize) -> bool {
        false
    }

    // Method to determine whether current token should be transformed into a different type of token.
    #[allow(unused_variables)]
    fn should_transform(&self, text: &str, current_column: usize) -> bool {
        false
    }

    // Method to handle the start of the block
    fn handle_start(&mut self, text: &str, current_column: usize) -> Box<dyn Node>;

    // Method to handle the continuation of the block
    #[allow(unused_variables)]
    fn handle_continuation(&mut self, text: &str, current_column: usize) -> Option<String> {
        None
    }

    // Method to convert the current token to a different type of token.
    #[allow(unused_variables)]
    fn handle_transform(&mut self, text: &str, current_column: usize) -> Option<Box<dyn Node>> {
        None
    }

    // Finalizes the token after processing.
    fn finalize(&mut self) {}
}

#[derive(Debug)]
pub struct RootNode {
    token_type: NodeType,
    location: Location,
    children: Vec<Box<dyn Node>>,
}

impl Default for RootNode {
    fn default() -> Self {
        RootNode {
            token_type: NodeType::Root,
            location: Location::default(),
            children: Vec::new(),
        }
    }
}

impl Node for RootNode {
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.push(child);
    }
    fn children(&self) -> Option<&Vec<Box<dyn Node>>> {
        Some(&self.children)
    }

    fn deepth(&self) -> usize {
        0
    }
    fn token_type(&self) -> &NodeType {
        &self.token_type
    }

    fn location(&self) -> &Location {
        &self.location
    }

    fn render(&self) -> String {
        let mut result = String::new();
        for child in &self.children {
            result.push_str(&format!("{}\n", child.render()));
        }
        result
    }

    fn should_start(&self, text: &str, current_column: usize) -> bool {
        todo!()
    }

    fn handle_start(&mut self, text: &str, current_column: usize) -> Box<dyn Node> {
        todo!()
    }
}

pub fn find_applicable_block_node(
    block_nodes: Vec<Box<dyn BlockNode>>,
    text: &str,
    current_column: usize,
) -> Option<Box<dyn BlockNode>> {
    block_nodes
        .into_iter()
        .find(|node| node.should_start(text, current_column))
}
