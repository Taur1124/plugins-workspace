// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! The Tauri updater.
//!
//! The updater is focused on making Tauri's application updates **as safe and transparent as updates to a website**.
//!
//! For a full guide on setting up the updater, see <https://tauri.app/v1/guides/distribution/updater>.
//!
//! Check [`UpdateBuilder`] to see how to manually trigger and customize the updater at runtime.
//!
//! ## Events
//!
//! To listen to the updater events, for example to check for error messages, you need to use [`RunEvent::Updater`](crate::RunEvent) in [`App::run`](crate::App#method.run).
//!
//! ```no_run
//! let app = tauri::Builder::default()
//!   // on an actual app, remove the string argument
//!   .build(tauri::generate_context!("tests/app-updater/tauri.conf.json"))
//!   .expect("error while building tauri application");
//! app.run(|_app_handle, event| match event {
//!   tauri::RunEvent::Updater(updater_event) => {
//!     match updater_event {
//!       tauri::UpdaterEvent::UpdateAvailable { body, date, version } => {
//!         println!("update available {} {:?} {}", body, date, version);
//!       }
//!       // Emitted when the download is about to be started.
//!       tauri::UpdaterEvent::Pending => {
//!         println!("update is pending!");
//!       }
//!       tauri::UpdaterEvent::DownloadProgress { chunk_length, content_length } => {
//!         println!("downloaded {} of {:?}", chunk_length, content_length);
//!       }
//!       // Emitted when the download has finished and the update is about to be installed.
//!       tauri::UpdaterEvent::Downloaded => {
//!         println!("update has been downloaded!");
//!       }
//!       // Emitted when the update was installed. You can then ask to restart the app.
//!       tauri::UpdaterEvent::Updated => {
//!         println!("app has been updated");
//!       }
//!       // Emitted when the app already has the latest version installed and an update is not needed.
//!       tauri::UpdaterEvent::AlreadyUpToDate => {
//!         println!("app is already up to date");
//!       }
//!       // Emitted when there is an error with the updater. We suggest to listen to this event even if the default dialog is enabled.
//!       tauri::UpdaterEvent::Error(error) => {
//!         println!("failed to update: {}", error);
//!       }
//!       _ => (),
//!     }
//!   }
//!   _ => {}
//! });
//! ```

mod core;

use std::time::Duration;

use http::header::{HeaderName, HeaderValue};
use semver::Version;
use time::OffsetDateTime;

pub use self::core::RemoteRelease;

use tauri::{AppHandle, EventLoopMessage, Manager, Runtime, UpdaterEvent};
use tauri_runtime::EventLoopProxy;

use crate::Result;

/// Check for new updates
pub const EVENT_CHECK_UPDATE: &str = "tauri://update";
/// New update available
pub const EVENT_UPDATE_AVAILABLE: &str = "tauri://update-available";
/// Used to initialize an update *should run check-update first (once you received the update available event)*
pub const EVENT_INSTALL_UPDATE: &str = "tauri://update-install";
/// Send updater status or error even if dialog is enabled, you should
/// always listen for this event. It'll send you the install progress
/// and any error triggered during update check and install
pub const EVENT_STATUS_UPDATE: &str = "tauri://update-status";
/// The name of the event that is emitted on download progress.
pub const EVENT_DOWNLOAD_PROGRESS: &str = "tauri://update-download-progress";
/// this is the status emitted when the download start
pub const EVENT_STATUS_PENDING: &str = "PENDING";
/// When you got this status, something went wrong
/// you can find the error message inside the `error` field.
pub const EVENT_STATUS_ERROR: &str = "ERROR";
/// The update has been downloaded.
pub const EVENT_STATUS_DOWNLOADED: &str = "DOWNLOADED";
/// When you receive this status, you should ask the user to restart
pub const EVENT_STATUS_SUCCESS: &str = "DONE";
/// When you receive this status, this is because the application is running last version
pub const EVENT_STATUS_UPTODATE: &str = "UPTODATE";

