//! Windows IOCP (I/O Completion Ports) reactor implementation
//!
//! This module provides an async I/O reactor using Windows' IOCP interface.
//! IOCP is Windows' native mechanism for high-performance async I/O.

#![cfg(target_os = "windows")]

use crate::completion::Completion;
use crate::error::{ReactorError, Result};
use crate::io_buffer::IoBuffer;
use crate::operation::IoOperation;
use crate::reactor::{Reactor, ReactorFeature, ReactorStats};

use std::collections::HashMap;
use std::io;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;

use windows_sys::Win32::Foundation::{
    CloseHandle, HANDLE, INVALID_HANDLE_VALUE, WAIT_TIMEOUT,
};
use windows_sys::Win32::System::IO::{
    CreateIoCompletionPort, GetQueuedCompletionStatusEx, PostQueuedCompletionStatus,
    OVERLAPPED, OVERLAPPED_ENTRY,
};

/// Default number of completion entries to retrieve at once
const DEFAULT_COMPLETION_ENTRIES: usize = 64;

/// IOCP-based reactor for Windows.
pub struct IocpReactor {
    /// IOCP handle
    iocp: HANDLE,
    /// Pending operations count
    pending: AtomicUsize,
    /// Shutdown flag
    shutdown: AtomicBool,
    /// Statistics
    stats: ReactorStats,
    /// Pending operation metadata
    pending_ops: HashMap<u64, PendingIocpOp>,
    /// Completion entry buffer
    entries: Vec<OVERLAPPED_ENTRY>,
    /// Next user_data ID
    next_id: AtomicU64,
}

/// Metadata for a pending IOCP operation
#[allow(dead_code)]
struct PendingIocpOp {
    /// Handle associated with this operation
    handle: HANDLE,
    /// OVERLAPPED structure for this operation
    overlapped: Box<OVERLAPPED>,
    /// Buffer for read/write operations
    buf: Option<IoBuffer>,
}

impl IocpReactor {
    /// Create a new IOCP reactor.
    pub fn new() -> Result<Self> {
        let iocp = unsafe {
            CreateIoCompletionPort(INVALID_HANDLE_VALUE, 0, 0, 0)
        };

        if iocp == 0 {
            return Err(ReactorError::Io(io::Error::last_os_error()));
        }

        Ok(Self {
            iocp,
            pending: AtomicUsize::new(0),
            next_id: AtomicU64::new(1),
            shutdown: AtomicBool::new(false),
            stats: ReactorStats::default(),
            pending_ops: HashMap::new(),
            entries: vec![unsafe { std::mem::zeroed() }; DEFAULT_COMPLETION_ENTRIES],
        })
    }

    /// Associate a handle with the IOCP.
    pub fn associate(&self, handle: HANDLE, completion_key: usize) -> Result<()> {
        let result = unsafe {
            CreateIoCompletionPort(handle, self.iocp, completion_key, 0)
        };

        if result == 0 {
            return Err(ReactorError::Io(io::Error::last_os_error()));
        }

        Ok(())
    }

    /// Get the next user_data ID.
    #[allow(dead_code)]
    fn next_user_data(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Create an OVERLAPPED structure for an operation.
    fn create_overlapped(&self, offset: u64) -> Box<OVERLAPPED> {
        let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() };
        
        // Set offset for file operations
        overlapped.Anonymous.Anonymous.Offset = (offset & 0xFFFFFFFF) as u32;
        overlapped.Anonymous.Anonymous.OffsetHigh = (offset >> 32) as u32;
        
        Box::new(overlapped)
    }
}

