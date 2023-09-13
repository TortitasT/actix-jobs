use std::{str::FromStr, time::Duration};

use chrono::Local;
use cron::Schedule;

#[cfg(test)]
mod tests;

pub trait Job {
    fn run(&mut self);

    // sec min hour day_of_month month day_of_week year
    // example: "0 0 0 1 1 * *" (every year on January 1st at midnight)
    fn cron(&self) -> &str;
}

pub struct Scheduler {
    jobs: Vec<Box<dyn Job>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn add(&mut self, job: Box<dyn Job>) {
        self.jobs.push(job);
    }

    pub fn run(&mut self) {
        for job in &mut self.jobs {
            let expression = job.cron();
            let schedule = Schedule::from_str(expression).unwrap();
            let now = Local::now();

            let mut upcoming = schedule.upcoming(Local).take(1);
            let upcoming = match upcoming.next() {
                Some(datetime) => datetime,
                None => continue,
            };

            if upcoming.timestamp() > (now.timestamp() + 1) {
                continue;
            }

            job.run();
        }
    }
}

pub fn run_forever(mut scheduler: Scheduler) {
    actix_rt::spawn(async move {
        loop {
            scheduler.run();

            actix_rt::time::sleep(Duration::from_millis(500)).await;
        }
    });
}
