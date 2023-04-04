// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Use native message and file open/save dialogs.
//!
//! This module exposes non-blocking APIs on its root, relying on callback closures
//! to give results back. This is particularly useful when running dialogs from the main thread.
//! When using on asynchronous contexts such as async commands, the [`blocking`] APIs are recommended.

use std::path::PathBuf;

use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::{models::*, FileDialogBuilder, MessageDialogBuilder};

const OK: &str = "Ok";

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Dialog<R>> {
    Ok(Dialog(app.clone()))
}

/// Access to the dialog APIs.
#[derive(Debug)]
pub struct Dialog<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Clone for Dialog<R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R: Runtime> Dialog<R> {
    pub(crate) fn app_handle(&self) -> &AppHandle<R> {
        &self.0
    }
}

#[cfg(not(target_os = "linux"))]
macro_rules! run_dialog {
    ($e:expr, $h: ident) => {{
        std::thread::spawn(move || {
            let response = $e;
            $h(response);
        });
    }};
}

#[cfg(target_os = "linux")]
macro_rules! run_dialog {
    ($e:expr, $h: ident) => {{
        std::thread::spawn(move || {
            let context = glib::MainContext::default();
            context.invoke_with_priority(glib::PRIORITY_HIGH, move || {
                let response = $e;
                $h(response);
            });
        });
    }};
}

#[cfg(not(target_os = "linux"))]
macro_rules! run_file_dialog {
    ($e:expr, $h: ident) => {{
        std::thread::spawn(move || {
            let response = crate::async_runtime::block_on($e);
            $h(response);
        });
    }};
}

#[cfg(target_os = "linux")]
macro_rules! run_file_dialog {
    ($e:expr, $h: ident) => {{
        std::thread::spawn(move || {
            let context = glib::MainContext::default();
            context.invoke_with_priority(glib::PRIORITY_HIGH, move || {
                let response = $e;
                $h(response);
            });
        });
    }};
}

impl From<MessageDialogKind> for rfd::MessageLevel {
    fn from(kind: MessageDialogKind) -> Self {
        match kind {
            MessageDialogKind::Info => Self::Info,
            MessageDialogKind::Warning => Self::Warning,
            MessageDialogKind::Error => Self::Error,
        }
    }
}

impl<R: Runtime> From<FileDialogBuilder<R>> for rfd::FileDialog {
    fn from(d: FileDialogBuilder<R>) -> Self {
        let mut builder = rfd::FileDialog::new();

        if let Some(title) = d.title {
            builder = builder.set_title(&title);
        }
        if let Some(starting_directory) = d.starting_directory {
            builder = builder.set_directory(starting_directory);
        }
        if let Some(file_name) = d.file_name {
            builder = builder.set_file_name(&file_name);
        }
        for filter in d.filters {
            let v: Vec<&str> = filter.extensions.iter().map(|x| &**x).collect();
            builder = builder.add_filter(&filter.name, &v);
        }
        #[cfg(desktop)]
        if let Some(_parent) = d.parent {
            // TODO builder = builder.set_parent(&parent);
        }

        builder
    }
}

impl<R: Runtime> From<MessageDialogBuilder<R>> for rfd::MessageDialog {
    fn from(d: MessageDialogBuilder<R>) -> Self {
        let mut dialog = rfd::MessageDialog::new()
            .set_title(&d.title)
            .set_description(&d.message)
            .set_level(d.kind.into());

        let buttons = match (d.ok_button_label, d.cancel_button_label) {
            (Some(ok), Some(cancel)) => Some(rfd::MessageButtons::OkCancelCustom(ok, cancel)),
            (Some(ok), None) => Some(rfd::MessageButtons::OkCustom(ok)),
            (None, Some(cancel)) => Some(rfd::MessageButtons::OkCancelCustom(OK.into(), cancel)),
            (None, None) => None,
        };
        if let Some(buttons) = buttons {
            dialog = dialog.set_buttons(buttons);
        }

        if let Some(_parent) = d.parent {
            // TODO dialog.set_parent(parent);
        }

        dialog
    }
}

pub fn pick_file<R: Runtime, F: FnOnce(Option<PathBuf>) + Send + 'static>(
    dialog: FileDialogBuilder<R>,
    f: F,
) {
    #[cfg(not(target_os = "linux"))]
    let f = |path: Option<rfd::FileHandle>| f(path.map(|p| p.path().to_path_buf()));
    run_file_dialog!(rfd::FileDialog::from(dialog).pick_file(), f)
}

pub fn pick_files<R: Runtime, F: FnOnce(Option<Vec<PathBuf>>) + Send + 'static>(
    dialog: FileDialogBuilder<R>,
    f: F,
) {
    #[cfg(not(target_os = "linux"))]
    let f = |paths: Option<Vec<rfd::FileHandle>>| {
        f(paths.map(|list| list.into_iter().map(|p| p.path().to_path_buf()).collect()))
    };
    run_file_dialog!(rfd::FileDialog::from(dialog).pick_files(), f)
}

pub fn pick_folder<R: Runtime, F: FnOnce(Option<PathBuf>) + Send + 'static>(
    dialog: FileDialogBuilder<R>,
    f: F,
) {
    #[cfg(not(target_os = "linux"))]
    let f = |path: Option<rfd::FileHandle>| f(path.map(|p| p.path().to_path_buf()));
    run_file_dialog!(rfd::FileDialog::from(dialog).pick_folder(), f)
}

pub fn pick_folders<R: Runtime, F: FnOnce(Option<Vec<PathBuf>>) + Send + 'static>(
    dialog: FileDialogBuilder<R>,
    f: F,
) {
    #[cfg(not(target_os = "linux"))]
    let f = |paths: Option<Vec<rfd::FileHandle>>| {
        f(paths.map(|list| list.into_iter().map(|p| p.path().to_path_buf()).collect()))
    };
    run_file_dialog!(rfd::FileDialog::from(dialog).pick_folders(), f)
}

pub fn save_file<R: Runtime, F: FnOnce(Option<PathBuf>) + Send + 'static>(
    dialog: FileDialogBuilder<R>,
    f: F,
) {
    #[cfg(not(target_os = "linux"))]
    let f = |path: Option<rfd::FileHandle>| f(path.map(|p| p.path().to_path_buf()));
    run_file_dialog!(rfd::FileDialog::from(dialog).save_file(), f)
}

/// Shows a message dialog
pub fn show_message_dialog<R: Runtime, F: FnOnce(bool) + Send + 'static>(
    dialog: MessageDialogBuilder<R>,
    f: F,
) {
    run_dialog!(rfd::MessageDialog::from(dialog).show(), f);
}
