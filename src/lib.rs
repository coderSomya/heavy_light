

struct Tree{
    edges: Vec<Node>, // edges[i] contains all nodes i has an edge with
    nodes: Vec<Node>,
    root: Node
}

impl Tree{

    pub fn add_edge(&self, i: Node, j: Node){
        self.edges[i].append(j);
        self.edges[j].append(i);
    }

    pub fn root(&self) -> Node{
        self.root
    }

    pub fn lca(i: Node, j: Node) -> Node{

    }

    fn binary_lift(i: Node, k: usize) -> Option<Node>{

    }

    pub fn node_at(t: usize) -> Node{
        self.nodes[t]
    }
}

struct HeavyLightDecomposition{
    tree: Tree,

}
