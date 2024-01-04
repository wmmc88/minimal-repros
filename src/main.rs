use tracing::{event, level_filters::LevelFilter, warn, Level};
use tracing_subscriber::filter::EnvFilter;

fn broken() {
    let tracing_filter = EnvFilter::default()
        .add_directive(LevelFilter::TRACE.into())
        .add_directive("[{message=hello}]=off".parse().unwrap())
        .add_directive("[{number=1337}]=off".parse().unwrap());

    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_filter)
        .init();

    warn!("hello");
    event!(Level::INFO, number = 1337);
}

fn working() {
    let tracing_filter = EnvFilter::default()
        .add_directive(LevelFilter::TRACE.into())
        .add_directive("[{message}]=off".parse().unwrap())
        .add_directive("[{number}]=off".parse().unwrap());

    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_filter)
        .init();

    warn!("hello");
    event!(Level::INFO, number = 1337);
}

fn main() {}

mod tests {
    use super::*;

    #[test]
    fn test_broken() {
        broken();
    }

    #[test]
    fn test_working() {
        working();
    }
}
