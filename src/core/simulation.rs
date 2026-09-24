use crate::core::wave::WaveSource;
use crate::core::processor::{PeakTracker};

/// The main orchestrator that holds everything together
pub struct Simulation {
    pub sources: Vec<WaveSource>,
    pub step_skip: usize,
    pub step_counter: usize,
    pub raw_peak_tracker: PeakTracker,
    pub sampled_peak_tracker: PeakTracker,
    pub sampled_points: Vec<[f64; 2]>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            step_skip: 24, // Initial default skip value set to 7
            step_counter: 0,
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
    pub fn update(&mut self, raw_points: &[ [f64; 2] ]) {
        // Track continuous physical signal in real time every frame
        self.raw_peak_tracker.process_frame(raw_points);

        // Process sampled signal only when skip threshold is met
        if self.step_counter >= self.step_skip {
            self.step_counter = 0;
            self.sampled_peak_tracker.process_frame(raw_points);
            self.sampled_points = raw_points.to_vec();
        } else {
            self.step_counter += 1;
        }
    }
}