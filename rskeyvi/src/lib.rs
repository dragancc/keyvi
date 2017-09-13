#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![crate_type = "lib"]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


pub mod dictionary;
pub mod keyvi_string;
pub mod keyvi_match;
pub mod keyvi_match_iterator;
mod bindings;

use dictionary::Dictionary;
use keyvi_match::KeyviMatch;

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn dictionary_size() {
        let dict = Dictionary::new("test.kv");
        assert_eq!(dict.size(), 3);
    }

    #[test]
    fn match_string() {
        let m = Dictionary::new("test.kv").get("a");
        assert_eq!(m.matched_string(), "a");
    }

    #[test]
    fn match_value() {
        let m = Dictionary::new("test.kv").get("a");
        assert_eq!(m.get_value(), "[12,13]");
    }

    #[test]
    fn match_is_empty() {
        let m = Dictionary::new("test.kv").get("a");
        assert_eq!(m.is_empty(), false);
    }

    #[test]
    fn match_iterator_count() {
        let mit = Dictionary::new("test.kv").get_prefix_completions("a");
        assert_eq!(mit.count(), 1);
    }

    #[test]
    fn match_iterator_values() {
        let mit = Dictionary::new("test.kv").get_prefix_completions("a");
        for m in mit {
            assert_eq!(m.matched_string(), "a");
            assert_eq!(m.get_value(), "[12,13]");
        }
    }

    #[test]
    fn multi_word_completions() {
        let mut values = vec![
            ("80", "mozilla firefox"),
            ("43", "mozilla fans"),
            ("30", "mozilla footprint"),
            ("12", "mozilla firebird")
        ];
        values.sort();
        let new_values: Vec<(String, String)> = values.into_iter().map(|(x, y)| (x.into(), y.into())).collect();

        let mit = Dictionary::new("completion_test.kv").get_multi_word_completions("mozilla f");
        let mut a: Vec<(String, String)> = mit.map(|m| (m.get_value(), m.matched_string())).collect();
        a.sort();

        assert_eq!(new_values, a);
    }
}
