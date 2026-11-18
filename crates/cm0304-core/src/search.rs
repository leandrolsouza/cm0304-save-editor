//! Local search key for the Latin-script names present in CM0304.
//! Preserve other scripts while folding Latin accents and combining marks.
pub fn key(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for c in text.chars().flat_map(char::to_lowercase) {
        match c {
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => result.push('a'),
            'ç' | 'ć' | 'č' | 'ĉ' | 'ċ' => result.push('c'),
            'ď' | 'đ' | 'ð' => result.push('d'),
            'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ė' | 'ę' | 'ě' | 'ĕ' => result.push('e'),
            'ĝ' | 'ğ' | 'ġ' | 'ģ' => result.push('g'),
            'ĥ' | 'ħ' => result.push('h'),
            'ì' | 'í' | 'î' | 'ï' | 'ī' | 'į' | 'ı' | 'ĭ' | 'ĩ' => result.push('i'),
            'ĵ' => result.push('j'),
            'ķ' => result.push('k'),
            'ĺ' | 'ļ' | 'ľ' | 'ł' | 'ŀ' => result.push('l'),
            'ñ' | 'ń' | 'ņ' | 'ň' => result.push('n'),
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ő' | 'ŏ' => result.push('o'),
            'ŕ' | 'ŗ' | 'ř' => result.push('r'),
            'ś' | 'ş' | 'š' | 'ŝ' | 'ș' => result.push('s'),
            'ţ' | 'ť' | 'ŧ' | 'ț' => result.push('t'),
            'ù' | 'ú' | 'û' | 'ü' | 'ū' | 'ů' | 'ű' | 'ŭ' | 'ũ' | 'ų' => result.push('u'),
            'ŵ' => result.push('w'),
            'ý' | 'ÿ' | 'ŷ' => result.push('y'),
            'ź' | 'ż' | 'ž' => result.push('z'),
            'æ' => result.push_str("ae"),
            'œ' => result.push_str("oe"),
            'ß' => result.push_str("ss"),
            '\u{0300}'..='\u{036f}' => {}
            _ => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::key;
    #[test]
    fn searches_latin_names_with_and_without_accents() {
        assert_eq!(key("João Águias São José"), "joao aguias sao jose");
        assert_eq!(key("Joa\u{0303}o"), key("João"));
        assert_eq!(key("Mølby Łódź Šimić"), "molby lodz simic");
        assert_eq!(key("ΠΑΟΚ 2001642"), "παοκ 2001642");
    }
}
