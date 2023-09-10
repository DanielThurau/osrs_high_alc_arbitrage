use crate::{
    tasks::{
        calculate_arbitrage::CalculateArbitrageTask, flush_data::FlushDataTask,
        update_latest::UpdateLatestTask,
    },
    types::Context,
};
use async_trait::async_trait;
use std::cmp::Ordering;

pub mod calculate_arbitrage;
pub mod flush_data;
pub mod update_latest;

#[async_trait]
pub trait SchedulerTask {
    // TODO return a enum error that is recoverable or non-recoverable
    async fn run(&mut self, context: &mut Context) -> Result<(), String>;
    fn timer(&self) -> u64;
}

#[derive(Debug)]
pub enum Task {
    UpdateLatest(UpdateLatestTask),
    CalculateArbitrage(CalculateArbitrageTask),
    FlushData(FlushDataTask),
}

#[async_trait]
impl SchedulerTask for Task {
    async fn run(&mut self, context: &mut Context) -> Result<(), String> {
        match self {
            Task::UpdateLatest(x) => x.run(context).await,
            Task::CalculateArbitrage(x) => x.run(context).await,
            Task::FlushData(x) => x.run(context).await,
        }
    }

    fn timer(&self) -> u64 {
        match self {
            Task::UpdateLatest(x) => x.timer(),
            Task::CalculateArbitrage(x) => x.timer(),
            Task::FlushData(x) => x.timer(),
        }
    }
}

impl Eq for Task {}

impl PartialEq<Self> for Task {
    fn eq(&self, other: &Self) -> bool {
        self.timer().eq(&other.timer())
    }
}

impl PartialOrd<Self> for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timer().cmp(&other.timer())
    }
}
