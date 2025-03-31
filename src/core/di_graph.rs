use std::collections::HashMap;

pub struct DiGraphMap<N, W> {
    adjacency_list: HashMap<N, HashMap<N, W>>,
}

impl<N: Eq + PartialEq + std::hash::Hash + Copy, W: Copy + Default + PartialEq> DiGraphMap<N, W> {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: N, to: N, weight: W) {
        self.adjacency_list.entry(from).or_default().insert(to, weight);
    }

    pub fn node_count(&self) -> usize {
        self.adjacency_list.len()
    }

    pub fn neighbors(&self, node: N) -> Vec<N> {
        self.adjacency_list.get(&node).map_or(vec![], |edges| edges.keys().copied().collect())
    }

    pub fn edge_weight(&self, from: N, to: N) -> Option<&W> {
        self.adjacency_list.get(&from)?.get(&to)
    }

    pub fn edge_weight_mut(&mut self, from: N, to: N) -> Option<&mut W> {
        self.adjacency_list.get_mut(&from)?.get_mut(&to)
    }

    pub fn add_node(&mut self, node: N) {
        self.adjacency_list.entry(node).or_default();
    }

    pub fn nodes(&self) -> impl Iterator<Item = N> + '_ {
        self.adjacency_list.keys().copied()
    }

    pub fn contains_edge(&self, from: N, to: N) -> bool {
        self.adjacency_list.get(&from).is_some_and(|edges| edges.contains_key(&to))
    }
}
