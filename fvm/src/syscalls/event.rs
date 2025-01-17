// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use super::Context;
use crate::kernel::Result;
use crate::Kernel;

/// Emits an actor event. It takes an DAG-CBOR encoded ActorEvent that has been
/// written to Wasm memory, as an offset and length tuple.
///
/// The FVM validates the structural, syntatic, and semantic correctness of the
/// supplied event, and errors with `IllegalArgument` if the payload was invalid.
///
/// Calling this syscall may immediately halt execution with an out of gas error,
/// if such condition arises.
pub fn emit_event(
    context: Context<'_, impl Kernel>,
    event_off: u32, // ActorEvent
    event_len: u32,
) -> Result<()> {
    // Disable the limits for gas calibration.
    #[cfg(not(feature = "gas_calibration"))]
    {
        const MAX_CBOR_EVENT_LEN: u32 = 1024;
        if event_len > MAX_CBOR_EVENT_LEN {
            return Err(crate::syscall_error!(IllegalArgument; "exceeded maximum event length ({} bytes); got {}", MAX_CBOR_EVENT_LEN, event_len).into());
        }
    }

    let raw = context.memory.try_slice(event_off, event_len)?;
    context.kernel.emit_event(raw)
}
