use crate::Kurinji;
use bevy::ecs::system::Res;
use bevy::prelude::*;

/// Event that is fired when action is active.
/// This depends on what event phase is set to
/// the action by default it will be OnProgress.
#[derive(Event)]
pub struct OnActionActive {
    pub action: String,
    pub strength: f32,
}
/// Event that gets fired at the beginning
/// of an action
#[derive(Event)]
pub struct OnActionBegin {
    pub action: String,
    pub strength: f32,
}
/// Event that gets fired during
/// an action
#[derive(Event)]
pub struct OnActionProgress {
    pub action: String,
    pub strength: f32,
}
/// Event that gets fired at the end
/// of an action
#[derive(Event)]
pub struct OnActionEnd {
    pub action: String,
}
impl Kurinji {
    pub(crate) fn action_event_producer(
        input_map: Res<Kurinji>,
        mut on_active_event: EventWriter<OnActionActive>,
        mut on_begin_event: EventWriter<OnActionBegin>,
        mut on_progress_event: EventWriter<OnActionProgress>,
        mut on_end_event: EventWriter<OnActionEnd>,
    ) {
        // merge keys, required for action end to be fired properly.
        // Since released actions won't be part of raw strength
        let mut _actions = input_map.action_raw_strength.clone();
        for (a, s) in input_map.action_prev_strength.iter() {
            if !_actions.contains_key(a) {
                _actions.insert(a.clone(), *s);
            }
        }
        for (action, strength) in _actions {
            if input_map.is_action_active(&action) {
                on_active_event.send(OnActionActive {
                    action: action.clone(),
                    strength,
                });
            }
            if input_map.did_action_just_began(&action) {
                on_begin_event.send(OnActionBegin {
                    action: action.clone(),
                    strength,
                });
            }
            if input_map.is_action_in_progress(&action) {
                on_progress_event.send(OnActionProgress {
                    action: action.clone(),
                    strength,
                });
            }
            if input_map.did_action_just_end(&action) {
                on_end_event.send(OnActionEnd {
                    action: action.clone(),
                });
            }
        }
    }
}
