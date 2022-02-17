use std::collections::HashMap;
use crate::lib::models::Id;
use crate::lib::models::item::{Item, Store};

pub async fn add_todo_list_item(
    item: Item,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.todo_list.write().insert(item.name, i32::from(item.finish));

    let mut result = HashMap::new();
    result.insert("message","Added items to the todo list");

    Ok(warp::reply::json(
        &result
    ))
}

pub async fn get_todo_list(
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    let r = store.todo_list.read();

    for (key,value) in r.iter() {
        result.insert(key, value);
    }

    Ok(warp::reply::json(
        &result
    ))
}

pub async fn update_todo_list(
    item: Item,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    store.todo_list.write().insert(item.name, i32::from(item.finish));

    let mut result = HashMap::new();
    result.insert("message","Updated items in the todo list");

    Ok(warp::reply::json(
        &result
    ))
}

pub async fn delete_todo_list_item(
    id: Id,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    store.todo_list.write().remove(&id.name);

    let mut result = HashMap::new();
    result.insert("message","Removed item from todo list");

    Ok(warp::reply::json(
        &result
    ))
}