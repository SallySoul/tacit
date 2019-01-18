use itertools::Itertools;
use std::fmt;
use std::hash::Hash;

pub const COMPONENT_BIT_COUNT: u32 = 21;

const ISOLATE_COMPONENT_MASKS: [u64; 3] = [
    0b100100100100100100100100100100100100100100100100100100100100100,
    0b010010010010010010010010010010010010010010010010010010010010010,
    0b001001001001001001001001001001001001001001001001001001001001001,
];

const ISOLATED_COMPONENT_SHIFTS: [u32; 3] = [2, 1, 0];

const CONVERSION_MASKS: [u64; 6] = [
    0b001001001001001001001001001001001001001001001001001001001001001,
    0b001000011000011000011000011000011000011000011000011000011000011,
    0b001000000001111000000001111000000001111000000001111000000001111,
    0b000000000011111000000000000000011111111000000000000000011111111,
    0b000000000011111000000000000000000000000000000001111111111111111,
    0b000000000000000000000000000000000000000000111111111111111111111,
];

#[derive(Hash, Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum NeighborRelation {
    Less = -1,
    Same = 0,
    More = 1,
}

const NeighborRelations: [NeighborRelation; 3] = [
    NeighborRelation::Same,
    NeighborRelation::Less,
    NeighborRelation::More,
];

const ComponentNeighbors: [Neighbor; 6] = [
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
    x: NeighborRelation,
    y: NeighborRelation,
    z: NeighborRelation,
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
        NeighborRelations
            .iter()
            .cartesian_product(NeighborRelations.iter())
            .cartesian_product(NeighborRelations.iter())
            .skip(1) // The First is Same, Same, Same which is the identity, not a neighbor
            .map(|((x, y), z)| Neighbor {
                x: x.clone(),
                y: y.clone(),
                z: z.clone(),
            })
    }

    pub fn component_neighbors() -> impl Iterator<Item = Neighbor> {
        ComponentNeighbors.iter().map(|neighbor| neighbor.clone())
    }
}

pub trait Key: Hash + Sized + Copy + Clone + PartialEq + Eq {
    fn root_key() -> Self;
    fn child_key(&self, i: u64) -> Self;
    fn level(&self) -> u32;
    fn neighbor_key(&self, n: Neighbor) -> Option<Self>;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct MortonKey(pub u64);

impl fmt::Debug for MortonKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MortonKey({:b})", self.0)
    }
}

impl Key for MortonKey {
    fn root_key() -> MortonKey {
        MortonKey(1)
    }

    fn child_key(&self, child: u64) -> MortonKey {
        let p = &self.0 << 3;
        MortonKey(p | child)
    }

    fn level(&self) -> u32 {
        (((self.0 as f64).log2() / 3.0).floor()) as u32
    }

    fn neighbor_key(&self, neighbor: Neighbor) -> Option<MortonKey> {
        let level = self.level();
        let mut x = self.get_component(0);
        let mut y = self.get_component(1);
        let mut z = self.get_component(2);

        let x_one_count = x.count_ones();
        let y_one_count = y.count_ones();
        let z_one_count = z.count_ones();

        let mut overflow = false;
        match neighbor.x {
            NeighborRelation::Less if x == 0 => {
                overflow = true;
            }
            NeighborRelation::Less => {
                x -= 1;
            }
            NeighborRelation::More if x_one_count < level => {
                x += 1;
            }
            NeighborRelation::More => {
                overflow = true;
            }
            NeighborRelation::Same => (),
        }

        match neighbor.y {
            NeighborRelation::Less if y == 0 => {
                overflow = true;
            }
            NeighborRelation::Less => {
                y -= 1;
            }
            NeighborRelation::More if y_one_count < level => {
                y += 1;
            }
            NeighborRelation::More => {
                overflow = true;
            }
            NeighborRelation::Same => (),
        }

        match neighbor.z {
            NeighborRelation::Less if z == 0 => {
                overflow = true;
            }
            NeighborRelation::Less => {
                z -= 1;
            }
            NeighborRelation::More if z_one_count < level => {
                z += 1;
            }
            NeighborRelation::More => {
                overflow = true;
            }
            NeighborRelation::Same => (),
        }

        if overflow {
            None
        } else {
            Some(MortonKey::from_components(x, y, z, level))
        }
    }
}

