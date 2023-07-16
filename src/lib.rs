// publics
pub use self::{
    kurinji::Kurinji, axis::MouseAxis, axis::GamepadAxis, bindings::Bindings,
    event_phase::EventPhase, action_event::OnActionBegin,
    action_event::OnActionActive, action_event::OnActionProgress,
    action_event::OnActionEnd,
};
// crates
mod axis;
mod util;
mod stack;
mod bindings;
mod kurinji;
mod event_phase;
mod action_event;
mod action;
mod gamepad;
mod keyboard;
mod mouse;
mod serde;
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum KurinjiStage {
    Reset,
    InputCapture,
    Event,
}

/// Adds input mapping (via code or json/ron) to an App
#[derive(Default)]
pub struct KurinjiPlugin;
impl Plugin for KurinjiPlugin {
    fn build(&self, app: &mut App) {
        app
            // input map
            .init_resource::<Kurinji>()
            // events
            .add_event::<OnActionActive>()
            .add_event::<OnActionBegin>()
            .add_event::<OnActionProgress>()
            .add_event::<OnActionEnd>()
            .configure_sets(PreUpdate, (KurinjiStage::Reset, KurinjiStage::InputCapture, KurinjiStage::Event).chain())
            .add_systems(
                PreUpdate,
                Kurinji::action_reset_system
                    
                    .in_set(KurinjiStage::Reset),
            )
            .add_systems(
                PreUpdate,
                (
                    Kurinji::gamepad_event_system,
                    Kurinji::gamepad_button_press_input_system,
                    Kurinji::kb_key_press_input_system,
                    Kurinji::mouse_button_press_input_system,
                    Kurinji::mouse_move_event_system,
                ).in_set(KurinjiStage::InputCapture)
            )
            .add_systems(
                PreUpdate,
                Kurinji::action_event_producer
                    
                    .in_set(KurinjiStage::Event)
            );
    }
}
