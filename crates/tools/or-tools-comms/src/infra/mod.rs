#[cfg(any(
    feature = "twilio",
    feature = "telegram",
    feature = "discord",
    feature = "whatsapp",
    feature = "facebook",
    feature = "messenger"
))]
pub(crate) mod shared;

#[cfg(feature = "discord")]
pub mod discord;
#[cfg(feature = "facebook")]
pub mod facebook;
#[cfg(feature = "messenger")]
pub mod messenger;
#[cfg(feature = "telegram")]
pub mod telegram;
#[cfg(feature = "twilio")]
pub mod twilio;
#[cfg(feature = "whatsapp")]
pub mod whatsapp;
