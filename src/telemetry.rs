use tracing::Subscriber;
use tracing::dispatcher::{self, Dispatch};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    tracing_log::LogTracer::init().expect("Failed to set logger");

    dispatcher::set_global_default(Dispatch::new(subscriber))
        .expect("Failed to set tracing subscriber");
}
