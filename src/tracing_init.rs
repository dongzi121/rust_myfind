use tracing;
use tracing_subscriber::FmtSubscriber;

pub fn tracing_init() {
    let subscr = FmtSubscriber::builder().with_max_level(tracing::Level::DEBUG).finish();
    tracing::subscriber::set_global_default(subscr).expect("tracing fail!");
}