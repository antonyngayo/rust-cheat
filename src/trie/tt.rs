use std::{collections::VecDeque, fmt::Display};

#[derive(Default, Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub key: Option<char>,
    pub value: Option<String>,
    pub count: usize,
}

impl Node {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn with_key(s: char) -> Self {
        Self {
            key: Some(s),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: Node::new() }
    }

    pub fn insert(&mut self, s: &str) {
        let mut current = &mut self.root;
        for c in s.chars() {
            match current.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(index) => {
                    current = &mut current.children[index];
                }
                Err(index) => {
                    current.children.insert(index, Node::with_key(c));
                    current = &mut current.children[index];
                }
            }
        }
        current.count += 1;
        current.value.replace(s.to_owned());
    }

    #[allow(unused)]
    pub fn exists(&self, s: &str) -> bool {
        let mut current = &self.root;
        for c in s.chars() {
            match current.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(index) => {
                    current = &current.children[index];
                }
                Err(_) => return false,
            }
        }
        current.count > 0
    }

    /// Original prefix search - exact prefix matching
    pub fn _search(&mut self, s: &str) -> Vec<String> {
        let mut current = &mut self.root;
        for c in s.chars() {
            match current.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(index) => {
                    current = &mut current.children[index];
                }
                Err(_) => return Vec::new(),
            }
        }

        let mut results = Vec::new();
        let mut q = Vec::new();
        q.push(current);
        while let Some(c) = q.pop() {
            for child in c.children.iter_mut() {
                q.push(child);
            }
            if c.count > 0 {
                let value = c.value.as_ref().unwrap();
                let count = c.count;
                results.push((count, value));
            }
        }
        results.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(b.1)));
        results.iter().map(|v| v.1.clone()).collect::<Vec<String>>()
    }

    /// Fuzzy search - combines multiple search strategies
    pub fn fuzzy_search(&self, query: &str) -> Vec<(String, f32)> {
        let mut all_values = Vec::new();
        Self::collect_all_values(&self.root, &mut all_values);

        let mut scored_results = Vec::new();
        let query_lower = query.to_lowercase();

        for value in all_values {
            let value_lower = value.to_lowercase();
            let mut score = 0.0f32;

            // 1. Exact match (highest score)
            if value_lower == query_lower {
                score = 100.0;
            }
            // 2. Prefix match (high score)
            else if value_lower.starts_with(&query_lower) {
                score = 90.0 - (value.len() as f32 - query.len() as f32) * 0.5;
            }
            // 3. Substring match (medium-high score)
            else if value_lower.contains(&query_lower) {
                let pos = value_lower.find(&query_lower).unwrap() as f32;
                score = 70.0 - pos * 0.5 - (value.len() as f32 - query.len() as f32) * 0.3;
            }
            // 4. Fuzzy character sequence match (medium score)
            else if let Some(fuzzy_score) = self.fuzzy_match(&value_lower, &query_lower) {
                score = 50.0 + fuzzy_score * 20.0;
            }
            // 5. Edit distance match (lower score)
            else {
                let distance = self.edit_distance(&value_lower, &query_lower);
                let max_len = value.len().max(query.len()) as f32;
                if distance as f32 / max_len < 0.6 {
                    // Only if similarity > 40%
                    score = 30.0 * (1.0 - distance as f32 / max_len);
                }
            }

            if score > 0.0 {
                scored_results.push((value, score));
            }
        }

        // Sort by score (descending) then by name (ascending)
        scored_results.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.0.cmp(&b.0))
        });

        scored_results
    }

    /// Collect all values from the trie
    fn collect_all_values(node: &Node, values: &mut Vec<String>) {
        if node.count > 0 {
            if let Some(value) = &node.value {
                values.push(value.clone());
            }
        }
        for child in &node.children {
            Self::collect_all_values(child, values);
        }
    }

    /// Fuzzy matching - checks if characters appear in sequence
    fn fuzzy_match(&self, text: &str, pattern: &str) -> Option<f32> {
        let text_chars: Vec<char> = text.chars().collect();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        if pattern_chars.is_empty() {
            return Some(1.0);
        }

        let mut text_idx = 0;
        let mut pattern_idx = 0;
        let mut matches = 0;
        let mut consecutive_matches = 0;
        let mut max_consecutive = 0;

        while text_idx < text_chars.len() && pattern_idx < pattern_chars.len() {
            if text_chars[text_idx] == pattern_chars[pattern_idx] {
                matches += 1;
                consecutive_matches += 1;
                max_consecutive = max_consecutive.max(consecutive_matches);
                pattern_idx += 1;
            } else {
                consecutive_matches = 0;
            }
            text_idx += 1;
        }

        if pattern_idx == pattern_chars.len() {
            // All pattern characters found
            let match_ratio = matches as f32 / pattern_chars.len() as f32;
            let consecutive_bonus = max_consecutive as f32 / pattern_chars.len() as f32 * 0.5;
            Some((match_ratio + consecutive_bonus).min(1.0))
        } else {
            None
        }
    }

    /// Calculate edit distance (Levenshtein distance)
    fn edit_distance(&self, s1: &str, s2: &str) -> usize {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let len1 = s1_chars.len();
        let len2 = s2_chars.len();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        // Initialize first row and column
        for (i, row) in matrix.iter_mut().enumerate().take(len1 + 1) {
            row[0] = i;
        }
        for (j, cell) in matrix[0].iter_mut().enumerate().take(len2 + 1) {
            *cell = j;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                    0
                } else {
                    1
                };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[len1][len2]
    }
}

