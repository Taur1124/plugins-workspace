// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import UIKit
import WebKit
import Tauri
import SwiftRs

class ClipboardPlugin: Plugin {
	@objc public func write(_ invoke: Invoke) throws {
		let options = invoke.getObject("options")
		if let options = options {
			let clipboard = UIPasteboard.general
			let kind = invoke.getString("kind", "")
			switch kind {
				case "PlainText":
					let text = options["text"] as? String
					clipboard.string = text
				default:
				  invoke.reject("Unknown kind \(kind)")
					return
			}
			invoke.resolve()
		} else {
			invoke.reject("Missing `options` input")
		}
	}

	@objc public func read(_ invoke: Invoke) throws {
		let clipboard = UIPasteboard.general
		if let text = clipboard.string {
			invoke.resolve([
				"kind": "PlainText",
				"options": text
			])
		} else {
			invoke.reject("Clipboard is empty")
		}
	}
}

@_cdecl("init_plugin_clipboard")
func initPlugin() -> Plugin {
  return ClipboardPlugin()
}
