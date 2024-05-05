pub trait CToken: std::fmt::Debug {
    fn add_child(&mut self, child: Box<dyn CToken>);

    fn token_type(&self) -> &TokenType;

    fn location(&self) -> Location;

    fn render(&self) -> String;

    // Method to determine the start condition of a block
    fn should_start(&self, text: &String, current_column: usize) -> bool;

    // Method to determine the continuation condition of a block
    fn should_continue(&self, text: &String, current_column: usize) -> bool;

    // Method to determine whether current token should be transformed into a different type of token.
    fn should_transform(&self, text: &String, current_column: usize) -> bool;

    // Method to handle the start of the block
    fn handle_start(&mut self, text: &String, current_column: usize) -> Box<dyn CToken>;

    // Method to handle the continuation of the block
    fn handle_continuation(&mut self, text: &String, current_column: usize) -> Option<String>;

    // Method to convert the current token to a different type of token.
    fn handle_transform(&mut self, text: &String, current_column: usize) -> Box<dyn CToken>;

    fn finalize(&mut self);
}

#[derive(Debug)]
pub enum TokenType {
    Root,
    BlockLeaf { name: String },
    BlockContainer { name: String },
    Inline { name: String },
}

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct Location {
    pub(crate) line: usize,
    pub(crate) column: usize,
}

#[derive(Debug)]
pub struct RootToken {
    token_type: TokenType,
    location: Location,
    children: Vec<Box<dyn CToken>>,
}

impl RootToken {
    pub fn new() -> Self {
        RootToken {
            token_type: TokenType::Root,
            location: Location { line: 0, column: 0 },
            children: Vec::new(),
        }
    }
}

impl CToken for RootToken {
    fn add_child(&mut self, child: Box<dyn CToken>) {
        self.children.push(child);
    }

    fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    fn location(&self) -> Location {
        self.location
    }

    fn render(&self) -> String {
        // let mut result = format!("{}:\n", self.name);
        let mut result = String::new();
        for child in &self.children {
            result.push_str(&format!("{}\n", child.render()));
        }
        result
    }

    fn finalize(&mut self) {
        todo!()
    }

    fn should_start(&self, text: &String, current_column: usize) -> bool {
        todo!()
    }

    fn should_continue(&self, text: &String, current_column: usize) -> bool {
        todo!()
    }

    fn should_transform(&self, text: &String, current_column: usize) -> bool {
        todo!()
    }
    fn handle_start(&mut self, text: &String, current_column: usize) -> Box<dyn CToken> {
        todo!()
    }

    fn handle_continuation(&mut self, text: &String, current_column: usize) -> Option<String> {
        todo!()
    }

    fn handle_transform(&mut self, text: &String, current_column: usize) -> Box<dyn CToken> {
        todo!()
    }
}

pub fn find_applicable_token(
    block_token: Vec<Box<dyn CToken>>,
    text: &String,
    current_column: usize,
) -> Option<Box<dyn CToken>> {
    block_token
        .into_iter()
        .find(|token| token.should_start(text, current_column))
}
