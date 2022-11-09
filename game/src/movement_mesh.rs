use bevy::prelude::*;

pub const MAX_VERT: usize = 50;
#[derive(Copy, Clone)]
pub struct Vertex{
    pub x: f32,
    pub y: f32,
    pub id: usize,
}
impl Vertex{
    pub fn new(a: f32, b: f32, i: usize) -> Self{
        Self{
            x: a,
            y: b,
            id: i,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Motion{
    Left,
    Right,
    Jump,
    JumpRight,
    JumpLeft,
    Fall,
    Stop,
}

#[derive(Copy, Clone)]
pub struct Edge{
    pub path: Motion,
}

impl Edge{
    
    fn new(m: Motion) -> Self{
        Self{
            path: m,
        }
    }
}

#[derive(Component, Clone)]
pub struct Graph{
    pub vertices: Vec<Vertex>,
    pub edges: [[Edge; MAX_VERT]; MAX_VERT],
}

impl Graph{
    pub fn new() -> Self {
        Self{
            vertices: Vec::new(),
            edges: [[Edge::new(Motion::Stop); MAX_VERT];MAX_VERT],
        }
    }
}

pub fn get_level_mesh(id: i8) -> Graph{

    let mut graph = Graph::new();
    //level id for testing
    if id == 0{
        graph.vertices.push(Vertex::new(-224., 32., 0));
        graph.vertices.push(Vertex::new(208., 32., 1));
        graph.vertices.push(Vertex::new(256., 96., 2));

        graph.edges[0][1] = Edge::new(Motion::Right);
        graph.edges[1][0] = Edge::new(Motion::Left);
        graph.edges[1][2] = Edge::new(Motion::JumpRight);
        graph.edges[2][1] = Edge::new(Motion::Left);
    }
    if id == 1{

    }
    return graph;
}