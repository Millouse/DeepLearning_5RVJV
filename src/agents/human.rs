use crate::traits::{Action, Agent, Env, Observation};
use std::cell::RefCell;
use std::rc::Rc;

pub type HumanActionSlot = Rc<RefCell<Option<Action>>>;

pub struct HumanAgent {
    pub slot: HumanActionSlot,
}

impl HumanAgent {
    pub fn new(slot: HumanActionSlot) -> Self {
        Self { slot }
    }

    pub fn new_slot() -> HumanActionSlot {
        Rc::new(RefCell::new(None))
    }
}

impl Agent for HumanAgent {
    fn select_action(
        &mut self,
        _observation: &Observation,
        _legal_actions: Vec<Action>,
        _env: Option<&dyn Env>,
    ) -> Action {
        if let Some(action) = self.slot.borrow_mut().take() {
            action
        } else {
            usize::MAX // sentinelle "pas encore de coup"
        }
    }
}