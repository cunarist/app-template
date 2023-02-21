mod model;
mod user_action_handler;

use ctor::dtor;
use once_cell::sync::OnceCell;
use rand::Rng;
use serde_json::json;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use tokio::task;
use user_action_handler::handle_user_action;

type UserActionReceiver = OnceCell<Mutex<Receiver<(String, String)>>>;
pub static USER_ACTION_RECEIVER: UserActionReceiver = OnceCell::new();
type ViewmodelUpdateSender = OnceCell<Mutex<Sender<(String, Vec<u8>)>>>;
pub static VIEWMODEL_UPDATE_SENDER: ViewmodelUpdateSender = OnceCell::new();

#[tokio::main]
pub async fn main() {
    // Thread dedicated for Rust
    let user_action_receiver = USER_ACTION_RECEIVER.get().unwrap().lock().unwrap();
    let viewmodel_update_sender = VIEWMODEL_UPDATE_SENDER.get().unwrap().lock().unwrap();
    loop {
        if let Ok(user_action) = user_action_receiver.recv() {
            let mut rng = rand::thread_rng();
            let number = rng.gen_range(1..101);
            let json_string = json!({ "value": number }).to_string();
            viewmodel_update_sender
                .send((
                    String::from("someDataCategory.count"),
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
