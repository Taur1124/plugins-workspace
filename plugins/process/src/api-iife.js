if("__TAURI__"in window){var __TAURI_PLUGIN_PROCESS__=function(n){"use strict";async function r(n,r={},_){return window.__TAURI_INTERNALS__.invoke(n,r,_)}return"function"==typeof SuppressedError&&SuppressedError,n.exit=async function(n=0){return r("plugin:process|exit",{code:n})},n.relaunch=async function(){return r("plugin:process|restart")},n}({});Object.defineProperty(window.__TAURI__,"process",{value:__TAURI_PLUGIN_PROCESS__})}