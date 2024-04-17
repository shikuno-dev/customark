trait CommonMarkRule {
    fn parse(&self, input: &str) -> Option<String>;
    fn match_start_condition(&self, input: &str) -> bool;
    fn match_continuance_condition(&self, input: &str) -> bool;
}
