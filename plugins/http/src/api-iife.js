if("__TAURI__"in window){var __TAURI_PLUGIN_HTTP__=function(e){"use strict";async function t(e,t={},r){return window.__TAURI_INTERNALS__.invoke(e,t,r)}return"function"==typeof SuppressedError&&SuppressedError,e.fetch=async function(e,r){const n=r?.maxRedirections,i=r?.maxRedirections,a=r?.proxy;r&&(delete r.maxRedirections,delete r.connectTimeout,delete r.proxy);const o=new Request(e,r),s=await o.arrayBuffer(),d=s.byteLength?Array.from(new Uint8Array(s)):null,u=await t("plugin:http|fetch",{method:o.method,url:o.url,headers:Array.from(o.headers.entries()),data:d,maxRedirections:n,connectTimeout:i,proxy:a});o.signal.addEventListener("abort",(()=>{t("plugin:http|fetch_cancel",{rid:u})}));const{status:_,statusText:c,url:p,headers:f}=await t("plugin:http|fetch_send",{rid:u}),h=await t("plugin:http|fetch_read_body",{rid:u}),l=new Response(new Uint8Array(h),{headers:f,status:_,statusText:c});return Object.defineProperty(l,"url",{value:p}),l},e}({});Object.defineProperty(window.__TAURI__,"http",{value:__TAURI_PLUGIN_HTTP__})}
