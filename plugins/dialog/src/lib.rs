// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::Serialize;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

use std::{
    path::{Path, PathBuf},
    sync::mpsc::sync_channel,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::*;
#[cfg(mobile)]
use mobile::*;

macro_rules! blocking_fn {
    ($self:ident, $fn:ident) => {{
        let (tx, rx) = sync_channel(0);
        let cb = move |response| {
            tx.send(response).unwrap();
        };
        $self.$fn(cb);
        rx.recv().unwrap()
    }};
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the dialog APIs.
pub trait DialogExt<R: Runtime> {
    fn dialog(&self) -> &Dialog<R>;
}

impl<R: Runtime, T: Manager<R>> crate::DialogExt<R> for T {
    fn dialog(&self) -> &Dialog<R> {
        self.state::<Dialog<R>>().inner()
    }
}

impl<R: Runtime> Dialog<R> {
    pub fn message(&self, message: impl Into<String>) -> MessageDialogBuilder<R> {
        MessageDialogBuilder::new(
            self.clone(),
            self.app_handle().package_info().name.clone(),
            message,
        )
    }

    pub fn file(&self) -> FileDialogBuilder<R> {
        FileDialogBuilder::new(self.clone())
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("dialog")
        .invoke_handler(tauri::generate_handler![
            commands::open,
            commands::save,
            commands::message,
            commands::ask,
            commands::confirm
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let dialog = mobile::init(app, api)?;
            #[cfg(desktop)]
            let dialog = desktop::init(app, api)?;
            app.manage(dialog);
            Ok(())
        })
        .build()
}

/// A builder for message dialogs.
pub struct MessageDialogBuilder<R: Runtime> {
    #[allow(dead_code)]
    pub(crate) dialog: Dialog<R>,
    pub(crate) title: String,
    pub(crate) message: String,
    pub(crate) kind: MessageDialogKind,
    pub(crate) ok_button_label: Option<String>,
    pub(crate) cancel_button_label: Option<String>,
    #[cfg(desktop)]
    pub(crate) parent: Option<raw_window_handle::RawWindowHandle>,
}

/// Payload for the message dialog mobile API.
#[cfg(mobile)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDialogPayload<'a> {
    title: &'a String,
    message: &'a String,
    kind: &'a MessageDialogKind,
    ok_button_label: &'a Option<String>,
    cancel_button_label: &'a Option<String>,
}

// raw window handle :(
unsafe impl<R: Runtime> Send for MessageDialogBuilder<R> {}

impl<R: Runtime> MessageDialogBuilder<R> {
    /// Creates a new message dialog builder.
    pub fn new(dialog: Dialog<R>, title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            dialog,
            title: title.into(),
            message: message.into(),
            kind: Default::default(),
            ok_button_label: None,
            cancel_button_label: None,
            #[cfg(desktop)]
            parent: None,
        }
    }

    #[cfg(mobile)]
    pub(crate) fn payload(&self) -> MessageDialogPayload<'_> {
        MessageDialogPayload {
            title: &self.title,
            message: &self.message,
            kind: &self.kind,
            ok_button_label: &self.ok_button_label,
            cancel_button_label: &self.cancel_button_label,
        }
    }

    /// Sets the dialog title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set parent windows explicitly (optional)
    ///
    /// ## Platform-specific
    ///
    /// - **Linux:** Unsupported.
    #[cfg(desktop)]
    pub fn parent<W: raw_window_handle::HasRawWindowHandle>(mut self, parent: &W) -> Self {
        self.parent.replace(parent.raw_window_handle());
        self
    }

    /// Sets the label for the OK button.
    pub fn ok_button_label(mut self, label: impl Into<String>) -> Self {
        self.ok_button_label.replace(label.into());
        self
    }

    /// Sets the label for the Cancel button.
    pub fn cancel_button_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_button_label.replace(label.into());
        self
    }

    /// Set type of a dialog.
    ///
    /// Depending on the system it can result in type specific icon to show up,
    /// the will inform user it message is a error, warning or just information.
    pub fn kind(mut self, kind: MessageDialogKind) -> Self {
        self.kind = kind;
        self
    }

    /// Shows a message dialog
    pub fn show<F: FnOnce(bool) + Send + 'static>(self, f: F) {
        show_message_dialog(self, f)
    }

    //// Shows a message dialog.
    pub fn blocking_show(self) -> bool {
        blocking_fn!(self, show)
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Filter {
    pub name: String,
    pub extensions: Vec<String>,
}

/// The file dialog builder.
///
/// Constructs file picker dialogs that can select single/multiple files or directories.
#[derive(Debug)]
pub struct FileDialogBuilder<R: Runtime> {
    #[allow(dead_code)]
    pub(crate) dialog: Dialog<R>,
    pub(crate) filters: Vec<Filter>,
    pub(crate) starting_directory: Option<PathBuf>,
    pub(crate) file_name: Option<String>,
    pub(crate) title: Option<String>,
    #[cfg(desktop)]
    pub(crate) parent: Option<raw_window_handle::RawWindowHandle>,
}

#[cfg(mobile)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDialogPayload<'a> {
    filters: &'a Vec<Filter>,
    starting_directory: &'a Option<PathBuf>,
    file_name: &'a Option<String>,
    title: &'a Option<String>,
}

