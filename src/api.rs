use governor::middleware::NoOpMiddleware;
use std::sync::Arc;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::PeerIpKeyExtractor;
use tower_governor::GovernorLayer;

pub mod docs;
mod extractors;
pub mod routes;

pub fn create_rate_limiter(
    burst_size: u32,
    per_seconds: u64,
) -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware> {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .burst_size(burst_size)
            .per_second(per_seconds)
            .finish()
            .unwrap(),
    );

    GovernorLayer {
        config: governor_conf,
    }
}
