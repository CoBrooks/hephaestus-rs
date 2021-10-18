use std::collections::HashMap;

use crate::{
    camera::Camera,
    light::DirectionalLight,
    logger::{ self, MessageEmitter },
    entity::{ Component, EntityBuilder }
};

#[derive(Clone)]
pub struct World {
    pub entities: HashMap<usize, Vec<Box<dyn Component>>>,
    pub lights: Vec<DirectionalLight>,
    pub camera: Camera,
    pub void_color: [f32; 4],
    next_id: usize
}

impl World {
    pub fn new(camera: Camera) -> Self {
        logger::log_debug("Instantiating world.", MessageEmitter::World);
        
        World {
            entities: HashMap::new(),
            lights: Vec::new(),
            camera,
            void_color: [0.01, 0.01, 0.01, 1.0],
            next_id: 0
        }
    }

    pub fn new_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new()
    }

    pub fn add_entity(&mut self, mut entity: EntityBuilder) {
        let id = self.get_next_entity_id();
        entity.set_id(id);

        self.entities.insert(id, entity.components);
    }

    pub fn get_next_entity_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }

    pub fn get_all_ids(&self) -> Option<Vec<usize>> {
        let keys: Vec<usize> = self.entities.keys().map(|&k| k.clone()).collect();

        if keys.is_empty() {
            None
        } else {
            Some(keys)
        }
    }

    pub fn get_entity(&self, id: usize) -> Option<&Vec<Box<dyn Component>>> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: usize) -> Option<&mut Vec<Box<dyn Component>>> {
        self.entities.get_mut(&id)
    }

    pub fn get_component_by_id<T: Component>(&self, id: usize) -> Option<&T> {
        let entity = self.entities.get(&id)?;
        entity.iter()
            .find(|c| c.downcast_ref::<T>().is_some())
            .map(|c| c.downcast_ref::<T>().unwrap())
    }
    
    pub fn get_component_by_id_mut<T: Component>(&mut self, id: usize) -> Option<&mut T> {
        let entity = self.entities.get_mut(&id)?;
        entity.iter_mut()
            .find(|c| c.downcast_ref::<T>().is_some())
            .map(|c| c.downcast_mut::<T>().unwrap())
    }

    pub fn get_components_of_type<T: Component>(&self) -> Option<Vec<&T>> {
        let components: Vec<&Box<dyn Component>> = self.entities.values().flatten().collect();
        let components: Vec<&T> = components.iter()
            .filter_map(|c| c.downcast_ref::<T>())
            .collect();

        if components.is_empty() {
            None
        } else {
            Some(components)
        }
    }

    pub fn add_light(&mut self, light: DirectionalLight) {
        logger::log_debug("Adding directional light to world.", MessageEmitter::World);
        
        self.lights.push(light);
    }
}
