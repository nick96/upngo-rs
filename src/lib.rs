pub mod accounts;
pub mod client;
pub mod init;
pub mod ping;
pub mod transactions;
pub mod webhooks;

mod rfc4217;

pub type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;

pub const SERVICE_NAME: &str = "upngo";
pub const USERNAME: &str = "upngo-token";
