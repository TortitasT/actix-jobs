# actix-jobs

A simple job scheduler for Actix.

## Usage

Simple example. For mor information please refer to [the Docs](https://docs.rs/actix-jobs/latest/actix_jobs/).

```rust
use actix_jobs::{Job, Scheduler, run_forever};

struct MyJob;

impl Job for MyJob {
    fn cron(&self) -> &str {
        "*/2 * * * * * *" // every two seconds
    }

    fn run(&mut self) {
        println!("Hello, world!");
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
