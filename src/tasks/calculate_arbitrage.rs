use crate::{
    tasks::SchedulerTask,
    types::{Context, Merch},
};
use async_trait::async_trait;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct CalculateArbitrageTask {
    pub(crate) timer: u64,
}

#[async_trait]
impl SchedulerTask for CalculateArbitrageTask {
    async fn run(&mut self, context: &mut Context) -> Result<(), String> {
        println!("Running CalculateArbitrageTask");
        let mut arbitrage_opportunities: BinaryHeap<Merch> = BinaryHeap::new();
        for (_, merch) in context.items.iter() {
            arbitrage_opportunities.push(merch.clone());
        }

        for i in 1..11 {
            if let Some(merch) = arbitrage_opportunities.pop() {
                println!(
                    "Interesting Arbitrage opportunity {i}.\n\
                - Item Name: {}\n\
                - High Alch Price: {:?}\n\
                - High Price: {:?}\n\
                - Low Price: {:?}\n\
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
            arbitrage_opportunities.len()
        );
        Ok(())
    }

    fn timer(&self) -> u64 {
        self.timer
    }
}
