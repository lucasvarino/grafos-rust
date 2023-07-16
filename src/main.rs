mod graph;

fn main() {
    let mut graph_result = graph::read_graph_from_file("test.txt").unwrap_or_else(|e| {
        eprintln!("Erro ao ler o arquivo de grafo: {}", e);
        std::process::exit(1);
    });

    graph_result.print_graph();

    graph_result.remove_edge(1, 2).unwrap();

    println!("Remoção do no 1");
    graph_result.print_graph();

    let weight:i32 = graph_result.get_edge_weight(1, 3).unwrap();
    println!("Peso da aresta entre 1 e 3: {}", weight);
}
