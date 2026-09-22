// =====================================================================
// MODULE: core/wave.rs
// Responsibility: Mathematical wave definitions and propagation
// =====================================================================
use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub enum WaveShape {
    Sine,
    Square,
    Sinc,
}

impl WaveShape {
    fn evaluate(&self, target: f64) -> f64 {
        match self {
            WaveShape::Sine => target.sin(),
            WaveShape::Square => if target.sin() >= 0.0 { 1.0 } else { -1.0 },
            WaveShape::Sinc => {
                if target.abs() < 1e-6 { 1.0 } else { target.sin() / target }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum PropagationMode {
    Traveling,
    Standing,
}

pub struct WaveSource {
    pub location: f64,
    pub frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub shape: WaveShape,
    pub mode: PropagationMode,
}

impl WaveSource {
    pub fn new(location: f64, frequency: f64, amplitude: f64, phase: f64, shape: WaveShape, mode: PropagationMode) -> Self {
        Self { location, frequency, amplitude, phase, shape, mode }
    }

    pub fn wave(&self, t: f64, x: f64) -> f64 {
        let angular_freq = 2.0 * PI * self.frequency;

        let normalized_value = match self.mode {
            PropagationMode::Traveling => {
                let target = angular_freq * t + self.phase + x;
                self.shape.evaluate(target)
            },
            PropagationMode::Standing => {
                let space_target = self.location + self.phase;
                let time_target = angular_freq * t;
                self.shape.evaluate(space_target) * time_target.cos()
            }
        };

        normalized_value * self.amplitude
    }
}