use bevy::{prelude::*, ecs::system::SystemState};

use crate::{ui::UIMessageWindow, game_commands::CommandCompleteIndicator, YNYNWalkLComp, YNYNIdleLComp, YNYNWalkRComp, YNYNIdleRComp};

pub struct YNYNWalkLCMD {pub entity: Entity}

impl bevy::ecs::system::Command for YNYNWalkLCMD {
    fn write(self, world: &mut World) {
        let mut state = SystemState::<(Commands)>::new(world);
        let mut cmds = state.get_mut(world);
        cmds.entity(self.entity).insert(YNYNWalkLComp);
        state.apply(world);
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;

    }
}

pub struct YNYNIdleLCMD {pub entity: Entity}

impl bevy::ecs::system::Command for YNYNIdleLCMD {
    fn write(self, world: &mut World) {
        let mut state = SystemState::<(Commands)>::new(world);
        let mut cmds = state.get_mut(world);
        cmds.entity(self.entity).insert(YNYNIdleLComp);
        state.apply(world);
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;
        print!("scroomScroom");
    }
}

pub struct YNYNWalkRCMD {pub entity: Entity}

impl bevy::ecs::system::Command for YNYNWalkRCMD {
    fn write(self, world: &mut World) {
        let mut state = SystemState::<(Commands)>::new(world);
        let mut cmds = state.get_mut(world);
        cmds.entity(self.entity).insert(YNYNWalkRComp);
        state.apply(world);
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;

    }
}

pub struct YNYNIdleRCMD {pub entity: Entity}

impl bevy::ecs::system::Command for YNYNIdleRCMD {
    fn write(self, world: &mut World) {
        let mut state = SystemState::<(Commands)>::new(world);
        let mut cmds = state.get_mut(world);
        cmds.entity(self.entity).insert(YNYNIdleRComp);
        state.apply(world);
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;
        print!("scroomScroom");
    }
}