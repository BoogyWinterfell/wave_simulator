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