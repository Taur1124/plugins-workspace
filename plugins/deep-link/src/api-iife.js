if("__TAURI__"in window){var __TAURI_PLUGIN_DEEPLINK__=function(e){"use strict";function n(e,n=!1){return window.__TAURI_INTERNALS__.transformCallback(e,n)}async function r(e,n={},r){return window.__TAURI_INTERNALS__.invoke(e,n,r)}var t;async function _(e,t,_){return r("plugin:event|listen",{event:e,windowLabel:_?.target,handler:n(t)}).then((n=>async()=>async function(e,n){await r("plugin:event|unlisten",{event:e,eventId:n})}(e,n)))}async function i(){return await r("plugin:deep-link|get_current")}return"function"==typeof SuppressedError&&SuppressedError,function(e){e.WINDOW_RESIZED="tauri://resize",e.WINDOW_MOVED="tauri://move",e.WINDOW_CLOSE_REQUESTED="tauri://close-requested",e.WINDOW_CREATED="tauri://window-created",e.WINDOW_DESTROYED="tauri://destroyed",e.WINDOW_FOCUS="tauri://focus",e.WINDOW_BLUR="tauri://blur",e.WINDOW_SCALE_FACTOR_CHANGED="tauri://scale-change",e.WINDOW_THEME_CHANGED="tauri://theme-changed",e.WINDOW_FILE_DROP="tauri://file-drop",e.WINDOW_FILE_DROP_HOVER="tauri://file-drop-hover",e.WINDOW_FILE_DROP_CANCELLED="tauri://file-drop-cancelled"}(t||(t={})),e.getCurrent=i,e.onOpenUrl=async function(e){const n=await i();return null!=n&&e(n),await _("deep-link://new-url",(n=>e(n.payload)))},e}({});Object.defineProperty(window.__TAURI__,"deepLink",{value:__TAURI_PLUGIN_DEEPLINK__})}
