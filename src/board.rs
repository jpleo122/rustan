use std::collections::HashMap;
use crate::coordinates::{Hex, HexEdge, HexVertex};
use crate::map::Map;

pub type Player = i8;

pub struct Road {
    player: Player,
    hex_edge: HexEdge
}

pub enum Building {
    Settlement {
        player: Player,
        hex_vertex: HexVertex
    },
    City {
        player: Player,
        hex_vertex: HexVertex
    }
}

pub struct Board {
    map: Map,
    robber: Option<Hex>,
    buildings: HashMap<HexVertex, Building>,
    roads: HashMap<HexVertex, Road>
}

impl Board {
    pub fn from_map(map: Map) -> Self {

        let robber = map.find_desert_tile();
        Self { map, robber, buildings: HashMap::new(), roads: HashMap::new() }
    }

    pub fn add_building(&mut self, hex_vertex: HexVertex, building: Building) {
        self.buildings.insert(hex_vertex, building);
    }
}