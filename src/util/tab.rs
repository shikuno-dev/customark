// n = 4 - position % 4
// position: Position of Tab from the beginning of the line
pub fn expand_tabs(n: usize) -> String {
    "\u{0020}".repeat(n)
}
