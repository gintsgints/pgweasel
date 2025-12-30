use std::{collections::HashMap, time::Duration};

use crate::{
    aggregators::{Aggregator, round_floor},
    severity::Severity,
};

#[derive(Clone)]
pub struct PeaksAggregator {
    bucket_interval: Duration,
    buckets_by_severity: HashMap<Severity, HashMap<String, u16>>,
    total_by_severity: HashMap<String, u16>,
}

impl PeaksAggregator {
    pub fn new(interval: Duration) -> Self {
        PeaksAggregator {
            bucket_interval: interval,
            buckets_by_severity: HashMap::new(),
            total_by_severity: HashMap::new(),
        }
    }
}

impl Aggregator for PeaksAggregator {
    fn update(
        &mut self,
        _record: &[u8],
        _fmt: &crate::format::Format,
        severity: &crate::severity::Severity,
        log_time: chrono::DateTime<chrono::Local>,
    ) -> crate::Result<()> {
        // TODO: Clarify from GO code, what means pglog.go:608 Line.
        if severity != &Severity::Error {
            return Ok(());
        };

        let bucket_time = round_floor(log_time, self.bucket_interval);
        let bucket_time_str = bucket_time.to_string();
        self.buckets_by_severity
            .entry(*severity)
            .and_modify(|count_by_duration| {
                count_by_duration
                    .entry(bucket_time_str.clone())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            })
            .or_insert_with(|| {
                let mut m: HashMap<String, u16> = HashMap::new();
                m.insert(bucket_time_str, 1);
                m
            });

        Ok(())
    }

    fn merge_box(&mut self, other: &dyn Aggregator) {
        let other = other
            .as_any()
            .downcast_ref::<PeaksAggregator>()
            .expect("Aggregator type mismatch");

        for (severity, source_buckets) in &other.buckets_by_severity {
            let target_buckets = self
                .buckets_by_severity
                .entry(*severity)
                .or_default();
            for (bucket, count) in source_buckets {
                *target_buckets.entry(bucket.clone()).or_insert(0) += count;
            }
        }
    }

    fn print(&mut self) {
        println!("Events by severity:");
        for (host, count) in &self.total_by_severity {
            println!("  {:>6}  {}", count, host);
        }
    }

    fn boxed_clone(&self) -> Box<dyn Aggregator> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
