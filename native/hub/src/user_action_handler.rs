use tokio::task;

pub async fn handle_user_action(user_action: (String, String)) {
    let task_address = user_action.0; // "some.task.address"
    let json_string = user_action.1; // "{'some':'json','string':true}"
    let task_layers = task_address.split('.').collect::<Vec<&str>>();

    if task_layers.is_empty() {
    } else if task_layers[0] == "someTaskCategory" {
        if task_layers.len() == 1 {
        } else if task_layers[1] == "addOne" {
            task::spawn_blocking(move || sample_feature::add_one(json_string));
        } else if task_layers[1] == "multiplyTwo" {
            task::spawn_blocking(move || sample_feature::multiply_two(json_string));
        } else {
        }
    } else {
    }
}
