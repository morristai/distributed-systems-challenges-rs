use async_trait::async_trait;
use maelstrom::protocol::Message;
use maelstrom::{done, Node, Result, Runtime};
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(Handler::default());
    Runtime::new().with_handler(handler).run().await
}

#[derive(Clone, Default)]
struct Handler {}

#[derive(Serialize)]
struct Response {
    #[serde(rename = "type", default, skip_serializing_if = "String::is_empty")]
    typ: String,
    id: String,
}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        if req.body.typ == "generate" {
            let uuid = Uuid::new_v4();
            let id = format!("{}_{}", req.dest, uuid);
            let echo = Response {
                typ: "generate_ok".to_string(),
                id,
            };
            return runtime.reply(req, echo).await;
        }
        done(runtime, req)
    }
}