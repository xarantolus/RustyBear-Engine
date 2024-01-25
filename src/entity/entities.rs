use glam::{Vec3, Vec4};
use hashbrown::HashMap;
use micro_ldtk::ldtk::{LdtkFromBytes, ParseError, Project};

use crate::{
    assets::{assets::Ptr, texture::Texture2D},
    utils::{Guid, GuidGenerator},
};

//A collection of entities that represents a set of worlds.
pub struct Worlds {
    // World: speichert entities
    worlds: HashMap<Guid, hecs::World>,
    generator: GuidGenerator,
    current_world: Option<Guid>,
}

impl Default for Worlds {
    fn default() -> Self {
        Self { worlds: HashMap::new(), generator: GuidGenerator::new(), current_world: None }
    }
}

impl Worlds {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_world(&mut self, world: hecs::World) -> Guid {
        let guid = self.generator.generate();
        self.worlds.insert(guid, world);
        guid
    }

    pub fn get_world(&mut self, guid: Guid) -> Option<&hecs::World> {
        self.worlds.get(&guid)
    }

    pub fn get_mut(&mut self) -> Option<&mut hecs::World> {
        if let Some(guid) = self.current_world {
            self.worlds.get_mut(&guid)
        } else {
            None
        }
    }

    pub fn get(&mut self) -> Option<&hecs::World> {
        if let Some(guid) = self.current_world {
            self.worlds.get(&guid)
        } else {
            None
        }
    }

    pub fn start_world(&mut self, guid: Guid) {
        self.current_world = Some(guid);
    }

    pub fn from_file(json: &[u8]) -> Result<Self, ParseError> {
        let mut worlds = Worlds::new();
        let project = Project::from_bytes(json)?;

        for level in project.worlds {
            let mut world = hecs::World::new();

            assert!(level.levels.len() == 1, "Only one level per world is supported");

            for level in level.levels {
                assert!(level.neighbours.is_empty(), "Neighbours are not supported");
                assert!(
                    level.layer_instances.is_some(),
                    "The LDTk option \"Save levels separately\" is not supported."
                );

                let layers = level.layer_instances.unwrap();

                // layer_instances is in display order with the top layer being first,
                // all other layers get assigned a lower value

                let z_coord_offset = 0.99 / layers.len() as f32;
                let mut z_coord = 1.0;
                for layer in layers {
                    for tile in layer.grid_tiles {
                        world.spawn((
                            Ptr::<Texture2D>::new(Guid::new(tile.t as u64)),
                            Vec3::new(tile.px[0] as f32, tile.px[1] as f32, z_coord),
                        ));
                    }

                    z_coord -= z_coord_offset;
                }
            }

            worlds.add_world(world);
        }

        Ok(worlds)
    }
}
