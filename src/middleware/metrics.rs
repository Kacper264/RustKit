use prometheus::{Encoder, IntCounter, Registry};

pub fn create_metrics() -> (IntCounter, Registry) {
    let counter = IntCounter::new(
        "user_api_request_count",
        "Number of requests received"
    ).unwrap();

    let registry = Registry::new();
    registry.register(Box::new(counter.clone())).unwrap();

    (counter, registry)
}
