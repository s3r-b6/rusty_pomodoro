use std::time::Instant;

pub struct Timer<'a> {
    name: &'a str,
    tick: Instant,
    seconds_til_end: u64,
    paused_at: Option<Instant>,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str, time: u64) -> Self {
        Timer {
            name,
            tick: Instant::now(),
            seconds_til_end: time * 60,
            paused_at: None,
        }
    }

    pub fn pause_timer(&mut self) {
        if self.paused_at.is_none() {
            self.paused_at = Some(Instant::now());
        }
    }

    pub fn unpause_timer(&mut self) {
        if let Some(paused_at) = self.paused_at {
            self.tick = self.tick.checked_add(paused_at.elapsed()).unwrap();
            self.paused_at = None;
        }
    }

    pub fn to_string(&self) -> String {
        if self.is_paused() {
            format!(
                "[ TIMER {} ][ PAUSED FOR {}s ]",
                self.name,
                self.paused_at.unwrap().elapsed().as_secs()
            )
        } else {
            format!(
                "[ TIMER {} {:.2}% ][ ELAPSED {} ][ ENDS IN {} ]",
                self.name,
                self.get_percent() * 100.,
                self.get_passed_time(),
                self.get_remaining_time()
            )
        }
    }

    pub fn is_done(&self) -> bool {
        !self.is_paused() && self.tick.elapsed().as_secs() >= self.seconds_til_end
    }

    pub fn is_paused(&self) -> bool {
        self.paused_at.is_some()
    }

    pub fn get_percent(&self) -> f64 {
        let elapsed_percent = self.tick.elapsed().as_secs() as f64 / self.seconds_til_end as f64;
        return (elapsed_percent * 100.) / 100.;
    }

    pub fn get_passed_time(&self) -> String {
        format!(
            "{:02}:{:02}",
            (self.tick.elapsed().as_secs() / 60),
            self.tick.elapsed().as_secs() % 60
        )
    }

    pub fn get_remaining_time(&self) -> String {
        let remaining_time = self.seconds_til_end - self.tick.elapsed().as_secs();
        format!("{:02}:{:02}", remaining_time / 60, remaining_time % 60,)
    }
}
