use crate::core::wave::WaveSource;
use crate::core::processor::{SignalReceiver, SignalSample};

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