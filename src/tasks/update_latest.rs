use crate::{
    tasks::SchedulerTask,
    types::{Context, LatestResponse},
    urls::Route,
};
use async_trait::async_trait;

#[derive(Debug)]
pub struct UpdateLatestTask {
    pub(crate) timer: u64,
}

#[async_trait]
impl SchedulerTask for UpdateLatestTask {
    async fn run(&mut self, context: &mut Context) -> Result<(), String> {
        println!("Running UpdateLatestTask");
        let latest = context
            .get(Route::Latest)
            .await
            .map_err(|err| err.to_string())?;
        let latest_response = LatestResponse::try_from(latest)?;

        for (id, latest_get_data) in latest_response.data.iter() {
            if context.items.contains_key(id) {
                if let Some(merch) = context.items.get_mut(id) {
                    merch.high = latest_get_data.high;
                    merch.high_time = latest_get_data.highTime;
                    merch.low = latest_get_data.low;
                    merch.low_time = latest_get_data.lowTime;
                }
            } else {
                println!("ERROR: Somehow, key {} does not exist in db...", id);
            }
        }

        println!(
            "Successfully ran UpdateLatestTask. Processed {} items",
            latest_response.data.len()
        );
        Ok(())
    }

    fn timer(&self) -> u64 {
        self.timer
    }
}
