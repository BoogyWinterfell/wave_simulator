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

#[derive(Debug, Clone, Copy)]
pub struct PeakInfo {
    pub x: f64,
    pub value: f64,
}

pub struct PeakTracker {
    pub current_peak: PeakInfo,
    pub max_peak: PeakInfo,
}

impl PeakTracker {
    pub fn new() -> Self {
        Self {
            current_peak: PeakInfo { x: 0.0, value: f64::NEG_INFINITY },
            max_peak: PeakInfo { x: 0.0, value: f64::NEG_INFINITY },
        }
    }

    pub fn process_frame(&mut self, points: &[[f64; 2]]) {
        let mut frame_best = PeakInfo { x: 0.0, value: f64::NEG_INFINITY };

        for &[x, y] in points {
            if y > frame_best.value {
                frame_best = PeakInfo { x, value: y };
            }
        }

        self.current_peak = frame_best;

        if frame_best.value > self.max_peak.value {
            self.max_peak = frame_best;
        }
    }
}