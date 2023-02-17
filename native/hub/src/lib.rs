use ctor::dtor;
use once_cell::sync::OnceCell;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use tokio::task;
use user_action_handler::handle_user_action;

mod user_action_handler;
mod view_model;

type ReceiverHolder = OnceCell<Mutex<Receiver<(String, String)>>>;
pub static USER_ACTION_RECEIVER: ReceiverHolder = OnceCell::new();
type SenderHolder = OnceCell<Mutex<Sender<(String, Vec<u8>)>>>;
pub static VIEW_UPDATE_SENDER: SenderHolder = OnceCell::new();

#[tokio::main]
pub async fn main() {
    // Thread dedicated for Rust
    loop {
        let receiver = USER_ACTION_RECEIVER.get().unwrap().lock().unwrap();
        if let Ok(user_action) = receiver.recv() {
            task::spawn(handle_user_action(user_action));
        }
    }
}

#[dtor]
fn finalize() {
    // Main thread by Flutter
    println!("Bye!");
}
