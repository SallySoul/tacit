use itertools::Itertools;

#[derive(Hash, Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum NeighborRelation {
    Less = -1,
    Same = 0,
    More = 1,
}

const NEIGHBOR_RELATIONS: [NeighborRelation; 3] = [
    NeighborRelation::Same,
    NeighborRelation::Less,
    NeighborRelation::More,
];

const COMPONENT_NEIGHBORS: [Neighbor; 6] = [
    Neighbor {
        x: NeighborRelation::Same,
        y: NeighborRelation::Same,
        z: NeighborRelation::Less,
    },
    Neighbor {
        x: NeighborRelation::Same,
        y: NeighborRelation::Same,
        z: NeighborRelation::More,
    },
    Neighbor {
        x: NeighborRelation::Same,
        y: NeighborRelation::Less,
        z: NeighborRelation::Same,
    },
    Neighbor {
        x: NeighborRelation::Same,
        y: NeighborRelation::More,
        z: NeighborRelation::Same,
    },
    Neighbor {
        x: NeighborRelation::Less,
        y: NeighborRelation::Same,
        z: NeighborRelation::Same,
    },
    Neighbor {
        x: NeighborRelation::More,
        y: NeighborRelation::Same,
        z: NeighborRelation::Same,
    },
];

#[derive(Hash, Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Neighbor {
    pub x: NeighborRelation,
    pub y: NeighborRelation,
    pub z: NeighborRelation,
}

impl Neighbor {
    pub fn from_components(
        x: NeighborRelation,
        y: NeighborRelation,
        z: NeighborRelation,
    ) -> Neighbor {
        Neighbor { x, y, z }
    }

    pub fn all_neighbors() -> impl Iterator<Item = Neighbor> {
        NEIGHBOR_RELATIONS
            .iter()
            .cartesian_product(NEIGHBOR_RELATIONS.iter())
            .cartesian_product(NEIGHBOR_RELATIONS.iter())
            .skip(1) // The First is Same, Same, Same which is the identity, not a neighbor
            .map(|((x, y), z)| Neighbor {
                x: x.clone(),
                y: y.clone(),
                z: z.clone(),
            })
    }

    pub fn component_neighbors() -> impl Iterator<Item = Neighbor> {
        COMPONENT_NEIGHBORS.iter().map(|neighbor| neighbor.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_and_order() {
        let mut neighbors: Vec<Neighbor> = Neighbor::component_neighbors().collect();
        neighbors.sort();
        assert_eq!(
            neighbors,
            vec![
                Neighbor {
                    x: NeighborRelation::Less,
                    y: NeighborRelation::Same,
                    z: NeighborRelation::Same,
                },
                Neighbor {
                    x: NeighborRelation::Same,
                    y: NeighborRelation::Less,
                    z: NeighborRelation::Same,
                },
                Neighbor {
                    x: NeighborRelation::Same,
                    y: NeighborRelation::Same,
                    z: NeighborRelation::Less,
                },
                Neighbor {
                    x: NeighborRelation::Same,
                    y: NeighborRelation::Same,
                    z: NeighborRelation::More,
                },
                Neighbor {
                    x: NeighborRelation::Same,
                    y: NeighborRelation::More,
                    z: NeighborRelation::Same,
                },
                Neighbor {
                    x: NeighborRelation::More,
                    y: NeighborRelation::Same,
                    z: NeighborRelation::Same,
                },
            ]
        );

        let mut neighbors: Vec<Neighbor> = Neighbor::all_neighbors().collect();
        neighbors.sort();

        assert_eq!(
            neighbors,
            vec![
                Neighbor {
                    x: NeighborRelation::Less,
                    y: NeighborRelation::Less,
                    z: NeighborRelation::Less,
                },
                Neighbor {
                    x: NeighborRelation::Less,
                    y: NeighborRelation::Less,
                    z: NeighborRelation::Same,
                },
                Neighbor {
                    x: NeighborRelation::Less,
                    y: NeighborRelation::Less,
                    z: NeighborRelation::More,
                },
                Neighbor {
                    x: NeighborRelation::Less,
                    y: NeighborRelation::Same,
                    z: NeighborRelation::Less,
                },
                Neighbor {
                    x: NeighborRelation::Less,
                    y: NeighborRelation::Same,
                    z: NeighborRelation::Same,
                },
                Neighbor {
                    x: NeighborRelation::Less,
                    y: NeighborRelation::Same,
                    z: NeighborRelation::More,
                },
                Neighbor {
                    x: NeighborRelation::Less,
                    y: NeighborRelation::More,
                    z: NeighborRelation::Less,
                },
                Neighbor {
                    x: NeighborRelation::Less,
                    y: NeighborRelation::More,
                    z: NeighborRelation::Same,
                },
                Neighbor {
                    x: NeighborRelation::Less,
                    y: NeighborRelation::More,
                    z: NeighborRelation::More,
                },
                Neighbor {
                    x: NeighborRelation::Same,
                    y: NeighborRelation::Less,
                    z: NeighborRelation::Less,
                },
                Neighbor {
                    x: NeighborRelation::Same,
                    y: NeighborRelation::Less,
                    z: NeighborRelation::Same,
                },
                Neighbor {
                    x: NeighborRelation::Same,
                    y: NeighborRelation::Less,
                    z: NeighborRelation::More,
                },
                Neighbor {
                    x: NeighborRelation::Same,
                    y: NeighborRelation::Same,
                    z: NeighborRelation::Less,
                },
                Neighbor {
                    x: NeighborRelation::Same,
                    y: NeighborRelation::Same,
                    z: NeighborRelation::More,
                },
                Neighbor {
                    x: NeighborRelation::Same,
                    y: NeighborRelation::More,
                    z: NeighborRelation::Less,
                },
                Neighbor {
                    x: NeighborRelation::Same,
                    y: NeighborRelation::More,
                    z: NeighborRelation::Same,
                },
                Neighbor {
                    x: NeighborRelation::Same,
                    y: NeighborRelation::More,
                    z: NeighborRelation::More,
                },
                Neighbor {
                    x: NeighborRelation::More,
                    y: NeighborRelation::Less,
                    z: NeighborRelation::Less,
                },
                Neighbor {
                    x: NeighborRelation::More,
                    y: NeighborRelation::Less,
                    z: NeighborRelation::Same,
                },
                Neighbor {
                    x: NeighborRelation::More,
                    y: NeighborRelation::Less,
                    z: NeighborRelation::More,
                },
                Neighbor {
                    x: NeighborRelation::More,
                    y: NeighborRelation::Same,
                    z: NeighborRelation::Less,
                },
                Neighbor {
                    x: NeighborRelation::More,
                    y: NeighborRelation::Same,
                    z: NeighborRelation::Same,
                },
                Neighbor {
                    x: NeighborRelation::More,
                    y: NeighborRelation::Same,
                    z: NeighborRelation::More,
                },
                Neighbor {
                    x: NeighborRelation::More,
                    y: NeighborRelation::More,
                    z: NeighborRelation::Less,
                },
                Neighbor {
                    x: NeighborRelation::More,
                    y: NeighborRelation::More,
                    z: NeighborRelation::Same,
                },
                Neighbor {
                    x: NeighborRelation::More,
                    y: NeighborRelation::More,
                    z: NeighborRelation::More,
                },
            ]
        );
    }
}