// raw window handle :(
unsafe impl<R: Runtime> Send for FileDialogBuilder<R> {}

impl<R: Runtime> FileDialogBuilder<R> {
    /// Gets the default file dialog builder.
    pub fn new(dialog: Dialog<R>) -> Self {
        Self {
            dialog,
            filters: Vec::new(),
            starting_directory: None,
            file_name: None,
            title: None,
            #[cfg(desktop)]
            parent: None,
        }
    }

    #[cfg(mobile)]
    pub(crate) fn payload(&self) -> FileDialogPayload<'_> {
        FileDialogPayload {
            filters: &self.filters,
            starting_directory: &self.starting_directory,
            file_name: &self.file_name,
            title: &self.title,
        }
    }

    /// Add file extension filter. Takes in the name of the filter, and list of extensions
    #[must_use]
    pub fn add_filter(mut self, name: impl Into<String>, extensions: &[&str]) -> Self {
        self.filters.push(Filter {
            name: name.into(),
            extensions: extensions.iter().map(|e| e.to_string()).collect(),
        });
        self
    }

    /// Set starting directory of the dialog.
    #[must_use]
    pub fn set_directory<P: AsRef<Path>>(mut self, directory: P) -> Self {
        self.starting_directory.replace(directory.as_ref().into());
        self
    }

    /// Set starting file name of the dialog.
    #[must_use]
    pub fn set_file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name.replace(file_name.into());
        self
    }

    /// Sets the parent window of the dialog.
    #[cfg(desktop)]
    #[must_use]
    pub fn set_parent<W: raw_window_handle::HasRawWindowHandle>(mut self, parent: &W) -> Self {
        self.parent.replace(parent.raw_window_handle());
        self
    }

    /// Set the title of the dialog.
    #[must_use]
    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title.replace(title.into());
        self
    }

    /// Shows the dialog to select a single file.
    /// This is not a blocking operation,
    /// and should be used when running on the main thread to avoid deadlocks with the event loop.
    ///
    /// For usage in other contexts such as commands, prefer [`Self::pick_file`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tauri::api::dialog::FileDialogBuilder;
    /// tauri::Builder::default()
    ///   .build(tauri::generate_context!("test/fixture/src-tauri/tauri.conf.json"))
    ///   .expect("failed to build tauri app")
    ///   .run(|_app, _event| {
    ///     FileDialogBuilder::new().pick_file(|file_path| {
    ///       // do something with the optional file path here
    ///       // the file path is `None` if the user closed the dialog
    ///     })
    ///   })
    /// ```
    pub fn pick_file<F: FnOnce(Option<PathBuf>) + Send + 'static>(self, f: F) {
        pick_file(self, f)
    }

    /// Shows the dialog to select multiple files.
    /// This is not a blocking operation,
    /// and should be used when running on the main thread to avoid deadlocks with the event loop.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tauri::api::dialog::FileDialogBuilder;
    /// tauri::Builder::default()
    ///   .build(tauri::generate_context!("test/fixture/src-tauri/tauri.conf.json"))
    ///   .expect("failed to build tauri app")
    ///   .run(|_app, _event| {
    ///     FileDialogBuilder::new().pick_files(|file_paths| {
    ///       // do something with the optional file paths here
    ///       // the file paths value is `None` if the user closed the dialog
    ///     })
    ///   })
    /// ```
    pub fn pick_files<F: FnOnce(Option<Vec<PathBuf>>) + Send + 'static>(self, f: F) {
        pick_files(self, f)
    }

    /// Shows the dialog to select a single folder.
    /// This is not a blocking operation,
    /// and should be used when running on the main thread to avoid deadlocks with the event loop.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tauri::api::dialog::FileDialogBuilder;
    /// tauri::Builder::default()
    ///   .build(tauri::generate_context!("test/fixture/src-tauri/tauri.conf.json"))
    ///   .expect("failed to build tauri app")
    ///   .run(|_app, _event| {
    ///     FileDialogBuilder::new().pick_folder(|folder_path| {
    ///       // do something with the optional folder path here
    ///       // the folder path is `None` if the user closed the dialog
    ///     })
    ///   })
    /// ```
    pub fn pick_folder<F: FnOnce(Option<PathBuf>) + Send + 'static>(self, f: F) {
        pick_folder(self, f)
    }

    /// Shows the dialog to select multiple folders.
    /// This is not a blocking operation,
    /// and should be used when running on the main thread to avoid deadlocks with the event loop.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tauri::api::dialog::FileDialogBuilder;
    /// tauri::Builder::default()
    ///   .build(tauri::generate_context!("test/fixture/src-tauri/tauri.conf.json"))
    ///   .expect("failed to build tauri app")
    ///   .run(|_app, _event| {
    ///     FileDialogBuilder::new().pick_folders(|file_paths| {
    ///       // do something with the optional folder paths here
    ///       // the folder paths value is `None` if the user closed the dialog
    ///     })
    ///   })
    /// ```
    pub fn pick_folders<F: FnOnce(Option<Vec<PathBuf>>) + Send + 'static>(self, f: F) {
        pick_folders(self, f)
    }

    /// Shows the dialog to save a file.
    ///
    /// This is not a blocking operation,
    /// and should be used when running on the main thread to avoid deadlocks with the event loop.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tauri::api::dialog::FileDialogBuilder;
    /// tauri::Builder::default()
    ///   .build(tauri::generate_context!("test/fixture/src-tauri/tauri.conf.json"))
    ///   .expect("failed to build tauri app")
    ///   .run(|_app, _event| {
    ///     FileDialogBuilder::new().save_file(|file_path| {
    ///       // do something with the optional file path here
    ///       // the file path is `None` if the user closed the dialog
    ///     })
    ///   })
    /// ```
    pub fn save_file<F: FnOnce(Option<PathBuf>) + Send + 'static>(self, f: F) {
        save_file(self, f)
    }
}

