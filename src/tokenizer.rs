use crate::token::CToken;

pub struct Tokenizer {
    block_tokens: Vec<Box<dyn CToken>>,
}

impl Tokenizer {
    pub fn new(block_tokens: Vec<Box<dyn CToken>>) -> Self {
        Tokenizer { block_tokens }
    }

    pub fn tokenize(&mut self, text: String) -> RootToken {
        let mut root_token: RootToken = RootToken::new();

        let normalized_text: String = normalize_newlines(text);

        let mut link_definitions: Vec<LinkReferenceDefinition> = Vec::new();

        for current_line in normalized_text.lines().enumerate() {
            let mut current_column: usize = 0; // Required for expanding tab

            for rule in &mut self.block_tokens {}
        }

        root_token
    }
}

fn get_last_node(nodes: Option<&Vec<Box<dyn Node>>>) -> Option<&Box<dyn Node>> {
    match nodes {
        Some(node_vec) => node_vec.last(),
        None => None,
    }
}

pub fn is_paragraph_node(node_type: &NodeType) -> bool {
    match node_type {
        NodeType::BlockLeaf { name } if name == "paragraph" => true,
        _ => false,
    }
}
