// SPDX-License-Identifier: MIT OR Apache-2.0

unsafe extern "C" {
	pub fn queue_signals_function();
	pub fn unqueue_signals_function();
}

// RAII type to temporarily stop handling signals with
pub struct SignalQueueGuard;

impl SignalQueueGuard {
	pub fn new() -> Self {
		unsafe {
			queue_signals_function();
		}
		Self
	}
}

impl Drop for SignalQueueGuard {
	fn drop(&mut self) {
		unsafe {
			unqueue_signals_function();
		}
	}
}