/// Blocking APIs.
impl<R: Runtime> FileDialogBuilder<R> {
    /// Shows the dialog to select a single file.
    /// This is a blocking operation,
    /// and should *NOT* be used when running on the main thread context.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tauri::api::dialog::blocking::FileDialogBuilder;
    /// #[tauri::command]
    /// async fn my_command() {
    ///   let file_path = FileDialogBuilder::new().pick_file();
    ///   // do something with the optional file path here
    ///   // the file path is `None` if the user closed the dialog
    /// }
    /// ```
    pub fn blocking_pick_file(self) -> Option<PathBuf> {
        blocking_fn!(self, pick_file)
    }

    /// Shows the dialog to select multiple files.
    /// This is a blocking operation,
    /// and should *NOT* be used when running on the main thread context.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tauri::api::dialog::blocking::FileDialogBuilder;
    /// #[tauri::command]
    /// async fn my_command() {
    ///   let file_path = FileDialogBuilder::new().pick_files();
    ///   // do something with the optional file paths here
    ///   // the file paths value is `None` if the user closed the dialog
    /// }
    /// ```
    pub fn blocking_pick_files(self) -> Option<Vec<PathBuf>> {
        blocking_fn!(self, pick_files)
    }

    /// Shows the dialog to select a single folder.
    /// This is a blocking operation,
    /// and should *NOT* be used when running on the main thread context.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tauri::api::dialog::blocking::FileDialogBuilder;
    /// #[tauri::command]
    /// async fn my_command() {
    ///   let folder_path = FileDialogBuilder::new().pick_folder();
    ///   // do something with the optional folder path here
    ///   // the folder path is `None` if the user closed the dialog
    /// }
    /// ```
    pub fn blocking_pick_folder(self) -> Option<PathBuf> {
        blocking_fn!(self, pick_folder)
    }

    /// Shows the dialog to select multiple folders.
    /// This is a blocking operation,
    /// and should *NOT* be used when running on the main thread context.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tauri::api::dialog::blocking::FileDialogBuilder;
    /// #[tauri::command]
    /// async fn my_command() {
    ///   let folder_paths = FileDialogBuilder::new().pick_folders();
    ///   // do something with the optional folder paths here
    ///   // the folder paths value is `None` if the user closed the dialog
    /// }
    /// ```
    pub fn blocking_pick_folders(self) -> Option<Vec<PathBuf>> {
        blocking_fn!(self, pick_folders)
    }

    /// Shows the dialog to save a file.
    /// This is a blocking operation,
    /// and should *NOT* be used when running on the main thread context.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tauri::api::dialog::blocking::FileDialogBuilder;
    /// #[tauri::command]
    /// async fn my_command() {
    ///   let file_path = FileDialogBuilder::new().save_file();
    ///   // do something with the optional file path here
    ///   // the file path is `None` if the user closed the dialog
    /// }
    /// ```
    pub fn blocking_save_file(self) -> Option<PathBuf> {
        blocking_fn!(self, save_file)
    }
}
