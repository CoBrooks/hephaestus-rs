use crate::{
    camera::Camera,
    light::DirectionalLight,
    logger::{ self, MessageEmitter },
    entity::{ Component, EntityBuilder, Entity }
};

#[derive(Clone)]
pub struct World {
    pub components: Vec<Box<dyn Component>>,
    pub lights: Vec<DirectionalLight>,
    pub camera: Camera,
    pub void_color: [f32; 4],
}

impl World {
    pub fn new(camera: Camera) -> Self {
        logger::log_debug("Instantiating world.", MessageEmitter::World);
        
        World {
            components: Vec::new(),
            lights: Vec::new(),
            camera,
            void_color: [0.01, 0.01, 0.01, 1.0]
        }
    }

    pub fn new_entity(&self) -> EntityBuilder {
        let id = self.get_next_entity_id();
        EntityBuilder::new(id)
    }

    pub fn add_entity(&mut self, mut entity: EntityBuilder) {
        self.components.append(&mut entity.components);
    }

    pub fn get_next_entity_id(&self) -> usize {
        self.get_components_of_type::<Entity>().unwrap_or(vec![]).len()
    }

    pub fn get_all_ids(&self) -> Option<Vec<&usize>> {
        let ids: Vec<&usize> = self.get_components_of_type::<Entity>()?
            .iter()
            .map(|e| e.get_id())
            .collect();

        if ids.is_empty() {
            None
        } else {
            Some(ids)
        }
    }

    pub fn get_components_by_id(&self, id: usize) -> Option<Vec<&Box<dyn Component>>> {
        let e: Vec<&Box<dyn Component>> = self.components.iter()
            .filter(|c| c.get_id() == &id)
            .collect();

        if e.is_empty() {
            None
        } else {
            Some(e)
        }
    }

    pub fn get_components_by_id_mut(&mut self, id: usize) -> Option<Vec<&mut Box<dyn Component>>> {
        let e: Vec<&mut Box<dyn Component>> = self.components.iter_mut()
            .filter(|c| c.get_id() == &id)
            .collect();

        if e.is_empty() {
            None
        } else {
            Some(e)
        }
    }

    pub fn get_component_by_id<T: Component>(&self, id: usize) -> Option<&T> {
        self.components.iter()
            .find(|c| c.get_id() == &id && c.downcast_ref::<T>().is_some())
            .map(|c| c.downcast_ref::<T>().unwrap())
    }
    
    pub fn get_component_by_id_mut<T: Component>(&mut self, id: usize) -> Option<&mut T> {
        self.components.iter_mut()
            .find(|c| c.get_id() == &id && c.downcast_ref::<T>().is_some())
            .map(|c| c.downcast_mut::<T>().unwrap())
    }

    pub fn get_components_of_type<T: Component>(&self) -> Option<Vec<&T>> {
        let c: Vec<&T> = self.components.iter()
            .filter_map(|c| c.downcast_ref::<T>())
            .collect();

        if c.is_empty() {
            None
        } else {
            Some(c)
        }
    }
    
    pub fn get_components_of_type_mut<T: Component>(&mut self) -> Option<Vec<&mut T>> {
        let c: Vec<&mut T> = self.components.iter_mut()
            .filter_map(|c| c.downcast_mut::<T>())
            .collect();

        if c.is_empty() {
            None
        } else {
            Some(c)
        }
    }

    pub fn add_light(&mut self, light: DirectionalLight) {
        logger::log_debug("Adding directional light to world.", MessageEmitter::World);
        
        self.lights.push(light);
    }
}
