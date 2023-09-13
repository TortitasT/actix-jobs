use std::{cell::Cell, rc::Rc};

use super::*;

struct TestJob {
    has_executed: Rc<Cell<bool>>,
}

impl Job for self::TestJob {
    fn cron(&self) -> &'static str {
        "* * * * * *"
    }

    fn run(&mut self) {
        self.has_executed.set(true);
    }
}

#[test]
fn scheduler_runs_job() {
    let mut scheduler = Scheduler::new();

    let has_executed = Rc::new(Cell::new(false));

    scheduler.add(Box::new(TestJob {
        has_executed: has_executed.clone(),
    }));

    scheduler.run();

    assert!(has_executed.get());
}
