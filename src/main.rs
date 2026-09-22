mod app;
mod core;

use crate::app::{Simulation, TerminalVisualizer, PeakTracker};
use crate::core::wave::{WaveSource, WaveShape, PropagationMode};

use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let mut sim = Simulation::new();

    // 1. Add Wave Sources
    sim.add_source(WaveSource::new(0.0, 2.5, 8.0, 0.0, WaveShape::Sine, PropagationMode::Traveling));
    sim.add_source(WaveSource::new(5.0, 0.5, 30.0, 0.0, WaveShape::Sinc, PropagationMode::Standing));

    // Calculate maximum possible amplitude for the visualizer
    let max_amp: f64 = sim.sources.iter().map(|s| s.amplitude).sum();

    // 2. Register Processing Chains
    sim.add_receiver(Box::new(TerminalVisualizer { max_amplitude: max_amp }));
    sim.add_receiver(Box::new(PeakTracker::new()));

    let start_time = Instant::now();
    println!("Starting 1D Simulation Pipeline. Press Ctrl+C to stop.\n");

    // 3. Run the loop
    // When you move to `egui`, this loop disappears and `sim.step()` is 
    // called inside the GUI framework's `update()` method.
    loop {
        let elapsed = start_time.elapsed().as_secs_f64();
        
        sim.step(elapsed);

        sleep(Duration::from_millis(16));
    }
}