# actix-jobs

[![Tests](https://github.com/TortitasT/actix-jobs/actions/workflows/tests.yaml/badge.svg)](https://github.com/TortitasT/actix-jobs/actions/workflows/tests.yaml)
![Crates.io](https://img.shields.io/crates/v/actix-jobs)
![Crates.io](https://img.shields.io/crates/d/actix-jobs)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/actix-jobs/latest/actix_jobs/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A simple job scheduler for Actix.

## Usage

Simple example. For more information please refer to [the Docs](https://docs.rs/actix-jobs/latest/actix_jobs/).

```rust
use actix_jobs::{Job, Scheduler, run_forever};

struct MyJob;

impl Job for MyJob {
    fn cron(&self) -> &str {
        "*/2 * * * * * *" // every two seconds
    }

    fn run(&mut self) {
        println!("Sending an email to all our clients...");
    }
}

#[actix_rt::main]
async fn main() {
    let mut scheduler = Scheduler::new();
    scheduler.add(Box::new(MyJob));

    run_forever(scheduler); // This will start the scheduler in a new thread.

    // The rest of your program...
}
```

## Install

```bash
cargo add actix-jobs
```
