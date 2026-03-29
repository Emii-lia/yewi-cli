use std::sync::Mutex;

pub static CWD_LOCK: Mutex<()> = Mutex::new(());
