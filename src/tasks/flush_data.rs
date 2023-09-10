use crate::{tasks::SchedulerTask, types::Context, SAVE_FILE};
use async_trait::async_trait;

#[derive(Debug)]
pub struct FlushDataTask {
    pub(crate) timer: u64,
}

#[async_trait]
impl SchedulerTask for FlushDataTask {
    async fn run(&mut self, context: &mut Context) -> Result<(), String> {
        println!("Running FlushDataTask");
        context.flush(SAVE_FILE).map_err(|err| err.to_string())?;
        println!("Successfully ran FlushDataTask");
        Ok(())
    }

    fn timer(&self) -> u64 {
        self.timer
    }
}
