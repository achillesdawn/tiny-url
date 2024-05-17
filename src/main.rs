#![allow(non_snake_case)]

use std::{collections::HashMap, str::FromStr};

struct Codec {
    base62: Vec<char>,
    decoder: HashMap<char, usize>,
    current_id: usize,
    base_url: String,
    database: HashMap<usize, TinyUrl>,
    reverse_lookup: HashMap<String, usize>
}

struct TinyUrl {
    id: usize,
    long_url: String,
    tiny_url: String,
}

impl Codec {
    fn new() -> Self {
        let base62: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .chars()
            .map(|item| item)
            .collect();

        let mut decoder = HashMap::with_capacity(base62.len());

        base62.iter().enumerate().for_each(|(idx, value)| {
            decoder.insert(*value, idx);
        });

        Codec {
            base62,
            decoder,
            current_id: 1,
            database: HashMap::with_capacity(1024),
            reverse_lookup: HashMap::new(),
            base_url: String::from_str("http://tinyurl.com/").unwrap()
        }
    }

    fn insert(&mut self, longURL: String, tinyURL: &str)  {

        self.reverse_lookup.insert(longURL.clone(), self.current_id.clone());
        
        let item = TinyUrl {
            id: self.current_id.clone(),
            long_url: longURL,
            tiny_url: tinyURL.to_owned()
        };

        self.database.insert(item.id, item );
        self.current_id += 1;

    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String {

        if self.reverse_lookup.contains_key(&longURL) {
            let id = self.reverse_lookup.get(&longURL).unwrap();
            let tiny_url = self.database.get(id).unwrap();
            return self.base_url.clone() + &tiny_url.tiny_url;
        }

        let mut id = self.current_id.clone();

        let mut tiny_url = String::with_capacity(7);

        while id != 0 {
            let index = id.rem_euclid(62);
            let item = self.base62.get(index).unwrap();
            tiny_url.push(*item);

            id /= 62;
        }

        self.insert(longURL, tiny_url.as_str());
        self.base_url.clone() + &tiny_url
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        let coded = shortURL.split("/").last().unwrap();

        let mut id = 0usize;
        for (idx, item) in coded.chars().enumerate() {
            let value = self.decoder.get(&item).unwrap();

            id += (idx * 62) + value;
        }

        let tiny_url = self.database.get(&id).unwrap();

        tiny_url.long_url.clone()
    }
}


fn main() {
    let mut codec = Codec::new();
    let tiny_url = codec.encode("https://www.google.com".to_owned());

    println!("encoded: {}", tiny_url);

    let result = codec.decode(tiny_url);

    println!("decoded {}", result);
}
