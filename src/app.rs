use crate::core::wave::WaveSource;
use crate::core::processor::{SignalReceiver, SignalSample};
// =====================================================================
// MODULE: app.rs (or processors)
// Responsibility: Concrete implementations of your processing chains
// =====================================================================

/// Chain 1: Handles the terminal visualization
pub struct TerminalVisualizer {
    pub max_amplitude: f64,
}

impl SignalReceiver for TerminalVisualizer {
    fn process(&mut self, sample: &SignalSample) {
        let normalized_width = (sample.combined_amplitude + self.max_amplitude).round() as usize;
        println!("{:>width$}", "*", width = normalized_width + 1);
    }
}

/// Chain 2: Tracks the highest peak encountered so far
pub struct PeakTracker {
    pub max_seen: f64,
    pub peak_time: f64,
}

impl PeakTracker {
    pub fn new() -> Self {
        Self { max_seen: f64::NEG_INFINITY, peak_time: 0.0 }
    }
}

impl SignalReceiver for PeakTracker {
    fn process(&mut self, sample: &SignalSample) {
        if sample.combined_amplitude > self.max_seen {
            self.max_seen = sample.combined_amplitude;
            self.peak_time = sample.time;
            // In a GUI, you'd save this to draw a dot. 
            // We won't print here to avoid ruining the terminal visualizer's formatting.
        }
    }
}

/// The main orchestrator that holds everything together
pub struct Simulation {
    pub sources: Vec<WaveSource>,
    pub receivers: Vec<Box<dyn SignalReceiver>>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            receivers: Vec::new(),
        }
    }

    pub fn add_source(&mut self, source: WaveSource) {
        self.sources.push(source);
    }

    pub fn add_receiver(&mut self, receiver: Box<dyn SignalReceiver>) {
        self.receivers.push(receiver);
    }

    /// Evaluates the wave at a specific time and passes it to all receivers
    pub fn step(&mut self, elapsed_time: f64) {
        // 1. Generate the combined signal
        let combined_amplitude: f64 = self.sources.iter()
            .map(|source| source.wave(elapsed_time, source.location))
            .sum();

        // 2. Package it into our standard struct
        let sample = SignalSample {
            time: elapsed_time,
            combined_amplitude,
        };

        // 3. Pass it to every receiver in the chain
        for receiver in self.receivers.iter_mut() {
            receiver.process(&sample);
        }
    }
}