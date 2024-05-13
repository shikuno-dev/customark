pub enum BreakType {
    SoftLineBreak, // softbreak
    HardLineBreak, // Hard line breaks
}

pub struct Config {
    break_type: BreakType,
}

pub struct Parser {
    pub config: Config,
}