/// Gets the target string used on the updater.
pub fn target() -> Option<String> {
    if let (Some(target), Some(arch)) = (core::get_updater_target(), core::get_updater_arch()) {
        Some(format!("{target}-{arch}"))
    } else {
        None
    }
}

#[derive(Clone, serde::Serialize)]
struct StatusEvent {
    status: String,
    error: Option<String>,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DownloadProgressEvent {
    chunk_length: usize,
    content_length: Option<u64>,
}

#[derive(Clone, serde::Serialize)]
struct UpdateManifest {
    version: String,
    date: Option<String>,
    body: String,
}

/// An update check builder.
#[derive(Debug)]
pub struct UpdateBuilder<R: Runtime> {
    inner: core::UpdateBuilder<R>,
    events: bool,
}

impl<R: Runtime> UpdateBuilder<R> {
    /// Do not use the event system to emit information or listen to install the update.
    pub fn skip_events(mut self) -> Self {
        self.events = false;
        self
    }

    /// Sets the current platform's target name for the updater.
    ///
    /// The target is injected in the endpoint URL by replacing `{{target}}`.
    /// Note that this does not affect the `{{arch}}` variable.
    ///
    /// If the updater response JSON includes the `platforms` field,
    /// that object must contain a value for the target key.
    ///
    /// By default Tauri uses `$OS_NAME` as the replacement for `{{target}}`
    /// and `$OS_NAME-$ARCH` as the key in the `platforms` object,
    /// where `$OS_NAME` is the current operating system name "linux", "windows" or "darwin")
    /// and `$ARCH` is one of the supported architectures ("i686", "x86_64", "armv7" or "aarch64").
    ///
    /// See [`Builder::updater_target`](crate::Builder#method.updater_target) for a way to set the target globally.
    ///
    /// # Examples
    ///
    /// ## Use a macOS Universal binary target name
    ///
    /// In this example, we set the updater target only on macOS.
    /// On other platforms, we set the default target.
    /// Note that `{{target}}` will be replaced with `darwin-universal`,
    /// but `{{arch}}` is still the running platform's architecture.
    ///
    /// ```no_run
    /// use tauri_plugin_updater::{target as updater_target, UpdaterExt};
    /// tauri::Builder::default()
    ///   .setup(|app| {
    ///     let handle = app.handle();
    ///     tauri::async_runtime::spawn(async move {
    ///       let builder = handle.updater().target(if cfg!(target_os = "macos") {
    ///         "darwin-universal".to_string()
    ///       } else {
    ///         updater_target().unwrap()
    ///       });
    ///       match builder.check().await {
    ///         Ok(update) => {}
    ///         Err(error) => {}
    ///       }
    ///     });
    ///     Ok(())
    ///   });
    /// ```
    ///
    /// ## Append debug information to the target
    ///
    /// This allows you to provide updates for both debug and release applications.
    ///
    /// ```no_run
    /// use tauri_plugin_updater::{UpdaterExt, target as updater_target};
    /// tauri::Builder::default()
    ///   .setup(|app| {
    ///     let handle = app.handle();
    ///     tauri::async_runtime::spawn(async move {
    ///       let kind = if cfg!(debug_assertions) { "debug" } else { "release" };
    ///       let builder = handle.updater().target(format!("{}-{kind}", updater_target().unwrap()));
    ///       match builder.check().await {
    ///         Ok(update) => {}
    ///         Err(error) => {}
    ///       }
    ///     });
    ///     Ok(())
    ///   });
    /// ```
    ///
    /// ## Use the platform's target triple
    ///
    /// ```no_run
    /// use tauri_plugin_updater::UpdaterExt;
    /// tauri::Builder::default()
    ///   .setup(|app| {
    ///     let handle = app.handle();
    ///     tauri::async_runtime::spawn(async move {
    ///       let builder = handle.updater().target(tauri::utils::platform::target_triple().unwrap());
    ///       match builder.check().await {
    ///         Ok(update) => {}
    ///         Err(error) => {}
    ///       }
    ///     });
    ///     Ok(())
    ///   });
    /// ```
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.inner = self.inner.target(target);
        self
    }

