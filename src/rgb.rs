use crate::*;

/// Type alias for the array of RGB output pins.
type RgbPins = [Output<'static, AnyPin>; 3];

/// RGB LED controller responsible for scanning red, green, and blue channels.
///
/// This struct handles time-multiplexed PWM control of RGB LED channels.
/// Each channel is toggled on for a duration proportional to its brightness level,
/// and the frame rate determines the total update frequency.
pub struct Rgb {
    /// GPIO pins for red, green, and blue LEDs.
    rgb: RgbPins,
    /// Cached brightness levels for each color (0..=LEVELS).
    levels: [u32; 3],
    /// Delay per brightness tick in microseconds, derived from frame rate.
    tick_time: u64,
}

impl Rgb {
    /// Compute the time per brightness tick based on the target frame rate.
    /// Each frame consists of scanning 3 color channels with up to LEVELS brightness steps.
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    /// Create a new `Rgb` instance from the given output pins and initial frame rate.
    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        let tick_time = Self::frame_tick_time(frame_rate);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    /// Turn on one LED channel for a time proportional to its brightness.
    /// The remaining portion of the frame is spent off to keep timing consistent.
    async fn step(&mut self, led: usize) {
        let level = self.levels[led];
        if level > 0 {
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await;
            self.rgb[led].set_low();
        }
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    /// Main RGB scan loop. Continuously updates brightness values and frame rate,
    /// then cycles through red, green, and blue LED channels with appropriate timing.
    pub async fn run(mut self) -> ! {
        loop {
            self.levels = get_rgb_levels().await;
            self.tick_time = Self::frame_tick_time(crate::get_frame_rate());

            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
