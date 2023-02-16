use bevy::prelude::*;

mod show_message_ui;
mod hide_message_ui;
mod move_to_loc_3d;
mod move_to_loc_2d;
mod move_to_loc_2d_i;
mod play_anim_once;
mod change_anim;
mod ynyn_anim_state_chgs;
mod type_writer_effect;
mod pause_queue;

use crate::components::CommandCompleteIndicator;
use crate::mandoqueue::{Mando, MandoQueue};

use self::show_message_ui::ShowMessageUICommand;
use self::hide_message_ui::HideMessageUICommand;
use self::move_to_loc_3d::MoveToLoc3DCommand;
use self::move_to_loc_2d::MoveToLoc2DCommand;
use self::move_to_loc_2d_i::MoveToLoc2DIv2Command;
use self::play_anim_once::PlayAnimOnceCommand;
use self::change_anim::ChangeAnim;
use self::type_writer_effect::AffectTypeWriterCommand;
use self::pause_queue::PauseQueueCommand;
use self::ynyn_anim_state_chgs::{YNYNIdleLCMD, YNYNWalkLCMD, YNYNWalkRCMD, YNYNIdleRCMD};

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const ATLAS_COLUMNS: usize = 8;
const TIME_STEP: f32 = 10.0;
const PLYR_SPEED: f32 = 2.0;

//TODO: Integrate this into the mandoqueue adding, or make a macro instead.
struct AddToMandoQueueCommand{
    mandos: Vec<Mando>,
}

impl bevy::ecs::system::Command for AddToMandoQueueCommand {
    fn write(self, world: &mut World) {
        let mut vecParams: Vec<Mando> = Vec::new();
        let mut mq = world.get_resource_mut::<MandoQueue>().unwrap();
        mq.mandos.push_back(vecParams);
    }
}

struct StringCommmand(String);
impl bevy::ecs::system::Command for StringCommmand {
    fn write(self, world: &mut World) {
        println!("{}", self.0)
    }
}

struct FillerMandoCommand;

impl bevy::ecs::system::Command for FillerMandoCommand {
    fn write(self, world: &mut World) {
        let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
        cc.completed = true;
    }
}

struct HolderMandoCommand; 

impl bevy::ecs::system::Command for HolderMandoCommand {
    fn write(self, world: &mut World) {
        //depends on the Event 
    }
}

pub trait GameCommandsExt {
    fn add_to_mando_queue(&mut self, params: Vec<Mando>);
    fn print_message(&mut self, msg: String);
    fn play_anim_once(&mut self, entity: Entity);
    fn ynyn_walk_l(&mut self, entity: Entity);
    fn ynyn_Idle_l(&mut self, entity: Entity);
    fn ynyn_walk_r(&mut self, entity: Entity);
    fn ynyn_Idle_r(&mut self, entity: Entity);
    fn change_anim(&mut self);
    fn move_to_loc_3d(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: Vec3, destination: Vec3, entity: Entity ); 
    fn move_to_loc_2d(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: Vec2, destination: Vec2, entity: Entity ); 
    fn move_to_loc_2d_i(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: IVec2, destination: IVec2, entity: Entity ); 
    fn spawn_message_ui(&mut self);
    fn despawn_message_ui(&mut self);
    fn pause_queue(&mut self);
    fn affect_typewriter(&mut self, elapsed_time: u128, message: &str);
    fn filler_mando(&mut self);
    fn holder_mando(&mut self);
    // fn get_currrent_mando() -> Mando;
}

impl<'w, 's> GameCommandsExt for Commands<'w, 's> {

    fn add_to_mando_queue(&mut self, mandos: Vec<Mando>) {
        self.add(AddToMandoQueueCommand {mandos: mandos});
    }
    fn print_message(&mut self, msg: String) {
        self.add(StringCommmand(msg));
    }

    fn play_anim_once(&mut self, entity: Entity) {
        self.add(PlayAnimOnceCommand {entity: entity});
    }

    fn ynyn_walk_l(&mut self, entity: Entity) {
        self.add(YNYNWalkLCMD {entity: entity});
    }

    fn ynyn_Idle_l(&mut self, entity: Entity) {
        self.add(YNYNIdleLCMD {entity: entity});
    }

    fn ynyn_walk_r(&mut self, entity: Entity) {
        self.add(YNYNWalkRCMD {entity: entity});
    }

    fn ynyn_Idle_r(&mut self, entity: Entity) {
        self.add(YNYNIdleRCMD {entity: entity});
    }

    fn change_anim(&mut self) {
        self.add(ChangeAnim);
    }

    fn move_to_loc_3d(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: Vec3, destination: Vec3, entity: Entity ) {// mParams: Vec<MandoParam>) {
        self.add(MoveToLoc3DCommand { delta: delta, elapsedTime: elapsedTime, duration: duration, location: location, destination: destination, entity: entity });
    }

    fn move_to_loc_2d(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: Vec2, destination: Vec2, entity: Entity ) {// mParams: Vec<MandoParam>) {
        self.add(MoveToLoc2DCommand { delta: delta, elapsedTime: elapsedTime, duration: duration, location: location, destination: destination, entity: entity });
    }

    fn move_to_loc_2d_i(&mut self, delta: u128, elapsedTime: u128, duration: f32, location: IVec2, destination: IVec2, entity: Entity ) {// mParams: Vec<MandoParam>) {
        self.add(MoveToLoc2DIv2Command { delta: delta, elapsedTime: elapsedTime, duration: duration, location: location, destination: destination, entity: entity });
    }

    fn spawn_message_ui(&mut self) {
        self.add(ShowMessageUICommand {});
    }
    fn despawn_message_ui(&mut self) {
        self.add(HideMessageUICommand {});
    }

    fn pause_queue(&mut self) {
        self.add(PauseQueueCommand {});
    }

    fn affect_typewriter(&mut self, elapsed_time: u128, message: &str) {
        self.add(AffectTypeWriterCommand {elapsed_time: elapsed_time, message: message.to_owned()})
    }

    fn filler_mando(&mut self) {
        self.add(FillerMandoCommand {});
    }
    fn holder_mando(&mut self) {
        self.add(HolderMandoCommand);
    }
    // fn get_currrent_mando() -> Mando {
    //     // self.add()
    // }
}