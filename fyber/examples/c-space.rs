/*
    Appellation: c-space <example>
    Contrib: @FL03
*/
use petgraph::graph::{Graph, NodeIndex};
use petgraph::prelude::*;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cspace = CSpace::new();
    let n1 = cspace.add_node();
    let n2 = cspace.add_node();
    let n3 = cspace.add_node();
    cspace.add_edge(n1, n2, 0.7);
    cspace.add_edge(n2, n3, 0.5);
    cspace.add_edge(n3, n1, 0.9);

    let agent1 = Arc::new(Mutex::new(Agent::new(1, n1)));
    let agent2 = Arc::new(Mutex::new(Agent::new(2, n2)));

    let agents = vec![agent1.clone(), agent2.clone()];

    let handles: Vec<_> = agents.into_iter().map(|agent| {
        let cspace = cspace.clone();
        thread::spawn(move || {
            loop {
                {
                    let mut agent = agent.lock().unwrap();
                    agent.decide_next_move(&cspace);
                    Observer::observe(&agent);
                }
                thread::sleep(Duration::from_millis(500));
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}


/// Represents the c-space as a dynamic weighted graph.
#[derive(Clone, Debug)]
struct CSpace {
    graph: Graph<(), f64, Directed>,
}

impl CSpace {
    fn new() -> Self {
        Self {
            graph: Graph::new(),
        }
    }

    fn add_node(&mut self) -> NodeIndex {
        self.graph.add_node(())
    }

    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: f64) {
        self.graph.add_edge(from, to, weight);
    }

    fn get_neighbors(&self, id: NodeIndex) -> Vec<(NodeIndex, f64)> {
        self.graph
            .edges(id)
            .map(|e| (e.target(), *e.weight()))
            .collect()
    }
}

/// Represents an agent operating in the c-space.
struct Agent {
    id: usize,
    position: NodeIndex,

    history: Vec<NodeIndex>,
}

impl Agent {
    fn new(id: usize, start_position: NodeIndex) -> Self {
        Self {
            id,
            position: start_position,
            history: vec![start_position],
        }
    }

    /// Probabilistically chooses the next move based on weighted edges.
    fn decide_next_move(&mut self, cspace: &CSpace) {
        let mut rng = rand::rng();
        let neighbors = cspace.get_neighbors(self.position);
        if neighbors.is_empty() {
            return;
        }

        let total_weight: f64 = neighbors.iter().map(|(_, w)| w).sum();
        let mut choice = rng.random_range(0.0..total_weight);

        for (target, weight) in neighbors {
            if choice < weight {
                self.position = target;
                self.history.push(self.position);
                break;
            }
            choice -= weight;
        }
    }
}

/// The observer that tracks state changes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Observer;

impl Observer {
    fn observe(agent: &Agent) {
        println!("Observer: Agent {} moved to node {:?}", agent.id, agent.position);
    }
}
