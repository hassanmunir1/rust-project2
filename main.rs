use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

// Function to count word occurrences
fn count_word_occurrences(content: &str) -> HashMap<String, i32> {
    let mut word_counts: HashMap<String, i32> = HashMap::new();
    let words: Vec<&str> = content.split_whitespace().collect();
    for word in words {
        let normalized_word = word.to_lowercase();
        let count = word_counts.entry(normalized_word).or_insert(0);
        *count += 1;
    }
    word_counts
}

// Function to count word occurrences from a file
fn count_word_occurrences_from_file(file_path: &str) -> Result<HashMap<String, i32>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n');
    }
    Ok(count_word_occurrences(&content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let content = "apple banana Apple cherry cherry";
        let word_counts = count_word_occurrences(content);
        assert_eq!(word_counts.get("apple"), Some(&2));
        assert_eq!(word_counts.get("banana"), Some(&1));
        assert_eq!(word_counts.get("cherry"), Some(&2));
        assert_eq!(word_counts.get("orange"), None);
    }

    #[test]
    fn test_word_count_from_file() {
        let content = "apple banana Apple cherry cherry";
        std::fs::write("test_file.txt", content).unwrap();
        let word_counts = count_word_occurrences_from_file("test_file.txt").unwrap();
        assert_eq!(word_counts.get("apple"), Some(&2));
        assert_eq!(word_counts.get("banana"), Some(&1));
        assert_eq!(word_counts.get("cherry"), Some(&2));
        assert_eq!(word_counts.get("orange"), None);
    }
}

fn main() -> Result<()> {
    let word_counts = count_word_occurrences_from_file("example.txt")?;
    for (word, count) in &word_counts {
        println!("Word: {}, Occurrences: {}", word, count);
    }
    Ok(())

}
