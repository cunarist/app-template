use std::sync::mpsc::channel;
mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
use ctor::ctor;
pub use flutter_rust_bridge::spawn;
use flutter_rust_bridge::ZeroCopyBuffer;
use std::sync::Mutex;

pub mod api;

#[ctor]
fn connect_and_start() {
    // Second thread only for Rust
    let (sender, receiver) = channel();
    api::USER_ACTION_SENDER.set(Mutex::new(sender)).ok();
    hub::USER_ACTION_RECEIVER.set(Mutex::new(receiver)).ok();
    let (sender, receiver) = channel();
    hub::VIEW_UPDATE_SENDER.set(Mutex::new(sender)).ok();
    api::VIEW_UPDATE_RECEIVER.set(Mutex::new(receiver)).ok();
    spawn!(|| {
        loop {
            let receiver = api::VIEW_UPDATE_RECEIVER.get().unwrap().lock().unwrap();
            let stream = api::VIEW_UPDATE_STREAM.get().unwrap();
            if let Ok(received) = receiver.recv() {
                let detail = api::ViewUpdateDetail {
                    data_address: received.0,
                    bytes: ZeroCopyBuffer(received.1),
                };
                stream.add(detail);
            }
        }
    });
    std::thread::spawn(hub::main);
}
