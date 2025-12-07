use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Job queue statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueueStats {
    pub total_enqueued: u64,
    pub total_processed: u64,
    pub total_failed: u64,
    pub total_retried: u64,
    pub pending_count: u64,
    pub running_count: u64,
    pub dead_letter_count: u64,
}

/// Thread-safe statistics tracker
#[derive(Clone)]
pub struct StatsTracker {
    stats: Arc<RwLock<QueueStats>>,
}

impl StatsTracker {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(RwLock::new(QueueStats::default())),
        }
    }

    pub async fn increment_enqueued(&self) {
        let mut stats = self.stats.write().await;
        stats.total_enqueued += 1;
        stats.pending_count += 1;
    }

    pub async fn increment_processed(&self) {
        let mut stats = self.stats.write().await;
        stats.total_processed += 1;
        if stats.running_count > 0 {
            stats.running_count -= 1;
        }
    }

    pub async fn increment_failed(&self) {
        let mut stats = self.stats.write().await;
        stats.total_failed += 1;
        if stats.running_count > 0 {
            stats.running_count -= 1;
        }
    }

    pub async fn increment_retried(&self) {
        let mut stats = self.stats.write().await;
        stats.total_retried += 1;
    }

    pub async fn increment_dead_letter(&self) {
        let mut stats = self.stats.write().await;
        stats.dead_letter_count += 1;
    }

    pub async fn mark_running(&self) {
        let mut stats = self.stats.write().await;
        if stats.pending_count > 0 {
            stats.pending_count -= 1;
        }
        stats.running_count += 1;
    }

    pub async fn get_stats(&self) -> QueueStats {
        self.stats.read().await.clone()
    }

    pub async fn reset(&self) {
        let mut stats = self.stats.write().await;
        *stats = QueueStats::default();
    }
}

impl Default for StatsTracker {
    fn default() -> Self {
        Self::new()
    }
}
