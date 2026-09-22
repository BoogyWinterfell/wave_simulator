// use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::f64::consts::PI;

// 1. Define the mathematical wave shapes
#[derive(Clone, Copy)]
pub enum WaveShape {
    Sine,
    Square,
    Sinc,
}

impl WaveShape {
    // This handles pure shape mapping independently of time/space logic
    fn evaluate(&self, target: f64) -> f64 {
        match self {
            WaveShape::Sine => target.sin(),
            WaveShape::Square => if target.sin() >= 0.0 { 1.0 } else { -1.0 },
            WaveShape::Sinc => {
                // Prevent divide-by-zero for Sinc waves
                if target.abs() < 1e-6 { 1.0 } else { target.sin() / target }
            }
        }
    }
}

// 2. Define how the wave moves
#[derive(Clone, Copy)]
pub enum PropagationMode {
    Traveling,
    Standing,
}

// 3. Core struct with no Box/dyn overhead
pub struct WaveSource {
    pub location: f64,
    pub frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub shape: WaveShape,
    pub mode: PropagationMode,
}

impl WaveSource {
    pub fn new(location: f64, frequency: f64, amplitude: f64, phase: f64, shape: WaveShape, mode: PropagationMode) -> Self {
        Self { location, frequency, amplitude, phase, shape, mode }
    }

    pub fn wave(&self, t: f64, x: f64) -> f64 {
        let angular_freq = 2.0 * PI * self.frequency;

        let normalized_value = match self.mode {
            PropagationMode::Traveling => {
                // f(x + wt + phase)
                let target = angular_freq * t + self.phase + x;
                self.shape.evaluate(target)
            },
            PropagationMode::Standing => {
                // f(x + phase) * cos(wt) -> time and space are calculated separately
                let space_target = self.location + self.phase;
                let time_target = angular_freq * t;
                
                self.shape.evaluate(space_target) * time_target.cos()
            }
        };

        normalized_value * self.amplitude
    }
}

fn main() {
    // println!("Starting 1D simulation, enter a location on the x axis (Default is 0.0):");
    // io::stdout().flush().unwrap();

    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");

    // let x = input.trim().parse::<f64>().unwrap_or(0.0);

    let sources: Vec<WaveSource> = vec![
        WaveSource::new(0.0, 2.5, 8.0, 0.0, WaveShape::Sine, PropagationMode::Traveling),
        // WaveSource::new(5.0, 4.0, 0.0, WaveShape::Square, PropagationMode::Traveling),
        WaveSource::new(5.0, 0.5, 30.0, 0.0, WaveShape::Sinc, PropagationMode::Standing),
    ];

    let start_time = Instant::now();
    println!("Press Ctrl+C to stop.\n");

    loop {
        let elapsed = start_time.elapsed().as_secs_f64();

        let combined_amplitude: f64 = sources.iter()
            .map(|source| source.wave(elapsed, source.location))
            .sum();
      
        let max_amplitude: f64 = sources.iter().map(|s| s.amplitude).sum();
        let normalized_width = (combined_amplitude + max_amplitude).round() as usize;
        
        println!("{:>width$}", "*", width = normalized_width + 1);

        sleep(Duration::from_millis(16));
    }
}