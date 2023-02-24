use crate::api::DotAddress;
use crate::bridge::update_viewmodel_with_json;
use crate::model;
use serde_json::json;

pub fn calculate_something(json_value: serde_json::Value) {
    let _ = json_value;

    let mut value = model::COUNT.write().unwrap();
    *value = sample_crate::add_seven(*value);
    println!("{:}", *value);
    let json_value = json!({ "value": *value });

    update_viewmodel_with_json(DotAddress::from("someDataCategory.count"), json_value)
}
