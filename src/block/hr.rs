pub fn is_thematic_breaks(line: &str) -> bool {
    // Count the number of leading spaces or tabs
    let mut indent_count = 0;
    for c in line.chars() {
        if c == ' ' || c == '\t' {
            indent_count += 1;
        } else {
            break;
        }
    }

    if indent_count >= 3 {
        let line_trimmed = &line[indent_count..];

        let thematic_chars = ['-', '_', '*'];

        let mut char_count = 0;
        let mut last_char: Option<char> = None;
        for c in line_trimmed.chars() {
            if thematic_chars.contains(&c) {
                char_count += 1;
                last_char = Some(c);
            } else if c != ' ' && c != '\t' {
                return false;
            }
        }

        if char_count >= 3 {
            if let Some(last_char) = last_char {
                for c in line_trimmed.chars().rev() {
                    if c != ' ' && c != '\t' {
                        return false;
                    } else if c != last_char {
                        break;
                    }
                }

                return true;
            }
        }
    }

    false
}
