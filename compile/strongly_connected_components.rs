pub type Vertex = usize;

/// TODO inline as Vec<Set...>
pub struct Graph {
    neighbors: Vec<std::collections::BTreeSet<Vertex>>,
}
struct TarjanState<'a> {
    graph: &'a Graph,
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

impl Graph {
    // won't be reached by regular Vertexes as a Vec can never hold that many items
    pub const node_unvisited: Vertex = usize::MAX;

    pub fn new() -> Graph {
        Graph {
            neighbors: Vec::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.neighbors.len()
    }
    pub fn add_node(&mut self) -> Vertex {
        let new_node = self.neighbors.len();
        self.neighbors
            .insert(new_node, std::collections::BTreeSet::new());
        new_node
    }
    /// TODO prefer `set_edges`
    pub fn add_edge(&mut self, from: Vertex, to: Vertex) {
        self.neighbors[from].insert(to);
    }
    pub fn set_edges(&mut self, from: Vertex, to: std::collections::BTreeSet<Vertex>) {
        self.neighbors[from] = to;
    }
    /// Strongly Connected Components (Tarjan's algorithm)
    pub fn find_strongly_connected_components(&self) -> Vec<std::collections::BTreeSet<Vertex>> {
        let mut state = TarjanState {
            graph: self,
            index: 0,
            stack: Vec::new(),
            vertex_state: std::iter::repeat_n(
                TarjanVertexState {
                    index: Graph::node_unvisited,
                    low_link: Graph::node_unvisited,
                    is_on_stack: false,
                },
                self.len(),
            )
            .collect(),
            strongly_connected_components: Vec::new(),
        };
        for (vertex, _) in state.graph.neighbors.iter().enumerate() {
            if state.vertex_state[vertex].index == Graph::node_unvisited {
                Graph::tarjan_visit(&mut state, vertex);
            }
        }
        state.strongly_connected_components
    }

    fn tarjan_visit(state: &mut TarjanState, vertex: Vertex) {
        // (don't ask me about any of this. I more or less just ported pseudo-code from wikipedia)
        let vertex_mut = &mut state.vertex_state[vertex];
        vertex_mut.index = state.index;
        vertex_mut.low_link = state.index;
        state.index += 1;
        state.stack.push(vertex);
        vertex_mut.is_on_stack = true;

        for &vertex_neighbor in &state.graph.neighbors[vertex] {
            let vertex_neighbor_state = state.vertex_state[vertex_neighbor];
            if vertex_neighbor_state.index == Graph::node_unvisited {
                Graph::tarjan_visit(state, vertex_neighbor);
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
}
