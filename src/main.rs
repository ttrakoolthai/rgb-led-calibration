//! Main entry point for the RGB LED calibration tool for the ACME Widget.
//!
//! Initializes the MicroBit board, configures peripherals for LED and knob input,
//! and spawns two concurrent tasks: one for scanning RGB LED output,
//! and one for reading UI input from buttons and potentiometer.

#![no_std]
#![no_main]

mod knob;
mod rgb;
mod ui;
pub use knob::*;
pub use rgb::*;
pub use ui::*;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer;
use microbit_bsp::{
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Level, Output, OutputDrive},
        saadc,
    },
    Button, Microbit,
};
use num_traits::float::FloatCore;

/// Shared memory for RGB brightness levels (Red, Green, Blue).
pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);

/// Shared memory for frame rate in Hz.
pub static FRAME_RATE: Mutex<ThreadModeRawMutex, u64> = Mutex::new(100);

/// Maximum level for each RGB channel.
pub const LEVELS: u32 = 16;

/// Get a copy of the current RGB levels from the shared state.
async fn get_rgb_levels() -> [u32; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}

/// Set RGB levels in-place using a closure.
async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u32; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}

/// Get the current frame rate.
pub fn get_frame_rate() -> u64 {
    FRAME_RATE.try_lock().map(|v| *v).unwrap_or(100)
}

/// Set the current frame rate.
pub async fn set_frame_rate(rate: u64) {
    let mut fr = FRAME_RATE.lock().await;
    *fr = rate;
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    // Initialize RTT for logging/debugging
    rtt_init_print!();

    // Access MicroBit v2 board peripherals
    let board = Microbit::default();

    // Bind SAADC interrupt handler
    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    // Initialize LED pins as push-pull outputs
    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
    let red = led_pin(AnyPin::from(board.p9));
    let green = led_pin(AnyPin::from(board.p8));
    let blue = led_pin(AnyPin::from(board.p16));
    let rgb: Rgb = Rgb::new([red, green, blue], 100);

    // Configure SAADC for 14-bit resolution to read potentiometer
    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT;
    let saadc = saadc::Saadc::new(
        board.saadc,
        Irqs,
        saadc_config,
        [saadc::ChannelConfig::single_ended(board.p2)],
    );

    // Create knob and UI handler
    let knob = Knob::new(saadc).await;
    let mut ui = Ui::new(knob, board.btn_a, board.btn_b);

    // Run RGB LED driver and UI event loop concurrently
    join::join(rgb.run(), ui.run()).await;

    // Should never reach this point
    panic!("fell off end of main loop");
}
