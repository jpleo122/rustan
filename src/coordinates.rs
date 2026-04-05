use std::cmp::{max, min};

/*
Implements a hexagonal grid system with direct relationships between the
Faces, edges, and vertices. Each hexagonal face shares 3 edges and 2 vertices.
Each face is represented in axial coordinates.

Implements the relationships between the faces, edges, and vertices.

Refs
- https://www.redblobgames.com/grids/hexagons/
- http://www-cs-students.stanford.edu/~amitp/game-programming/grids/
 */


/*
     N
    ___
 W /   \ E
  <     >
   \___/
 */
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum EdgeRef {
    N,
    W,
    E
}

/*
    ___
   /   \
L <     > R
   \___/
 */
#[derive(Eq, Hash, PartialEq)]
pub enum VertexRef {
    L,
    R
}

// Axial coordinates of a hexagonal face in the grid
#[derive(Eq, Hash, PartialEq)]
#[derive(Debug)]
pub struct Hex {
    pub q: i8,
    pub r: i8,
}

impl Hex {
   pub fn new(q: i8, r: i8) -> Hex {
       Hex{q, r}
   }
}

/*
Axial coordinates of a hexagonal face in the grid + edge ref
     N
    ___
 W /   \ E
  < q,r >
   \___/
 */
#[derive(Eq, Hash, PartialEq)]
pub struct HexEdge {
    q: i8,
    r: i8,
    e: EdgeRef
}

/*
Axial coordinates of a hexagonal face in the grid + vertex ref
    ___
   /   \
L < q,r > R
   \___/
 */
#[derive(Eq, Hash, PartialEq)]
pub struct HexVertex {
    q: i8,
    r: i8,
    v: VertexRef
}

fn neighbors(hex: &Hex) -> Vec<Hex> {
    [-1, 1]
        .iter()
        .map(|o|
            [
                Hex {q: hex.q, r: hex.r + o},
                Hex {q: hex.q + o, r: hex.r},
                Hex {q: hex.q + o, r: hex.r + o}
            ]
        )
        .flatten()
        .collect()
}


pub fn from_center(center: &Hex, n: i8) -> Vec<Hex> {
    (-n..=n)
        .into_iter()
        .map(|q| {
            (max(-n, -q-n)..=min(n, q+n))
                .into_iter()
                .map(move |r| Hex::new(center.q + q, center.r + r))
        })
        .flatten().collect::<Vec<Hex>>()
}

#[cfg(test)]
mod test {
    use crate::coordinates::{from_center, neighbors, Hex};
    use std::ops::Not;

    #[test]
    fn test_neighbors() {
        let f = Hex { q: 0, r: 0 };
        let neighbors = neighbors(&f);

        assert!(neighbors.contains(&Hex {q: 0, r: 0}).not());
        assert_eq!(neighbors.len(), 6)
    }

    #[test]
    fn test_from_center() {
        let coords_1 = from_center(&Hex::new(0, 0), 1);

        assert!(coords_1.contains(&Hex::new(0, 0)));
        assert_eq!(coords_1.len(), 7);

        let coords_2 = from_center(&Hex::new(0, 0), 2);

        assert!(coords_2.contains(&Hex::new(0, 0)));
        assert_eq!(coords_2.len(), 19);
    }
}