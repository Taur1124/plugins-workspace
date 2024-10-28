// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! [![](https://github.com/tauri-apps/plugins-workspace/raw/v2/plugins/http/banner.png)](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/http)
//!
//! Access the HTTP client written in Rust.

pub use reqwest;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use error::{Error, Result};

mod commands;
mod error;
mod scope;

pub(crate) struct Http {
    #[cfg(feature = "cookies")]
    cookies_jar_path: std::path::PathBuf,
    #[cfg(feature = "cookies")]
    cookies_jar: std::sync::Arc<reqwest_cookie_store::CookieStoreMutex>,
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::<R>::new("http")
        .setup(|app, _| {
            #[cfg(feature = "cookies")]
            let (cookies_jar_path, cookies_jar) = {
                use reqwest_cookie_store::*;
                use std::fs::File;
                use std::io::BufReader;
                use std::sync::Arc;

                let cache_dir = app.path().app_cache_dir()?;
                std::fs::create_dir_all(&cache_dir)?;

                let path = cache_dir.join("Cookies");
                let file = File::options()
                    .create(true)
                    .write(true)
                    .read(true)
                    .open(&path)?;

                let reader = BufReader::new(file);
                let store = CookieStore::load_json(reader).map_err(|e| e.to_string())?;

                (path, Arc::new(CookieStoreMutex::new(store)))
            };

            let state = Http {
                #[cfg(feature = "cookies")]
                cookies_jar_path,
                #[cfg(feature = "cookies")]
                cookies_jar,
            };

            app.manage(state);

            Ok(())
        })
        .on_event(|app, event| {
            #[cfg(feature = "cookies")]
            if let tauri::RunEvent::Exit = event {
                use std::fs::File;
                use std::io::BufWriter;

                let state = app.state::<Http>();

                if let Ok(file) = File::create(&state.cookies_jar_path) {
                    let store = state.cookies_jar.lock().expect("poisoned cookie jar mutex");
                    let mut writer = BufWriter::new(file);
                    let _ = store.save_json(&mut writer);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::fetch,
            commands::fetch_cancel,
            commands::fetch_send,
            commands::fetch_read_body,
        ])
        .build()
}
