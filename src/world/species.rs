use std::{cell::RefCell, rc::Rc};

use super::grid::GridPos;
use crate::world::World;

pub struct Species {
    members: Vec<GridPos>,
    world: Rc<RefCell<World>>,
}

impl Species {
    pub fn new(world: &Rc<RefCell<World>>) -> Self {
        Self {
            members: Vec::new(),
            world: world.clone(),
        }
    }
}
