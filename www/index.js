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
    // graph.update()
    return
}

let plot = () => {
    let data = graph.get_nodes()
    container.innerHTML = ''

    data.forEach(node => {
        var new_node = document.createElement('div');
        new_node.style.width = '100px';
        new_node.style.height = '100px';
        new_node.style.borderRadius = '50px';
        new_node.style.backgroundColor = 'red';
        new_node.style.position = 'absolute';
        new_node.style.left = node.pos[0] + 'px';
        new_node.style.top = node.pos[1] + 'px';
        new_node.style.marginTop = '-50px';
        new_node.style.marginLeft = '-50px';
        container.appendChild(new_node);
    })
}

