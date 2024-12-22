use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;

pub trait MenuAction {
    fn execute(&self, commands: &mut Commands);
}

pub struct ChangeStateMenuAction<State: FreelyMutableState> {
    next_state: State,
}

impl<State: FreelyMutableState> ChangeStateMenuAction<State> {
    pub fn new(next_state: State) -> Self {
        Self { next_state }
    }
}

impl<State: FreelyMutableState> MenuAction for ChangeStateMenuAction<State> {
    fn execute(&self, commands: &mut Commands) {
        commands.set_state(self.next_state.clone());
    }
}

pub struct UpdateResourceMenuAction<R: Resource + Copy> {
    resource: R,
}

impl<R: Resource + Copy> UpdateResourceMenuAction<R> {
    pub fn new(resource: R) -> Self {
        Self { resource }
    }
}

impl<R: Resource + Copy> MenuAction for UpdateResourceMenuAction<R> {
    fn execute(&self, commands: &mut Commands) {
        commands.insert_resource(self.resource);
    }
}

pub struct QuitMenuAction;

impl MenuAction for QuitMenuAction {
    fn execute(&self, commands: &mut Commands) {
        commands.send_event(AppExit::Success);
    }
}