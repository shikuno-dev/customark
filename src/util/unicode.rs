pub const Tab: char = '\u{0009}';
pub const Space: char = '\u{0020}';

// A backslash at the end of the line is a hard line break

fn is_backslash_escaped(char: char) -> bool {
    // 文字の ASCII コードを取得
    let char_code = char as u8;

    // Any **ASCII punctuation** character may be backslash-escaped
    // Backslash escapes do not work in **code blocks**, **code spans**, **autolinks**, or **raw HTML**
    // But they work in all other contexts, including URLs and link titles, link references, and info strings in fenced code blocks
    is_ascii_punctuation_char(char_code)
}

pub fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

pub fn is_blank_line(line: &str) -> bool {
    line.chars().all(|c| c == ' ' || c == '\t')
}

pub fn trim_end_blank_line(text: String) -> String {
    text.trim_end_matches(|c: char| c == ' ' || c == '\t' || c == '\n')
        .to_string()
}

pub fn count_consecutive_char(line: String, target_char: char) -> usize {
    line.chars().take_while(|&c| c == target_char).count()
}

// For security reasons, U+0000 must be replaced with U+FFFD.
pub fn replaced_insecure_characters(text: String) -> String {
    text.replace('\u{0000}', "\u{FFFD}")
}

// Unicode whitespace character
pub fn is_unicode_whitespace_char(char: char) -> bool {
    match char {
        // Unicode Zs general category
        '\u{0020}'
        | '\u{00A0}'
        | '\u{2000}'..='\u{200A}'
        | '\u{202F}'
        | '\u{205F}'
        | '\u{3000}'
        | '\u{1680}' => true,
        // Tab, line feed, form feed, carriage return
        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' => true,
        _ => false,
    }
}

// ASCII control character: U+0000–1F (both including) or U+007F
fn is_ascii_control_char(code_point: u32) -> bool {
    (0x0000..=0x001F).contains(&code_point) || code_point == 0x007F
}

// ASCII punctuation character: U+0021–2F, U+005B–0060 or U+007B–007E
fn is_ascii_punctuation_char(code_point: u8) -> bool {
    (0x0021..=0x002F).contains(&code_point) // !, ", #, $, %, &, ', (, ), *, +, ,, -, ., /
        || (0x003A..=0x0040).contains(&code_point) // :, ;, <, =, >, ?, @
        || (0x005B..=0x0060).contains(&code_point) // [, \, ], ^, _, `
        || (0x007B..=0x007E).contains(&code_point) // {, |, }, ~
}
// Example 12
// > Any ASCII punctuation character may be backslash-escaped

