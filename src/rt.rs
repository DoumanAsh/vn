#[cfg(not(debug_assertions))]
fn logger() {
}

#[cfg(debug_assertions)]
fn logger() {
    cute_log::init().expect("To initialize log");
}

#[cfg(not(debug_assertions))]
fn panic() {
    lazy_panic::set_panic_message!(lazy_panic::formatter::Simple);
}

#[cfg(debug_assertions)]
fn panic() {
    lazy_panic::set_panic_message!(lazy_panic::formatter::Debug);
}

pub fn init() {
    panic();
    logger();
}
