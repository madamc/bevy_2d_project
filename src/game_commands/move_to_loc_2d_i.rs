use bevy::prelude::*;
use seldom_pixel::prelude::PxPosition;

use super::CommandCompleteIndicator;

pub struct MoveToLoc2DIv2Command {
    pub delta: u128,
    pub elapsedTime: u128,
    pub duration: f32,
    pub location: IVec2,
    pub destination: IVec2,
    pub entity: Entity,
}

impl bevy::ecs::system::Command for MoveToLoc2DIv2Command {
    fn write(self, world: &mut World) {
        
        let elapsedTime_sec= (self.elapsedTime as f32) / 1000.0;

        let percentage: f32 = if (elapsedTime_sec as f32 > self.duration) {1.0} else {elapsedTime_sec / self.duration};
       
        let mut ynynPxP = world.get_mut::<PxPosition>(self.entity).unwrap();
        let locv2 = Vec2::new(self.location.x as f32, self.location.y as f32);
        let destv2 = Vec2::new(self.destination.x as f32, self.destination.y as f32);
        let newPos: Vec2 = locv2.lerp(destv2, percentage);
        let newIVec2 = IVec2 { x: newPos.x as i32, y: newPos.y as i32 };
        // let newPos: Vec2 = self.location.lerp(self.destination, percentage);

        ynynPxP.0 = newIVec2;
        
        if (percentage >= 1.0) {
            let mut cc = world.get_resource_mut::<CommandCompleteIndicator>().unwrap();
            cc.completed = true;
        } 
    }
}