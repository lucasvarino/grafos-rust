use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::path::Path;

#[derive(Debug, Eq, Copy, Clone)]
pub struct Edge {
    dest: usize,
    weight: i32,
}
impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.dest == other.dest
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dest.hash(state);
    }
}

impl Borrow<usize> for Edge {
    fn borrow(&self) -> &usize {
        &self.dest
    }
}

impl Edge {

    pub fn new(dest: usize, weight: i32) -> Self {
        Edge { dest, weight }
    }
    
    pub fn get_dest(&self) -> usize {
        self.dest
    }

    pub fn get_weight(&self) -> i32 {
        self.weight
    }

}


#[derive(Debug)]
pub struct Graph {
    adj_list: HashMap<usize, HashSet<Edge>>,
    pub order: i32,
}


impl Graph {
    pub fn new() -> Self {
        Graph {
            order: 0,
            adj_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, src: usize, dest: usize, weight: i32) {
        if weight < 1 {
            panic!("Peso da aresta deve ser maior que 0");
        }

        self.adj_list
            .entry(src)
            .or_insert_with(HashSet::new)
            .insert(Edge { dest, weight });

        self.adj_list
            .entry(dest)
            .or_insert_with(HashSet::new)
            .insert(Edge { dest:src, weight });
    }

    pub fn get_edge_weight(&mut self, src:i32, dest:i32) -> io::Result<i32>{
        let src_usize = src as usize;
        let dest_usize = dest as usize;
        let weight: i32 = self.adj_list.entry(src_usize).or_default().get(&dest_usize).unwrap().get_weight();
        Ok(weight)
    }

    pub fn remove_edge(&mut self, src: i32, dest: i32) -> io::Result<()> {
        let src_usize = src as usize;
        let dest_usize = dest as usize;
        // let weight: i32 = self.get_edge_weight(src, dest)?;

        let remove_src = self
            .adj_list
            .entry(src_usize)
            .or_default()
            .remove(&dest_usize);

        let remove_dest = self
            .adj_list
            .entry(dest_usize)
            .or_default()
            .remove(&src_usize);

        if remove_src && remove_dest {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Uma ou ambas as arestas não foram encontradas",
            ))
        }
    }

    fn remove_all_edges(&mut self, node: i32) {
        for (_node, edges) in self.adj_list.iter_mut() {
            edges.remove(&(node as usize));
        }
    }

    pub fn remove_node(&mut self, node: i32) {
        match self.adj_list.remove(&(node as usize)) {
            Some(_) => self.remove_all_edges(node),
            None => (),
        }
    }

    pub fn get_open_neighborhood(&mut self, node: i32) -> &HashSet<Edge> {
        self.adj_list.entry(node as usize).or_default()
    }

    pub fn get_closed_neighborhood(&mut self, node: i32) -> HashSet<Edge> {
        self.adj_list
            .iter()
            .filter(|(_index, edge)| edge.contains(&(node as usize)))
            .flat_map(|(_index, edge)| edge)
            .cloned()
            .collect()
    }

    pub fn print_graph(&self) {
        for (node, edges) in &self.adj_list {
            println!("Adjacency list of node {}:", node);
            print!("head");

            for e in edges {
                print!(" -> {}", e.dest);
            }

            println!();
        }
    }
}

pub fn read_graph_from_file<P>(filename: P) -> Result<Graph, io::Error>
where
    P: AsRef<Path>,
{
    let mut graph = Graph::new();

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if index > 0 && parts[0] == "e" {
            let src: usize = parts[1].parse().unwrap();
            let dst: usize = parts[2].parse().unwrap();
            let weight: i32 = parts[3].parse().unwrap();
            graph.add_edge(src, dst, weight);
        } else {
            graph.order = parts[0].parse().unwrap();
        }
    }

    Ok(graph)
}
