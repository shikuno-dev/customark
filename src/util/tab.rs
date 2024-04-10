// n = 4 - position % 4
// position: Position of Tab from the beginning of the line
pub fn expand_tabs(n: usize) -> String {
    "\u{0020}".repeat(n)
}

pub fn calculate_spaces_until_next_tab_stop(position: usize) -> usize {
    let tab_stop_size = 4; // タブ停止のサイズ
    tab_stop_size - (position % tab_stop_size)
}