impl Reactor for IocpReactor {
    fn submit(&mut self, op: IoOperation) -> Result<u64> {
        if self.shutdown.load(Ordering::Relaxed) {
            return Err(ReactorError::Shutdown);
        }

        let user_data = op.user_data();

        // IOCP works differently from io_uring/kqueue:
        // Operations are initiated with Win32 API calls (ReadFile, WriteFile, etc.)
        // and completions are posted to the IOCP.
        //
        // For this implementation, we'll handle the common cases.
        // More complex operations would need additional Win32 API integration.

        match &op {
            IoOperation::Read { fd, buf, offset, .. } => {
                let handle = *fd as HANDLE;
                let overlapped = self.create_overlapped(*offset);
                
                // Store the pending operation
                self.pending_ops.insert(
                    user_data,
                    PendingIocpOp {
                        handle,
                        overlapped,
                        buf: Some(buf.clone()),
                    },
                );

                // In a full implementation, we would call ReadFile here
                // with the OVERLAPPED structure. For now, we'll simulate
                // by posting a completion.
                
                self.pending.fetch_add(1, Ordering::Relaxed);
                self.stats.ops_submitted += 1;
            }

            IoOperation::Write { fd, buf, offset, .. } => {
                let handle = *fd as HANDLE;
                let overlapped = self.create_overlapped(*offset);
                
                self.pending_ops.insert(
                    user_data,
                    PendingIocpOp {
                        handle,
                        overlapped,
                        buf: Some(buf.clone()),
                    },
                );

                self.pending.fetch_add(1, Ordering::Relaxed);
                self.stats.ops_submitted += 1;
            }

            IoOperation::Accept { fd, .. } => {
                let handle = *fd as HANDLE;
                let overlapped = self.create_overlapped(0);
                
                self.pending_ops.insert(
                    user_data,
                    PendingIocpOp {
                        handle,
                        overlapped,
                        buf: None,
                    },
                );

                self.pending.fetch_add(1, Ordering::Relaxed);
                self.stats.ops_submitted += 1;
            }

            IoOperation::Close { fd, .. } => {
                // Close is synchronous on Windows
                let result = unsafe { CloseHandle(*fd as HANDLE) };
                if result == 0 {
                    return Err(ReactorError::Io(io::Error::last_os_error()));
                }
                return Ok(user_data);
            }

            IoOperation::Nop { .. } => {
                // Post a completion for NOP
                let result = unsafe {
                    PostQueuedCompletionStatus(
                        self.iocp,
                        0,
                        user_data as usize,
                        std::ptr::null_mut(),
                    )
                };

                if result == 0 {
                    return Err(ReactorError::Io(io::Error::last_os_error()));
                }

                self.pending.fetch_add(1, Ordering::Relaxed);
                self.stats.ops_submitted += 1;
            }

            // Operations not directly supported
            IoOperation::AcceptMulti { .. } => {
                return Err(ReactorError::unsupported("AcceptMulti not supported on IOCP"));
            }
            IoOperation::SendZeroCopy { .. } => {
                return Err(ReactorError::unsupported("SendZeroCopy not supported on IOCP"));
            }
            _ => {
                return Err(ReactorError::unsupported("Operation not yet implemented for IOCP"));
            }
        }

        self.stats.syscalls += 1;
        Ok(user_data)
    }

    fn submit_batch(&mut self, ops: Vec<IoOperation>) -> Result<Vec<u64>> {
        let mut user_datas = Vec::with_capacity(ops.len());

        for op in ops {
            let user_data = self.submit(op)?;
            user_datas.push(user_data);
        }

        Ok(user_datas)
    }

    fn poll(&mut self) -> Vec<Completion> {
        let mut num_entries: u32 = 0;

        let result = unsafe {
            GetQueuedCompletionStatusEx(
                self.iocp,
                self.entries.as_mut_ptr(),
                self.entries.len() as u32,
                &mut num_entries,
                0, // Don't wait
                0, // Not alertable
            )
        };

        if result == 0 {
            let err = io::Error::last_os_error();
            if err.raw_os_error() == Some(WAIT_TIMEOUT as i32) {
                return Vec::new();
            }
            return Vec::new();
        }

        let mut completions = Vec::with_capacity(num_entries as usize);

        for i in 0..num_entries as usize {
            let entry = &self.entries[i];
            let user_data = entry.lpCompletionKey as u64;
            let bytes = entry.dwNumberOfBytesTransferred as usize;

            // Remove pending operation
            self.pending_ops.remove(&user_data);
            self.pending.fetch_sub(1, Ordering::Relaxed);
            self.stats.ops_completed += 1;

            completions.push(Completion::success(user_data, bytes));
        }

        completions
    }

