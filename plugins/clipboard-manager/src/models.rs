// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "options")]
pub enum ClipKind {
    PlainText { label: Option<String>, text: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "options")]
pub enum ClipboardContents {
    PlainText(String),
}
