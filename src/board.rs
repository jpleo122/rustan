use std::collections::HashMap;
use crate::coordinates::{Hex, HexEdge, HexVertex};
use crate::map::Map;

pub struct Player(i8);

pub struct Robber { hex: Hex }

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
    robber: Robber,
    buildings: HashMap<HexVertex, Building>,
    roads: HashMap<HexVertex, Road>
}

impl Board {
    pub fn from_map(map: Map) -> Self {

        let robber = match map.find_desert_tile() {
            Some(hex) => Robber { hex },
            None => Robber { hex: Hex::new( i8::MAX,  i8::MAX) }
        };

        Self { map, robber, buildings: HashMap::new(), roads: HashMap::new() }
    }

    pub fn add_building(&mut self, hex_vertex: HexVertex, building: Building) {
        self.buildings.insert(hex_vertex, building);
    }
}