use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub order_id: String,
    pub business: String,
    pub state: String,
}
