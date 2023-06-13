use std::{
    ops::Sub,
    time::{Duration, Instant},
};

pub struct Timer<'a> {
    name: &'a str,
    start_time: Instant,
    ending_time: Duration,
    paused_at: Option<Instant>,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str, time: u64) -> Self {
        Timer {
            name,
            start_time: Instant::now(),
            ending_time: Duration::new(time * 60, 0),
            paused_at: None,
        }
    }

    pub fn to_string(&self) -> String {
        let started_at = self.start_time.elapsed();
        let remaining = self.ending_time.sub(started_at).as_secs();
        if self.is_paused() {
            format!("Timer: {}, PAUSED", self.name)
        } else {
            format!(
                "Timer: {} {}%, elapsed {} remaining {} ",
                self.name,
                (self.get_percent() * 100.).round(),
                format!(
                    "{:02}:{:02}",
                    (started_at.as_secs() / 60),
                    started_at.as_secs() % 60
                ),
                format!("{:02}:{:02}", (remaining / 60), remaining % 60),
            )
        }
    }

    pub fn is_done(&self) -> bool {
        self.start_time.elapsed().as_secs() >= self.ending_time.as_secs()
    }

    pub fn is_paused(&self) -> bool {
        self.paused_at.is_some()
    }

    pub fn get_percent(&self) -> f32 {
        let elapsed_percent =
            self.start_time.elapsed().as_secs_f32() / self.ending_time.as_secs_f32();
        return (elapsed_percent * 100.).round() / 100.;
    }

    pub fn pause_timer(&mut self) {
        if self.paused_at.is_none() {
            self.paused_at = Some(Instant::now());
        }
    }

    pub fn unpause_timer(&mut self) {
        if let Some(paused_at) = self.paused_at {
            self.ending_time = self.ending_time.checked_add(paused_at.elapsed()).unwrap();
            self.paused_at = None;
        }
    }
}
