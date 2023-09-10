use crate::{scheduler::Scheduler, types::Context};
use reqwest::{Client, Error};
use std::collections::BTreeMap;

mod scheduler;
mod tasks;
mod types;
mod urls;

/// User Agent for the request. Wiki Required
// TODO: Move to environmental variable
const USER_AGENT: &str = "price_getter - #Discord@rootgroot";

/// Save file for the item mappings. There are only 3993 items
/// so going for a full DB is over-engineering for now.
const SAVE_FILE: &str = "saved.json";

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Build the context item for the scheduler via a startup sequence
    let context = run_startup_sequence().await;

    // Build the scheduler
    let mut scheduler = Scheduler::new(context);

    // Run the scheduler
    scheduler.run().await;
    Ok(())
}

/// Run the arbitrage program startup sequence. Do one time
/// setup of all the resources that the program will use.
/// Panicking here is fine.
async fn run_startup_sequence() -> Context {
    println!("Running Startup Sequence 🚀...");

    let client = Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("Could not build reqwest client. Aborting startup sequence...");

    let mut context = Context {
        items: BTreeMap::new(),
        client,
    };

    context
        .build_items_db()
        .await
        .expect("Could not build the items db. Aborting startup sequence...");

    context
}
