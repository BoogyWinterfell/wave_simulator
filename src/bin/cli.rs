use wave_sim::core::wave::{WaveSource, WaveShape, PropagationMode};
use wave_sim::core::processor::{SignalReceiver, SignalSample};
use wave_sim::core::simulation::Simulation;
use web_time::Instant;
use std::thread::sleep;
use std::time::Duration;

struct TerminalVisualizer {
    max_amplitude: f64,
}

impl SignalReceiver for TerminalVisualizer {
    fn process(&mut self, sample: &SignalSample) {
        let normalized_width = (sample.combined_amplitude + self.max_amplitude).round() as usize;
        println!("{:>width$}", "*", width = normalized_width + 1);
    }
}

fn main() {
    let mut sim = Simulation::new();
    sim.add_source(WaveSource::new(0.0, 2.5, 8.0, 0.0, WaveShape::Sine, PropagationMode::Traveling));
    sim.add_source(WaveSource::new(-4.3, 10.0, 3.5, 0.0, WaveShape::Sine, PropagationMode::Traveling));

    let max_amp: f64 = sim.sources.iter().map(|s| s.amplitude).sum();
    sim.add_receiver(Box::new(TerminalVisualizer { max_amplitude: max_amp }));

    let start_time = Instant::now();
    loop {
        sim.step(start_time.elapsed().as_secs_f64());
        sleep(Duration::from_millis(16));
    }
}