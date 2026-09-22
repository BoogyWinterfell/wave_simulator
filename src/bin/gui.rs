use eframe::egui::{self, Color32};
use egui_plot::{Line, Plot, PlotPoints, Points};
use wave_sim::core::wave::{WaveSource, WaveShape, PropagationMode};
use wave_sim::core::processor::PeakTracker;
use wave_sim::core::simulation::Simulation;
use web_time::Instant;

pub struct WaveApp {
    sim: Simulation,
    peak_tracker: PeakTracker,
    start_time: Instant,
    x_min: f64,
    x_max: f64,
    max_amplitude: f64,
    sample_count: usize,
}

impl WaveApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut sim = Simulation::new();
        sim.add_source(WaveSource::new(0.0, 2.5, 8.0, 0.0, WaveShape::Sine, PropagationMode::Traveling));
        sim.add_source(WaveSource::new(5.0, 0.5, 30.0, 0.0, WaveShape::Sinc, PropagationMode::Standing));

        let max_amplitude: f64 = sim.sources.iter().map(|s| s.amplitude).sum();

        Self {
            sim,
            peak_tracker: PeakTracker::new(),
            start_time: Instant::now(),
            x_min: -10.0,
            x_max: 10.0,
            max_amplitude,
            sample_count: 500,
        }
    }
}

impl eframe::App for WaveApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let step = (self.x_max - self.x_min) / (self.sample_count as f64);
        let mut points: Vec<[f64; 2]> = Vec::with_capacity(self.sample_count);

        for i in 0..self.sample_count {
            let x = self.x_min + (i as f64) * step;
            let y: f64 = self.sim.sources.iter().map(|s| s.wave(elapsed, x)).sum();
            points.push([x, y]);
        }

        // Delegate peak analysis completely to processor
        self.peak_tracker.process_frame(&points);

        ui.heading("1D Wave Signal Simulator");
        ui.label(format!(
            "Elapsed: {:.2}s | Current Peak: {:.2} at x = {:.2} | Max Peak: {:.2} at x = {:.2}",
            elapsed, 
            self.peak_tracker.current_peak.value, self.peak_tracker.current_peak.x,
            self.peak_tracker.max_peak.value, self.peak_tracker.max_peak.x
        ));

        let y_margin = self.max_amplitude * 1.1;

        Plot::new("wave_canvas")
            .view_aspect(2.0)
            .include_x(self.x_min)
            .include_x(self.x_max)
            .include_y(y_margin)
            .include_y(-y_margin)
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new("Combined Wave", PlotPoints::from(points)));

                // Blue marker for current step peak
                let current_marker = Points::new(
                    "Current Peak", 
                    vec![[self.peak_tracker.current_peak.x, self.peak_tracker.current_peak.value]]
                )
                .color(Color32::LIGHT_BLUE)
                .radius(6.0);

                // Red marker for max peak overall
                let max_marker = Points::new(
                    "Max Peak Overall", 
                    vec![[self.peak_tracker.max_peak.x, self.peak_tracker.max_peak.value]]
                )
                .color(Color32::LIGHT_RED)
                .radius(6.0);

                plot_ui.points(current_marker);
                plot_ui.points(max_marker);
            });

        ui.ctx().request_repaint();
    }
}

// ==========================================
// Native Entry Point (Desktop: Linux/Mac/Win)
// ==========================================
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([900.0, 500.0])
            .with_title("Wave & Signal Simulator"),
        ..Default::default()
    };

    eframe::run_native(
        "Wave & Signal Simulator",
        options,
        Box::new(|cc| Ok(Box::new(WaveApp::new(cc)))),
    )
}

// ==========================================
// WebAssembly Entry Point (Browser / Trunk)
// ==========================================
#[cfg(target_arch = "wasm32")]
fn main() {
    // Use eframe's re-exported JsCast trait
    use eframe::wasm_bindgen::JsCast;

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window found")
            .document()
            .expect("No document found");

        let canvas = document
            .get_element_by_id("wave_canvas")
            .expect("Failed to find element with id 'wave_canvas'")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Element 'wave_canvas' is not an HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(WaveApp::new(cc)))),
            )
            .await;

        if let Err(e) = start_result {
            log::error!("Failed to start eframe: {e:?}");
        }
    });
}