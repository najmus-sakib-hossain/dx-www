//! Binary message definitions for test protocol

use dx_py_core::{ProtocolError, TestCase, TestId, TestResult, TestStatus, AssertionStats};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Magic bytes for protocol mes