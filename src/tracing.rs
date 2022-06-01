use crate::env::ensure_env;
use opentelemetry::{
    sdk::{trace, Resource},
    trace::TraceError,
};
use opentelemetry_otlp::WithExportConfig;
use std::str::FromStr;
use tonic::{
    metadata::{MetadataKey, MetadataMap},
    transport::ClientTlsConfig,
};
use tracing_actix_web::root_span_macro::private::tracing;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};
use url::Url;

const ENDPOINT: &str = "OTLP_TONIC_ENDPOINT";
const API_KEY: &str = "OTLP_TONIC_X_HONEYCOMB_TEAM";

pub fn shutdown() {
    opentelemetry::global::shutdown_tracer_provider();
}

pub fn init(service_name: String) -> Result<(), TraceError> {
    let endpoint = ensure_env(ENDPOINT);
    let endpoint = Url::parse(&endpoint).expect("endpoint is not a valid url");

    let api_key = ensure_env(API_KEY);

    let mut metadata = MetadataMap::new();

    metadata.insert(
        MetadataKey::from_str("x-honeycomb-team").unwrap(),
        api_key.parse().unwrap(),
    );

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(trace::config().with_resource(Resource::new(vec![
            opentelemetry::KeyValue::new("service.name", service_name),
        ])))
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint.as_str())
                .with_metadata(metadata)
                .with_tls_config(
                    ClientTlsConfig::new().domain_name(
                        endpoint
                            .host_str()
                            .expect("the specified endpoint should have a valid host"),
                    ),
                ),
        )
        .install_batch(opentelemetry::runtime::TokioCurrentThread)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    LogTracer::init().expect("Failed to initialize log tracer.");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("app".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default tracing subscriber.");

    Ok(())
}
