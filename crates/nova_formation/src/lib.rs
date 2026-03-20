//! Nova Formation - 编队系统

pub mod formation;
pub mod movement;
pub mod patterns;
pub mod prelude;
pub mod slots;

use bevy::prelude::*;
pub use formation::{Formation, FormationId, FormationManager, FormationMember};
pub use movement::FormationMoveTarget;
pub use patterns::FormationPattern;
pub use slots::SlotAssignment;

pub struct NovaFormationPlugin;

impl Plugin for NovaFormationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FormationManager>()
            .init_resource::<FormationMoveTarget>()
            .add_systems(Update, movement::formation_follow_system);
    }
}
