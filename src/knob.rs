use crate::*;

/// Alias for single-channel ADC interface.
pub type Adc = saadc::Saadc<'static, 1>;

/// Wrapper around SAADC for reading potentiometer values.
pub struct Knob(Adc);

impl Knob {
    /// Creates a new `Knob` from an ADC and calibrates it.
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await;
        Self(adc)
    }

    /// Measures the knob position and maps it to an integer in 0..LEVELS.
    pub async fn measure(&mut self) -> u32 {
        let mut buf = [0];
        self.0.sample(&mut buf).await;

        // Clamp to ADC range (0..0x7fff) to avoid overflows
        let raw = buf[0].clamp(0, 0x7fff) as u16;

        // Scale raw ADC value (0..32767) into a float around 0.0..3.2
        let scaled = raw as f32 / 10_000.0;

        // Apply linear transform to map to discrete levels (0..LEVELS-1)
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();

        // Result is an integer in 0..=15 (LEVELS - 1)
        result as u32
    }
}
