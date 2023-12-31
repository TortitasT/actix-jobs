use std::{cell::Cell, rc::Rc};

use super::*;

struct TestJob {
    execution_count_ref: Rc<Cell<i32>>,
}

impl Job for self::TestJob {
    fn cron(&self) -> &str {
        "*/2 * * * * * *" // every 2 seconds
    }

    fn run(&mut self) {
        self.execution_count_ref
            .set(self.execution_count_ref.get() + 1);
    }
}

#[test]
fn scheduler_runs_job_at_time() {
    let mut scheduler = Scheduler::new();

    let execution_count = Rc::new(Cell::new(0));
    let execution_count_ref = execution_count.clone();

    scheduler.add(Box::new(TestJob {
        execution_count_ref,
    }));

    for _ in 0..4 {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        scheduler.run();
    }

    assert_eq!(execution_count.get(), 2);
}

struct TestFileJob;

impl Job for self::TestFileJob {
    fn cron(&self) -> &str {
        "*/2 * * * * * *" // every 2 seconds
    }

    fn run(&mut self) {
        // TODO: something to check if the job is running
    }
}

#[actix_rt::test]
#[ignore = "not ready yet"]
async fn scheduler_runs_job_forever() {
    let mut scheduler = Scheduler::new();

    scheduler.add(Box::new(TestFileJob));

    run_forever(scheduler);

    // std::thread::sleep(std::time::Duration::from_millis(6000));

    // TODO: something to check if the job is running
}
