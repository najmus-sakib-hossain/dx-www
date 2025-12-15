//! # dx-debug — DevTools Bridge
//!
//! Debug binary protocols with human-readable output.
//!
//! ## Features
//! - Binary message decoding
//! - Timeline visualization
//! - State inspection
//! - Performance profiling

use serde::{Deserialize, Serialize};
use serde_json;

/// Debug message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugMessage {
    pub timestamp: i64,
    pub opcode: u8,
    pub opcode_name: String,
    pub payload_size: usize,
    pub decoded_payload: serde_json::Value,
}

/// Binary decoder
pub struct BinaryDecoder;

impl BinaryDecoder {
    /// Decode binary message
    pub fn decode(data: &[u8]) -> Option<DebugMessage> {
        if data.is_empty() {
            return None;
        }

        let opcode = data[0];
        let opcode_name = Self::opcode_to_name(opcode);
        let payload = &data[1..];

        // Try to decode payload as JSON (simplified)
        let decoded_payload =
            serde_json::to_value(format!("{:?}", payload)).unwrap_or(serde_json::Value::Null);

        Some(DebugMessage {
            timestamp: chrono::Utc::now().timestamp_millis(),
            opcode,
            opcode_name,
            payload_size: payload.len(),
            decoded_payload,
        })
    }

    /// Convert opcode to human-readable name
    pub fn opcode_to_name(opcode: u8) -> String {
        match opcode {
            // dx-form
            0x60 => "VALIDATE_FIELD".to_string(),
            0x61 => "VALIDATION_RESULT".to_string(),
            0x62 => "FORM_VALID".to_string(),

            // dx-query
            0x70 => "QUERY_REQUEST".to_string(),
            0x71 => "QUERY_RESPONSE".to_string(),
            0x72 => "QUERY_ERROR".to_string(),
            0x73 => "QUERY_INVALIDATE".to_string(),
            0x74 => "QUERY_SUBSCRIBE".to_string(),
            0x75 => "QUERY_UPDATE".to_string(),

            // dx-state
            0x80 => "STATE_INIT".to_string(),
            0x81 => "STATE_SET".to_string(),
            0x82 => "STATE_GET".to_string(),
            0x83 => "STATE_SUBSCRIBE".to_string(),
            0x84 => "STATE_NOTIFY".to_string(),

            // dx-db
            0x90 => "DB_QUERY".to_string(),
            0x91 => "DB_RESULT".to_string(),
            0x92 => "DB_ROW".to_string(),
            0x93 => "DB_ERROR".to_string(),
            0x94 => "DB_TRANSACTION".to_string(),

            // dx-sync
            0xA0 => "SYNC_SUBSCRIBE".to_string(),
            0xA1 => "SYNC_UNSUBSCRIBE".to_string(),
            0xA2 => "SYNC_MESSAGE".to_string(),
            0xA3 => "SYNC_DELTA".to_string(),
            0xA4 => "SYNC_ACK".to_string(),

            // dx-error
            0xB0 => "ERROR_BOUNDARY".to_string(),
            0xB1 => "ERROR_RECOVER".to_string(),
            0xB2 => "ERROR_REPORT".to_string(),

            // dx-interaction
            0xC0 => "INTERACTION_SAVE".to_string(),
            0xC1 => "INTERACTION_RESTORE".to_string(),

            _ => format!("UNKNOWN_{:02X}", opcode),
        }
    }

    /// Format message for console
    pub fn format_for_console(msg: &DebugMessage) -> String {
        format!(
            "[{:013}] {} (0x{:02X}) - {} bytes",
            msg.timestamp, msg.opcode_name, msg.opcode, msg.payload_size
        )
    }
}

/// Performance tracker
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub total_messages: u64,
    pub total_bytes: u64,
    pub messages_by_opcode: std::collections::HashMap<u8, u64>,
    pub start_time: i64,
}

impl PerformanceMetrics {
    /// Create new metrics tracker
    pub fn new() -> Self {
        Self {
            total_messages: 0,
            total_bytes: 0,
            messages_by_opcode: std::collections::HashMap::new(),
            start_time: chrono::Utc::now().timestamp_millis(),
        }
    }

