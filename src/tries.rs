use std::{collections::HashMap};

#[derive(Debug, Clone)]
pub struct TriesNode {
    pub value: Option<char>,
    pub is_final: bool,
    pub child_nodes: HashMap<char, TriesNode>
}

impl TriesNode {
    pub fn new(value: char, is_final: bool) -> TriesNode{
        Self{
            value: Some(value),
            is_final,
            child_nodes: HashMap::new()
        }
    }
    pub fn new_root() -> TriesNode {
        Self { value: Option::None, is_final: false, child_nodes: HashMap::new() }
    }
    #[allow(unused)]
    pub fn check_value(self, c: char) -> bool {
        self.value == Some(c)
    }
    pub fn insert_value(&mut self, value: char, is_final: bool) {
        self.child_nodes.insert(value, TriesNode::new(value, is_final));
    }
}


// trie structure
#[derive(Debug, Clone)]
pub struct TrieStructure {
    pub root_node: TriesNode
}
impl TrieStructure {
    #[allow(unused)]
    pub fn new() -> TrieStructure {
        Self { root_node: TriesNode::new_root() }
    }
    #[allow(unused)]
    pub fn insert(&mut self, file_name: String) {
        let mut current_node = &mut self.root_node;
        let list_of_characters: Vec<char> = file_name.chars().collect();
        let mut last_match = 0;

        for letter_counter in 0..list_of_characters.len() {
            if current_node.child_nodes.contains_key(&list_of_characters[letter_counter]){
                current_node = current_node.child_nodes.get_mut(&list_of_characters[letter_counter]).unwrap();
            }else{
                last_match = letter_counter;
                break;
            }
            last_match = last_match + 1;
        }
        if last_match == list_of_characters.len() {
            current_node.is_final = true;
        }else {
            for new_counter in last_match..list_of_characters.len() {
                // println!("Inserting {} into {:?}", &list_of_characters[new_counter], current_node.value.unwrap_or_default());
                current_node.insert_value(list_of_characters[new_counter], false);
                current_node = current_node.child_nodes.get_mut(&list_of_characters[new_counter]).unwrap();
            }
            current_node.is_final =  true;
        }
    }
    #[allow(unused)]
    pub fn find(&mut self, file_name: String) -> (bool, String) {
        let mut container = String::new();
        let mut current_node = &mut self.root_node;
        let list_of_characters: Vec<char> = file_name.chars().collect();
        for counter in 0..list_of_characters.len() {
            if !&current_node.child_nodes.contains_key(&list_of_characters[counter]){
                return (false, String::new());
            }else{
                container.push(current_node.clone().value.unwrap_or_default());
                current_node = current_node.child_nodes.get_mut(&list_of_characters[counter]).unwrap();
            }
        }
        return (true, container);
    }
}