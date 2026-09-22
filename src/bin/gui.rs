use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Points};
use wave_sim::core::wave::{WaveSource, WaveShape, PropagationMode};
use wave_sim::core::simulation::Simulation;
use std::time::Instant;

pub struct WaveApp {
    sim: Simulation,
    start_time: Instant,
    x_min: f64,
    x_max: f64,
    max_amplitude: f64, // Track fixed scale bound
    sample_count: usize,
}

impl WaveApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut sim = Simulation::new();
        // sim.add_source(WaveSource::new(0.0, 2.5, 8.0, 0.0, WaveShape::Sine, PropagationMode::Traveling));
        sim.add_source(WaveSource::new(5.0, 0.5, 30.0, 0.0, WaveShape::Sinc, PropagationMode::Standing));
        sim.add_source(WaveSource::new(0.0, 2.5, 8.0, 0.0, WaveShape::Sine, PropagationMode::Traveling));
        sim.add_source(WaveSource::new(-4.3, 10.0, 3.5, 0.0, WaveShape::Sine, PropagationMode::Traveling));

        // Theoretical maximum bound is the sum of all source amplitudes
        let max_amplitude: f64 = sim.sources.iter().map(|s| s.amplitude).sum();

        Self {
            sim,
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

        let mut peak_val = f64::NEG_INFINITY;
        let mut peak_x = 0.0;

        for i in 0..self.sample_count {
            let x = self.x_min + (i as f64) * step;
            let y: f64 = self.sim.sources.iter().map(|s| s.wave(elapsed, x)).sum();

            if y > peak_val {
                peak_val = y;
                peak_x = x;
            }

            points.push([x, y]);
        }

        ui.heading("1D Wave Signal Simulator");
        ui.label(format!(
            "Elapsed Time: {:.2}s | Peak: {:.2} at x = {:.2}",
            elapsed, peak_val, peak_x
        ));

        // Adding margin to bounds so peaks don't touch the very top/bottom edges
        let y_margin = self.max_amplitude * 1.1;

        Plot::new("wave_canvas")
            .view_aspect(2.0)
            // Fix X-axis limits
            .include_x(self.x_min)
            .include_x(self.x_max)
            // Fix Y-axis limits so grid never zooms/shrinks during oscillation
            .include_y(y_margin)
            .include_y(-y_margin)
            .show(ui, |plot_ui| {
                let line = Line::new("Combined Wave", PlotPoints::from(points));
                plot_ui.line(line);

                let peak_marker = Points::new("Peak Marker", vec![[peak_x, peak_val]])
                    .radius(6.0);
                plot_ui.points(peak_marker);
            });

        ui.ctx().request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
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