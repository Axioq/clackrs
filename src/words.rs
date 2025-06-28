use rand::prelude::*;

pub fn get_random_words(count: usize) -> Vec<&'static str> {
    const WORDS: &[&str] = &[
        "apple", "banana", "cat", "dog", "elephant", "fast", "green", "house", "ice", "jump",
        "kite", "lion", "mouse", "night", "open", "purple", "queen", "rain", "sun", "tree",
    ];

    let mut rng = rand::rng();
    WORDS.choose_multiple(&mut rng, count).cloned().collect()
}