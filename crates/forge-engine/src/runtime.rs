use tokio_stream::Stream;

use crate::error::Error;

#[async_trait::async_trait]
pub trait Runtime<Action, Command> {
    async fn run(&self, command: Command) -> Result<Box<dyn Stream<Item = Action> + Unpin>, Error>;
}
