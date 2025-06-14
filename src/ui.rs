use crate::*;

/// Internal UI state representing RGB levels and frame rate.
struct UiState {
    /// RGB intensity levels (0..LEVELS)
    levels: [u32; 3],
    /// Refresh rate in Hz
    frame_rate: u64,
}

impl UiState {
    /// Print the current state to RTT output.
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

impl Default for UiState {
    /// Initialize UI state with max brightness and default frame rate.
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

/// Struct handling user input via knob and buttons.
pub struct Ui {
    knob: Knob,       // Analog input from potentiometer
    button_a: Button, // Button A
    button_b: Button, // Button B
    state: UiState,   // Current UI state
}

impl Ui {
    /// Create a new UI handler from input devices.
    pub fn new(knob: Knob, button_a: Button, button_b: Button) -> Self {
        Self {
            knob,
            button_a,
            button_b,
            state: UiState::default(),
        }
    }

    /// Main UI event loop. Updates RGB levels or frame rate depending on button state.
    pub async fn run(&mut self) -> ! {
        loop {
            let a = self.button_a.is_low();
            let b = self.button_b.is_low();
            let val = self.knob.measure().await;
            let mut changed = false;

            if a && b {
                // Both buttons pressed: update red level
                if self.state.levels[0] != val {
                    self.state.levels[0] = val;
                    changed = true;
                }
            } else if b {
                // Button B only: update green level
                if self.state.levels[1] != val {
                    self.state.levels[1] = val;
                    changed = true;
                }
            } else if a {
                // Button A only: update blue level
                if self.state.levels[2] != val {
                    self.state.levels[2] = val;
                    changed = true;
                }
            } else {
                // No button: update frame rate based on knob position
                let fr = 10 + 10 * val;
                if self.state.frame_rate != fr as u64 {
                    self.state.frame_rate = fr as u64;
                    changed = true;
                }
            }

            if changed {
                set_rgb_levels(|rgb| {
                    *rgb = self.state.levels;
                })
                .await;
                self.state.show();
            }

            Timer::after_millis(50).await;
        }
    }
}
