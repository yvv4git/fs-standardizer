/// Transliterates non-ASCII characters to ASCII equivalents.
/// Cyrillic → Latin transliteration, other non-ASCII → removed.

pub fn transliterate(text: &str) -> String {
    let mut result = String::new();
    
    for ch in text.chars() {
        let trans = match ch {
            // Cyrillic → Latin transliteration
            'А' | 'а' => "a",
            'Б' | 'б' => "b",
            'В' | 'в' => "v",
            'Г' | 'г' => "g",
            'Д' | 'д' => "d",
            'Е' | 'е' => "e",
            'Ё' | 'ё' => "yo",
            'Ж' | 'ж' => "zh",
            'З' | 'з' => "z",
            'И' | 'и' => "i",
            'Й' | 'й' => "y",
            'К' | 'к' => "k",
            'Л' | 'л' => "l",
            'М' | 'м' => "m",
            'Н' | 'н' => "n",
            'О' | 'о' => "o",
            'П' | 'п' => "p",
            'Р' | 'р' => "r",
            'С' | 'с' => "s",
            'Т' | 'т' => "t",
            'У' | 'у' => "u",
            'Ф' | 'ф' => "f",
            'Х' | 'х' => "h",
            'Ц' | 'ц' => "ts",
            'Ч' | 'ч' => "ch",
            'Ш' | 'ш' => "sh",
            'Щ' | 'щ' => "sch",
            'Ъ' | 'ъ' => "",
            'Ы' | 'ы' => "y",
            'Ь' | 'ь' => "",
            'Э' | 'э' => "e",
            'Ю' | 'ю' => "yu",
            'Я' | 'я' => "ya",
            // Keep ASCII characters as-is
            _ if ch.is_ascii() => "",
            // Remove all other non-ASCII characters
            _ => "",
        };
        // For ASCII, add the character itself
        if ch.is_ascii() {
            result.push(ch);
        } else if !trans.is_empty() {
            result.push_str(trans);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cyrillic_transliteration() {
        assert_eq!(transliterate("Привет"), "privet");
        assert_eq!(transliterate("Файл"), "fayl");
    }
    
    #[test]
    fn test_ascii_unchanged() {
        assert_eq!(transliterate("hello_world"), "hello_world");
    }
    
    #[test]
    fn test_mixed() {
        assert_eq!(transliterate("file_Привет.txt"), "file_privet.txt");
    }

    #[test]
    fn test_keeps_english() {
        assert_eq!(transliterate("avto_Kolya Funk"), "avto_Kolya Funk");
    }
}