    /// Sets a closure that is invoked to compare the current version and the latest version returned by the updater server.
    /// The first argument is the current version, and the second one is the latest version.
    ///
    /// The closure must return `true` if the update should be installed.
    ///
    /// # Examples
    ///
    /// - Always install the version returned by the server:
    ///
    /// ```no_run
    /// use tauri_plugin_updater::UpdaterExt;
    /// tauri::Builder::default()
    ///   .setup(|app| {
    ///     app.handle().updater().should_install(|_current, _latest| true);
    ///     Ok(())
    ///   });
    /// ```
    pub fn should_install<F: FnOnce(&Version, &RemoteRelease) -> bool + Send + 'static>(
        mut self,
        f: F,
    ) -> Self {
        self.inner = self.inner.should_install(f);
        self
    }

    /// Sets the timeout for the requests to the updater endpoints.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.inner = self.inner.timeout(timeout);
        self
    }

    /// Add a `Header` to the request.
    pub fn header<K, V>(mut self, key: K, value: V) -> Result<Self>
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        self.inner = self.inner.header(key, value)?;
        Ok(self)
    }

    /// Check if an update is available.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tauri_plugin_updater::UpdaterExt;
    /// tauri::Builder::default()
    ///   .setup(|app| {
    ///     let handle = app.handle();
    ///     tauri::async_runtime::spawn(async move {
    ///       match handle.updater().check().await {
    ///         Ok(update) => {
    ///           if update.is_update_available() {
    ///             update.download_and_install().await.unwrap();
    ///           }
    ///         }
    ///         Err(e) => {
    ///           println!("failed to get update: {}", e);
    ///         }
    ///       }
    ///     });
    ///     Ok(())
    ///   });
    /// ```
    pub async fn check(self) -> Result<UpdateResponse<R>> {
        let handle = self.inner.app.clone();
        let events = self.events;
        // check updates
        match self.inner.build().await {
            Ok(update) => {
                if events {
                    // send notification if we need to update
                    if update.should_update {
                        let body = update.body.clone().unwrap_or_else(|| String::from(""));

                        // Emit `tauri://update-available`
                        let _ = handle.emit_all(
                            EVENT_UPDATE_AVAILABLE,
                            UpdateManifest {
                                body: body.clone(),
                                date: update.date.map(|d| d.to_string()),
                                version: update.version.clone(),
                            },
                        );
                        let _ = handle.create_proxy().send_event(EventLoopMessage::Updater(
                            UpdaterEvent::UpdateAvailable {
                                body,
                                date: update.date,
                                version: update.version.clone(),
                            },
                        ));

                        // Listen for `tauri://update-install`
                        let update_ = update.clone();
                        handle.once_global(EVENT_INSTALL_UPDATE, move |_msg| {
                            tauri::async_runtime::spawn(async move {
                                let _ = download_and_install(update_).await;
                            });
                        });
                    } else {
                        send_status_update(&handle, UpdaterEvent::AlreadyUpToDate);
                    }
                }
                Ok(UpdateResponse { update })
            }
            Err(e) => {
                if self.events {
                    send_status_update(&handle, UpdaterEvent::Error(e.to_string()));
                }
                Err(e)
            }
        }
    }
}

/// The response of an updater check.
pub struct UpdateResponse<R: Runtime> {
    update: core::Update<R>,
}

impl<R: Runtime> Clone for UpdateResponse<R> {
    fn clone(&self) -> Self {
        Self {
            update: self.update.clone(),
        }
    }
}

impl<R: Runtime> UpdateResponse<R> {
    /// Whether the updater found a newer release or not.
    pub fn is_update_available(&self) -> bool {
        self.update.should_update
    }

    /// The current version of the application as read by the updater.
    pub fn current_version(&self) -> &Version {
        &self.update.current_version
    }

    /// The latest version of the application found by the updater.
    pub fn latest_version(&self) -> &str {
        &self.update.version
    }

