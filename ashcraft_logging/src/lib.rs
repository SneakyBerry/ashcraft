use tracing::Level;

// Init logging
pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_thread_names(true)
        .with_target(true)
        .init();
    // tracing_subscriber::fmt()
    //     .json()
    //     .with_max_level(Level::TRACE)
    //     .with_current_span(true)
    //     .with_thread_names(true)
    //     .with_line_number(true)
    //     .with_target(true)
    //     .pretty()
    //     .init();
}
