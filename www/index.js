import { Graph, Node } from "graph_wasm";

const graph = Graph.new()
let node = Node.new(1.0, 2.0, 20, "Hello")
console.log(node.get_pos())
graph.add_node(node)
console.log(graph.get_nodes())