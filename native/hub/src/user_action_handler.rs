use tokio::task;

pub async fn handle_user_action(user_action: (String, String)) {
    let task_address = user_action.0; // "some.task.address"
    let json_string = user_action.1; // "{'some':'json','string':true}"
    let layered_task_address = task_address.split('.').collect::<Vec<&str>>();

    if layered_task_address.is_empty() {
    } else if layered_task_address[0] == "someTaskCategory" {
        if layered_task_address.len() == 1 {
        } else if layered_task_address[1] == "addOne" {
            task::spawn_blocking(move || sample_crate_first::add_one(json_string));
        } else if layered_task_address[1] == "multiplyTwo" {
            task::spawn_blocking(move || sample_crate_second::multiply_two(json_string));
        } else {
        }
    } else {
    }
}
