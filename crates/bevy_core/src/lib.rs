#![warn(missing_docs)]
//! This crate provides core functionality for Bevy Engine.

mod float_ord;
mod name;
mod task_pool_options;
mod time;

pub use bytemuck::{bytes_of, cast_slice, Pod, Zeroable};
pub use float_ord::*;
pub use name::*;
pub use task_pool_options::DefaultTaskPoolOptions;
pub use time::*;

pub mod prelude {
    //! The Bevy Core Prelude.
    #[doc(hidden)]
    pub use crate::{DefaultTaskPoolOptions, Name, Time, Timer};
}

use bevy_app::prelude::*;
use bevy_ecs::{
    entity::Entity,
    schedule::{ExclusiveSystemDescriptorCoercion, SystemLabel},
    system::IntoExclusiveSystem,
};
use bevy_utils::HashSet;
use std::ops::Range;

/// Adds core functionality to Apps.
#[derive(Default)]
pub struct CorePlugin;

/// A `SystemLabel` enum for ordering systems relative to core Bevy systems.
#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub enum CoreSystem {
    /// Updates the elapsed time. Any system that interacts with [Time] component should run after
    /// this.
    Time,
}

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Setup the default bevy task pools
        app.consume_initialization_resource::<DefaultTaskPoolOptions>()
            .unwrap_or_default()
            .create_default_pools(&mut app.world);

        app.init_resource::<Time>()
            .init_resource::<FixedTimesteps>()
            .register_type::<HashSet<String>>()
            .register_type::<Option<String>>()
            .register_type::<Entity>()
            .register_type::<Name>()
            .register_type::<Range<f32>>()
            .register_type::<Timer>()
            // time system is added as an "exclusive system" to ensure it runs before other systems
            // in CoreStage::First
            .add_system_to_stage(
                CoreStage::First,
                time_system.exclusive_system().label(CoreSystem::Time),
            );

        register_rust_types(app);
        register_math_types(app);
    }
}

fn register_rust_types(app: &mut App) {
    app.register_type::<bool>()
        .register_type::<u8>()
        .register_type::<u16>()
        .register_type::<u32>()
        .register_type::<u64>()
        .register_type::<u128>()
        .register_type::<usize>()
        .register_type::<i8>()
        .register_type::<i16>()
        .register_type::<i32>()
        .register_type::<i64>()
        .register_type::<i128>()
        .register_type::<isize>()
        .register_type::<f32>()
        .register_type::<f64>()
        .register_type::<String>()
        .register_type::<Option<String>>();
}

fn register_math_types(app: &mut App) {
    app.register_type::<bevy_math::IVec2>()
        .register_type::<bevy_math::IVec3>()
        .register_type::<bevy_math::IVec4>()
        .register_type::<bevy_math::UVec2>()
        .register_type::<bevy_math::UVec3>()
        .register_type::<bevy_math::UVec4>()
        .register_type::<bevy_math::Vec2>()
        .register_type::<bevy_math::Vec3>()
        .register_type::<bevy_math::Vec4>()
        .register_type::<bevy_math::Mat3>()
        .register_type::<bevy_math::Mat4>()
        .register_type::<bevy_math::Quat>();
}