// Unicode punctuation character: character in the Unicode P (puncuation) or S (symbol) general categories.
fn is_unicode_punctuation_char(code_point: u32) -> bool {
    // Unicode P (puncuation)
    (0x0021..=0x0023).contains(&code_point) // Po
    || (0x0025..=0x0027).contains(&code_point) // Po
    || code_point == 0x0028 // Ps
    || code_point == 0x0029 // Pe
    || code_point == 0x002A // Po
    || code_point == 0x002C // Po
    || code_point == 0x002D // Pd
    || (0x002E..=0x002F).contains(&code_point) // Po
    || (0x003A..=0x003B).contains(&code_point) // Po
    || (0x003F..=0x0040).contains(&code_point) // Po
    || code_point == 0x005B // Ps
    || code_point == 0x005C // Po
    || code_point == 0x005D // Pe
    || code_point == 0x005F // Pc
    || code_point == 0x007B // Ps
    || code_point == 0x007D // Pe
    || code_point == 0x00A1 // Po
    || code_point == 0x00A7 // Po
    || code_point == 0x00AB // Pi
    || (0x00B6..=0x00B7).contains(&code_point) // Po
    || code_point == 0x00BB // Pf
    || code_point == 0x00BF // Po
    || code_point == 0x037E // Po
    || code_point == 0x0387 // Po
    || code_point == 0x060C // Po
    || code_point == 0x061B // Po
    || code_point == 0x061F // Po
    || (0x0964..=0x0965).contains(&code_point) // Po
    || code_point == 0x10FB // Po
    || (0x16EB..=0x16ED).contains(&code_point) // Po
    || (0x1735..=0x1736).contains(&code_point) // Po
    || (0x1802..=0x1803).contains(&code_point) // Po
    || code_point == 0x1805 // Po
    || code_point == 0x1CD3 // Po
    || (0x2010..=0x2015).contains(&code_point) // Pd
    || (0x2016..=0x2017).contains(&code_point) // Po
    || code_point == 0x2018 // Pi
    || code_point == 0x2019 // Pf
    || code_point == 0x201A // Ps
    || (0x201B..=0x201C).contains(&code_point) // Pi
    || code_point == 0x201D // Pf
    || code_point == 0x201E // Ps
    || code_point == 0x201F // Pi
    || (0x2020..=0x2027).contains(&code_point) // Po
    || (0x2030..=0x2038).contains(&code_point) // Po
    || code_point == 0x2039 // Pi
    || code_point == 0x203A // Pf
    || (0x203B..=0x203E).contains(&code_point) // Po
    || (0x203F..=0x2040).contains(&code_point) // Pc
    || (0x2041..=0x2043).contains(&code_point) // Po
    || code_point == 0x2045 // Ps
    || code_point == 0x2046 // Pe
    || (0x2047..=0x2051).contains(&code_point) // Po
    || code_point == 0x2053 // Po
    || code_point == 0x2054 // Pc
    || (0x2055..=0x205E).contains(&code_point) // Po
    || code_point == 0x207D // Ps
    || code_point == 0x207E // Pe
    || code_point == 0x208D // Ps
    || code_point == 0x208E // Pe
    || code_point == 0x2308 // Ps
    || code_point == 0x2309 // Pe
    || code_point == 0x230A // Ps
    || code_point == 0x230B // Pe
    || code_point == 0x2329 // Ps
    || code_point == 0x232A // Pe
    || code_point == 0x2768 // Ps
    || code_point == 0x2769 // Pe
    || code_point == 0x276A // Ps
    || code_point == 0x276B // Pe
    || code_point == 0x276C // Ps
    || code_point == 0x276D // Pe
    || code_point == 0x276E // Ps
    || code_point == 0x276F // Pe
    || code_point == 0x2770 // Ps
    || code_point == 0x2771 // Pe
    || code_point == 0x2772 // Ps
    || code_point == 0x2773 // Pe
    || code_point == 0x2774 // Ps
    || code_point == 0x2775 // Pe
    || code_point == 0x27C5 // Ps
    || code_point == 0x27C6 // Pe
    || code_point == 0x27E6 // Ps
    || code_point == 0x27E7 // Pe
    || code_point == 0x27E8 // Ps
    || code_point == 0x27E9 // Pe
    || code_point == 0x27EA // Ps
    || code_point == 0x27EB // Pe
    || code_point == 0x27EC // Ps
    || code_point == 0x27ED // Pe
    || code_point == 0x27EE // Ps
    || code_point == 0x27EF // Pe
    || code_point == 0x2983 // Ps
    || code_point == 0x2984 // Pe
    || code_point == 0x2985 // Ps
    || code_point == 0x2986 // Pe
    || code_point == 0x2987 // Ps
    || code_point == 0x2988 // Pe
    || code_point == 0x2989 // Ps
    || code_point == 0x298A // Pe
    || code_point == 0x298B // Ps
    || code_point == 0x298C // Pe
    || code_point == 0x298D // Ps
    || code_point == 0x298E // Pe
    || code_point == 0x298F // Ps
    || code_point == 0x2990 // Pe
    || code_point == 0x2991 // Ps
    || code_point == 0x2992 // Pe
    || code_point == 0x2993 // Ps
    || code_point == 0x2994 // Pe
    || code_point == 0x2995 // Ps
    || code_point == 0x2996 // Pe
    || code_point == 0x2997 // Ps
    || code_point == 0x2998 // Pe
    || code_point == 0x29D8 // Ps
    || code_point == 0x29D9 // Pe
    || code_point == 0x29DA // Ps
    || code_point == 0x29DB // Pe
    || code_point == 0x29FC // Ps
    || code_point == 0x29FD // Pe
    || (0x2E00..=0x2E01).contains(&code_point) // Po
    || code_point == 0x2E02 // Pi
    || code_point == 0x2E03 // Pf
    || code_point == 0x2E04 // Pi
    || code_point == 0x2E05 // Pf
    || (0x2E06..=0x2E08).contains(&code_point) // Po
    || code_point == 0x2E09 // Pi
    || code_point == 0x2E0A // Pf
    || code_point == 0x2E0B // Po
    || code_point == 0x2E0C // Pi
    || code_point == 0x2E0D // Pf
    || (0x2E0E..=0x2E16).contains(&code_point) // Po
    || code_point == 0x2E17 // Pd
    || (0x2E18..=0x2E19).contains(&code_point) // Po
    || code_point == 0x2E1A // Pd
    || code_point == 0x2E1B // Po
    || code_point == 0x2E1C // Pi
    || code_point == 0x2E1D // Pf
    || (0x2E1E..=0x2E1F).contains(&code_point) // Po
    || code_point == 0x2E20 // Pi
    || code_point == 0x2E21 // Pf
    || code_point == 0x2E22 // Ps
    || code_point == 0x2E23 // Pe
    || code_point == 0x2E24 // Ps
    || code_point == 0x2E25 // Pe
    || code_point == 0x2E26 // Ps
    || code_point == 0x2E27 // Pe
    || code_point == 0x2E28 // Ps
    || code_point == 0x2E29 // Pe
    || (0x2E2A..=0x2E2E).contains(&code_point) // Po
    || (0x2E30..=0x2E39).contains(&code_point) // Po
    || (0x2E3A..=0x2E3B).contains(&code_point) // Pd
    || (0x2E3C..=0x2E3F).contains(&code_point) // Po
    || code_point == 0x2E40 // Pd
    || code_point == 0x2E41 // Po
    || code_point == 0x2E42 // Ps
    || (0x2E43..=0x2E4F).contains(&code_point) // Po
    || (0x2E52..=0x2E54).contains(&code_point) // Po
    || code_point == 0x2E55 // Ps
    || code_point == 0x2E56 // Pe
    || code_point == 0x2E57 // Ps
    || code_point == 0x2E58 // Pe
    || code_point == 0x2E59 // Ps
    || code_point == 0x2E5A // Pe
    || code_point == 0x2E5B // Ps
    || code_point == 0x2E5C // Pe
    || code_point == 0x2E5D // Pd
    || (0x3001..=0x3003).contains(&code_point) // Po
    || code_point == 0x3008 // Ps
    || code_point == 0x3009 // Pe
    || code_point == 0x300A // Ps
    || code_point == 0x300B // Pe
    || code_point == 0x300C // Ps
    || code_point == 0x300D // Pe
    || code_point == 0x300E // Ps
    || code_point == 0x300F // Pe
    || code_point == 0x3010 // Ps
    || code_point == 0x3011 // Pe
    || code_point == 0x3014 // Ps
    || code_point == 0x3015 // Pe
    || code_point == 0x3016 // Ps
    || code_point == 0x3017 // Pe
    || code_point == 0x3018 // Ps
    || code_point == 0x3019 // Pe
    || code_point == 0x301A // Ps
    || code_point == 0x301B // Pe
    || code_point == 0x301C // Pd
    || code_point == 0x301D // Ps
    || (0x301E..=0x301F).contains(&code_point) // Pe
    || code_point == 0x3030 // Pd
    || code_point == 0x303D // Po
    || code_point == 0x30A0 // Pd
    || code_point == 0x30FB // Po
    || code_point == 0xA92E // Po
    || code_point == 0xFD3E // Pe
    || code_point == 0xFD3F // Ps
    || (0xFE10..=0xFE16).contains(&code_point) // Po
    || code_point == 0xFE17 // Ps
    || code_point == 0xFE18 // Pe
    || code_point == 0xFE19 // Po
    || code_point == 0xFE30 // Po
    || (0xFE31..=0xFE32).contains(&code_point) // Pd
    || (0xFE33..=0xFE34).contains(&code_point) // Pc
    || code_point == 0xFE35 // Ps
    || code_point == 0xFE36 // Pe
    || code_point == 0xFE37 // Ps
    || code_point == 0xFE38 // Pe
    || code_point == 0xFE39 // Ps
    || code_point == 0xFE3A // Pe
    || code_point == 0xFE3B // Ps
    || code_point == 0xFE3C // Pe
    || code_point == 0xFE3D // Ps
    || code_point == 0xFE3E // Pe
    || code_point == 0xFE3F // Ps
    || code_point == 0xFE40 // Pe
    || code_point == 0xFE41 // Ps
    || code_point == 0xFE42 // Pe
    || code_point == 0xFE43 // Ps
    || code_point == 0xFE44 // Pe
    || (0xFE45..=0xFE46).contains(&code_point) // Po
    || code_point == 0xFE47 // Ps
    || code_point == 0xFE48 // Pe
    || (0xFE49..=0xFE4C).contains(&code_point) // Po
    || (0xFE4D..=0xFE4F).contains(&code_point) // Pc
    || (0xFE50..=0xFE52).contains(&code_point) // Po
    || (0xFE54..=0xFE57).contains(&code_point) // Po
    || code_point == 0xFE58 // Pd
    || code_point == 0xFE59 // Ps
    || code_point == 0xFE5A // Pe
    || code_point == 0xFE5B // Ps
    || code_point == 0xFE5C // Pe
    || code_point == 0xFE5D // Ps
    || code_point == 0xFE5E // Pe
    || (0xFE5F..=0xFE61).contains(&code_point) // Po
    || code_point == 0xFE63 // Pd
    || code_point == 0xFE68 // Po
    || (0xFE6A..=0xFE6B).contains(&code_point) // Po
    || (0xFF01..=0xFF03).contains(&code_point) // Po
    || (0xFF05..=0xFF07).contains(&code_point) // Po
    || code_point == 0xFF08 // Ps
    || code_point == 0xFF09 // Pe
    || code_point == 0xFF0A // Po
    || code_point == 0xFF0C // Po
    || code_point == 0xFF0D // Pd
    || (0xFF0E..=0xFF0F).contains(&code_point) // Po
    || (0xFF1A..=0xFF1B).contains(&code_point) // Po
    || (0xFF1F..=0xFF20).contains(&code_point) // Po
    || code_point == 0xFF3B // Ps
    || code_point == 0xFF3C // Po
    || code_point == 0xFF3D // Pe
    || code_point == 0xFF3F // Pc
    || code_point == 0xFF5B // Ps
    || code_point == 0xFF5D // Pe
    || code_point == 0xFF5F // Ps
    || code_point == 0xFF60 // Pe
    || code_point == 0xFF61 // Po
    || code_point == 0xFF62 // Ps
    || code_point == 0xFF63 // Pe
    || (0xFF64..=0xFF65).contains(&code_point) // Po
    || (0x10100..=0x10102).contains(&code_point) // Po
    || code_point == 0xA673 // Po
    || code_point == 0xA67E // Po
    || (0x055A..=0x055F).contains(&code_point) // Po
    || code_point == 0x0589 // Po
    || code_point == 0x058A // Pd
    || code_point == 0x05BE // Pd
    || code_point == 0x05C0 // Po
    || code_point == 0x05C3 // Po
    || code_point == 0x05C6 // Po
    || (0x05F3..=0x05F4).contains(&code_point) // Po
    || (0x0609..=0x060A).contains(&code_point) // Po
    || code_point == 0x060D // Po
    || (0x061D..=0x061E).contains(&code_point) // Po
    || (0x066A..=0x066D).contains(&code_point) // Po
    || code_point == 0x06D4 // Po
    || (0x0700..=0x070D).contains(&code_point) // Po
    || code_point == 0x0970 // Po
    || (0xA8F8..=0xA8FA).contains(&code_point) // Po
    || code_point == 0xA8FC // Po
    || (0x11B00..=0x11B09).contains(&code_point) // Po
    || code_point == 0x09FD // Po
    || code_point == 0x0A76 // Po
    || code_point == 0x0AF0 // Po
    || code_point == 0x11FFF // Po
    || code_point == 0x0C77 // Po
    || code_point == 0x0C84 // Po
    || code_point == 0x0DF4 // Po
    || code_point == 0x0E4F // Po
    || (0x0E5A..=0x0E5B).contains(&code_point) // Po
    || (0x0F04..=0x0F12).contains(&code_point) // Po
    || code_point == 0x0F14 // Po
    || code_point == 0x0F3A // Ps
    || code_point == 0x0F3B // Pe
    || code_point == 0x0F3C // Ps
    || code_point == 0x0F3D // Pe
    || code_point == 0x0F85 // Po
    || (0x0FD0..=0x0FD4).contains(&code_point) // Po
    || (0x0FD9..=0x0FDA).contains(&code_point) // Po
    || (0x104A..=0x104F).contains(&code_point) // Po
    || (0x1360..=0x1368).contains(&code_point) // Po
    || code_point == 0x1400 // Pd
    || code_point == 0x166E // Po
    || code_point == 0x169B // Ps
    || code_point == 0x169C // Pe
    || (0x17D4..=0x17D6).contains(&code_point) // Po
    || (0x17D8..=0x17DA).contains(&code_point) // Po
    || (0x1800..=0x1801).contains(&code_point) // Po
    || code_point == 0x1804 // Po
    || code_point == 0x1806 // Pd
    || (0x1807..=0x180A).contains(&code_point) // Po
    || (0x11660..=0x1166C).contains(&code_point) // Po
    || code_point == 0x16FE2 // Po
    || (0x1944..=0x1945).contains(&code_point) // Po
    || code_point == 0x1039F // Po
    || (0x1A1E..=0x1A1F).contains(&code_point) // Po
    || (0x2CF9..=0x2CFC).contains(&code_point) // Po
    || (0x2CFE..=0x2CFF).contains(&code_point) // Po
    || code_point == 0x2D70 // Po
    || code_point == 0x103D0 // Po
    || (0x10A50..=0x10A58).contains(&code_point) // Po
    || (0x1B5A..=0x1B60).contains(&code_point) // Po
    || (0x1B7D..=0x1B7E).contains(&code_point) // Po
    || (0x12470..=0x12474).contains(&code_point) // Po
    || code_point == 0x1091F // Po
    || (0xA874..=0xA877).contains(&code_point) // Po
    || (0x07F7..=0x07F9).contains(&code_point) // Po
    || (0x1CC0..=0x1CC7).contains(&code_point) // Po
    || (0x1C3B..=0x1C3F).contains(&code_point) // Po
    || (0x1C7E..=0x1C7F).contains(&code_point) // Po
    || (0xA60D..=0xA60F).contains(&code_point) // Po
    || (0xA8CE..=0xA8CF).contains(&code_point) // Po
    || code_point == 0xA92F // Po
    || code_point == 0xA95F // Po
    || code_point == 0x1093F // Po
    || (0xAA5C..=0xAA5F).contains(&code_point) // Po
    || (0x1AA0..=0x1AA6).contains(&code_point) // Po
    || (0x1AA8..=0x1AAD).contains(&code_point) // Po
    || (0xAADE..=0xAADF).contains(&code_point) // Po
    || (0x10B39..=0x10B3F).contains(&code_point) // Po
    || (0x0830..=0x083E).contains(&code_point) // Po
    || (0xA4FE..=0xA4FF).contains(&code_point) // Po
    || (0xA6F2..=0xA6F7).contains(&code_point) // Po
    || (0xA9C1..=0xA9CD).contains(&code_point) // Po
    || (0xA9DE..=0xA9DF).contains(&code_point) // Po
    || (0xAAF0..=0xAAF1).contains(&code_point) // Po
    || code_point == 0xABEB // Po
    || code_point == 0x10857 // Po
    || code_point == 0x10A7F // Po
    || (0x110BB..=0x110BC).contains(&code_point) // Po
    || (0x110BE..=0x110C1).contains(&code_point) // Po
    || (0x1BFC..=0x1BFF).contains(&code_point) // Po
    || (0x11047..=0x1104D).contains(&code_point) // Po
    || code_point == 0x085E // Po
    || (0x11140..=0x11143).contains(&code_point) // Po
    || (0x111C5..=0x111C8).contains(&code_point) // Po
    || code_point == 0x111CD // Po
    || code_point == 0x111DB // Po
    || (0x111DD..=0x111DF).contains(&code_point) // Po
    || code_point == 0x116B9 // Po
    || code_point == 0x1056F // Po
    || code_point == 0x16AF5 // Po
    || code_point == 0x1BC9F // Po
    || (0x16B37..=0x16B3B).contains(&code_point) // Po
    || code_point == 0x16B44 // Po
    || (0x11238..=0x1123D).contains(&code_point) // Po
    || (0x11174..=0x11175).contains(&code_point) // Po
    || (0x10AF0..=0x10AF6).contains(&code_point) // Po
    || (0x11641..=0x11643).contains(&code_point) // Po
    || (0x16A6E..=0x16A6F).contains(&code_point) // Po
    || (0x10B99..=0x10B9C).contains(&code_point) // Po
    || (0x115C1..=0x115D7).contains(&code_point) // Po
    || code_point == 0x114C6 // Po
    || (0x1173C..=0x1173E).contains(&code_point) // Po
    || code_point == 0x112A9 // Po
    || (0x1DA87..=0x1DA8B).contains(&code_point) // Po
    || (0x1E95E..=0x1E95F).contains(&code_point) // Po
    || (0x11C41..=0x11C45).contains(&code_point) // Po
    || (0x11C70..=0x11C71).contains(&code_point) // Po
    || (0x1144B..=0x1144F).contains(&code_point) // Po
    || (0x1145A..=0x1145B).contains(&code_point) // Po
    || code_point == 0x1145D // Po
    || (0x11A9A..=0x11A9C).contains(&code_point) // Po
    || (0x11A9E..=0x11AA2).contains(&code_point) // Po
    || (0x11A3F..=0x11A46).contains(&code_point) // Po
    || code_point == 0x1183B // Po
    || (0x11EF7..=0x11EF8).contains(&code_point) // Po
    || (0x16E97..=0x16E9A).contains(&code_point) // Po
    || (0x10F55..=0x10F59).contains(&code_point) // Po
    || code_point == 0x119E2 // Po
    || (0x11944..=0x11946).contains(&code_point) // Po
    || code_point == 0x10EAD // Pd
    || (0x12FF1..=0x12FF2).contains(&code_point) // Po
    || (0x10F86..=0x10F89).contains(&code_point) // Po
    || (0x11F43..=0x11F4F).contains(&code_point) // Po

    //  Unicode S (symbol)
    || code_point == 0x0024 // Sc
    || code_point == 0x002B // Sm
    || (0x003C..=0x003E).contains(&code_point) // Sm
    || code_point == 0x005E // Sk
    || code_point == 0x0060 // Sk
    || code_point == 0x007C // Sm
    || code_point == 0x007E // Sm
    || (0x00A2..=0x00A5).contains(&code_point) // Sc
    || code_point == 0x00A6 // So
    || code_point == 0x00A8 // Sk
    || code_point == 0x00A9 // So
    || code_point == 0x00AC // Sm
    || code_point == 0x00AE // So
    || code_point == 0x00AF // Sk
    || code_point == 0x00B0 // So
    || code_point == 0x00B1 // Sm
    || code_point == 0x00B4 // Sk
    || code_point == 0x00B8 // Sk
    || code_point == 0x00D7 // Sm
    || code_point == 0x00F7 // Sm
    || (0x02C2..=0x02C5).contains(&code_point) // Sk
    || (0x02D2..=0x02DF).contains(&code_point) // Sk
    || (0x02E5..=0x02E9).contains(&code_point) // Sk
    || code_point == 0x02ED // Sk
    || (0x02EF..=0x02FF).contains(&code_point) // Sk
    || code_point == 0x0385 // Sk
    || code_point == 0x0E3F // Sc
    || (0x0FD5..=0x0FD8).contains(&code_point) // So
    || code_point == 0x2044 // Sm
    || code_point == 0x2052 // Sm
    || (0x207A..=0x207C).contains(&code_point) // Sm
    || (0x208A..=0x208C).contains(&code_point) // Sm
    || (0x20A0..=0x20C0).contains(&code_point) // Sc
    || (0x2100..=0x2101).contains(&code_point) // So
    || (0x2103..=0x2106).contains(&code_point) // So
    || (0x2108..=0x2109).contains(&code_point) // So
    || code_point == 0x2114 // So
    || (0x2116..=0x2117).contains(&code_point) // So
    || code_point == 0x2118 // Sm
    || (0x211E..=0x2123).contains(&code_point) // So
    || code_point == 0x2125 // So
    || code_point == 0x2127 // So
    || code_point == 0x2129 // So
    || code_point == 0x212E // So
    || (0x213A..=0x213B).contains(&code_point) // So
    || (0x2140..=0x2144).contains(&code_point) // Sm
    || code_point == 0x214A // So
    || code_point == 0x214B // Sm
    || (0x214C..=0x214D).contains(&code_point) // So
    || code_point == 0x214F // So
    || (0x218A..=0x218B).contains(&code_point) // So
    || (0x2190..=0x2194).contains(&code_point) // Sm
    || (0x2195..=0x2199).contains(&code_point) // So
    || (0x219A..=0x219B).contains(&code_point) // Sm
    || (0x219C..=0x219F).contains(&code_point) // So
    || code_point == 0x21A0 // Sm
    || (0x21A1..=0x21A2).contains(&code_point) // So
    || code_point == 0x21A3 // Sm
    || (0x21A4..=0x21A5).contains(&code_point) // So
    || code_point == 0x21A6 // Sm
    || (0x21A7..=0x21AD).contains(&code_point) // So
    || code_point == 0x21AE // Sm
    || (0x21AF..=0x21CD).contains(&code_point) // So
    || (0x21CE..=0x21CF).contains(&code_point) // Sm
    || (0x21D0..=0x21D1).contains(&code_point) // So
    || code_point == 0x21D2 // Sm
    || code_point == 0x21D3 // So
    || code_point == 0x21D4 // Sm
    || (0x21D5..=0x21F3).contains(&code_point) // So
    || (0x21F4..=0x22FF).contains(&code_point) // Sm
    || (0x2300..=0x2307).contains(&code_point) // So
    || (0x230C..=0x231F).contains(&code_point) // So
    || (0x2320..=0x2321).contains(&code_point) // Sm
    || (0x2322..=0x2328).contains(&code_point) // So
    || (0x232B..=0x237B).contains(&code_point) // So
    || code_point == 0x237C // Sm
    || (0x237D..=0x239A).contains(&code_point) // So
    || (0x239B..=0x23B3).contains(&code_point) // Sm
    || (0x23B4..=0x23DB).contains(&code_point) // So
    || (0x23DC..=0x23E1).contains(&code_point) // Sm
    || (0x23E2..=0x2426).contains(&code_point) // So
    || (0x2440..=0x244A).contains(&code_point) // So
    || (0x249C..=0x24E9).contains(&code_point) // So
    || (0x2500..=0x25B6).contains(&code_point) // So
    || code_point == 0x25B7 // Sm
    || (0x25B8..=0x25C0).contains(&code_point) // So
    || code_point == 0x25C1 // Sm
    || (0x25C2..=0x25F7).contains(&code_point) // So
    || (0x25F8..=0x25FF).contains(&code_point) // Sm
    || (0x2600..=0x266E).contains(&code_point) // So
    || code_point == 0x266F // Sm
    || (0x2670..=0x2767).contains(&code_point) // So
    || (0x2794..=0x27BF).contains(&code_point) // So
    || (0x27C0..=0x27C4).contains(&code_point) // Sm
    || (0x27C7..=0x27E5).contains(&code_point) // Sm
    || (0x27F0..=0x27FF).contains(&code_point) // Sm
    || (0x2900..=0x2982).contains(&code_point) // Sm
    || (0x2999..=0x29D7).contains(&code_point) // Sm
    || (0x29DC..=0x29FB).contains(&code_point) // Sm
    || (0x29FE..=0x2AFF).contains(&code_point) // Sm
    || (0x2B00..=0x2B2F).contains(&code_point) // So
    || (0x2B30..=0x2B44).contains(&code_point) // Sm
    || (0x2B45..=0x2B46).contains(&code_point) // So
    || (0x2B47..=0x2B4C).contains(&code_point) // Sm
    || (0x2B4D..=0x2B73).contains(&code_point) // So
    || (0x2B76..=0x2B95).contains(&code_point) // So
    || (0x2B97..=0x2BFF).contains(&code_point) // So
    || (0x2E50..=0x2E51).contains(&code_point) // So
    || (0x2FF0..=0x2FFF).contains(&code_point) // So
    || code_point == 0x3004 // So
    || (0x3012..=0x3013).contains(&code_point) // So
    || code_point == 0x3020 // So
    || (0x3036..=0x3037).contains(&code_point) // So
    || (0x303E..=0x303F).contains(&code_point) // So
    || (0x309B..=0x309C).contains(&code_point) // Sk
    || (0x3190..=0x3191).contains(&code_point) // So
    || (0x3196..=0x319F).contains(&code_point) // So
    || (0x31C0..=0x31E3).contains(&code_point) // So
    || code_point == 0x31EF // So
    || (0x322A..=0x3247).contains(&code_point) // So
    || code_point == 0x3250 // So
    || code_point == 0x327F // So
    || (0x328A..=0x32B0).contains(&code_point) // So
    || (0x32C0..=0x32CF).contains(&code_point) // So
    || code_point == 0x32FF // So
    || (0x3358..=0x33FF).contains(&code_point) // So
    || (0x4DC0..=0x4DFF).contains(&code_point) // So
    || (0xA700..=0xA716).contains(&code_point) // Sk
    || (0xA720..=0xA721).contains(&code_point) // Sk
    || (0xA789..=0xA78A).contains(&code_point) // Sk
    || (0xA836..=0xA837).contains(&code_point) // So
    || code_point == 0xA838 // Sc
    || code_point == 0xA839 // So
    || code_point == 0xAB5B // Sk
    || (0xAB6A..=0xAB6B).contains(&code_point) // Sk
    || code_point == 0xFE62 // Sm
    || (0xFE64..=0xFE66).contains(&code_point) // Sm
    || code_point == 0xFE69 // Sc
    || code_point == 0xFF04 // Sc
    || code_point == 0xFF0B // Sm
    || (0xFF1C..=0xFF1E).contains(&code_point) // Sm
    || code_point == 0xFF3E // Sk
    || code_point == 0xFF40 // Sk
    || code_point == 0xFF5C // Sm
    || code_point == 0xFF5E // Sm
    || (0xFFE0..=0xFFE1).contains(&code_point) // Sc
    || code_point == 0xFFE2 // Sm
    || code_point == 0xFFE3 // Sk
    || code_point == 0xFFE4 // So
    || (0xFFE5..=0xFFE6).contains(&code_point) // Sc
    || code_point == 0xFFE8 // So
    || (0xFFE9..=0xFFEC).contains(&code_point) // Sm
    || (0xFFED..=0xFFEE).contains(&code_point) // So
    || (0xFFFC..=0xFFFD).contains(&code_point) // So
    || (0x10137..=0x1013F).contains(&code_point) // So
    || (0x10190..=0x1019C).contains(&code_point) // So
    || (0x101D0..=0x101FC).contains(&code_point) // So
    || (0x1CF50..=0x1CFC3).contains(&code_point) // So
    || (0x1D000..=0x1D0F5).contains(&code_point) // So
    || (0x1D100..=0x1D126).contains(&code_point) // So
    || (0x1D129..=0x1D164).contains(&code_point) // So
    || (0x1D16A..=0x1D16C).contains(&code_point) // So
    || (0x1D183..=0x1D184).contains(&code_point) // So
    || (0x1D18C..=0x1D1A9).contains(&code_point) // So
    || (0x1D1AE..=0x1D1EA).contains(&code_point) // So
    || (0x1D300..=0x1D356).contains(&code_point) // So
    || code_point == 0x1D6C1 // Sm
    || code_point == 0x1D6DB // Sm
    || code_point == 0x1D6FB // Sm
    || code_point == 0x1D715 // Sm
    || code_point == 0x1D735 // Sm
    || code_point == 0x1D74F // Sm
    || code_point == 0x1D76F // Sm
    || code_point == 0x1D789 // Sm
    || code_point == 0x1D7A9 // Sm
    || code_point == 0x1D7C3 // Sm
    || code_point == 0x1ECAC // So
    || code_point == 0x1ECB0 // Sc
    || code_point == 0x1ED2E // So
    || (0x1F000..=0x1F02B).contains(&code_point) // So
    || (0x1F030..=0x1F093).contains(&code_point) // So
    || (0x1F0A0..=0x1F0AE).contains(&code_point) // So
    || (0x1F0B1..=0x1F0BF).contains(&code_point) // So
    || (0x1F0C1..=0x1F0CF).contains(&code_point) // So
    || (0x1F0D1..=0x1F0F5).contains(&code_point) // So
    || (0x1F10D..=0x1F1AD).contains(&code_point) // So
    || (0x1F1E6..=0x1F1FF).contains(&code_point) // So
    || (0x1F201..=0x1F202).contains(&code_point) // So
    || (0x1F210..=0x1F23B).contains(&code_point) // So
    || (0x1F240..=0x1F248).contains(&code_point) // So
    || (0x1F250..=0x1F251).contains(&code_point) // So
    || (0x1F260..=0x1F265).contains(&code_point) // So
    || (0x1F300..=0x1F3FA).contains(&code_point) // So
    || (0x1F3FB..=0x1F3FF).contains(&code_point) // Sk
    || (0x1F400..=0x1F6D7).contains(&code_point) // So
    || (0x1F6DC..=0x1F6EC).contains(&code_point) // So
    || (0x1F6F0..=0x1F6FC).contains(&code_point) // So
    || (0x1F700..=0x1F776).contains(&code_point) // So
    || (0x1F77B..=0x1F7D9).contains(&code_point) // So
    || (0x1F7E0..=0x1F7EB).contains(&code_point) // So
    || code_point == 0x1F7F0 // So
    || (0x1F800..=0x1F80B).contains(&code_point) // So
    || (0x1F810..=0x1F847).contains(&code_point) // So
    || (0x1F850..=0x1F859).contains(&code_point) // So
    || (0x1F860..=0x1F887).contains(&code_point) // So
    || (0x1F890..=0x1F8AD).contains(&code_point) // So
    || (0x1F8B0..=0x1F8B1).contains(&code_point) // So
    || (0x1F900..=0x1FA53).contains(&code_point) // So
    || (0x1FA60..=0x1FA6D).contains(&code_point) // So
    || (0x1FA70..=0x1FA7C).contains(&code_point) // So
    || (0x1FA80..=0x1FA88).contains(&code_point) // So
    || (0x1FA90..=0x1FABD).contains(&code_point) // So
    || (0x1FABF..=0x1FAC5).contains(&code_point) // So
    || (0x1FACE..=0x1FADB).contains(&code_point) // So
    || (0x1FAE0..=0x1FAE8).contains(&code_point) // So
    || (0x1FAF0..=0x1FAF8).contains(&code_point) // So
    || (0x1FB00..=0x1FB92).contains(&code_point) // So
    || (0x1FB94..=0x1FBCA).contains(&code_point) // So
    || code_point == 0x0375 // Sk
    || code_point == 0x0384 // Sk
    || code_point == 0x03F6 // Sm
    || code_point == 0x1FBD // Sk
    || (0x1FBF..=0x1FC1).contains(&code_point) // Sk
    || (0x1FCD..=0x1FCF).contains(&code_point) // Sk
    || (0x1FDD..=0x1FDF).contains(&code_point) // Sk
    || (0x1FED..=0x1FEF).contains(&code_point) // Sk
    || (0x1FFD..=0x1FFE).contains(&code_point) // Sk
    || (0x10179..=0x10189).contains(&code_point) // So
    || (0x1018C..=0x1018E).contains(&code_point) // So
    || code_point == 0x101A0 // So
    || (0x1D200..=0x1D241).contains(&code_point) // So
    || code_point == 0x1D245 // So
    || code_point == 0x0482 // So
    || (0x058D..=0x058E).contains(&code_point) // So
    || code_point == 0x058F // Sc
    || code_point == 0xFB29 // Sm
    || (0x0606..=0x0608).contains(&code_point) // Sm
    || code_point == 0x060B // Sc
    || (0x060E..=0x060F).contains(&code_point) // So
    || code_point == 0x06DE // So
    || code_point == 0x06E9 // So
    || (0x06FD..=0x06FE).contains(&code_point) // So
    || code_point == 0x0888 // Sk
    || (0xFBB2..=0xFBC2).contains(&code_point) // Sk
    || (0xFD40..=0xFD4F).contains(&code_point) // So
    || code_point == 0xFDCF // So
    || code_point == 0xFDFC // Sc
    || (0xFDFD..=0xFDFF).contains(&code_point) // So
    || (0x1EEF0..=0x1EEF1).contains(&code_point) // Sm
    || (0x09F2..=0x09F3).contains(&code_point) // Sc
    || code_point == 0x09FA // So
    || code_point == 0x09FB // Sc
    || code_point == 0x0AF1 // Sc
    || code_point == 0x0B70 // So
    || (0x0BF3..=0x0BF8).contains(&code_point) // So
    || code_point == 0x0BF9 // Sc
    || code_point == 0x0BFA // So
    || (0x11FD5..=0x11FDC).contains(&code_point) // So
    || (0x11FDD..=0x11FE0).contains(&code_point) // Sc
    || (0x11FE1..=0x11FF1).contains(&code_point) // So
    || code_point == 0x0C7F // So
    || code_point == 0x0D4F // So
    || code_point == 0x0D79 // So
    || (0x0F01..=0x0F03).contains(&code_point) // So
    || code_point == 0x0F13 // So
    || (0x0F15..=0x0F17).contains(&code_point) // So
    || (0x0F1A..=0x0F1F).contains(&code_point) // So
    || code_point == 0x0F34 // So
    || code_point == 0x0F36 // So
    || code_point == 0x0F38 // So
    || (0x0FBE..=0x0FC5).contains(&code_point) // So
    || (0x0FC7..=0x0FCC).contains(&code_point) // So
    || (0x0FCE..=0x0FCF).contains(&code_point) // So
    || (0x109E..=0x109F).contains(&code_point) // So
    || (0xAA77..=0xAA79).contains(&code_point) // So
    || (0x3200..=0x321E).contains(&code_point) // So
    || (0x3260..=0x327E).contains(&code_point) // So
    || (0x1390..=0x1399).contains(&code_point) // So
    || code_point == 0x166D // So
    || code_point == 0x17DB // Sc
    || (0x19E0..=0x19FF).contains(&code_point) // So
    || code_point == 0x1F200 // So
    || (0x32D0..=0x32FE).contains(&code_point) // So
    || (0x3300..=0x3357).contains(&code_point) // So
    || (0x02EA..=0x02EB).contains(&code_point) // Sk
    || (0x2E80..=0x2E99).contains(&code_point) // So
    || (0x2E9B..=0x2EF3).contains(&code_point) // So
    || (0x2F00..=0x2FD5).contains(&code_point) // So
    || (0xA490..=0xA4C6).contains(&code_point) // So
    || code_point == 0x1940 // So
    || (0x2800..=0x28FF).contains(&code_point) // So
    || (0x2CE5..=0x2CEA).contains(&code_point) // So
    || (0x19DE..=0x19DF).contains(&code_point) // So
    || (0xA828..=0xA82B).contains(&code_point) // So
    || (0x1B61..=0x1B6A).contains(&code_point) // So
    || (0x1B74..=0x1B7C).contains(&code_point) // So
    || code_point == 0x07F6 // So
    || (0x07FE..=0x07FF).contains(&code_point) // Sc
    || code_point == 0x1BC9C // So
    || (0x16B3C..=0x16B3F).contains(&code_point) // So
    || code_point == 0x16B45 // So
    || code_point == 0x10AC8 // So
    || (0x10877..=0x10878).contains(&code_point) // So
    || code_point == 0x1173F // So
    || (0x1D800..=0x1D9FF).contains(&code_point) // So
    || (0x1DA37..=0x1DA3A).contains(&code_point) // So
    || (0x1DA6D..=0x1DA74).contains(&code_point) // So
    || (0x1DA76..=0x1DA83).contains(&code_point) // So
    || (0x1DA85..=0x1DA86).contains(&code_point) // So
    || code_point == 0x1E14F // So
    || code_point == 0x1E2FF // Sc
}
