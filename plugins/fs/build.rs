// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

#[path = "src/scope.rs"]
#[allow(dead_code)]
mod scope;

/// FS scope entry.
#[derive(schemars::JsonSchema)]
#[serde(untagged)]
#[allow(unused)]
enum FsScopeEntry {
    /// FS scope path.
    Value(PathBuf),
    Object {
        /// FS scope path.
        path: PathBuf,
    },
}

// Ensure scope entry is kept up to date
impl From<FsScopeEntry> for scope::Entry {
    fn from(value: FsScopeEntry) -> Self {
        let path = match value {
            FsScopeEntry::Value(path) => path,
            FsScopeEntry::Object { path } => path,
        };

        scope::Entry { path }
    }
}

const BASE_DIR_VARS: &[&str] = &[
    "AUDIO",
    "CACHE",
    "CONFIG",
    "DATA",
    "LOCALDATA",
    "DESKTOP",
    "DOCUMENT",
    "DOWNLOAD",
    "EXE",
    "FONT",
    "HOME",
    "PICTURE",
    "PUBLIC",
    "RUNTIME",
    "TEMPLATE",
    "VIDEO",
    "RESOURCE",
    "APP",
    "LOG",
    "TEMP",
    "APPCONFIG",
    "APPDATA",
    "APPLOCALDATA",
    "APPCACHE",
    "APPLOG",
];
const COMMANDS: &[&str] = &[
    "mkdir",
    "create",
    "copy_file",
    "remove",
    "rename",
    "truncate",
    "ftruncate",
    "write",
    "write_file",
    "write_text_file",
    "read_dir",
    "read_file",
    "read",
    "open",
    "read_text_file",
    "read_text_file_lines",
    "read_text_file_lines_next",
    "seek",
    "stat",
    "lstat",
    "fstat",
    "exists",
    "watch",
    "unwatch",
];

fn main() {
    let autogenerated = Path::new("permissions/autogenerated/");
    let base_dirs = &autogenerated.join("base-directories");

    if !base_dirs.exists() {
        create_dir_all(base_dirs).expect("unable to create autogenerated base directories dir");
    }

    for base_dir in BASE_DIR_VARS {
        let upper = base_dir;
        let lower = base_dir.to_lowercase();
        let toml = format!(
            r###"# Automatically generated - DO NOT EDIT!

"$schema" = "../../schemas/schema.json"

# Scopes Section
# This section contains scopes, which define file level access

[[permission]]
identifier = "scope-{lower}-recursive"
description = "This scope recursive access to the complete `${upper}` folder, including sub directories and files."

[[permission.scope.allow]]
path = "${upper}/**"

[[permission]]
identifier = "scope-{lower}"
description = "This scope permits access to all files and list content of top level directories in the `${upper}`folder."

[[permission.scope.allow]]
path = "${upper}/*"

[[permission]]
identifier = "scope-{lower}-index"
description = "This scope permits to list all files and folders in the `${upper}`folder."

[[permission.scope.allow]]
path = "${upper}/"

# Sets Section
# This section combines the scope elements with enablement of commands

[[set]]
identifier = "allow-{lower}-read-recursive"
description = "This allows full recursive read access to the complete `${upper}` folder, files and subdirectories."
permissions = [
    "read-all",
    "scope-{lower}-recursive"
]

[[set]]
identifier = "allow-{lower}-write-recursive"
description = "This allows full recusrive write access to the complete `${upper}` folder, files and subdirectories."
permissions = [
    "write-all",
    "scope-{lower}-recursive"
]

[[set]]
identifier = "allow-{lower}-read"
description = "This allows non-recursive read access to the `${upper}` folder."
permissions = [
    "read-all",
    "scope-{lower}"
]

[[set]]
identifier = "allow-{lower}-write"
description = "This allows non-recursive write access to the `${upper}` folder."
permissions = [
    "write-all",
    "scope-{lower}"
]

[[set]]
identifier = "allow-{lower}-meta-recursive"
description = "This allows read access to metadata of the `${upper}` folder, including file listing and statistics."
permissions = [
    "read-meta",
    "scope-{lower}-recursive"
]

[[set]]
identifier = "allow-{lower}-meta"
description = "This allows read access to metadata of the `${upper}` folder, including file listing and statistics."
permissions = [
    "read-meta",
    "scope-{lower}-index"
]"###
        );

        let permission_path = base_dirs.join(format!("{lower}.toml"));
        if toml != std::fs::read_to_string(&permission_path).unwrap_or_default() {
            std::fs::write(permission_path, toml)
                .unwrap_or_else(|e| panic!("unable to autogenerate ${lower}: {e}"));
        }
    }

    tauri_plugin::Builder::new(COMMANDS)
        .global_api_script_path("./api-iife.js")
        .global_scope_schema(schemars::schema_for!(FsScopeEntry))
        .build();
}
