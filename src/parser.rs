pub struct CommonmarkParser {
    rules: Vec<Box<dyn BlockRule>>,
}

impl CommonmarkParser {
    fn new(rules: Vec<Box<dyn BlockRule>>) -> Self {
        // Initialize with Commonmark rules

        CommonmarkParser { rules }
    }
}
