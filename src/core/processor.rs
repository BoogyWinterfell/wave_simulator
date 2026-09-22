// =====================================================================
// MODULE: core/processor.rs
// Responsibility: Defining the data pipeline and processing traits
// =====================================================================

/// The standardized data packet passed down the processing chain
pub struct SignalSample {
    pub time: f64,
    pub combined_amplitude: f64,
}

/// The trait that all processing chains (UI, Peak Finders, etc.) must implement
pub trait SignalReceiver {
    fn process(&mut self, sample: &SignalSample);
}

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