    /// The update date.
    pub fn date(&self) -> Option<&OffsetDateTime> {
        self.update.date.as_ref()
    }

    /// The update description.
    pub fn body(&self) -> Option<&String> {
        self.update.body.as_ref()
    }

    /// Downloads and installs the update.
    pub async fn download_and_install(self) -> Result<()> {
        download_and_install(self.update).await
    }
}

pub(crate) async fn download_and_install<R: Runtime>(update: core::Update<R>) -> Result<()> {
    // Start installation
    // emit {"status": "PENDING"}
    send_status_update(&update.app, UpdaterEvent::Pending);

    let handle = update.app.clone();
    let handle_ = handle.clone();

    // Launch updater download process
    // macOS we display the `Ready to restart dialog` asking to restart
    // Windows is closing the current App and launch the downloaded MSI when ready (the process stop here)
    // Linux we replace the AppImage by launching a new install, it start a new AppImage instance, so we're closing the previous. (the process stop here)
    let update_result = update
        .download_and_install(
            update.app.config().tauri.updater.pubkey.clone(),
            move |chunk_length, content_length| {
                send_download_progress_event(&handle, chunk_length, content_length);
            },
            move || {
                send_status_update(&handle_, UpdaterEvent::Downloaded);
            },
        )
        .await;

    if let Err(err) = &update_result {
        // emit {"status": "ERROR", "error": "The error message"}
        send_status_update(&update.app, UpdaterEvent::Error(err.to_string()));
    } else {
        // emit {"status": "DONE"}
        send_status_update(&update.app, UpdaterEvent::Updated);
    }
    update_result
}

/// Initializes the [`UpdateBuilder`] using the app configuration.
pub fn builder<R: Runtime>(handle: AppHandle<R>) -> UpdateBuilder<R> {
    let updater_config = &handle.config().tauri.updater;
    let package_info = handle.package_info().clone();

    // prepare our endpoints
    let endpoints = updater_config
        .endpoints
        .as_ref()
        .expect("Something wrong with endpoints")
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>();

    let mut builder = self::core::builder(handle.clone())
        .urls(&endpoints[..])
        .current_version(package_info.version);
    if let Some(target) = &handle.state::<crate::UpdaterState>().target {
        builder = builder.target(target);
    }
    UpdateBuilder {
        inner: builder,
        events: true,
    }
}

// Send a status update via `tauri://update-download-progress` event.
fn send_download_progress_event<R: Runtime>(
    handle: &AppHandle<R>,
    chunk_length: usize,
    content_length: Option<u64>,
) {
    let _ = handle.emit_all(
        EVENT_DOWNLOAD_PROGRESS,
        DownloadProgressEvent {
            chunk_length,
            content_length,
        },
    );
    let _ = handle.create_proxy().send_event(EventLoopMessage::Updater(
        UpdaterEvent::DownloadProgress {
            chunk_length,
            content_length,
        },
    ));
}

// Send a status update via `tauri://update-status` event.
fn send_status_update<R: Runtime>(handle: &AppHandle<R>, message: UpdaterEvent) {
    let _ = handle.emit_all(
        EVENT_STATUS_UPDATE,
        if let UpdaterEvent::Error(error) = &message {
            StatusEvent {
                error: Some(error.clone()),
                status: status_message(&message).into(),
            }
        } else {
            StatusEvent {
                error: None,
                status: status_message(&message).into(),
            }
        },
    );
    let _ = handle
        .create_proxy()
        .send_event(EventLoopMessage::Updater(message));
}

pub(crate) fn status_message(message: &UpdaterEvent) -> &'static str {
    match message {
      UpdaterEvent::Pending => EVENT_STATUS_PENDING,
      UpdaterEvent::Downloaded => EVENT_STATUS_DOWNLOADED,
      UpdaterEvent::Updated => EVENT_STATUS_SUCCESS,
      UpdaterEvent::AlreadyUpToDate => EVENT_STATUS_UPTODATE,
      UpdaterEvent::Error(_) => EVENT_STATUS_ERROR,
      _ => unreachable!(),
    }
  }