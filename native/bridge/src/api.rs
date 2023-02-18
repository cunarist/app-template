use flutter_rust_bridge::StreamSink;
use flutter_rust_bridge::SyncReturn;
use flutter_rust_bridge::ZeroCopyBuffer;
use once_cell::sync::OnceCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

pub struct ViewUpdateDetail {
    pub data_address: String,
    pub bytes: ZeroCopyBuffer<Vec<u8>>,
}

type UserSenderHolder = OnceCell<Mutex<Sender<(String, String)>>>;
pub static USER_ACTION_SENDER: UserSenderHolder = OnceCell::new();
type ViewSenderHolder = OnceCell<Mutex<Sender<(String, Vec<u8>)>>>;
pub static VIEW_UPDATE_SENDER: ViewSenderHolder = OnceCell::new();
type ViewReceiverHolder = OnceCell<Mutex<Receiver<(String, Vec<u8>)>>>;
pub static VIEW_UPDATE_RECEIVER: ViewReceiverHolder = OnceCell::new();

static IS_RUST_LOGIC_STARTED: AtomicBool = AtomicBool::new(false);
static DART_HOT_RESTART_COUNT: AtomicI32 = AtomicI32::new(0);

pub fn start_and_get_view_update_stream(stream: StreamSink<ViewUpdateDetail>) {
    // Thread by flutter_rust_bridge
    DART_HOT_RESTART_COUNT.fetch_add(1, Ordering::SeqCst);
    let hot_restart_number = DART_HOT_RESTART_COUNT.load(Ordering::SeqCst);

    if !IS_RUST_LOGIC_STARTED.load(Ordering::SeqCst) {
        // Dart first run
        IS_RUST_LOGIC_STARTED.store(true, Ordering::SeqCst);
        let (sender, receiver) = channel();
        USER_ACTION_SENDER.set(Mutex::new(sender)).ok();
        hub::USER_ACTION_RECEIVER.set(Mutex::new(receiver)).ok();
        let (sender, receiver) = channel();
        VIEW_UPDATE_SENDER.set(Mutex::new(sender.clone())).ok();
        hub::VIEW_UPDATE_SENDER.set(Mutex::new(sender)).ok();
        VIEW_UPDATE_RECEIVER.set(Mutex::new(receiver)).ok();
        std::thread::spawn(hub::main);
    } else {
        // Dart hot restart
        let sender = VIEW_UPDATE_SENDER.get().unwrap().lock().unwrap();
        sender.send((String::from("breakTheLoop"), vec![])).ok();
    }

    std::thread::spawn(move || {
        let receiver = VIEW_UPDATE_RECEIVER.get().unwrap().lock().unwrap();
        loop {
            if let Ok(received) = receiver.recv() {
                let detail = ViewUpdateDetail {
                    data_address: received.0,
                    bytes: ZeroCopyBuffer(received.1),
                };
                stream.add(detail);
            }
            if hot_restart_number < DART_HOT_RESTART_COUNT.load(Ordering::SeqCst) {
                // When another `StreamSink` is established by hot restart
                break;
            }
        }
    });
}

pub fn pass_user_action(task_address: String, json_string: String) -> SyncReturn<()> {
    // Main thread by Flutter

    let user_action = (task_address, json_string);
    let sender = USER_ACTION_SENDER.get().unwrap().lock().unwrap();
    sender.send(user_action).ok();
    SyncReturn(())
}
