#![feature(trait_upcasting)]

use std::{any::Any, cell::RefCell, rc::Rc};

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

trait Component: Any {}

pub struct Transform {
    pub position: Vec3,
}

impl Component for Transform {}

struct Parent {
    object: Rc<GameObject>,
}

impl Component for Parent {}

struct GameObject {
    name: String,
    // uuid: Uuid,
    components: Vec<Rc<dyn Any>>,
}

impl GameObject {
    fn new(name: String) -> Self {
        Self {
            name,
            components: Vec::new(),
        }
    }

    fn add_component<T: Component>(&mut self, component: T) {
        self.components.push(Rc::new(RefCell::new(component)));
    }

    fn get_component<T: Component>(&self) -> Option<Rc<RefCell<T>>> {
        for component in &self.components {
            if let Ok(result) = Rc::clone(component).downcast() {
                return Some(result);
            }
        }
        None
    }
}

struct Scene {
    name: String,
    game_objects: Vec<Rc<GameObject>>,
}

struct Program {
    game_scenes: Vec<Scene>,
}

// ECS - entity-component-system

fn main() {
    let mut game_object = GameObject::new("Mario".to_string());
    game_object.add_component(Transform {
        position: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
    });
    let transform = game_object.get_component::<Transform>().unwrap();
    transform.borrow_mut().position.x += 4.0;
    println!(
        "{}, {}, {}",
        transform.borrow().position.x,
        transform.borrow().position.y,
        transform.borrow().position.z
    );
}
