if("__TAURI__"in window){var __TAURI_PLUGIN_UPDATER__=function(e){"use strict";function r(e,r,t,n){if("a"===t&&!n)throw new TypeError("Private accessor was defined without a getter");if("function"==typeof r?e!==r||!n:!r.has(e))throw new TypeError("Cannot read private member from an object whose class did not declare it");return"m"===t?n:"a"===t?n.call(e):n?n.value:r.get(e)}function t(e,r,t,n,s){if("m"===n)throw new TypeError("Private method is not writable");if("a"===n&&!s)throw new TypeError("Private accessor was defined without a setter");if("function"==typeof r?e!==r||!s:!r.has(e))throw new TypeError("Cannot write private member to an object whose class did not declare it");return"a"===n?s.call(e,t):s?s.value=t:r.set(e,t),t}var n,s;"function"==typeof SuppressedError&&SuppressedError;class i{constructor(){this.__TAURI_CHANNEL_MARKER__=!0,n.set(this,(()=>{})),this.id=function(e,r=!1){return window.__TAURI_INTERNALS__.transformCallback(e,r)}((e=>{r(this,n,"f").call(this,e)}))}set onmessage(e){t(this,n,e,"f")}get onmessage(){return r(this,n,"f")}toJSON(){return`__CHANNEL__:${this.id}`}}async function o(e,r={},t){return window.__TAURI_INTERNALS__.invoke(e,r,t)}n=new WeakMap;class a{get rid(){return r(this,s,"f")}constructor(e){s.set(this,void 0),t(this,s,e,"f")}async close(){return o("plugin:resources|close",{rid:this.rid})}}s=new WeakMap;class c extends a{constructor(e){super(e.rid),this.currentVersion=e.currentVersion,this.version=e.version,this.date=e.date,this.body=e.body}async downloadAndInstall(e){const r=new i;return e&&(r.onmessage=e),o("plugin:updater|download_and_install",{onEvent:r,rid:this.rid})}}return e.Update=c,e.check=async function(e){return e?.headers&&(e.headers=Array.from(new Headers(e.headers).entries())),o("plugin:updater|check",{...e}).then((e=>e.available?new c(e):null))},e}({});Object.defineProperty(window.__TAURI__,"updater",{value:__TAURI_PLUGIN_UPDATER__})}
