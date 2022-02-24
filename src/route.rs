use crate::lib::controllers::item::{
    add_todo_list_item, delete_todo_list_item, get_todo_list, update_todo_list,
};
use crate::lib::models::item::Store;
use crate::lib::services::item::{delete_json, post_json};
use warp::Filter;

pub fn create_routes() -> impl Filter<Extract = impl warp::Reply> + Clone {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("todos"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(add_todo_list_item);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("todos"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_todo_list);

    let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("todos"))
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(delete_todo_list_item);

    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("todos"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_todo_list);

    let routes = add_items
        .or(get_items)
        .or(update_item)
        .or(delete_item);

    routes
}
