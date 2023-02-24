#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.64.0.

use crate::bridge::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_start_and_get_viewmodel_update_stream_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "start_and_get_viewmodel_update_stream",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            move |task_callback| {
                Ok(start_and_get_viewmodel_update_stream(
                    task_callback.stream_sink(),
                ))
            }
        },
    )
}
fn wire_read_viewmodel_impl(
    data_address: impl Wire2Api<DotAddress> + UnwindSafe,
    take_ownership: impl Wire2Api<bool> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "read_viewmodel",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_data_address = data_address.wire2api();
            let api_take_ownership = take_ownership.wire2api();
            Ok(read_viewmodel(api_data_address, api_take_ownership))
        },
    )
}
fn wire_send_user_action_impl(
    task_address: impl Wire2Api<DotAddress> + UnwindSafe,
    json_string: impl Wire2Api<String> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "send_user_action",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_task_address = task_address.wire2api();
            let api_json_string = json_string.wire2api();
            Ok(send_user_action(api_task_address, api_json_string))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<bool> for bool {
    fn wire2api(self) -> bool {
        self
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
