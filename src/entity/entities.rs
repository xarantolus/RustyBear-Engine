use hashbrown::HashMap;

use std::path::Path;

use crate::assets::{self};
use crate::utils::{Guid, GuidGenerator};

//A collection of entities that represents a set of worlds.
pub struct Worlds {
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
    pub fn new() -> Self { Self::default() }

    pub fn add_world(&mut self, world: hecs::World) -> Guid {
        let guid = self.generator.generate();
        self.worlds.insert(guid, world);
        guid
    }

    pub fn get_world(&mut self, guid: Guid) -> Option<&hecs::World> { self.worlds.get(&guid) }

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

    pub fn start_world(&mut self, guid: Guid) { self.current_world = Some(guid); }

    pub fn from_ldtk_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = std::fs::read(path)?;
        let project: assets::ldtk::Project = serde_json::from_slice(&file_content)?;

        assert_eq!(project.worlds.len(), 0, "Ldtk Multi-worlds setting is not supported");
        assert_eq!(project.levels.len(), 1, "Cannot have more than one level in a ldtk file");

        assert_eq!(
            project.json_version, "1.5.3",
            "Ldtk version {} is not supported - only 1.5.3 is supported",
            project.json_version
        );

        let level = &project.levels[0];

        let li = match &level.layer_instances {
            Some(li) => li,
            None => return Err("Level has no layer instances".into()),
        };

        let layer_z_coord_offset = 0.99 / li.len() as f32;
        let mut layer_z = 1.0;

        let mut world = hecs::World::new();
        for layer in li {
            for tile in layer.grid_tiles.iter() {}

            for entity in layer.entity_instances.iter() {}

            layer_z -= layer_z_coord_offset;
        }

        let mut worlds = Worlds::new();
        let guid = worlds.add_world(world);
        worlds.start_world(guid);
        return Ok(worlds);
    }
}
