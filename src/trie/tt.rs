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
    pub fn search(&mut self, s: &str) -> Vec<String> {
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
