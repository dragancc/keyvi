#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(test)]
#![crate_type = "lib"]

extern crate serde_json;
extern crate rayon;
extern crate rand;


extern crate test;


pub mod dictionary;
pub mod keyvi_string;
pub mod keyvi_match;
pub mod keyvi_match_iterator;
mod bindings;


#[cfg(test)]
mod tests {
    use dictionary::Dictionary;
    use serde_json::{self, Value};
    use rayon::prelude::*;
    use test::Bencher;
    use rand::{thread_rng, sample};


    #[test]
    fn dictionary_error() {
        let dict = Dictionary::new("fake_file_name.kv");
        assert!(dict.is_err());
    }

    #[test]
    fn dictionary_size() {
        let dict = Dictionary::new("test.kv").unwrap();
        assert_eq!(dict.size(), 3);
    }

    #[test]
    fn match_string() {
        let m = Dictionary::new("test.kv").unwrap().get("a");
        assert_eq!(m.matched_string(), "a");
    }

    #[test]
    fn match_value_int() {
        let m = Dictionary::new("completion_test.kv").unwrap().get("mozilla footprint");
        match m.get_value() {
            Value::Number(n) => assert_eq!(n.as_i64().unwrap(), 30),
            _ => assert!(false)
        }
    }

    #[test]
    fn match_value_array() {
        let m = Dictionary::new("test.kv").unwrap().get("a");
        match m.get_value() {
            Value::Array(n) => assert_eq!(n, vec![12, 13]),
            _ => assert!(false)
        }
    }

    #[test]
    fn match_value() {
        let m = Dictionary::new("test.kv").unwrap().get("a");
        assert_eq!(m.get_value_as_string(), "[12,13]");
    }

    #[test]
    fn match_is_empty() {
        let m = Dictionary::new("test.kv").unwrap().get("a");
        assert_eq!(m.is_empty(), false);
    }

    #[test]
    fn match_iterator_count() {
        let mit = Dictionary::new("test.kv").unwrap().get_prefix_completions("a", 10);
        assert_eq!(mit.count(), 1);
    }

    #[test]
    fn match_iterator_values() {
        let mit = Dictionary::new("test.kv").unwrap().get_prefix_completions("a", 10);
        for m in mit {
            assert_eq!(m.matched_string(), "a");
            assert_eq!(m.get_value_as_string(), "[12,13]");
        }
    }

    #[test]
    fn match_iterator_into() {
        for m in Dictionary::new("test.kv").unwrap().get_prefix_completions("a", 10) {
            let (k, v) = m.into();
            assert_eq!(k, "a");

            match v {
                Value::Array(n) => assert_eq!(n, vec![12, 13]),
                _ => assert!(false)
            }
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

        let mit = Dictionary::new("completion_test.kv").unwrap().get_multi_word_completions("mozilla f", 10);
        let mut a: Vec<(String, String)> = mit.map(|m| (m.get_value_as_string(), m.matched_string())).collect();
        a.sort();

        assert_eq!(new_values, a);
    }

    #[test]
    fn multi_word_completions_cutoff() {
        let mut values = vec![
            ("80", "mozilla firefox"),
        ];
        values.sort();
        let new_values: Vec<(String, String)> = values.into_iter().map(|(x, y)| (x.into(), y.into())).collect();

        let mit = Dictionary::new("completion_test.kv").unwrap().get_multi_word_completions("mozilla f", 1);
        let mut a: Vec<(String, String)> = mit.map(|m| (m.get_value_as_string(), m.matched_string())).collect();
        a.sort();

        assert_eq!(new_values, a);
    }
    
    #[bench]
    fn multithread_get_performance(b: &mut Bencher) {
	let dicts:Vec<String> = (0..10).map(|i| format!("/raid/clq/word_embeddings/2017-05-30T08-26-55/word_embeddings.kv-{}", i)).collect();
	let kvs:Vec<Dictionary> = dicts.iter().map(|f| Dictionary::new(f).unwrap()).collect();
	let keys = include_str!("../keys");
	let parsed_keys:Vec<String> = serde_json::from_str(keys).unwrap();
	let mut rng = thread_rng();
        let shards: Vec<u32>= (0..10000).map(|i| sample(&mut rng, 1..10, 1)[0]).collect();
	b.iter(||  {
		let m = parsed_keys.iter().enumerate().map(|(i, k)| kvs[shards[i] as usize].get(k));
		let res:Vec<String> = m.map(|i| i.get_value_as_string()).collect();
	});
	
    }

}
