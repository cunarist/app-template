use ctor::dtor;
use once_cell::sync::OnceCell;
use rand::Rng;
use serde_json::json;
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
    let receiver = USER_ACTION_RECEIVER.get().unwrap().lock().unwrap();
    let sender = VIEW_UPDATE_SENDER.get().unwrap().lock().unwrap();
    loop {
        if let Ok(user_action) = receiver.recv() {
            let mut rng = rand::thread_rng();
            let number = rng.gen_range(1..101);
            let json_string = json!({ "value": number }).to_string();
            sender
                .send((
                    "someDataCategory.count".to_string(),
                    json_string.as_bytes().to_vec(),
                ))
                .ok();
            task::spawn(handle_user_action(user_action));
        }
    }
}

#[dtor]
fn finalize() {
    // Main thread by Flutter
    // This code is executed before closing unless crashed
    println!("Bye!");
}
