/// TODO move module into sloe

pub type Vertex = usize;
pub type Neighbors = Vec<std::collections::BTreeSet<Vertex>>;

pub fn neighbors_add_vertex(graph: &mut Neighbors) -> Vertex {
    let new_node = graph.len();
    graph.push(std::collections::BTreeSet::new());
    new_node
}

/// using Tarjan's algorithm
pub fn find(neighbors: &Neighbors) -> Vec<std::collections::BTreeSet<Vertex>> {
    // won't be reached by regular Vertexes as a Vec can never hold that many items
    const vertex_unvisited: Vertex = usize::MAX;
    struct TarjanState {
        index: Vertex,
        stack: Vec<Vertex>,
        // consider splitting up vertex_state into 3 vecs, one for each property
        vertex_state: Vec<TarjanVertexState>,
        strongly_connected_components: Vec<std::collections::BTreeSet<Vertex>>,
    }
    #[derive(Clone, Copy)]
    struct TarjanVertexState {
        index: Vertex,
        low_link: Vertex,
        // invariant: the same vertex is contained in TarjanState::stack
        is_on_stack: bool,
    }
    let mut state = TarjanState {
        index: 0,
        stack: Vec::new(),
        vertex_state: std::iter::repeat_n(
            TarjanVertexState {
                index: vertex_unvisited,
                low_link: vertex_unvisited,
                is_on_stack: false,
            },
            neighbors.len(),
        )
        .collect(),
        strongly_connected_components: Vec::new(),
    };
    for (vertex, _) in neighbors.iter().enumerate() {
        if state.vertex_state[vertex].index == vertex_unvisited {
            visit(neighbors, &mut state, vertex);
        }
    }
    fn visit(neighbors: &Neighbors, state: &mut TarjanState, vertex: Vertex) {
        // (don't ask me about any of this. I more or less just ported pseudo-code from wikipedia)
        let vertex_mut = &mut state.vertex_state[vertex];
        vertex_mut.index = state.index;
        vertex_mut.low_link = state.index;
        state.index += 1;
        state.stack.push(vertex);
        vertex_mut.is_on_stack = true;

        for &vertex_neighbor in &neighbors[vertex] {
            let vertex_neighbor_state = state.vertex_state[vertex_neighbor];
            if vertex_neighbor_state.index == vertex_unvisited {
                visit(neighbors, state, vertex_neighbor);
                let vertex_neighbor_low_link = state.vertex_state[vertex_neighbor].low_link;
                let vertex_mut = &mut state.vertex_state[vertex];
                vertex_mut.low_link = vertex_mut.low_link.min(vertex_neighbor_low_link);
            } else if vertex_neighbor_state.is_on_stack {
                let vertex_neighbor_index = state.vertex_state[vertex_neighbor].index;
                let vertex_mut = &mut state.vertex_state[vertex];
                vertex_mut.low_link = vertex_mut.low_link.min(vertex_neighbor_index);
            }
        }
        if {
            let vertex_state = state.vertex_state[vertex];
            vertex_state.low_link == vertex_state.index
        } {
            let mut strongly_connected_component = std::collections::BTreeSet::new();
            loop {
                let vertex_off_stack = state.stack.pop().unwrap();
                state.vertex_state[vertex_off_stack].is_on_stack = false;
                strongly_connected_component.insert(vertex_off_stack);
                if vertex_off_stack == vertex {
                    break;
                }
            }
            state
                .strongly_connected_components
                .push(strongly_connected_component);
        }
    }
    state.strongly_connected_components
}
