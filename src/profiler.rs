use std::time::Instant;

pub struct Profiler {
    start_time: Instant,
}

impl Profiler {
    pub fn start() -> Profiler {
        tracing::trace!("Starting Profiler");
        Profiler {
            start_time: Instant::now(),
        }
    }

    pub fn stop(&self) {
        let elapsed = self.start_time.elapsed();
        tracing::trace!("Elapsed time: {:#?}\n\n", elapsed);
    }
}