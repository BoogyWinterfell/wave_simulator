use crate::core::wave::WaveSource;
use crate::core::processor::{PeakTracker};

/// The main orchestrator that holds everything together
pub struct Simulation {
    pub sources: Vec<WaveSource>,
    pub sim_time: f64,          // Internal simulation clock in seconds
    pub sample_interval: f64,   // Fixed step size (e.g., 0.05s = 20 Hz sampling)
    pub accumulator: f64,       // Unprocessed frame time pool
    pub raw_peak_tracker: PeakTracker,
    pub sampled_peak_tracker: PeakTracker,
    pub sampled_points: Vec<[f64; 2]>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            sim_time: 0.0,
            sample_interval: 1.0,
            accumulator: 0.0,
            raw_peak_tracker: PeakTracker::new(),
            sampled_peak_tracker: PeakTracker::new(),
            sampled_points: Vec::new(),
        }
    }

    pub fn add_source(&mut self, source: WaveSource) {
        self.sources.push(source);
    }

    pub fn generate_raw_points(&self, x_min: f64, x_max: f64, sample_count: usize, elapsed: f64) -> Vec<[f64; 2]> {
        let step = (x_max - x_min) / (sample_count as f64);
        let mut points = Vec::with_capacity(sample_count);

        for i in 0..sample_count {
            let x = x_min + (i as f64) * step;
            let y: f64 = self.sources.iter().map(|s| s.wave(elapsed, x)).sum();
            points.push([x, y]);
        }

        points
    }

    /// Advances simulation frame, updates peak tracking, and samples processor data based on step_skip
pub fn update(&mut self, dt_frame: f64, x_min: f64, x_max: f64, spatial_samples: usize) {
        // 1. Add elapsed real-world frame time to pool
        self.accumulator += dt_frame;

        // 2. Consume accumulator in fixed delta-t ticks
        while self.accumulator >= self.sample_interval {
            self.sim_time += self.sample_interval;

            // Generate spatial slice at the exact internal time tick
            let sampled_frame = self.generate_raw_points(x_min, x_max, spatial_samples, self.sim_time);

            // Feed slice to the receiver processor chain
            self.sampled_peak_tracker.process_frame(&sampled_frame);
            self.sampled_points = sampled_frame;

            // Subtract fixed interval from pool
            self.accumulator -= self.sample_interval;
        }
    }
}