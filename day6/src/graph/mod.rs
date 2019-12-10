// Messy graph implementation... I can't do rust
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

type Key = usize;

#[derive(Debug, Clone)] // Do
pub struct Node<T: Clone + PartialEq + Eq> {
    key: Key,
    value: T,
    parent: Option<Key>,
}

// I
impl<T: Clone + PartialEq + Eq + Hash> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

// Need
impl<T: Clone + PartialEq + Eq> Node<T> {
    fn new(key: Key, value: T) -> Self {
        Node {
            key,
            value,
            parent: None,
        }
    }

    // Sorri
    pub fn get_key(&self) -> Key {
        return self.key;
    }
}

// All these
pub struct Graph<T: Clone + PartialEq + Eq> {
    edges: HashMap<Key, HashMap<Key, usize>>, // Edges
    nodes: HashMap<Key, Node<T>>,             // Nodes
    next_key: Key,
    root: i32,
}

// Declarations?
impl<T: Clone + PartialEq + Eq> Graph<T> {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            nodes: HashMap::new(),
            next_key: 1,
            root: -1,
        }
    }

    pub fn set_root(&mut self, root: &Node<T>) {
        self.root = root.key as i32;
    }

    pub fn insert_node(&mut self, value: T) -> Node<T> {
        if let Some(n) = self.get_node(&value) {
            return n.clone();
        }

        let key = self.next_key();
        let node = Node::new(key, value);
        self.edges.insert(key, HashMap::new());
        self.nodes.insert(key, node.clone());
        node
    }

    pub fn get_node(&self, value: &T) -> Option<Node<T>> {
        for (_, n) in self.nodes.iter() {
            if n.value == *value {
                return Some(n.clone());
            }
        }
        None
    }

    pub fn insert_edge(&mut self, node_a: Node<T>, node_b: Node<T>, bi_dir: bool) {
        let key_a = node_a.key;
        let key_b = node_b.key;

        if let Some(edge) = self.edges.get_mut(&key_a) {
            edge.insert(key_b, 1);
        }

        if bi_dir {
            if let Some(edge) = self.edges.get_mut(&key_b) {
                edge.insert(key_a, 1);
            }
        }
    }

    fn next_key(&mut self) -> Key {
        let key = self.next_key;
        self.next_key += 1;
        return key;
    }

    pub fn checksum(&self) -> i64 {
        if let Some(root) = self.nodes.get(&(self.root as usize)) {
            return self
                .nodes
                .iter()
                .map(|(_, n)| self.get_distance_between(root.key, n.key))
                .sum();
        } else {
            panic!("Need to set root in order to calculate checksum");
        }
    }

    // BFS
    pub fn get_distance_between(&self, key_a: Key, key_b: Key) -> i64 {
        let mut history: HashMap<usize, usize> = HashMap::new();

        let mut q = Vec::new();
        let mut searched = HashSet::new();
        q.insert(0, key_a);
        searched.insert(key_a);

        while !q.is_empty() {
            let curr = q.pop().unwrap();

            if curr == key_b {
                break;
            }

            searched.insert(curr);

            if let Some(edge) = self.edges.get(&curr) {
                for (key, _) in edge.into_iter() {
                    if !searched.contains(key) {
                        history.insert(*key, curr);
                        q.insert(0, *key);
                    }
                }
            }
        }

        // Track back from end by following the history
        let mut path = Vec::new();
        let mut waypoint = Some(key_b);
        let mut counter = -1;

        while let Some(next) = waypoint {
            path.push(next);
            match history.get(&next) {
                Some(review) => waypoint = Some(*review),
                None => waypoint = None,
            }
            counter += 1;
        }
        return counter;
    }
}
