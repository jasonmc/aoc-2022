pub fn part_one(input: &str) -> Option<u32> {
    let (tree, _) = create_tree(input);
    let res: u32 = tree
        .sizes_for_all_directories()
        .iter()
        .filter(|s| **s < 100000)
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (tree, root) = create_tree(input);
    let total_root = tree.total_size(root);
    let max_space = 70000000 - 30000000;
    let need_to_free_min = total_root - max_space;

    let sizes = tree.sizes_for_all_directories();
    let two = sizes
        .iter()
        .filter(|s| **s >= need_to_free_min)
        .min()
        .unwrap();

    Some(*two)
}

fn create_tree(input: &str) -> (ArenaTree, usize) {
    // for each line
    // if command then: if cd, store directory, if ls then nothing?
    // if starts dir, add that to directory, if starts size, add that file
    // maintain a currentIdx which is the name of the node that represents the directory we're currently in
    let mut tree: ArenaTree = ArenaTree::default();
    let root = tree.node("/".into());

    let mut curr_node = root;

    for l in input.lines() {
        if l.starts_with("$ cd ") {
            let dir = l.trim_start_matches("$ cd ");
            if dir == "/" {
                curr_node = root;
            } else if dir == ".." {
                curr_node = tree.get_parent(curr_node).unwrap();
            } else {
                curr_node = tree.get_child(curr_node, dir).unwrap();
            }
        } else if l.starts_with("$ ls") {
            // ignore
        } else if l.starts_with("dir ") {
            let dir_name = l.trim_start_matches("dir ");
            tree.add_child(curr_node, dir_name.into(), None);
        } else if l.starts_with(|x: char| x.is_numeric()) {
            let (size, name) = l.split_once(' ').unwrap();
            let size = size.parse::<u32>().unwrap();
            tree.add_child(curr_node, name.into(), Some(size));
        }
    }
    (tree, root)
}

#[derive(Debug)]
struct Node {
    idx: usize,
    name: String,
    size: Option<u32>,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn new(idx: usize, name: String) -> Self {
        Self {
            idx,
            size: None,
            name: name,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
struct ArenaTree {
    arena: Vec<Node>,
}

impl ArenaTree {
    fn node(&mut self, name: String) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, name));
        idx
    }

    fn get_parent(&self, idx: usize) -> Option<usize> {
        self.arena[idx].parent
    }

    fn add_child(&mut self, idx: usize, name: String, size: Option<u32>) -> usize {
        let new_node = self.node(name);
        self.arena[new_node].parent = Some(idx);
        self.arena[new_node].size = size;
        self.arena[idx].children.push(new_node);
        new_node
    }

    fn get_child(&self, idx: usize, name: &str) -> Option<usize> {
        //self.arena[idx].children
        for p in &self.arena[idx].children {
            if self.arena[*p].name == name {
                return Some(*p);
            }
        }
        return None;
    }

    fn total_size(&self, idx: usize) -> u32 {
        let a = &self.arena[idx]
            .children
            .iter()
            .map(|x| -> u32 {
                let n = &self.arena[*x];
                if n.size.is_some() {
                    n.size.unwrap()
                } else {
                    self.total_size(n.idx)
                }
            })
            .sum();
        *a
    }

    fn sizes_for_all_directories(&self) -> Vec<u32> {
        //for node in &self.arena {
        let size = &self
            .arena
            .iter()
            .filter(|n| n.size.is_none())
            .map(|n| self.total_size(n.idx))
            .collect::<Vec<u32>>();
        size.clone()
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
