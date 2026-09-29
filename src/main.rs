fn main() {
    let mut text = String::from("Пик Балмера");
    reverse_words_at_odd_positions(&mut text);
    println!("{text}");
}

fn reverse_words_at_odd_positions(text: &mut String) {
    let mut search = 0;
    let mut position_is_odd = true;

    while search < text.len() {
        let (start, end) = {
            let rest = &text[search..];
            let Some(offset) = rest.find(|c: char| !c.is_whitespace()) else {
                break;
            };
            let start = search + offset;
            let end = match rest[offset..].find(char::is_whitespace) {
                Some(word_len) => start + word_len,
                None => text.len(),
            };
            (start, end)
        };

        if position_is_odd {
            let bytes = unsafe { text.as_mut_vec() };
            reverse_chars_in_place(&mut bytes[start..end]);
        }

        position_is_odd = !position_is_odd;
        search = end;
    }
}

fn reverse_chars_in_place(slice: &mut [u8]) {
    let mut start = 0;
    while start < slice.len() {
        let char_len = utf8_char_len(slice[start]);
        reverse_bytes(&mut slice[start..start + char_len]);
        start += char_len;
    }
    reverse_bytes(slice);
}

fn reverse_bytes(slice: &mut [u8]) {
    let mut left = 0;
    let mut right = slice.len();
    while left + 1 < right {
        right -= 1;
        slice.swap(left, right);
        left += 1;
    }
}

fn utf8_char_len(leading_byte: u8) -> usize {
    match leading_byte {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        _ => 4,
    }
}

#[cfg(test)]
#[path = "../tests/unit/monologue.rs"]
mod monologue_tests;
