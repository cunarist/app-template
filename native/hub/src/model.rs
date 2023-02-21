// use anymap::AnyMap;
// use once_cell::sync::OnceCell;
// use std::sync::Mutex;

// type Model = OnceCell<Mutex<AnyMap>>;
// static MODEL: Model = OnceCell::new();

// pub fn createModel() {
//     let model = AnyMap::new();
//     MODEL.set(Mutex::new(model)).ok();
// }
