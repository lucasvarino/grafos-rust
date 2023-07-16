use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug)]
pub struct Graph {
    adj_list: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj_list
            .entry(src)
            .or_insert_with(HashSet::new)
            .insert(dest);

        self.adj_list
            .entry(dest)
            .or_insert_with(HashSet::new)
            .insert(src);
    }

    pub fn remove_edge(&mut self, src: i32, dest: i32) -> io::Result<()> {
        let src_usize = src as usize;
        let dest_usize = dest as usize;

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

    pub fn print_graph(&self) {
        for (node, edges) in &self.adj_list {
            println!("Adjacency list of node {}:", node);
            print!("head");

            for e in edges {
                print!(" -> {}", e);
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

            graph.add_edge(src, dst);
        }
    }

    Ok(graph)
}
