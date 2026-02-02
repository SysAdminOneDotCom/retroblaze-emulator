use anyhow::Result;
use gilrs::{Gilrs, Button, Event, EventType, Gamepad, GamepadId};
use log::{info, warn};
use crate::input_state::InputState;

/// PlayStation DualShock 4 Controller Manager
pub struct ControllerManager {
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
    input_state: InputState,
}

impl ControllerManager {
    pub fn new() -> Result<Self> {
        let gilrs = Gilrs::new().map_err(|e| anyhow::anyhow!("Failed to init gilrs: {}", e))?;
        
        let mut manager = Self {
            gilrs,
            active_gamepad: None,
            input_state: InputState::default(),
        };
        
        manager.detect_controllers();
        Ok(manager)
    }
    
    fn detect_controllers(&mut self) {
        for (_id, gamepad) in self.gilrs.gamepads() {
            info!("🎮 Controller detected: {}", gamepad.name());
            
            // Check if it's a PlayStation DualShock 4 (CUH-ZCT2U or similar)
            let name = gamepad.name().to_lowercase();
            if name.contains("dualshock") || name.contains("ps4") || name.contains("playstation") {
                info!("✅ PlayStation DualShock 4 detected!");
                self.active_gamepad = Some(gamepad.id());
                return;
            }
        }
        
        if self.active_gamepad.is_none() {
            warn!("⚠ No DualShock 4 controller detected. You can still use keyboard.");
            warn!("   Connect a PS4 controller and it will be auto-detected.");
        }
    }
    
    pub fn update(&mut self) {
        // Process controller events
        while let Some(Event { id, event, .. }) = self.gilrs.next_event() {
            match event {
                EventType::Connected => {
                    if let Some(gamepad) = self.gilrs.connected_gamepad(id) {
                        info!("🎮 Controller connected: {}", gamepad.name());
                        
                        let name = gamepad.name().to_lowercase();
                        if name.contains("dualshock") || name.contains("ps4") || name.contains("playstation") {
                            info!("✅ PlayStation DualShock 4 now active!");
                            self.active_gamepad = Some(id);
                        }
                    }
                }
                EventType::Disconnected => {
                    info!("🔌 Controller disconnected");
                    if Some(id) == self.active_gamepad {
                        self.active_gamepad = None;
                        self.input_state = InputState::default();
                    }
                }
                EventType::ButtonPressed(button, _) => {
                    if Some(id) == self.active_gamepad {
                        self.handle_button(button, true);
                    }
                }
                EventType::ButtonReleased(button, _) => {
                    if Some(id) == self.active_gamepad {
                        self.handle_button(button, false);
                    }
                }
                _ => {}
            }
        }
        
        // Update analog stick state if controller is active
        if let Some(gamepad_id) = self.active_gamepad {
            self.update_analog_state(gamepad_id);
        }
    }
    
    fn handle_button(&mut self, button: Button, pressed: bool) {
        match button {
            // D-Pad
            Button::DPadUp => self.input_state.up = pressed,
            Button::DPadDown => self.input_state.down = pressed,
            Button::DPadLeft => self.input_state.left = pressed,
            Button::DPadRight => self.input_state.right = pressed,
            
            // Face buttons (PlayStation layout)
            Button::South => self.input_state.a = pressed,      // X button (Cross)
            Button::East => self.input_state.b = pressed,       // O button (Circle)
            Button::West => self.input_state.x = pressed,       // □ button (Square)
            Button::North => self.input_state.y = pressed,      // △ button (Triangle)
            
            // Shoulder buttons
            Button::LeftTrigger => self.input_state.l = pressed,   // L1
            Button::RightTrigger => self.input_state.r = pressed,  // R1
            
            // Start/Select
            Button::Start => self.input_state.start = pressed,     // Options button
            Button::Select => self.input_state.select = pressed,   // Share button
            
            _ => {}
        }
    }
    
    fn update_analog_state(&mut self, gamepad_id: GamepadId) {
        use gilrs::Axis;
        
        let gamepad = self.gilrs.gamepad(gamepad_id);
        
        // Left analog stick for directional input
        const DEADZONE: f32 = 0.3;
        
        if let Some(left_stick_x) = gamepad.axis_data(Axis::LeftStickX) {
            let value = left_stick_x.value();
            if value < -DEADZONE {
                self.input_state.left = true;
                self.input_state.right = false;
            } else if value > DEADZONE {
                self.input_state.right = true;
                self.input_state.left = false;
            }
        }
        
        if let Some(left_stick_y) = gamepad.axis_data(Axis::LeftStickY) {
            let value = left_stick_y.value();
            if value < -DEADZONE {
                self.input_state.up = true;
                self.input_state.down = false;
            } else if value > DEADZONE {
                self.input_state.down = true;
                self.input_state.up = false;
            }
        }
    }
    
    pub fn get_state(&self) -> InputState {
        self.input_state.clone()
    }
    
    /// Check if a DualShock 4 controller is currently connected
    pub fn is_ds4_connected(&self) -> bool {
        self.active_gamepad.is_some()
    }
    
    /// Get the name of the active controller
    pub fn get_controller_name(&self) -> Option<String> {
        self.active_gamepad.map(|id| {
            self.gilrs.gamepad(id).name().to_string()
        })
    }
}
