#[cfg(test)]
mod tests;

pub trait Job {
    fn run(&mut self);

    fn cron(&self) -> &'static str;
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
            job.run();
        }
    }
}