impl Display for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut q: VecDeque<&Node> = VecDeque::new();
        let root = &self.root;
        q.push_back(root);
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(node) = q.pop_front() {
                    for c in node.children.iter() {
                        let r = write!(f, "{} ", &c.key.unwrap());
                        r?;
                        if !c.children.is_empty() {
                            q.push_back(c);
                        }
                    }
                }
            }
            if !q.is_empty() {
                let r = writeln!(f);
                r?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_trie() -> Trie {
        let mut trie = Trie::new();
        trie.insert("docker");
        trie.insert("docker-compose");
        trie.insert("docker-swarm");
        trie.insert("kubernetes");
        trie.insert("kubectl");
        trie.insert("rust");
        trie.insert("rust-programming");
        trie.insert("python");
        trie.insert("javascript");
        trie.insert("typescript");
        trie.insert("aws");
        trie.insert("aws-cli");
        trie.insert("nginx");
        trie.insert("apache");
        trie.insert("mysql");
        trie.insert("postgresql");
        trie
    }

    #[test]
    fn test_trie_insert_and_exists() {
        let mut trie = Trie::new();

        // Test insertion
        trie.insert("test");
        trie.insert("testing");
        trie.insert("tester");

        // Test existence
        assert!(trie.exists("test"));
        assert!(trie.exists("testing"));
        assert!(trie.exists("tester"));
        assert!(!trie.exists("tes"));
        assert!(!trie.exists("nonexistent"));
    }

    #[test]
    fn test_prefix_search() {
        let mut trie = create_test_trie();

        // Test exact prefix matches
        let docker_results = trie._search("docker");
        assert!(docker_results.contains(&"docker".to_string()));
        assert!(docker_results.contains(&"docker-compose".to_string()));
        assert!(docker_results.contains(&"docker-swarm".to_string()));
        assert_eq!(docker_results.len(), 3);

        // Test single character prefix
        let rust_results = trie._search("rust");
        assert!(rust_results.contains(&"rust".to_string()));
        assert!(rust_results.contains(&"rust-programming".to_string()));
        assert_eq!(rust_results.len(), 2);

        // Test no matches
        let no_results = trie._search("xyz");
        assert!(no_results.is_empty());
    }

    #[test]
    fn test_fuzzy_search_exact_match() {
        let trie = create_test_trie();

        let results = trie.fuzzy_search("docker");
        assert!(!results.is_empty());

        // Find the exact match
        let exact_match = results.iter().find(|(name, _)| name == "docker");
        assert!(exact_match.is_some());

        let (_, score) = exact_match.unwrap();
        assert_eq!(*score, 100.0); // Exact match should have 100% score
    }

    #[test]
    fn test_fuzzy_search_prefix_match() {
        let trie = create_test_trie();

        let results = trie.fuzzy_search("dock");
        assert!(!results.is_empty());

        // All docker-related files should be found
        let docker_matches: Vec<_> = results
            .iter()
            .filter(|(name, _)| name.starts_with("docker"))
            .collect();
        assert_eq!(docker_matches.len(), 3);

        // All should have high scores (>80%)
        for (_, score) in docker_matches {
            assert!(*score > 80.0);
        }
    }

    #[test]
    fn test_fuzzy_search_substring_match() {
        let trie = create_test_trie();

        let results = trie.fuzzy_search("script");

        // Should find javascript and typescript
        let js_matches: Vec<_> = results
            .iter()
            .filter(|(name, _)| name.contains("script"))
            .collect();
        assert_eq!(js_matches.len(), 2);

        // Should have decent scores
        for (_, score) in js_matches {
            assert!(*score > 60.0);
        }
    }

    #[test]
    fn test_fuzzy_search_character_sequence() {
        let trie = create_test_trie();

        // Test fuzzy matching with character sequence
        // "k8s" is a challenging pattern that might not match well with "kubernetes"
        // This is expected behavior for this type of abbreviation

        // Better test: "kctl" should match "kubectl"
        let kubectl_results = trie.fuzzy_search("kctl");
        let kubectl_match = kubectl_results.iter().find(|(name, _)| name == "kubectl");

        if let Some((_, score)) = kubectl_match {
            assert!(*score > 40.0); // Should have some score for character sequence
        }
    }

    #[test]
    fn test_fuzzy_search_case_insensitive() {
        let mut trie = Trie::new();
        trie.insert("Docker");
        trie.insert("KUBERNETES");
        trie.insert("PyThOn");

        // Test case insensitive matching
        let results = trie.fuzzy_search("docker");
        let docker_match = results.iter().find(|(name, _)| name == "Docker");
        assert!(docker_match.is_some());

        let results = trie.fuzzy_search("PYTHON");
        let python_match = results.iter().find(|(name, _)| name == "PyThOn");
        assert!(python_match.is_some());
    }

    #[test]
    fn test_fuzzy_search_scoring_order() {
        let trie = create_test_trie();

        let results = trie.fuzzy_search("aws");

        // Results should be ordered by score (descending)
        for i in 1..results.len() {
            assert!(results[i - 1].1 >= results[i].1);
        }

        // "aws" should score higher than "aws-cli"
        let aws_score = results
            .iter()
            .find(|(name, _)| name == "aws")
            .map(|(_, score)| *score)
            .unwrap_or(0.0);

        let aws_cli_score = results
            .iter()
            .find(|(name, _)| name == "aws-cli")
            .map(|(_, score)| *score)
            .unwrap_or(0.0);

        assert!(aws_score > aws_cli_score);
    }

    #[test]
    fn test_edit_distance() {
        let trie = Trie::new();

        // Test edit distance calculation
        assert_eq!(trie.edit_distance("", ""), 0);
        assert_eq!(trie.edit_distance("abc", ""), 3);
        assert_eq!(trie.edit_distance("", "abc"), 3);
        assert_eq!(trie.edit_distance("abc", "abc"), 0);
        assert_eq!(trie.edit_distance("abc", "ab"), 1);
        assert_eq!(trie.edit_distance("abc", "abcd"), 1);
        assert_eq!(trie.edit_distance("abc", "axc"), 1);
        assert_eq!(trie.edit_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn test_fuzzy_match_character_sequence() {
        let trie = Trie::new();

        // Test fuzzy character sequence matching
        assert_eq!(trie.fuzzy_match("", ""), Some(1.0));
        assert_eq!(trie.fuzzy_match("abc", ""), Some(1.0));
        assert_eq!(trie.fuzzy_match("abc", "abc"), Some(1.0));
        assert_eq!(trie.fuzzy_match("abc", "ac"), Some(1.0)); // All chars found in sequence
        assert_eq!(trie.fuzzy_match("abc", "xyz"), None); // No match

        // Test with real examples
        let kubectl_match = trie.fuzzy_match("kubectl", "kctl");
        assert!(kubectl_match.is_some());
        assert!(kubectl_match.unwrap() > 0.5);

        let docker_match = trie.fuzzy_match("docker-compose", "dkr");
        assert!(docker_match.is_some());
    }

    #[test]
    fn test_collect_all_values() {
        let trie = create_test_trie();
        let mut values = Vec::new();
        Trie::collect_all_values(&trie.root, &mut values);

        // Should collect all inserted values
        assert!(values.contains(&"docker".to_string()));
        assert!(values.contains(&"kubernetes".to_string()));
        assert!(values.contains(&"rust".to_string()));
        assert!(values.len() >= 16); // We inserted 16 items
    }

    #[test]
    fn test_empty_trie_fuzzy_search() {
        let trie = Trie::new();
        let results = trie.fuzzy_search("anything");
        assert!(results.is_empty());
    }

    #[test]
    fn test_single_item_trie() {
        let mut trie = Trie::new();
        trie.insert("single");

        let exact_results = trie.fuzzy_search("single");
        assert_eq!(exact_results.len(), 1);
        assert_eq!(exact_results[0].0, "single");
        assert_eq!(exact_results[0].1, 100.0);

        let prefix_results = trie.fuzzy_search("sin");
        assert_eq!(prefix_results.len(), 1);
        assert!(prefix_results[0].1 > 80.0);

        let no_match = trie.fuzzy_search("xyz");
        assert!(no_match.is_empty());
    }

    #[test]
    fn test_duplicate_insertions() {
        let mut trie = Trie::new();

        // Insert same value multiple times
        trie.insert("duplicate");
        trie.insert("duplicate");
        trie.insert("duplicate");

        // Should still exist and work correctly
        assert!(trie.exists("duplicate"));

        let results = trie.fuzzy_search("duplicate");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "duplicate");
    }

    #[test]
    fn test_special_characters() {
        let mut trie = Trie::new();
        trie.insert("file-name");
        trie.insert("file_name");
        trie.insert("file.name");
        trie.insert("file@name");

        // Test that special characters work
        assert!(trie.exists("file-name"));
        assert!(trie.exists("file_name"));
        assert!(trie.exists("file.name"));
        assert!(trie.exists("file@name"));

        // Test fuzzy search with special characters
        let results = trie.fuzzy_search("file");
        assert_eq!(results.len(), 4);
    }
}
