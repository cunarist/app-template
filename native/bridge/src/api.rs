use flutter_rust_bridge::spawn;
use flutter_rust_bridge::StreamSink;
use flutter_rust_bridge::SyncReturn;
use flutter_rust_bridge::ZeroCopyBuffer;
use once_cell::sync::OnceCell;
use std::sync::mpsc::channel;
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
    // Thread by flutter_rust_bridge system
    VIEW_UPDATE_STREAM.set(sink).ok();
}

pub fn connect_and_start() -> SyncReturn<()> {
    // Main thread by Flutter
    let (sender, receiver) = channel();
    USER_ACTION_SENDER.set(Mutex::new(sender)).ok();
    hub::USER_ACTION_RECEIVER.set(Mutex::new(receiver)).ok();
    let (sender, receiver) = channel();
    hub::VIEW_UPDATE_SENDER.set(Mutex::new(sender)).ok();
    VIEW_UPDATE_RECEIVER.set(Mutex::new(receiver)).ok();
    spawn!(|| {
        // Thread by flutter_rust_bridge system
        let receiver = VIEW_UPDATE_RECEIVER.get().unwrap().lock().unwrap();
        loop {
            let stream = VIEW_UPDATE_STREAM.get().unwrap();
            if let Ok(received) = receiver.recv() {
                let detail = ViewUpdateDetail {
                    data_address: received.0,
                    bytes: ZeroCopyBuffer(received.1),
                };
                stream.add(detail);
            }
        }
    });
    std::thread::spawn(hub::main);
    SyncReturn(())
}

pub fn pass_user_action(task_address: String, json_string: String) -> SyncReturn<()> {
    // Main thread by Flutter
    let user_action = (task_address, json_string);
    let sender = USER_ACTION_SENDER.get().unwrap().lock().unwrap();
    sender.send(user_action).ok();
    SyncReturn(())
}
