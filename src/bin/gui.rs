use eframe::egui::{self, Color32};
use egui_plot::{Line, Plot, PlotPoints, Points};
use wave_sim::core::wave::{WaveSource, WaveShape, PropagationMode};
use wave_sim::core::simulation::Simulation;
use web_time::Instant;

pub struct WaveApp {
    sim: Simulation,
    start_time: Instant,
    last_frame_instant: Instant,
    x_min: f64,
    x_max: f64,
    max_amplitude: f64,
    sample_count: usize,
}

impl WaveApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut sim = Simulation::new();
        sim.add_source(WaveSource::new(0.0, 2.5, 10.0, 0.0, WaveShape::Sine, PropagationMode::Traveling));
        sim.add_source(WaveSource::new(5.0, 0.5, 50.0, 0.0, WaveShape::Sinc, PropagationMode::Standing));
        sim.add_source(WaveSource::new(120.0, 1.0, 100.0, 0.0, WaveShape::GaussianPulse {width: 1.5}, PropagationMode::Traveling));

        let max_amplitude: f64 = sim.sources.iter().map(|s| s.amplitude).sum();
        
        let x_min = -10.0;
        let x_max = 10.0;
        let sample_count = 500;
        let step = (x_max - x_min) / (sample_count as f64);
        let mut initial_points = Vec::with_capacity(sample_count);

        for i in 0..sample_count {
            let x = x_min + (i as f64) * step;
            let y: f64 = sim.sources.iter().map(|s| s.wave(0.0, x)).sum();
            initial_points.push([x, y]);
        }
        let now =Instant::now();
        Self {
            sim,
            start_time: now,
            last_frame_instant: now,
            x_min,
            x_max,
            max_amplitude,
            sample_count,
        }
    }
}

impl eframe::App for WaveApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let dt_frame = now.duration_since(self.last_frame_instant).as_secs_f64();
        self.last_frame_instant = now;

        let total_elapsed = self.start_time.elapsed().as_secs_f64();

        // 1. Continuous physical view evaluated at real frame time
        let raw_points = self.sim.generate_raw_points(self.x_min, self.x_max, self.sample_count, total_elapsed);
        self.sim.raw_peak_tracker.process_frame(&raw_points);

        // 2. Advance fixed-time sampling engine using dt_frame
        self.sim.update(dt_frame, self.x_min, self.x_max, self.sample_count);

        // UI sliders can now directly adjust physical time in seconds
        ui.horizontal(|ui| {
            ui.label("Sample Interval:");
            ui.add(
                egui::DragValue::new(&mut self.sim.sample_interval)
                    .speed(0.005)
                    .range(0.001..=2.0)
                    .suffix(" s")
            );
        });

        let y_margin = self.max_amplitude * 1.1;

        // --- CANVAS 1: RAW SIGNAL (PASSTHROUGH) ---
        ui.label(format!(
            "Raw Signal (Continuous Passthrough) | Current Peak: {:.2} at x = {:.2} | Max Peak: {:.2} at x = {:.2}",
            self.sim.raw_peak_tracker.current_peak.value, self.sim.raw_peak_tracker.current_peak.x,
            self.sim.raw_peak_tracker.max_peak.value, self.sim.raw_peak_tracker.max_peak.x
        ));
        
        Plot::new("raw_signal_canvas")
            .height(180.0)
            .include_x(self.x_min)
            .include_x(self.x_max)
            .include_y(y_margin)
            .include_y(-y_margin)
            .show(ui, |plot_ui| {
                let raw_line = Line::new("Raw Signal", PlotPoints::from(raw_points))
                    .color(Color32::LIGHT_RED);
                plot_ui.line(raw_line);

                let raw_current_marker = Points::new(
                    "Raw Current Peak", 
                    vec![[self.sim.raw_peak_tracker.current_peak.x, self.sim.raw_peak_tracker.current_peak.value]]
                )
                .color(Color32::LIGHT_RED)
                .radius(6.0);

                let raw_max_marker = Points::new(
                    "Raw Max Peak Overall", 
                    vec![[self.sim.raw_peak_tracker.max_peak.x, self.sim.raw_peak_tracker.max_peak.value]]
                )
                .color(Color32::RED)
                .radius(6.0);

                plot_ui.points(raw_current_marker);
                plot_ui.points(raw_max_marker);
            });

        ui.add_space(8.0);

        // --- CANVAS 2: SAMPLED SIGNAL (PROCESSED) ---
        ui.label(format!(
            "Sampled Signal (Processor Input) | Current Peak: {:.2} at x = {:.2} | Max Peak: {:.2} at x = {:.2}",
            self.sim.sampled_peak_tracker.current_peak.value, self.sim.sampled_peak_tracker.current_peak.x,
            self.sim.sampled_peak_tracker.max_peak.value, self.sim.sampled_peak_tracker.max_peak.x
        ));

        Plot::new("sampled_signal_canvas")
            .height(180.0)
            .include_x(self.x_min)
            .include_x(self.x_max)
            .include_y(y_margin)
            .include_y(-y_margin)
            .show(ui, |plot_ui| {
                let sampled_line = Line::new("Sampled Signal", PlotPoints::from(self.sim.sampled_points.clone()))
                    .color(Color32::LIGHT_BLUE);
                plot_ui.line(sampled_line);

                let sampled_current_marker = Points::new(
                    "Sampled Current Peak", 
                    vec![[self.sim.sampled_peak_tracker.current_peak.x, self.sim.sampled_peak_tracker.current_peak.value]]
                )
                .color(Color32::LIGHT_BLUE)
                .radius(6.0);

                let sampled_max_marker = Points::new(
                    "Sampled Max Peak Overall", 
                    vec![[self.sim.sampled_peak_tracker.max_peak.x, self.sim.sampled_peak_tracker.max_peak.value]]
                )
                .color(Color32::BLUE)
                .radius(6.0);

                plot_ui.points(sampled_current_marker);
                plot_ui.points(sampled_max_marker);
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
            .with_inner_size([900.0, 650.0])
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