    /// Record message
    pub fn record(&mut self, msg: &DebugMessage) {
        self.total_messages += 1;
        self.total_bytes += msg.payload_size as u64;

        *self.messages_by_opcode.entry(msg.opcode).or_insert(0) += 1;
    }

    /// Get messages per second
    pub fn messages_per_second(&self) -> f64 {
        let elapsed_ms = chrono::Utc::now().timestamp_millis() - self.start_time;
        if elapsed_ms > 0 {
            (self.total_messages as f64 / elapsed_ms as f64) * 1000.0
        } else {
            0.0
        }
    }

    /// Get average message size
    pub fn avg_message_size(&self) -> f64 {
        if self.total_messages > 0 {
            self.total_bytes as f64 / self.total_messages as f64
        } else {
            0.0
        }
    }

    /// Get most frequent opcode
    pub fn most_frequent_opcode(&self) -> Option<(u8, u64)> {
        self.messages_by_opcode
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(op, count)| (*op, *count))
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Debug logger
pub struct DebugLogger {
    messages: Vec<DebugMessage>,
    metrics: PerformanceMetrics,
    max_messages: usize,
}

impl DebugLogger {
    /// Create new logger
    pub fn new(max_messages: usize) -> Self {
        Self {
            messages: Vec::new(),
            metrics: PerformanceMetrics::new(),
            max_messages,
        }
    }

    /// Log message
    pub fn log(&mut self, msg: DebugMessage) {
        self.metrics.record(&msg);
        self.messages.push(msg);

        // Keep only last N messages
        if self.messages.len() > self.max_messages {
            self.messages.drain(0..self.messages.len() - self.max_messages);
        }
    }

    /// Get all messages
    pub fn messages(&self) -> &[DebugMessage] {
        &self.messages
    }

    /// Get metrics
    pub fn metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
        self.metrics = PerformanceMetrics::new();
    }

    /// Export to JSON
    pub fn export_json(&self) -> String {
        serde_json::to_string_pretty(&self.messages).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_names() {
        assert_eq!(BinaryDecoder::opcode_to_name(0x60), "VALIDATE_FIELD");
        assert_eq!(BinaryDecoder::opcode_to_name(0x70), "QUERY_REQUEST");
        assert_eq!(BinaryDecoder::opcode_to_name(0x80), "STATE_INIT");
        assert_eq!(BinaryDecoder::opcode_to_name(0xA0), "SYNC_SUBSCRIBE");
    }

    #[test]
    fn test_decode() {
        let data = vec![0x60, 1, 2, 3, 4];
        let msg = BinaryDecoder::decode(&data).unwrap();

        assert_eq!(msg.opcode, 0x60);
        assert_eq!(msg.opcode_name, "VALIDATE_FIELD");
        assert_eq!(msg.payload_size, 4);
    }

    #[test]
    fn test_format_console() {
        let msg = DebugMessage {
            timestamp: 1234567890,
            opcode: 0x70,
            opcode_name: "QUERY_REQUEST".to_string(),
            payload_size: 128,
            decoded_payload: serde_json::Value::Null,
        };

        let formatted = BinaryDecoder::format_for_console(&msg);
        assert!(formatted.contains("QUERY_REQUEST"));
        assert!(formatted.contains("128 bytes"));
    }

    #[test]
    fn test_performance_metrics() {
        let mut metrics = PerformanceMetrics::new();

        let msg = DebugMessage {
            timestamp: 0,
            opcode: 0x70,
            opcode_name: "TEST".to_string(),
            payload_size: 100,
            decoded_payload: serde_json::Value::Null,
        };

        metrics.record(&msg);
        metrics.record(&msg);

        assert_eq!(metrics.total_messages, 2);
        assert_eq!(metrics.total_bytes, 200);
        assert_eq!(metrics.avg_message_size(), 100.0);
    }

    #[test]
    fn test_debug_logger() {
        let mut logger = DebugLogger::new(10);

        for i in 0..15 {
            let msg = DebugMessage {
                timestamp: i,
                opcode: 0x70,
                opcode_name: "TEST".to_string(),
                payload_size: 50,
                decoded_payload: serde_json::Value::Null,
            };
            logger.log(msg);
        }

        // Should keep only last 10 messages
        assert_eq!(logger.messages().len(), 10);
        assert_eq!(logger.metrics().total_messages, 15);
    }
}
