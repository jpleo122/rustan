use std::ops::Not;
use enum_iterator::Sequence;

/*
Implements a hexagonal grid system with direct relationships between the
Faces, edges, and vertices. Each hexagonal face shares 3 edges and 2 vertices.
Each face is represented in axial coordinates.

Implements the relationships between the faces, edges, and vertices.

Ref: http://www-cs-students.stanford.edu/~amitp/game-programming/grids/
 */


/*
     N
    ___
 W /   \ E
  <     >
   \___/
 */

#[derive(Eq, Hash, PartialEq, Sequence)]
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
#[derive(Eq, Hash, PartialEq, Sequence)]
pub enum VertexRef {
    L,
    R
}

// Axial coordinates of a hexagonal face in the grid
#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Hex {
    q: i8,
    r: i8,
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

#[test]
fn test_neighbors() {
    let f = Hex { q: 0, r: 0 };
    let neighbors = neighbors(&f);

    assert!(neighbors.contains(&Hex {q: 0, r: 0}).not());
    assert_eq!(neighbors.len(), 6)
}