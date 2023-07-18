mod edge;
mod graph;
mod node;

fn main() {
    let mut graph_result = graph::read_graph_from_file("complete_graph.txt").unwrap_or_else(|e| {
        eprintln!("Erro ao ler o arquivo de grafo: {}", e);
        std::process::exit(1);
    });

    let mut num_edges: usize = graph_result.get_num_edges();

    println!("Numero de arestas: {}", num_edges);

    graph_result.print_graph();

    println!("É completo? {}", graph_result.is_complete());

    graph_result.remove_edge(1, 2).unwrap();    

    println!("Remoção da aresta entre 1 e 2:");
    graph_result.print_graph();

    num_edges = graph_result.get_num_edges();

    let complement = graph_result.get_complement().unwrap();

    println!("Numero de arestas: {}", num_edges);

    let weight: i32 = graph_result.get_edge_weight(1, 3).unwrap();
    println!("Peso da aresta entre 1 e 3: {}", weight);

    let neighbors = graph_result.get_closed_neighborhood(1);
    println!("Vizinhos fechados do no 1: {:?}", neighbors.iter().map(|e| e.get_dest()).collect::<Vec<usize>>());

    let neighbors = graph_result.get_open_neighborhood(1);
    println!("Vizinhos abertos do no 1: {:?}", neighbors.iter().map(|e| e.get_dest()).collect::<Vec<usize>>());

    println!("É completo? {}", graph_result.is_complete());

    println!("Grafo complementar: ");
    complement.print_graph();
}
