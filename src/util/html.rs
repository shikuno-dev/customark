fn escape_html(text: &str) -> String {
    let mut escaped_text = String::new();

    for ch in text.chars() {
        match ch {
            '&' => escaped_text.push_str("&amp;"),
            '<' => escaped_text.push_str("&lt;"),
            '>' => escaped_text.push_str("&gt;"),
            '"' => escaped_text.push_str("&quot;"),
            _ => escaped_text.push(ch),
        }
    }

    escaped_text
}
