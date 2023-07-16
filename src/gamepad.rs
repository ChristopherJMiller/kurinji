use crate::{GamepadAxis, Kurinji};
use bevy::input::gamepad::{GamepadEvent, GamepadConnection};
use bevy::prelude::*;
use bevy::ecs::system::ResMut;

impl Kurinji {
    // publics
    // buttons
    pub fn bind_gamepad_button_pressed(
        &mut self,
        pad_button: GamepadButtonType,
        action: &str,
    ) -> &mut Kurinji {
        self.bind_gamepad_button_pressed_for_player(0, pad_button, action)
    }

    pub fn bind_gamepad_button_pressed_for_player(
        &mut self,
        player: usize,
        pad_button: GamepadButtonType,
        action: &str,
    ) -> &mut Kurinji {
        *self
            .joystick_button_binding
            .entry((player, pad_button))
            .or_default() = action.to_string();
        self
    }

    pub fn unbind_gamepad_button_pressed(
        &mut self,
        pad_button: GamepadButtonType,
    ) -> &mut Kurinji {
        self.unbind_gamepad_button_pressed_for_player(0, pad_button)
    }

    pub fn unbind_gamepad_button_pressed_for_player(
        &mut self,
        player: usize,
        pad_button: GamepadButtonType,
    ) -> &mut Kurinji {
        self.joystick_button_binding.remove(&(player, pad_button));
        self
    }

    // axis
    pub fn bind_gamepad_axis(
        &mut self,
        axis: GamepadAxis,
        action: &str,
    ) -> &mut Kurinji {
        self.bind_gamepad_axis_for_player(0, axis, action)
    }

    pub fn bind_gamepad_axis_for_player(
        &mut self,
        player: usize,
        axis: GamepadAxis,
        action: &str,
    ) -> &mut Kurinji {
        *self
            .joystick_axis_binding
            .entry((player, axis))
            .or_default() = action.to_string();
        self
    }

    pub fn unbind_gamepad_axis(
        &mut self,
        pad_axis: GamepadAxis,
    ) -> &mut Kurinji {
        self.unbind_gamepad_axis_for_player(0, pad_axis);
        self
    }

    pub fn unbind_gamepad_axis_for_player(
        &mut self,
        player: usize,
        axis: GamepadAxis,
    ) -> &mut Kurinji {
        self.joystick_axis_binding.remove(&(player, axis));
        self
    }

    // crates
    pub(crate) fn get_available_player_handle(self) -> Option<usize> {
        for i in 0..(Kurinji::MAX_PLAYER_HANDLES - 1) {
            if !self.player_handles_in_use.contains(&i) {
                return Some(i);
            }
        }
        None
    }

    pub(crate) fn get_player_handle_for_gamepad(
        self,
        pad: Gamepad,
    ) -> Option<usize> {
        return match self.joystick_to_player_map.get(&pad) {
            Some(a) => Some(*a),
            _ => None,
        };
    }

    // systems
    pub(crate) fn gamepad_button_press_input_system(
        mut input_map: ResMut<Kurinji>,
        joystick_button_input: Res<Input<GamepadButton>>,
    ) {
        let button_bindings_iter = input_map.joystick_button_binding.clone();
        for (player_button_bind, action) in button_bindings_iter.iter() {
            if joystick_button_input.pressed(GamepadButton::new(
                Gamepad::new(player_button_bind.0),
                player_button_bind.1,
            )) {
                input_map.set_raw_action_strength(action, 1.0);
            }
        }
    }

    pub(crate) fn gamepad_event_system(
        mut input_map: ResMut<Kurinji>,
        mut reader: EventReader<GamepadEvent>,
    ) {
        for event in reader.iter() {
            match event {
                GamepadEvent::Axis(axis_event) => {
                    if let Some(axis_type) =
                        Kurinji::get_pad_axis_from_bevy_gamepad_axis_type(
                            axis_event.axis_type, axis_event.value,
                        )
                    {
                        if let Some(action) = input_map
                            .clone()
                            .joystick_axis_binding
                            .get(&(axis_event.gamepad.id, axis_type))
                        {
                            input_map
                                .joystick_last_action_data
                                .insert(action.to_string(), axis_event.value.abs());
                        }
                    }
                },
                GamepadEvent::Connection(conn) => match conn.connection {
                    GamepadConnection::Connected(_) => {
                        let res_player_handle =
                        input_map.clone().get_available_player_handle();
                    match res_player_handle {
                        Some(player_handle) => {
                            println!(
                                "InputMap: Gamepad Connected {:?} to player {}",
                                conn.gamepad, player_handle
                            );
                            input_map
                                .player_handles_in_use
                                .insert(player_handle);
                            input_map
                                .joystick_to_player_map
                                .insert(conn.gamepad, player_handle);
                            input_map
                                .player_to_joystick_map
                                .insert(player_handle, conn.gamepad);
                        }
                        None => {
                            println!("InputMap: Housefull. No space for more gamepads");
                        }
                    }
                    },
                    GamepadConnection::Disconnected => {
                        let opt_player_handle = input_map
                        .clone()
                        .get_player_handle_for_gamepad(conn.gamepad);
                    if let Some(player_handle) = opt_player_handle {
                        println!(
                            "InputMap: Gamepad Disconnected {:?} for player {}",
                            conn.gamepad, player_handle
                        );
                        input_map.player_handles_in_use.remove(&player_handle);
                        input_map.joystick_to_player_map.remove(&conn.gamepad);
                        input_map.player_to_joystick_map.remove(&player_handle);
                    }
                    },
                },
                _ => (),
            }
        }
        // converting events into continuous input
        for (a, s) in input_map.joystick_last_action_data.clone() {
            input_map.set_raw_action_strength(&a.to_string(), s.abs());
        }
    }
}