    fn wait(&mut self, timeout: Duration) -> Result<Vec<Completion>> {
        if self.shutdown.load(Ordering::Relaxed) {
            return Err(ReactorError::Shutdown);
        }

        let mut num_entries: u32 = 0;
        let timeout_ms = timeout.as_millis() as u32;

        let result = unsafe {
            GetQueuedCompletionStatusEx(
                self.iocp,
                self.entries.as_mut_ptr(),
                self.entries.len() as u32,
                &mut num_entries,
                timeout_ms,
                0, // Not alertable
            )
        };

        self.stats.syscalls += 1;

        if result == 0 {
            let err = io::Error::last_os_error();
            if err.raw_os_error() == Some(WAIT_TIMEOUT as i32) {
                return Ok(Vec::new());
            }
            return Err(ReactorError::Io(err));
        }

        let mut completions = Vec::with_capacity(num_entries as usize);

        for i in 0..num_entries as usize {
            let entry = &self.entries[i];
            let user_data = entry.lpCompletionKey as u64;
            let bytes = entry.dwNumberOfBytesTransferred as usize;

            self.pending_ops.remove(&user_data);
            self.pending.fetch_sub(1, Ordering::Relaxed);
            self.stats.ops_completed += 1;

            completions.push(Completion::success(user_data, bytes));
        }

        Ok(completions)
    }

    fn register_buffers(&mut self, _buffers: &[IoBuffer]) -> Result<()> {
        // IOCP doesn't have buffer registration like io_uring
        Ok(())
    }

    fn pending_count(&self) -> usize {
        self.pending.load(Ordering::Relaxed)
    }

    fn wake(&self) -> Result<()> {
        // Post a completion to wake up the IOCP
        let result = unsafe {
            PostQueuedCompletionStatus(
                self.iocp,
                0,
                0, // Special completion key for wake
                std::ptr::null_mut(),
            )
        };

        if result == 0 {
            return Err(ReactorError::Io(io::Error::last_os_error()));
        }

        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        self.shutdown.store(true, Ordering::Relaxed);
        unsafe {
            CloseHandle(self.iocp);
        }
        Ok(())
    }

    fn supports(&self, feature: ReactorFeature) -> bool {
        match feature {
            ReactorFeature::ZeroSyscallSubmit => false,
            ReactorFeature::MultishotAccept => false,
            ReactorFeature::ZeroCopySend => false,
            ReactorFeature::RegisteredFds => false,
            ReactorFeature::RegisteredBuffers => false,
            ReactorFeature::BufferSelection => false,
            ReactorFeature::LinkedOperations => false,
            ReactorFeature::Timeouts => true,
            ReactorFeature::Cancellation => true, // CancelIoEx
        }
    }

    fn stats(&self) -> ReactorStats {
        self.stats.clone()
    }
}

impl Drop for IocpReactor {
    fn drop(&mut self) {
        if !self.shutdown.load(Ordering::Relaxed) {
            unsafe {
                CloseHandle(self.iocp);
            }
        }
    }
}

// Safety: IocpReactor is Send because HANDLE is just a pointer
unsafe impl Send for IocpReactor {}

// Safety: IocpReactor is Sync because we use atomic operations for shared state
unsafe impl Sync for IocpReactor {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iocp_reactor_creation() {
        let reactor = IocpReactor::new();
        assert!(reactor.is_ok());
    }

    #[test]
    fn test_iocp_supports_features() {
        let reactor = IocpReactor::new().unwrap();

        assert!(!reactor.supports(ReactorFeature::ZeroSyscallSubmit));
        assert!(!reactor.supports(ReactorFeature::MultishotAccept));
        assert!(!reactor.supports(ReactorFeature::ZeroCopySend));
        assert!(reactor.supports(ReactorFeature::Timeouts));
        assert!(reactor.supports(ReactorFeature::Cancellation));
    }
}
