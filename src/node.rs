use action::Action;
use state::State;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct NodeId {
    index: usize
}

#[derive(Debug)]
pub struct NodeArena {
    nodes: Vec<Node>
}

impl NodeArena {
    pub fn new() -> NodeArena {
        NodeArena { nodes: Vec::new() }
    }

    pub fn new_node<S: State>(&mut self, state: S) -> NodeId {
        let index = self.nodes.len();

        self.nodes.push(Node {
            id: NodeId { index: index },
            action: None,
            action_string: None,
            parent: None,
            children: Vec::new(),
            wins: 0.0,
            visits: 0,
            untried_actions: state.get_actions(),
            untried_action_strings: state.get_action_strings(),
            player_just_moved: state.get_player_just_moved()
        });

        NodeId{ index: index }
    }

    pub fn new_child_node<S: State>(&mut self, parent: Option<NodeId>, action: Option<u32>, action_string: Option<String>, state: &S) -> NodeId {
        let index = self.nodes.len();

        self.nodes.push(Node {
            id: NodeId { index: index },
            action: action,
            action_string: action_string,
            parent: parent,
            children: Vec::new(),
            wins: 0.0,
            visits: 0,
            untried_actions: state.get_actions(),
            untried_action_strings: state.get_action_strings(),
            player_just_moved: state.get_player_just_moved()
        });

        NodeId{ index: index }
    }

    pub fn as_tree(&self) -> String {
        let rootnode = &self.nodes[0];
        let display_str = self.display_node(rootnode.id, 0);
        display_str
    }

    pub fn simple_display(&self) -> String {
        let mut display_str = String::from("");
        let rootnode = &self.nodes[0];
        display_str.push_str(format!("{}\n", rootnode).as_str());
        for child in rootnode.children.iter() {
            display_str.push_str(format!("   {}\n", self.nodes[child.index]).as_str());
        }
        display_str
    }

    pub fn display_node(&self, node_id: NodeId, indent: usize) -> String {
        let node = &self.nodes[node_id.index];
        let mut display_str = String::from("\n");
        for i in 1..indent+1 {
            display_str.push_str("| ");
        }
        display_str.push_str(format!("{}", node).as_str());
        
        for child in node.children.iter() {
            display_str.push_str(self.display_node(*child, indent+1).as_str());
        }
        display_str
    }
}

impl Index<NodeId> for NodeArena {
    type Output = Node;

    fn index(&self, node: NodeId) -> &Node {
        &self.nodes[node.index]
    }
}

impl IndexMut<NodeId> for NodeArena {
    fn index_mut(&mut self, node: NodeId) -> &mut Node {
        &mut self.nodes[node.index]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// Id of the node itself to find itself in the NodeArena
    pub id: NodeId,
    /// Action that got us to this node - None for root
    pub action: Option<u32>,
    /// Action that got us to this node - None for root
    pub action_string: Option<String>,
    /// Parent node - None for root
    pub parent: Option<NodeId>,
    /// Children nodes
    pub children: Vec<NodeId>,
    /// Number of current wins for this node
    pub wins: f32,
    /// Number of visits for this node
    pub visits: u32,
    /// Vector of actions left to take 
    pub untried_actions: Vec<u32>,
    /// Vector of the string representation of the actions left to take
    pub untried_action_strings: Vec<String>,
    /// Number of the player who has just played
    pub player_just_moved: usize,
}

impl Node {
    pub fn add_child<S: State>(&self, arena: &mut NodeArena, action: Option<u32>, action_string: Option<String>, state: S) -> NodeId {
        let new_node = arena.new_child_node(Some(self.id), action, action_string, &state);
        

        new_node
    }

    pub fn update(&mut self, result: f32) {
        self.visits += 1;
        self.wins += result;
    }
}

impl ::std::fmt::Display for Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {

        write!(f, "[({}) M: {:?} W/V: {}/{} A: {:?}]", 
            self.id.index, 
            self.clone().action_string.unwrap_or(String::from("None")), 
            self.wins, 
            self.visits, 
            self.untried_action_strings)
    }
}

