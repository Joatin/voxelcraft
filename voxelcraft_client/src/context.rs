use crate::game::{Game, LocalGame};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Context {
    current_route: std::sync::RwLock<String>,
    show_debug: AtomicBool,
    current_fps: std::sync::Mutex<f64>,
    time_to_draw_frame: std::sync::Mutex<Duration>,
    game: Option<Arc<dyn Game>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            current_route: std::sync::RwLock::new("HOME".to_owned()),
            show_debug: AtomicBool::new(false),
            current_fps: std::sync::Mutex::new(0.0),
            time_to_draw_frame: std::sync::Mutex::new(Duration::default()),
            game: None,
        }
    }

    pub fn get_current_route(&self) -> String {
        self.current_route.read().unwrap().to_string()
    }

    pub fn get_game(&self) -> &Option<Arc<dyn Game>> {
        &self.game
    }

    // Measured
    pub fn get_current_fps(&self) -> f64 {
        *self.current_fps.lock().unwrap()
    }

    pub fn set_current_fps_from_duration(&self, duration: Duration) {
        let fps = 1.0 / duration.as_secs_f64();
        let mut lock = self.current_fps.lock().unwrap();
        *lock = fps;
    }

    pub fn set_time_to_draw_frame(&self, duration: Duration) {
        let mut lock = self.time_to_draw_frame.lock().unwrap();
        *lock = duration
    }

    pub fn get_time_to_draw_frame(&self) -> Duration {
        let lock = self.time_to_draw_frame.lock().unwrap();
        lock.clone()
    }

    pub fn toggle_debug(&self) {
        let current = self.show_debug.load(Ordering::Relaxed);
        self.show_debug.store(!current, Ordering::Relaxed);
    }

    pub fn show_debug(&self) -> bool {
        self.show_debug.load(Ordering::Relaxed)
    }
}
