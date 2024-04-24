if("__TAURI__"in window){var __TAURI_PLUGIN_WEBSOCKET__=function(){"use strict";function e(e,t,s,r){if("a"===s&&!r)throw new TypeError("Private accessor was defined without a getter");if("function"==typeof t?e!==t||!r:!t.has(e))throw new TypeError("Cannot read private member from an object whose class did not declare it");return"m"===s?r:"a"===s?r.call(e):r?r.value:t.get(e)}function t(e,t,s,r,n){if("m"===r)throw new TypeError("Private method is not writable");if("a"===r&&!n)throw new TypeError("Private accessor was defined without a setter");if("function"==typeof t?e!==t||!n:!t.has(e))throw new TypeError("Cannot write private member to an object whose class did not declare it");return"a"===r?n.call(e,s):n?n.value=s:t.set(e,s),s}var s,r,n;"function"==typeof SuppressedError&&SuppressedError;class i{constructor(){this.__TAURI_CHANNEL_MARKER__=!0,s.set(this,(()=>{})),r.set(this,0),n.set(this,{}),this.id=function(e,t=!1){return window.__TAURI_INTERNALS__.transformCallback(e,t)}((({message:i,id:a})=>{if(a===e(this,r,"f")){t(this,r,a+1,"f"),e(this,s,"f").call(this,i);const o=Object.keys(e(this,n,"f"));if(o.length>0){let t=a+1;for(const r of o.sort()){if(parseInt(r)!==t)break;{const i=e(this,n,"f")[r];delete e(this,n,"f")[r],e(this,s,"f").call(this,i),t+=1}}}}else e(this,n,"f")[a.toString()]=i}))}set onmessage(e){t(this,s,e,"f")}get onmessage(){return e(this,s,"f")}toJSON(){return`__CHANNEL__:${this.id}`}}async function a(e,t={},s){return window.__TAURI_INTERNALS__.invoke(e,t,s)}s=new WeakMap,r=new WeakMap,n=new WeakMap;class o{constructor(e,t){this.id=e,this.listeners=t}static async connect(e,t){const s=[],r=new i;return r.onmessage=e=>{s.forEach((t=>{t(e)}))},null!=t?.headers&&(t.headers=Array.from(new Headers(t.headers).entries())),await a("plugin:websocket|connect",{url:e,onMessage:r,config:t}).then((e=>new o(e,s)))}addListener(e){this.listeners.push(e)}async send(e){let t;if("string"==typeof e)t={type:"Text",data:e};else if("object"==typeof e&&"type"in e)t=e;else{if(!Array.isArray(e))throw new Error("invalid `message` type, expected a `{ type: string, data: any }` object, a string or a numeric array");t={type:"Binary",data:e}}await a("plugin:websocket|send",{id:this.id,message:t})}async disconnect(){await this.send({type:"Close",data:{code:1e3,reason:"Disconnected by client"}})}}return o}();Object.defineProperty(window.__TAURI__,"websocket",{value:__TAURI_PLUGIN_WEBSOCKET__})}