impl MortonKey {
    pub fn neighbors(self) -> impl Iterator<Item = MortonKey> {
        Neighbor::all_neighbors()
            .map(move |neighbor| self.neighbor_key(neighbor))
            .filter(|maybe_key| maybe_key.is_some())
            .map(|maybe_key| maybe_key.unwrap())
    }

    pub fn component_neighbors(self) -> impl Iterator<Item = MortonKey> {
        Neighbor::component_neighbors()
            .map(move |neighbor| self.neighbor_key(neighbor))
            .filter(|maybe_key| maybe_key.is_some())
            .map(|maybe_key| maybe_key.unwrap())
    }

    pub fn get_component(&self, component: usize) -> u32 {
        let shifted_dilated_component = &self.0 & ISOLATE_COMPONENT_MASKS[component];
        let dilated_component = shifted_dilated_component >> ISOLATED_COMPONENT_SHIFTS[component];

        let mut component = dilated_component;
        for i in 0..5 {
            // Since we have three dimensions, the gaps starts as 2, then doubles
            // so 2^1, 2^2, .., 2^4
            let gap_size = 1 << (i + 1);

            // close the gap
            let shifted_component = component >> gap_size;

            // combine original and gap shifted, then mask
            let combined_segments = shifted_component | component;
            component = combined_segments & CONVERSION_MASKS[i + 1];
        }

        // Since root is one, this ends up looking like a part of the z (2) component
        // So we make an extra mask to remove it. We really only need this for 2
        // component, but its cheaper than if statement
        let level = self.level();
        let length_mask: u64 = (1 << level) - 1;
        component &= length_mask;

        // This is a type safety thing, since component can be at most 16 bits
        component as u32
    }

    pub fn dilate_component(c: u32) -> u64 {
        let mut component = c as u64;
        for i in 0..5 {
            let gap_size = 1 << (5 - i);
            let shifted_component = component << gap_size;
            let combined_components = shifted_component | component;
            component = combined_components & CONVERSION_MASKS[4 - i];
        }
        component
    }

