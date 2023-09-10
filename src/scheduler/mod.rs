use crate::{
    scheduler::{
        timers::Timers,
        Task::{CalculateArbitrage, UpdateLatest},
    },
    tasks::{
        calculate_arbitrage::CalculateArbitrageTask, flush_data::FlushDataTask,
        update_latest::UpdateLatestTask, SchedulerTask, Task, Task::FlushData,
    },
    types::Context,
    SAVE_FILE,
};
use std::{
    collections::VecDeque,
    thread::sleep,
    time::{Duration, SystemTime},
};

mod timers;

/// The Scheduler. It runs at hertz.
pub struct Scheduler {
    context: Context,
    task_count: u128,
    timers: Timers,
    hertz: u64,
    tasks: VecDeque<Task>,
}

impl Scheduler {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            task_count: 0,
            timers: Timers::new(),
            hertz: 1,
            tasks: VecDeque::new(),
        }
    }

    /// Run the scheduler at hertz.
    pub async fn run(&mut self) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        // For now, always schedule an UpdateLatestTask right away. This will pull in
        // the latest data from the wiki and update the items.db
        self.tasks
            .push_front(UpdateLatest(UpdateLatestTask { timer: 0 }));

        // Schedule a arbitrage calculation for 10 seconds from now.
        self.timers
            .add_task(CalculateArbitrage(CalculateArbitrageTask {
                timer: now.as_secs() + 10,
            }));

        // Flush the most up to date data to disk for later use.
        self.timers.add_task(FlushData(FlushDataTask {
            timer: now.as_secs() + 30,
        }));

        // Scheduler loop
        loop {
            self.task_count = self.task_count.saturating_add(1);
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            println!("Running task {}", self.task_count);

            // If there are timers for a task that are ringing, add the task to the queue.
            if let Some(timer) = self.timers.next_timer() {
                if timer <= now.as_secs() {
                    let task = self.timers.pop().unwrap();
                    self.add_task(task);
                }
            }

            // Add some tasks to timers if a task is being completed this round
            if let Some(task) = self.tasks.front() {
                self.timers.add_task(match task {
                    // Run calculate arbitrage every 1 minute
                    CalculateArbitrage(_) => CalculateArbitrage(CalculateArbitrageTask {
                        timer: now.as_secs() + 60,
                    }),
                    // Run update latest every 1 minute
                    UpdateLatest(_) => UpdateLatest(UpdateLatestTask {
                        timer: now.as_secs() + 60,
                    }),
                    // Run flush data every 5 minutes
                    FlushData(_) => FlushData(FlushDataTask {
                        timer: now.as_secs() + 300,
                    }),
                });
            }

            // If there's a task to run, run it!
            if let Some(task) = &mut self.pop_task() {
                task.run(&mut self.context).await.unwrap();
            }

            sleep(Duration::from_secs(1 / self.hertz));
        }
    }

    /// Helper function for consistent access to the task queue
    fn add_task(&mut self, task: Task) {
        self.tasks.push_back(task)
    }

    /// Helper function for consistent access to the task queue
    fn pop_task(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        self.context
            .flush(SAVE_FILE)
            .expect("Could not flush to disk");
    }
}
