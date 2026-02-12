use std::sync::LazyLock;

use smol::{channel::Sender, lock::Mutex};

pub mod app;
pub mod logging;
pub mod web_server;

pub enum Event {}

type Singleton<T> = LazyLock<Mutex<Option<T>>>;

pub static GLOBAL_SENDER: Singleton<Sender<Event>> = LazyLock::new(|| Mutex::default());
