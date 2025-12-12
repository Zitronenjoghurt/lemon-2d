#[derive(Default)]
pub struct GameDebug {
    pub update_samples: usize,
    pub total_update_time: f64,
    pub update_avg_ms: f64,
    pub frame_samples: usize,
    pub total_frame_time: f32,
    pub frame_avg_ms: f32,
    pub avg_fps: f32,
}

impl GameDebug {
    pub fn frame_time(&mut self, dt: f32) {
        self.frame_samples += 1;
        self.total_frame_time += dt;

        if self.frame_samples >= 100 {
            self.frame_avg_ms = (self.total_frame_time * 1000.0) / (self.frame_samples as f32);
            self.avg_fps = 1000.0 / self.frame_avg_ms;
            tracing::trace!(
                "FPS: {:.2} | avg.: {:.2} ms",
                self.avg_fps,
                self.frame_avg_ms
            );
            self.frame_samples = 0;
            self.total_frame_time = 0.0;
        }
    }

    pub fn update_time(&mut self, update_time: f64) {
        self.update_samples += 1;
        self.total_update_time += update_time;

        if self.update_samples >= 100 {
            self.update_avg_ms = (self.total_update_time * 1000.0) / (self.update_samples as f64);
            tracing::trace!("Update avg.: {:.2} ms", self.update_avg_ms);
            self.update_samples = 0;
            self.total_update_time = 0.0;
        }
    }
}
