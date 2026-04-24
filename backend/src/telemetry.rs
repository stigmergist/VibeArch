use std::{sync::{atomic::{AtomicI64, AtomicU64, Ordering}, Arc, OnceLock}, time::Instant};

use serde::Serialize;

static SHARED_TELEMETRY: OnceLock<Arc<ServiceTelemetry>> = OnceLock::new();
static TRACING_INITIALIZED: OnceLock<()> = OnceLock::new();

pub fn shared_telemetry() -> Arc<ServiceTelemetry> {
    SHARED_TELEMETRY
        .get_or_init(|| Arc::new(ServiceTelemetry::default()))
        .clone()
}

pub fn init_tracing() {
    let _ = TRACING_INITIALIZED.get_or_init(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .json()
            .with_current_span(false)
            .with_span_list(false)
            .init();
    });
}

pub struct ServiceTelemetry {
    started_at: Instant,
    auth_requests_total: AtomicU64,
    auth_failures_total: AtomicU64,
    websocket_connect_attempts_total: AtomicU64,
    websocket_connect_failures_total: AtomicU64,
    active_websocket_connections: AtomicI64,
    websocket_messages_total: AtomicU64,
    websocket_rejected_messages_total: AtomicU64,
    websocket_runtime_errors_total: AtomicU64,
    broadcast_attempts_total: AtomicU64,
    broadcast_targets_total: AtomicU64,
    broadcast_deliveries_total: AtomicU64,
    broadcast_failures_total: AtomicU64,
}

impl Default for ServiceTelemetry {
    fn default() -> Self {
        Self {
            started_at: Instant::now(),
            auth_requests_total: AtomicU64::new(0),
            auth_failures_total: AtomicU64::new(0),
            websocket_connect_attempts_total: AtomicU64::new(0),
            websocket_connect_failures_total: AtomicU64::new(0),
            active_websocket_connections: AtomicI64::new(0),
            websocket_messages_total: AtomicU64::new(0),
            websocket_rejected_messages_total: AtomicU64::new(0),
            websocket_runtime_errors_total: AtomicU64::new(0),
            broadcast_attempts_total: AtomicU64::new(0),
            broadcast_targets_total: AtomicU64::new(0),
            broadcast_deliveries_total: AtomicU64::new(0),
            broadcast_failures_total: AtomicU64::new(0),
        }
    }
}

impl ServiceTelemetry {
    pub fn record_auth(&self, success: bool) {
        self.auth_requests_total.fetch_add(1, Ordering::Relaxed);
        if !success {
            self.auth_failures_total.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_websocket_connect(&self, success: bool) {
        self.websocket_connect_attempts_total
            .fetch_add(1, Ordering::Relaxed);
        if success {
            self.active_websocket_connections
                .fetch_add(1, Ordering::Relaxed);
        } else {
            self.websocket_connect_failures_total
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_websocket_disconnect(&self) {
        let _ = self.active_websocket_connections.fetch_update(
            Ordering::Relaxed,
            Ordering::Relaxed,
            |current| Some(current.saturating_sub(1)),
        );
    }

    pub fn record_message_accepted(&self) {
        self.websocket_messages_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_message_rejected(&self) {
        self.websocket_rejected_messages_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_runtime_error(&self) {
        self.websocket_runtime_errors_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_broadcast(&self, targets: usize, delivered: usize, failed: usize) {
        self.broadcast_attempts_total.fetch_add(1, Ordering::Relaxed);
        self.broadcast_targets_total
            .fetch_add(targets as u64, Ordering::Relaxed);
        self.broadcast_deliveries_total
            .fetch_add(delivered as u64, Ordering::Relaxed);
        self.broadcast_failures_total
            .fetch_add(failed as u64, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> TelemetrySnapshot {
        let auth_requests_total = self.auth_requests_total.load(Ordering::Relaxed);
        let auth_failures_total = self.auth_failures_total.load(Ordering::Relaxed);
        let websocket_connect_attempts_total = self
            .websocket_connect_attempts_total
            .load(Ordering::Relaxed);
        let websocket_connect_failures_total = self
            .websocket_connect_failures_total
            .load(Ordering::Relaxed);
        let active_websocket_connections = self
            .active_websocket_connections
            .load(Ordering::Relaxed)
            .max(0) as u64;
        let websocket_messages_total = self.websocket_messages_total.load(Ordering::Relaxed);
        let websocket_rejected_messages_total = self
            .websocket_rejected_messages_total
            .load(Ordering::Relaxed);
        let websocket_runtime_errors_total = self
            .websocket_runtime_errors_total
            .load(Ordering::Relaxed);
        let broadcast_attempts_total = self.broadcast_attempts_total.load(Ordering::Relaxed);
        let broadcast_targets_total = self.broadcast_targets_total.load(Ordering::Relaxed);
        let broadcast_deliveries_total = self.broadcast_deliveries_total.load(Ordering::Relaxed);
        let broadcast_failures_total = self.broadcast_failures_total.load(Ordering::Relaxed);

        TelemetrySnapshot {
            uptime_seconds: self.started_at.elapsed().as_secs(),
            counters: TelemetryCounters {
                auth_requests_total,
                auth_failures_total,
                websocket_connect_attempts_total,
                websocket_connect_failures_total,
                active_websocket_connections,
                websocket_messages_total,
                websocket_rejected_messages_total,
                websocket_runtime_errors_total,
                broadcast_attempts_total,
                broadcast_targets_total,
                broadcast_deliveries_total,
                broadcast_failures_total,
            },
            indicators: TelemetryIndicators {
                auth_success_rate: ratio(auth_requests_total.saturating_sub(auth_failures_total), auth_requests_total),
                websocket_connect_success_rate: ratio(
                    websocket_connect_attempts_total.saturating_sub(websocket_connect_failures_total),
                    websocket_connect_attempts_total,
                ),
                broadcast_delivery_success_rate: ratio(
                    broadcast_deliveries_total,
                    broadcast_deliveries_total.saturating_add(broadcast_failures_total),
                ),
            },
            slo_targets: SloTargets {
                availability_target: 0.999,
                websocket_connect_success_target: 0.99,
                broadcast_delivery_success_target: 0.99,
            },
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetrySnapshot {
    pub uptime_seconds: u64,
    pub counters: TelemetryCounters,
    pub indicators: TelemetryIndicators,
    pub slo_targets: SloTargets,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryCounters {
    pub auth_requests_total: u64,
    pub auth_failures_total: u64,
    pub websocket_connect_attempts_total: u64,
    pub websocket_connect_failures_total: u64,
    pub active_websocket_connections: u64,
    pub websocket_messages_total: u64,
    pub websocket_rejected_messages_total: u64,
    pub websocket_runtime_errors_total: u64,
    pub broadcast_attempts_total: u64,
    pub broadcast_targets_total: u64,
    pub broadcast_deliveries_total: u64,
    pub broadcast_failures_total: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryIndicators {
    pub auth_success_rate: f64,
    pub websocket_connect_success_rate: f64,
    pub broadcast_delivery_success_rate: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SloTargets {
    pub availability_target: f64,
    pub websocket_connect_success_target: f64,
    pub broadcast_delivery_success_target: f64,
}

fn ratio(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        1.0
    } else {
        numerator as f64 / denominator as f64
    }
}