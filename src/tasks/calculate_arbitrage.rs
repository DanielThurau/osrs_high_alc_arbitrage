use crate::{
    tasks::SchedulerTask,
    types::{Context, Merch},
};
use async_trait::async_trait;
use std::{collections::BinaryHeap, time::SystemTime};

const TWELVE_HOUR_SECONDS: u64 = 60 * 60 * 12;
const TEN_MINUTES_SECONDS: u64 = 60 * 10;

#[derive(Debug)]
pub struct CalculateArbitrageTask {
    pub(crate) timer: u64,
}

#[async_trait]
impl SchedulerTask for CalculateArbitrageTask {
    async fn run(&mut self, context: &mut Context) -> Result<(), String> {
        println!("Running CalculateArbitrageTask");

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut arbitrage_opportunities: BinaryHeap<Merch> = BinaryHeap::new();
        for (_, merch) in context.items.iter() {
            arbitrage_opportunities.push(merch.clone());
        }

        let mut prioritized_arbitrage_opportunities = Vec::new();
        while let Some(merch) = arbitrage_opportunities.pop() {
            match (merch.high_time, merch.low_time) {
                (Some(high_time), Some(_low_time)) => {
                    if now - high_time < TEN_MINUTES_SECONDS {
                        if merch.high != Some(1) {
                            prioritized_arbitrage_opportunities.push(merch);
                        }
                    }
                }
                _ => {}
            }
        }

        // TODO don't go out of bounds
        for i in 1..6 {
            if let Some(merch) = prioritized_arbitrage_opportunities.get(i) {
                println!(
                    "Interesting Arbitrage opportunity {i}.\n\
                - Item Name: {}\n   \
                - High Alch Price: {:?}\n   \
                - High Price: {:?}\n   \
                - Low Price: {:?}\n   \
                - Arbitrage(High): {}",
                    merch.name,
                    merch.highalch,
                    merch.high,
                    merch.low,
                    merch.highalch.unwrap() - merch.high.unwrap()
                );
            } else {
                println!("No more interesting arbitrage opportunities");
                return Ok(());
            }
        }
        println!(
            "Successfully ran UpdateLatestTask. Processed {} items",
            prioritized_arbitrage_opportunities.len()
        );
        Ok(())
    }

    fn timer(&self) -> u64 {
        self.timer
    }
}
