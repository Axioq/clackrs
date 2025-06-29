use rand::{SeedableRng};
use rand::seq::IndexedRandom; 
use rand_chacha::ChaCha8Rng;
use std::fs;
use std::path::Path;

pub fn get_random_words(count: usize, seed: u64, list_name: &str) -> Vec<String> {
    let path = format!("assets/words_{}.txt", list_name);
    let path_obj = Path::new(&path);

    if !path_obj.exists() {
        panic!("Word list file does not exist: {}", path);
    }

    let contents = fs::read_to_string(path_obj)
        .unwrap_or_else(|_| panic!("Failed to read word list: {}", path));

    let all_words: Vec<&str> = contents.lines().collect();

    if all_words.is_empty() {
        panic!("Word list file is empty: {}", path);
    }

    println!("Loaded {} words from {}", all_words.len(), path);

    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    all_words
        .choose_multiple(&mut rng, count)
        .map(|s| s.to_string())
        .collect()
}

pub fn list_available_word_lists() -> Vec<String> {
    let assets_dir = Path::new("assets");
    let mut lists = Vec::new();

    if let Ok(entries) = fs::read_dir(assets_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "txt" {
                        if let Some(stem) = path.file_stem() {
                            if let Some(stem_str) = stem.to_str() {
                                lists.push(stem_str.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    lists
}