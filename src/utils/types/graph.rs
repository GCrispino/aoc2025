use std::collections::{BTreeMap, HashSet, LinkedList};

#[derive(Debug)]
pub struct Graph {
    adj_list: BTreeMap<i32, LinkedList<i32>>,
}

pub fn noop_dfs(_node: i32) {}

impl Graph {
    pub fn get_adj(&self, node: i32) -> Option<&LinkedList<i32>> {
        return self.adj_list.get(&node);
    }

    pub fn add_node(&mut self, node: i32) -> bool {
        if self.adj_list.contains_key(&node) {
            return false;
        }

        self.adj_list.entry(node).or_insert_with(LinkedList::new);
        true
    }

    pub fn dfs<F>(
        &self,
        node: i32,
        cb: &mut F,
        visited_opt: Option<&mut HashSet<i32>>,
    ) -> LinkedList<i32>
    where
        F: Fn(i32),
    {
        let mut new_set = HashSet::from([node]);
        let visited: &mut HashSet<i32> = &mut visited_opt.unwrap_or(&mut new_set);
        visited.extend([node]);
        // println!("  node: {}, visited: {:?}", node, visited);

        if !&self.adj_list.contains_key(&node) {
            return LinkedList::new();
        }

        let node_adj = &self.adj_list[&node];
        cb(node);

        let mut stack: LinkedList<i32> = LinkedList::new();
        for adj in node_adj {
            if !visited.contains(adj) {
                let res = self.dfs(*adj, cb, Some(visited));
                stack.extend(res);
            }
        }

        stack.push_back(node);
        stack
    }
}

impl From<BTreeMap<i32, LinkedList<i32>>> for Graph {
    fn from(adj_list: BTreeMap<i32, LinkedList<i32>>) -> Self {
        Graph { adj_list }
    }
}

impl FromIterator<(i32, i32)> for Graph {
    fn from_iter<I: IntoIterator<Item = (i32, i32)>>(iter: I) -> Self {
        let mut adj_list: BTreeMap<i32, LinkedList<i32>> = BTreeMap::new();
        iter.into_iter().for_each(|(k, v)| {
            adj_list
                .entry(k)
                .and_modify(|list| list.push_back(v))
                .or_insert(LinkedList::from([v]));
            ()
        });

        Graph::from(adj_list)
    }
}
