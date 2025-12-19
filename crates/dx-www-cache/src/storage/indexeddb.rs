//! # IndexedDB Storage
//!
//! Primary storage for templates, snapshots, and state

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, IdbDatabase, IdbOpenDbRequest, IdbTransactionMode};

/// Open or create IndexedDB database
pub async fn open_database(db_name: &str, version: u32) -> Result<IdbDatabase, JsValue> {
    let window = window().ok_or("No window")?;
    let indexed_db = window.indexed_db()?.ok_or("No IndexedDB")?;

    let request: IdbOpenDbRequest = indexed_db.open_with_u32(db_name, version)?;

    // Set up onupgradeneeded handler
    let onupgradeneeded = Closure::once(move |event: web_sys::IdbVersionChangeEvent| {
        let target = event.target().unwrap();
        let request: IdbOpenDbRequest = target.dyn_into().unwrap();
        let db: IdbDatabase = request.result().unwrap().dyn_into().unwrap();

        // Create object stores
        if !db.object_store_names().contains("templates") {
            db.create_object_store("templates").unwrap();
        }

        if !db.object_store_names().contains("snapshots") {
            db.create_object_store("snapshots").unwrap();
        }

        if !db.object_store_names().contains("metadata") {
            db.create_object_store("metadata").unwrap();
        }
    });

    request.set_onupgradeneeded(Some(onupgradeneeded.as_ref().unchecked_ref()));
    onupgradeneeded.forget();

    // Wait for success
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let onsuccess = Closure::once(move |event: web_sys::Event| {
            let target = event.target().unwrap();
            let request: IdbOpenDbRequest = target.dyn_into().unwrap();
            let db: IdbDatabase = request.result().unwrap().dyn_into().unwrap();
            resolve.call1(&JsValue::NULL, &db).unwrap();
        });

        let onerror = Closure::once(move |event: web_sys::Event| {
            let target = event.target().unwrap();
            let request: IdbOpenDbRequest = target.dyn_into().unwrap();
            let error = request.error().unwrap().unwrap();
            reject.call1(&JsValue::NULL, &error).unwrap();
        });

        request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
        request.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        onsuccess.forget();
        onerror.forget();
    });

    let result = JsFuture::from(promise).await?;
    Ok(result.dyn_into()?)
}

/// Store binary data in IndexedDB
pub async fn store_binary(
    db: &IdbDatabase,
    store_name: &str,
    key: &str,
    data: &[u8],
) -> Result<(), JsValue> {
    let transaction =
        db.transaction_with_str_and_mode(store_name, IdbTransactionMode::Readwrite)?;

    let store = transaction.object_store(store_name)?;

    // Convert Rust bytes to JS Uint8Array
    let uint8_array = Uint8Array::from(data);

    store.put_with_key(&uint8_array, &JsValue::from_str(key))?;

    Ok(())
}

/// Retrieve binary data from IndexedDB
pub async fn get_binary(
    db: &IdbDatabase,
    store_name: &str,
    key: &str,
) -> Result<Option<Vec<u8>>, JsValue> {
    let transaction = db.transaction_with_str(store_name)?;
    let store = transaction.object_store(store_name)?;

    let request = store.get(&JsValue::from_str(key))?;

    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let onsuccess = Closure::once(move |event: web_sys::Event| {
            let target = event.target().unwrap();
            let request: web_sys::IdbRequest = target.dyn_into().unwrap();
            resolve.call1(&JsValue::NULL, &request.result().unwrap()).unwrap();
        });

        let onerror = Closure::once(move |event: web_sys::Event| {
            let target = event.target().unwrap();
            let request: web_sys::IdbRequest = target.dyn_into().unwrap();
            let error = request.error().unwrap().unwrap();
            reject.call1(&JsValue::NULL, &error).unwrap();
        });

        request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
        request.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        onsuccess.forget();
        onerror.forget();
    });

    let result = JsFuture::from(promise).await?;

    if result.is_undefined() {
        return Ok(None);
    }

    // Convert JS Uint8Array to Rust Vec<u8>
    let uint8_array: Uint8Array = result.dyn_into()?;
    let mut vec = vec![0u8; uint8_array.length() as usize];
    uint8_array.copy_to(&mut vec);

    Ok(Some(vec))
}

/// Delete IndexedDB database
pub async fn delete_database(db_name: &str) -> Result<(), JsValue> {
    let window = window().ok_or("No window")?;
    let indexed_db = window.indexed_db()?.ok_or("No IndexedDB")?;

    let request = indexed_db.delete_database(db_name)?;

    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let onsuccess = Closure::once(move |_: web_sys::Event| {
            resolve.call0(&JsValue::NULL).unwrap();
        });

        let onerror = Closure::once(move |event: web_sys::Event| {
            let target = event.target().unwrap();
            let request: IdbOpenDbRequest = target.dyn_into().unwrap();
            let error = request.error().unwrap().unwrap();
            reject.call1(&JsValue::NULL, &error).unwrap();
        });

        request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
        request.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        onsuccess.forget();
        onerror.forget();
    });

    JsFuture::from(promise).await?;

    Ok(())
}