    pub fn from_components(x: u32, y: u32, z: u32, level: u32) -> MortonKey {
        let dilated_x = MortonKey::dilate_component(x) << 2;
        let dilated_y = MortonKey::dilate_component(y) << 1;
        let dilated_z = MortonKey::dilate_component(z);
        let root = 1 << level * 3;

        let mut key = root;
        key |= dilated_x;
        key |= dilated_y;
        key |= dilated_z;

        MortonKey(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level() {
        let mut k = MortonKey::root_key();
        assert_eq!(k.level(), 0);
        for i in 0..16 {
            k = k.child_key(4);
            assert_eq!(k.level(), i + 1);
        }

        let mut k = MortonKey::root_key();
        assert_eq!(k.level(), 0);
        for i in 0..16 {
            k = k.child_key(i % 8);
            assert_eq!(k.level(), (i + 1) as u32);
        }
    }

    #[test]
    fn get_x() {
        let mut k = MortonKey::root_key();
        for i in 0..5 {
            k = k.child_key(4);
        }
        let x = k.get_component(0);
        assert_eq!(x, (2u32.pow(5) - 1) as u32);

        k = MortonKey::root_key();
        for i in 0..COMPONENT_BIT_COUNT {
            k = k.child_key(4);
        }
        let x = k.get_component(0);
        assert_eq!(x, (2u32.pow(COMPONENT_BIT_COUNT) - 1) as u32);
    }

    #[test]
    fn get_y() {
        let mut k = MortonKey::root_key();
        for i in 0..5 {
            k = k.child_key(2);
        }
        let y = k.get_component(1);
        assert_eq!(y, (2u32.pow(5) - 1) as u32);

        k = MortonKey::root_key();
        for i in 0..COMPONENT_BIT_COUNT {
            k = k.child_key(2);
        }
        let y = k.get_component(1);
        assert_eq!(y, (2u32.pow(COMPONENT_BIT_COUNT) - 1) as u32);
    }

    #[test]
    fn get_z() {
        let mut k = MortonKey::root_key();
        for i in 0..5 {
            k = k.child_key(1);
        }
        let z = k.get_component(2);
        assert_eq!(z, (2u32.pow(5) - 1) as u32);

        k = MortonKey::root_key();
        for i in 0..COMPONENT_BIT_COUNT {
            k = k.child_key(1);
        }
        let z = k.get_component(2);
        assert_eq!(z, (2u32.pow(COMPONENT_BIT_COUNT) - 1) as u32);
    }

    #[test]
    fn get_all() {
        let mut k = MortonKey::root_key();
        for i in 0..7 {
            k = k.child_key(7);
        }
        let x = k.get_component(0);
        let y = k.get_component(1);
        let z = k.get_component(2);
        assert_eq!(x, (2u32.pow(7) - 1) as u32);
        assert_eq!(y, (2u32.pow(7) - 1) as u32);
        assert_eq!(z, (2u32.pow(7) - 1) as u32);

        k = MortonKey::root_key();
        for i in 0..16 {
            k = k.child_key(7);
        }
        let x = k.get_component(0);
        let y = k.get_component(1);
        let z = k.get_component(2);
        assert_eq!(x, (2u32.pow(16) - 1) as u32);
        assert_eq!(y, (2u32.pow(16) - 1) as u32);
        assert_eq!(z, (2u32.pow(16) - 1) as u32);
    }

    #[test]
    fn get_mix() {
        let mut k = MortonKey::root_key();
        for i in 0..16 {
            k = k.child_key(i % 8)
        }
        let x = k.get_component(0);
        let y = k.get_component(1);
        let z = k.get_component(2);
        assert_eq!(x, 0b0000111100001111);
        assert_eq!(y, 0b0011001100110011);
        assert_eq!(z, 0b0101010101010101);

        let mut k = MortonKey::root_key();
        for i in 0..16 {
            k = k.child_key((i % 4) + 3)
        }
        let x = k.get_component(0);
        let y = k.get_component(1);
        let z = k.get_component(2);
        assert_eq!(x, 0b0111011101110111);
        assert_eq!(y, 0b1001100110011001);
        assert_eq!(z, 0b1010101010101010);
    }

    #[test]
    fn dilate_ones() {
        let mut component = 0;
        let mut expected_dilated_component = 0;

        let mut dilated_component = MortonKey::dilate_component(component);
        assert_eq!(
            dilated_component, expected_dilated_component,
            "Failed at component << 0"
        );
        for i in 0..COMPONENT_BIT_COUNT {
            component = 1 | component << 1;
            expected_dilated_component = 1 | expected_dilated_component << 3;

            dilated_component = MortonKey::dilate_component(component);
            assert_eq!(
                dilated_component,
                expected_dilated_component,
                "Failed at component << {}",
                i + 1
            );
        }
    }

    #[test]
    fn from_components() {
        let x = 0;
        let y = 0;
        let z = 0;
        let level = 0;

        let key = MortonKey::from_components(x, y, z, level);
        assert_eq!(key, MortonKey::root_key());

        let x = 0b1101010;
        let y = 0b0011010;
        let z = 0b0101011;
        let level = 7;

        let key = MortonKey::from_components(x, y, z, level);
        assert_eq!(key.0, 0b1100101010111000111001);
    }

    #[test]
    fn get_root_neighbors() {
        let mut key = MortonKey::root_key();

        // All same returns self
        assert_eq!(
            key.neighbor_key(Neighbor {
                x: NeighborRelation::Same,
                y: NeighborRelation::Same,
                z: NeighborRelation::Same
            }),
            Some(MortonKey::root_key())
        );

        // Root overlfows for x
        assert_eq!(
            key.neighbor_key(Neighbor {
                x: NeighborRelation::Less,
                y: NeighborRelation::Same,
                z: NeighborRelation::Same
            }),
            None
        );
        assert_eq!(
            key.neighbor_key(Neighbor {
                x: NeighborRelation::More,
                y: NeighborRelation::Same,
                z: NeighborRelation::Same
            }),
            None
        );
        // Root overflows for y
        assert_eq!(
            key.neighbor_key(Neighbor {
                x: NeighborRelation::Same,
                y: NeighborRelation::Less,
                z: NeighborRelation::Same
            }),
            None
        );
        assert_eq!(
            key.neighbor_key(Neighbor {
                x: NeighborRelation::Same,
                y: NeighborRelation::More,
                z: NeighborRelation::Same
            }),
            None
        );
        // Root overflows for z
        assert_eq!(
            key.neighbor_key(Neighbor {
                x: NeighborRelation::Same,
                y: NeighborRelation::Same,
                z: NeighborRelation::Less
            }),
            None
        );
        assert_eq!(
            key.neighbor_key(Neighbor {
                x: NeighborRelation::Same,
                y: NeighborRelation::Same,
                z: NeighborRelation::More
            }),
            None
        );

        // Misc
        assert_eq!(
            key.neighbor_key(Neighbor {
                x: NeighborRelation::Less,
                y: NeighborRelation::More,
                z: NeighborRelation::Less
            }),
            None
        );
        assert_eq!(
            key.neighbor_key(Neighbor {
                x: NeighborRelation::More,
                y: NeighborRelation::More,
                z: NeighborRelation::More
            }),
            None
        );
    }

    #[test]
    fn get_neighbors() {
        let root = MortonKey::root_key();

        let mut child = root.child_key(0);
        assert_eq!(child.0, 0b1000);
        assert_eq!(
            child.neighbor_key(Neighbor {
                x: NeighborRelation::Same,
                y: NeighborRelation::Same,
                z: NeighborRelation::Same
            }),
            Some(MortonKey(0b1000))
        );
        assert_eq!(
            child.neighbor_key(Neighbor {
                x: NeighborRelation::More,
                y: NeighborRelation::More,
                z: NeighborRelation::Same
            }),
            Some(MortonKey(0b1110))
        );
        assert_eq!(
            child.neighbor_key(Neighbor {
                x: NeighborRelation::Less,
                y: NeighborRelation::More,
                z: NeighborRelation::Same
            }),
            None
        );

        child = root.child_key(7);
        assert_eq!(child.0, 0b1111);
        assert_eq!(
            child.neighbor_key(Neighbor {
                x: NeighborRelation::Same,
                y: NeighborRelation::Same,
                z: NeighborRelation::Same
            }),
            Some(MortonKey(0b1111))
        );
        assert_eq!(
            child.neighbor_key(Neighbor {
                x: NeighborRelation::More,
                y: NeighborRelation::More,
                z: NeighborRelation::Same
            }),
            None
        );
        assert_eq!(
            child.neighbor_key(Neighbor {
                x: NeighborRelation::Less,
                y: NeighborRelation::More,
                z: NeighborRelation::Same
            }),
            None
        );
        assert_eq!(
            child.neighbor_key(Neighbor {
                x: NeighborRelation::Same,
                y: NeighborRelation::Same,
                z: NeighborRelation::Less
            }),
            Some(MortonKey(0b1110))
        );

        child = MortonKey(0b1111000111000111000111000);
        assert_eq!(
            child.neighbor_key(Neighbor {
                x: NeighborRelation::Less,
                y: NeighborRelation::Less,
                z: NeighborRelation::Less
            }),
            Some(MortonKey(0b1111000111000111000000111))
        );
        assert_eq!(
            child.neighbor_key(Neighbor {
                x: NeighborRelation::More,
                y: NeighborRelation::More,
                z: NeighborRelation::More
            }),
            Some(MortonKey(0b1111000111000111000111111))
        );
    }

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
