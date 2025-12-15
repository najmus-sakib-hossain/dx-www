//! dx-sync — Realtime binary WebSocket protocol
pub const SYNC_SUBSCRIBE: u8 = 0xA0;
pub const SYNC_UNSUBSCRIBE: u8 = 0xA1;
pub const SYNC_MESSAGE: u8 = 0xA2;
pub const SYNC_DELTA: u8 = 0xA3;
pub const SYNC_ACK: u8 = 0xA4;
