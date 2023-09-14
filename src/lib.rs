use std::{str::FromStr, time::Duration};

use chrono::Local;
use cron::Schedule;

#[cfg(test)]
mod tests;

/// A trait for jobs that can be scheduled.
/// The `cron` method should return a cron expression that defines when the job should run.
/// The `run` method will be called when the job is scheduled to run.
pub trait Job {
    /// | sec | min | hour | day_of_month | month | day_of_week | year |
    /// |-----|-----|------|--------------|-------|-------------|------|
    /// | */2 | * | * | * | * | * | * |
    ///
    /// This means that the job will run every 2 seconds.
    ///
    /// And it would be written as:
    ///
    /// ```rust
    /// fn cron(&self) -> &str {
    ///     "*/2 * * * * * *"
    /// }
    /// ```
    fn cron(&self) -> &str;

    /// This method will be called when the job is scheduled to run.
    /// It should contain the logic that the job should run.
    ///
    /// ```rust
    /// fn run(&mut self) {
    ///    println!("Hello, world!");
    /// }
    /// ```
    fn run(&mut self);
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

            actix_rt::time::sleep(Duration::from_millis(1000)).await;
        }
    });
}
