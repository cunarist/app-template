use flutter_rust_bridge::StreamSink;
use flutter_rust_bridge::SyncReturn;
use flutter_rust_bridge::ZeroCopyBuffer;
use once_cell::sync::OnceCell;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

pub struct ViewUpdateDetail {
    pub data_address: String,
    pub bytes: ZeroCopyBuffer<Vec<u8>>,
}

type SenderHolder = OnceCell<Mutex<Sender<(String, String)>>>;
pub static USER_ACTION_SENDER: SenderHolder = OnceCell::new();
type ReceiverHolder = OnceCell<Mutex<Receiver<(String, Vec<u8>)>>>;
pub static VIEW_UPDATE_RECEIVER: ReceiverHolder = OnceCell::new();
type StreamHolder = OnceCell<StreamSink<ViewUpdateDetail>>;
pub static VIEW_UPDATE_STREAM: StreamHolder = OnceCell::new();

pub fn create_view_update_stream(sink: StreamSink<ViewUpdateDetail>) {
    // Second thread only for Rust
    VIEW_UPDATE_STREAM.set(sink).ok();
}

pub fn pass_user_action(task_address: String, json_string: String) -> SyncReturn<()> {
    // First thread mainly for Dart
    let user_action = (task_address, json_string);
    let sender = USER_ACTION_SENDER.get().unwrap().lock().unwrap();
    sender.send(user_action).ok();
    SyncReturn(())
}
