use chrono::{DateTime, Utc};

///
#[derive(Debug)]
pub struct ClientInformation {
    pub client_description: String,
    pub host_id: String,
    pub last_access: DateTime<Utc>,
    pub user_id: String,
}
