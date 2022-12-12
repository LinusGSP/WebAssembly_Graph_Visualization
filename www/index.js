import { Graph, Node } from "graph_wasm";


const container = document.querySelector('.container')

const graph = Graph.new()

container.addEventListener("click", (event) => {
    graph.add_node(Node.new(event.x, event.y, event.x))
    console.log("added")
})


setInterval(() => {
    update()
    plot()
}, 1000);


let update = () => {

}

let plot = () => {
    let html = graph.get_html()
    container.innerHTML = html
}

