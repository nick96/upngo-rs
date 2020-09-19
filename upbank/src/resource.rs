use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ResourceType {
    Transactions,
    Accounts,
    Webhooks,
    Categories,
    Tags,
    WebhookEvents,
    WebhookDeliveryLogs,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Resource<A, R> {
    #[serde(rename = "type")]
    pub resource_type: ResourceType,
    pub id: String,
    #[serde(bound(deserialize = "A: Deserialize<'de>"))]
    pub attributes: A,
    #[serde(bound(deserialize = "R: Deserialize<'de>"))]
    pub relationships: R,
    pub links: Option<SelfLinks>,
}

#[derive(Deserialize, Debug)]
pub struct SelfLinks {
    #[serde(rename = "self")]
    pub self_: String,
}
