use flutter_rust_bridge::StreamSink;
use flutter_rust_bridge::SyncReturn;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

type UserActionSender = OnceCell<Mutex<Sender<(String, String)>>>;
pub static USER_ACTION_SENDER: UserActionSender = OnceCell::new();
type ViewmodelUpdateSender = OnceCell<Mutex<Sender<(String, Vec<u8>)>>>;
pub static VIEWMODEL_UPDATE_SENDER: ViewmodelUpdateSender = OnceCell::new();
type ViewmodelUpdateReceiver = OnceCell<Mutex<Receiver<(String, Vec<u8>)>>>;
pub static VIEWMODEL_UPDATE_RECEIVER: ViewmodelUpdateReceiver = OnceCell::new();
type ViewModel = OnceCell<Mutex<HashMap<String, Vec<u8>>>>;
static VIEWMODEL: ViewModel = OnceCell::new();

static IS_RUST_LOGIC_STARTED: AtomicBool = AtomicBool::new(false);
static DART_HOT_RESTART_COUNT: AtomicI32 = AtomicI32::new(0);

pub fn start_and_get_view_update_stream(view_update_stream: StreamSink<String>) {
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
        VIEWMODEL_UPDATE_SENDER.set(Mutex::new(sender.clone())).ok();
        hub::VIEWMODEL_UPDATE_SENDER.set(Mutex::new(sender)).ok();
        VIEWMODEL_UPDATE_RECEIVER.set(Mutex::new(receiver)).ok();
        let viewmodel = HashMap::<String, Vec<u8>>::new();
        VIEWMODEL.set(Mutex::new(viewmodel)).ok();
        std::thread::spawn(hub::main);
    } else {
        // Dart hot restart
        let sender = VIEWMODEL_UPDATE_SENDER.get().unwrap().lock().unwrap();
        sender.send((String::from("breakTheLoop"), vec![])).ok();
    }

    std::thread::spawn(move || {
        let receiver = VIEWMODEL_UPDATE_RECEIVER.get().unwrap().lock().unwrap();
        loop {
            if let Ok(received) = receiver.recv() {
                let data_address = received.0;
                let bytes = received.1;
                let mut viewmodel = VIEWMODEL.get().unwrap().lock().unwrap();
                viewmodel.insert(data_address.clone(), bytes);
                view_update_stream.add(data_address);
            }
            if hot_restart_number < DART_HOT_RESTART_COUNT.load(Ordering::SeqCst) {
                // When another `StreamSink` is established by hot restart
                break;
            }
        }
    });
}

pub fn read_viewmodel(data_address: String) -> SyncReturn<Option<Vec<u8>>> {
    let viewmodel = VIEWMODEL.get().unwrap().lock().unwrap();
    let bytes = viewmodel.get(&data_address).map(|val| val.clone());
    SyncReturn(bytes)
}

pub fn pass_user_action(task_address: String, json_string: String) -> SyncReturn<()> {
    // Main thread by Flutter

    let user_action = (task_address, json_string);
    let sender = USER_ACTION_SENDER.get().unwrap().lock().unwrap();
    sender.send(user_action).ok();
    SyncReturn(())
}
