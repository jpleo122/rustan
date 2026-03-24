use std::collections::HashMap;
use std::iter::repeat_with;
use rand::seq::SliceRandom;
use crate::coordinates::{from_center, Hex, HexVertex};

pub enum Resource {
    WOOD,
    BRICK,
    SHEEP,
    WHEAT,
    ORE
}

pub struct Port {
    resource: Resource,
    hex_vertex: HexVertex
}

pub enum Tile {
    LandTile {
        hex: Hex,
        resource: Option<Resource>,
        number: i8
    },
    WaterTile {
        hex: Hex,
        ports: Vec<Port>
    }
}

pub struct Map {
    tiles: HashMap<Hex, Tile>,
}
//
// impl Map {
//
//     pub fn neighbors(&self, hex: Hex) -> Vec<Hex> {
//
//
//     }
// }

pub struct MapTemplate {
    number_distribution: Vec<i8>,
    resources: Vec<Resource>,
    desert_tiles: i8,
    land_tile_positions: Vec<Hex>,
}

impl MapTemplate {

    pub fn new(
        mut number_distribution: Vec<i8>,
        mut resources: Vec<Resource>,
        desert_tiles: i8,
        mut land_tile_positions: Vec<Hex>
    ) -> Self {

        assert_eq!(number_distribution.len(), resources.len(),
                   "template must have the same number of resources as numbers");
        assert_eq!(land_tile_positions.len() - desert_tiles as usize, resources.len(),
                   "template must have the same number of tiles as resources");

        let mut rng = rand::rng();

        number_distribution.shuffle(&mut rng);
        resources.shuffle(&mut rng);
        land_tile_positions.shuffle(&mut rng);

        MapTemplate {
            number_distribution,
            resources,
            desert_tiles,
            land_tile_positions
        }
    }

    pub fn to_map(self) -> Map {

        let true_resources = self.resources.into_iter().map(|r| Some(r))
            .chain(repeat_with(|| None).take(self.desert_tiles as usize).collect::<Vec<Option<Resource>>>())
            .collect::<Vec<Option<Resource>>>();

        let true_numbers = self.number_distribution.into_iter()
            .chain(vec![7; self.desert_tiles as usize]).collect::<Vec<i8>>();

        let mut tiles: HashMap<Hex, Tile> = HashMap::new();

        for (hex, (number, resource)) in self.land_tile_positions.into_iter()
            .zip(true_numbers.into_iter().zip(true_resources.into_iter())) {
            let tile = Tile::LandTile { hex, resource, number };
            tiles.insert(hex, tile);
        }

        Map { tiles }
    }

    pub fn base_map_template() -> Self {
        MapTemplate::new(
            vec![2, 3, 3, 4, 4, 5, 5, 6, 6, 8, 8, 9, 9, 10, 10, 11, 11, 12],
            vec![
                Resource::WOOD, Resource::WOOD, Resource::WOOD, Resource::WOOD,
                Resource::BRICK, Resource::BRICK, Resource::BRICK,
                Resource::SHEEP, Resource::SHEEP, Resource::SHEEP, Resource::SHEEP,
                Resource::WHEAT, Resource::WHEAT, Resource::WHEAT, Resource::WHEAT,
                Resource::ORE, Resource::ORE, Resource::ORE
            ],
            1,
            Vec::from(
                from_center(&Hex::new(0, 0), 2)
            )
        )
    }
}

#[cfg(test)]
mod test {
    use crate::map::MapTemplate;

    #[test]
    fn test_base_map() {
        let template = MapTemplate::base_map_template();

        let tile_count = template.land_tile_positions.len();

        let map = template.to_map();

        assert_eq!(map.tiles.len(), tile_count);
    }
}