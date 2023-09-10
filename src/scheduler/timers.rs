use crate::tasks::{SchedulerTask, Task};
use std::{cmp::Reverse, collections::BinaryHeap};

/// Timers structure. Its a min_heap sorted on the unix timestamp of the task timer.
/// Tasks with timer() == 0 are executed immediately
#[derive(Debug)]
pub(crate) struct Timers {
    min_heap: BinaryHeap<Reverse<Task>>,
}

impl Timers {
    pub(crate) fn new() -> Self {
        Self {
            min_heap: BinaryHeap::new(),
        }
    }

    pub(crate) fn add_task(&mut self, task: Task) {
        self.min_heap.push(Reverse(task))
    }

    pub(crate) fn next_timer(&self) -> Option<u64> {
        if let Some(task) = self.min_heap.peek() {
            return Some(task.0.timer());
        }
        None
    }

    pub(crate) fn pop(&mut self) -> Option<Task> {
        if let Some(task) = self.min_heap.pop() {
            Some(task.0)
        } else {
            None
        }
    }
}
