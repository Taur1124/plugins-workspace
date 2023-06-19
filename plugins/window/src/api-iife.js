<<<<<<< HEAD
if("__TAURI__"in window){var __TAURI_WINDOW__=function(e){"use strict";var n=Object.defineProperty,i=(e,i)=>{for(var t in i)n(e,t,{get:i[t],enumerable:!0})},t=(e,n,i)=>{if(!n.has(e))throw TypeError("Cannot "+i)},l=(e,n,i)=>(t(e,n,"read from private field"),i?i.call(e):n.get(e)),a=(e,n,i,l)=>(t(e,n,"write to private field"),l?l.call(e,i):n.set(e,i),i);function s(e,n=!1){let i=window.crypto.getRandomValues(new Uint32Array(1))[0],t=`_${i}`;return Object.defineProperty(window,t,{value:i=>(n&&Reflect.deleteProperty(window,t),e?.(i)),writable:!1,configurable:!0}),i}i({},{Channel:()=>o,PluginListener:()=>_,addPluginListener:()=>w,convertFileSrc:()=>c,invoke:()=>u,transformCallback:()=>s});var r,o=class{constructor(){this.__TAURI_CHANNEL_MARKER__=!0,((e,n,i)=>{if(n.has(e))throw TypeError("Cannot add the same private member more than once");n instanceof WeakSet?n.add(e):n.set(e,i)})(this,r,(()=>{})),this.id=s((e=>{l(this,r).call(this,e)}))}set onmessage(e){a(this,r,e)}get onmessage(){return l(this,r)}toJSON(){return`__CHANNEL__:${this.id}`}};r=new WeakMap;var _=class{constructor(e,n,i){this.plugin=e,this.event=n,this.channelId=i}async unregister(){return u(`plugin:${this.plugin}|remove_listener`,{event:this.event,channelId:this.channelId})}};async function w(e,n,i){let t=new o;return t.onmessage=i,u(`plugin:${e}|register_listener`,{event:n,handler:t}).then((()=>new _(e,n,t.id)))}async function u(e,n={}){return new Promise(((i,t)=>{let l=s((e=>{i(e),Reflect.deleteProperty(window,`_${a}`)}),!0),a=s((e=>{t(e),Reflect.deleteProperty(window,`_${l}`)}),!0);window.__TAURI_IPC__({cmd:e,callback:l,error:a,...n})}))}function c(e,n="asset"){let i=encodeURIComponent(e);return navigator.userAgent.includes("Windows")?`https://${n}.localhost/${i}`:`${n}://localhost/${i}`}async function d(e,n){await u("plugin:event|unlisten",{event:e,eventId:n})}async function h(e,n,i){return u("plugin:event|listen",{event:e,windowLabel:n,handler:s(i)}).then((n=>async()=>d(e,n)))}i({},{TauriEvent:()=>I,emit:()=>E,listen:()=>b,once:()=>g});var y,p,I=((y=I||{}).WINDOW_RESIZED="tauri://resize",y.WINDOW_MOVED="tauri://move",y.WINDOW_CLOSE_REQUESTED="tauri://close-requested",y.WINDOW_CREATED="tauri://window-created",y.WINDOW_DESTROYED="tauri://destroyed",y.WINDOW_FOCUS="tauri://focus",y.WINDOW_BLUR="tauri://blur",y.WINDOW_SCALE_FACTOR_CHANGED="tauri://scale-change",y.WINDOW_THEME_CHANGED="tauri://theme-changed",y.WINDOW_FILE_DROP="tauri://file-drop",y.WINDOW_FILE_DROP_HOVER="tauri://file-drop-hover",y.WINDOW_FILE_DROP_CANCELLED="tauri://file-drop-cancelled",y.MENU="tauri://menu",y);async function b(e,n){return h(e,null,n)}async function g(e,n){return async function(e,n,i){return h(e,n,(n=>{i(n),d(e,n.id).catch((()=>{}))}))}(e,null,n)}async function E(e,n){return async function(e,n,i){await u("plugin:event|emit",{event:e,windowLabel:n,payload:i})}(e,void 0,n)}async function O(e,n){await window.__TAURI_INVOKE__("plugin:event|unlisten",{event:e,eventId:n})}async function A(e,n,i){return window.__TAURI_INVOKE__("plugin:event|listen",{event:e,windowLabel:n,handler:window.__TAURI__.transformCallback(i)}).then((n=>async()=>O(e,n)))}class R{constructor(e,n){this.type="Logical",this.width=e,this.height=n}}class N{constructor(e,n){this.type="Physical",this.width=e,this.height=n}toLogical(e){return new R(this.width/e,this.height/e)}}class v{constructor(e,n){this.type="Logical",this.x=e,this.y=n}}class T{constructor(e,n){this.type="Physical",this.x=e,this.y=n}toLogical(e){return new v(this.x/e,this.y/e)}}e.UserAttentionType=void 0,(p=e.UserAttentionType||(e.UserAttentionType={}))[p.Critical=1]="Critical",p[p.Informational=2]="Informational";class m{constructor(e){this._preventDefault=!1,this.event=e.event,this.windowLabel=e.windowLabel,this.id=e.id}preventDefault(){this._preventDefault=!0}isPreventDefault(){return this._preventDefault}}function U(){return new V(window.__TAURI_METADATA__.__currentWindow.label,{skip:!0})}function f(){return window.__TAURI_METADATA__.__windows.map((e=>new V(e.label,{skip:!0})))}const D=["tauri://created","tauri://error"];class V{constructor(e,n={}){this.label=e,this.listeners=Object.create(null),(null==n?void 0:n.skip)||window.__TAURI_INVOKE__("plugin:window|create",{options:{...n,label:e}}).then((async()=>this.emit("tauri://created"))).catch((async e=>this.emit("tauri://error",e)))}static getByLabel(e){return f().some((n=>n.label===e))?new WebviewWindow(e,{skip:!0}):null}static getCurrent(){return U()}static getAll(){return f()}async listen(e,n){return this._handleTauriEvent(e,n)?Promise.resolve((()=>{const i=this.listeners[e];i.splice(i.indexOf(n),1)})):A(e,this.label,n)}async once(e,n){return this._handleTauriEvent(e,n)?Promise.resolve((()=>{const i=this.listeners[e];i.splice(i.indexOf(n),1)})):async function(e,n,i){return A(e,n,(n=>{i(n),O(e,n.id).catch((()=>{}))}))}(e,this.label,n)}async emit(e,n){if(D.includes(e)){for(const i of this.listeners[e]||[])i({event:e,id:-1,windowLabel:this.label,payload:n});return Promise.resolve()}return async function(e,n,i){await window.__TAURI_INVOKE__("plugin:event|emit",{event:e,windowLabel:n,payload:i})}(e,this.label,n)}_handleTauriEvent(e,n){return!!D.includes(e)&&(e in this.listeners?this.listeners[e].push(n):this.listeners[e]=[n],!0)}async scaleFactor(){return window.__TAURI_INVOKE__("plugin:window|scale_factor",{label:this.label})}async innerPosition(){return window.__TAURI_INVOKE__("plugin:window|inner_position",{label:this.label}).then((({x:e,y:n})=>new T(e,n)))}async outerPosition(){return window.__TAURI_INVOKE__("plugin:window|outer_position",{label:this.label}).then((({x:e,y:n})=>new T(e,n)))}async innerSize(){return window.__TAURI_INVOKE__("plugin:window|inner_size",{label:this.label}).then((({width:e,height:n})=>new N(e,n)))}async outerSize(){return window.__TAURI_INVOKE__("plugin:window|outer_size",{label:this.label}).then((({width:e,height:n})=>new N(e,n)))}async isFullscreen(){return window.__TAURI_INVOKE__("plugin:window|is_fullscreen",{label:this.label})}async isMinimized(){return window.__TAURI_INVOKE__("plugin:window|is_minimized",{label:this.label})}async isMaximized(){return window.__TAURI_INVOKE__("plugin:window|is_maximized",{label:this.label})}async isDecorated(){return window.__TAURI_INVOKE__("plugin:window|is_decorated",{label:this.label})}async isResizable(){return window.__TAURI_INVOKE__("plugin:window|is_resizable",{label:this.label})}async isVisible(){return window.__TAURI_INVOKE__("plugin:window|is_visible",{label:this.label})}async title(){return window.__TAURI_INVOKE__("plugin:window|title",{label:this.label})}async theme(){return window.__TAURI_INVOKE__("plugin:window|theme",{label:this.label})}async center(){return window.__TAURI_INVOKE__("plugin:window|center",{label:this.label})}async requestUserAttention(n){let i=null;return n&&(i=n===e.UserAttentionType.Critical?{type:"Critical"}:{type:"Informational"}),window.__TAURI_INVOKE__("plugin:window|request_user_attention",{label:this.label,value:i})}async setResizable(e){return window.__TAURI_INVOKE__("plugin:window|set_resizable",{label:this.label,value:e})}async setTitle(e){return window.__TAURI_INVOKE__("plugin:window|set_title",{label:this.label,value:e})}async maximize(){return window.__TAURI_INVOKE__("plugin:window|maximize",{label:this.label})}async unmaximize(){return window.__TAURI_INVOKE__("plugin:window|unmaximize",{label:this.label})}async toggleMaximize(){return window.__TAURI_INVOKE__("plugin:window|toggle_maximize",{label:this.label})}async minimize(){return window.__TAURI_INVOKE__("plugin:window|minimize",{label:this.label})}async unminimize(){return window.__TAURI_INVOKE__("plugin:window|unminimize",{label:this.label})}async show(){return window.__TAURI_INVOKE__("plugin:window|show",{label:this.label})}async hide(){return window.__TAURI_INVOKE__("plugin:window|hide",{label:this.label})}async close(){return window.__TAURI_INVOKE__("plugin:window|close",{label:this.label})}async setDecorations(e){return window.__TAURI_INVOKE__("plugin:window|set_decorations",{label:this.label,value:e})}async setShadow(e){return window.__TAURI_INVOKE__("plugin:window|set_shadow",{label:this.label,value:e})}async setAlwaysOnTop(e){return window.__TAURI_INVOKE__("plugin:window|set_always_on_top",{label:this.label,value:e})}async setContentProtected(e){return window.__TAURI_INVOKE__("plugin:window|set_content_protected",{label:this.label,value:e})}async setSize(e){if(!e||"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");return window.__TAURI_INVOKE__("plugin:window|set_size",{label:this.label,value:{type:e.type,data:{width:e.width,height:e.height}}})}async setMinSize(e){if(e&&"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");return window.__TAURI_INVOKE__("plugin:window|set_min_size",{label:this.label,value:e?{type:e.type,data:{width:e.width,height:e.height}}:null})}async setMaxSize(e){if(e&&"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");return window.__TAURI_INVOKE__("plugin:window|set_max_size",{label:this.label,value:e?{type:e.type,data:{width:e.width,height:e.height}}:null})}async setPosition(e){if(!e||"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `position` argument must be either a LogicalPosition or a PhysicalPosition instance");return window.__TAURI_INVOKE__("plugin:window|set_position",{label:this.label,value:{type:e.type,data:{x:e.x,y:e.y}}})}async setFullscreen(e){return window.__TAURI_INVOKE__("plugin:window|set_fullscreen",{label:this.label,value:e})}async setFocus(){return window.__TAURI_INVOKE__("plugin:window|set_focus",{label:this.label})}async setIcon(e){return window.__TAURI_INVOKE__("plugin:window|set_icon",{label:this.label,value:"string"==typeof e?e:Array.from(e)})}async setSkipTaskbar(e){return window.__TAURI_INVOKE__("plugin:window|set_skip_taskbar",{label:this.label,value:e})}async setCursorGrab(e){return window.__TAURI_INVOKE__("plugin:window|set_cursor_grab",{label:this.label,value:e})}async setCursorVisible(e){return window.__TAURI_INVOKE__("plugin:window|set_cursor_visible",{label:this.label,value:e})}async setCursorIcon(e){return window.__TAURI_INVOKE__("plugin:window|set_cursor_icon",{label:this.label,value:e})}async setCursorPosition(e){if(!e||"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `position` argument must be either a LogicalPosition or a PhysicalPosition instance");return window.__TAURI_INVOKE__("plugin:window|set_cursor_position",{label:this.label,value:{type:e.type,data:{x:e.x,y:e.y}}})}async setIgnoreCursorEvents(e){return window.__TAURI_INVOKE__("plugin:window|set_ignore_cursor_events",{label:this.label,value:e})}async startDragging(){return window.__TAURI_INVOKE__("plugin:window|start_dragging",{label:this.label})}async onResized(e){return this.listen(I.WINDOW_RESIZED,(n=>{n.payload=P(n.payload),e(n)}))}async onMoved(e){return this.listen(I.WINDOW_MOVED,(n=>{n.payload=K(n.payload),e(n)}))}async onCloseRequested(e){return this.listen(I.WINDOW_CLOSE_REQUESTED,(n=>{const i=new m(n);Promise.resolve(e(i)).then((()=>{if(!i.isPreventDefault())return this.close()}))}))}async onFocusChanged(e){const n=await this.listen(I.WINDOW_FOCUS,(n=>{e({...n,payload:!0})})),i=await this.listen(I.WINDOW_BLUR,(n=>{e({...n,payload:!1})}));return()=>{n(),i()}}async onScaleChanged(e){return this.listen(I.WINDOW_SCALE_FACTOR_CHANGED,e)}async onMenuClicked(e){return this.listen(I.MENU,e)}async onFileDropEvent(e){const n=await this.listen(I.WINDOW_FILE_DROP,(n=>{e({...n,payload:{type:"drop",paths:n.payload}})})),i=await this.listen(I.WINDOW_FILE_DROP_HOVER,(n=>{e({...n,payload:{type:"hover",paths:n.payload}})})),t=await this.listen(I.WINDOW_FILE_DROP_CANCELLED,(n=>{e({...n,payload:{type:"cancel"}})}));return()=>{n(),i(),t()}}async onThemeChanged(e){return this.listen(I.WINDOW_THEME_CHANGED,e)}}function W(e){return null===e?null:{name:e.name,scaleFactor:e.scaleFactor,position:K(e.position),size:P(e.size)}}function K(e){return new T(e.x,e.y)}function P(e){return new N(e.width,e.height)}return e.CloseRequestedEvent=m,e.LogicalPosition=v,e.LogicalSize=R,e.PhysicalPosition=T,e.PhysicalSize=N,e.availableMonitors=async function(){return window.__TAURI_INVOKE__("plugin:window|available_monitors").then((e=>e.map(W)))},e.currentMonitor=async function(){return window.__TAURI_INVOKE__("plugin:window|current_monitor").then(W)},e.getAll=f,e.getCurrent=U,e.primaryMonitor=async function(){return window.__TAURI_INVOKE__("plugin:window|primary_monitor").then(W)},e}({});Object.defineProperty(window.__TAURI__,"window",{value:__TAURI_WINDOW__})}
=======
if("__TAURI__"in window){var __TAURI_WINDOW__=function(e){"use strict";var i=Object.defineProperty,n=(e,n)=>{for(var t in n)i(e,t,{get:n[t],enumerable:!0})},t=(e,i,n)=>{if(!i.has(e))throw TypeError("Cannot "+n)},a=(e,i,n)=>(t(e,i,"read from private field"),n?n.call(e):i.get(e)),l=(e,i,n,a)=>(t(e,i,"write to private field"),a?a.call(e,n):i.set(e,n),n);function s(e,i=!1){let n=window.crypto.getRandomValues(new Uint32Array(1))[0],t=`_${n}`;return Object.defineProperty(window,t,{value:n=>(i&&Reflect.deleteProperty(window,t),e?.(n)),writable:!1,configurable:!0}),n}n({},{Channel:()=>o,PluginListener:()=>_,addPluginListener:()=>w,convertFileSrc:()=>c,invoke:()=>u,transformCallback:()=>s});var r,o=class{constructor(){this.__TAURI_CHANNEL_MARKER__=!0,((e,i,n)=>{if(i.has(e))throw TypeError("Cannot add the same private member more than once");i instanceof WeakSet?i.add(e):i.set(e,n)})(this,r,(()=>{})),this.id=s((e=>{a(this,r).call(this,e)}))}set onmessage(e){l(this,r,e)}get onmessage(){return a(this,r)}toJSON(){return`__CHANNEL__:${this.id}`}};r=new WeakMap;var _=class{constructor(e,i,n){this.plugin=e,this.event=i,this.channelId=n}async unregister(){return u(`plugin:${this.plugin}|remove_listener`,{event:this.event,channelId:this.channelId})}};async function w(e,i,n){let t=new o;return t.onmessage=n,u(`plugin:${e}|register_listener`,{event:i,handler:t}).then((()=>new _(e,i,t.id)))}async function u(e,i={}){return new Promise(((n,t)=>{let a=s((e=>{n(e),Reflect.deleteProperty(window,`_${l}`)}),!0),l=s((e=>{t(e),Reflect.deleteProperty(window,`_${a}`)}),!0);window.__TAURI_IPC__({cmd:e,callback:a,error:l,...i})}))}function c(e,i="asset"){let n=encodeURIComponent(e);return navigator.userAgent.includes("Windows")?`https://${i}.localhost/${n}`:`${i}://localhost/${n}`}n({},{TauriEvent:()=>h,emit:()=>y,listen:()=>b,once:()=>I});var d,h=(e=>(e.WINDOW_RESIZED="tauri://resize",e.WINDOW_MOVED="tauri://move",e.WINDOW_CLOSE_REQUESTED="tauri://close-requested",e.WINDOW_CREATED="tauri://window-created",e.WINDOW_DESTROYED="tauri://destroyed",e.WINDOW_FOCUS="tauri://focus",e.WINDOW_BLUR="tauri://blur",e.WINDOW_SCALE_FACTOR_CHANGED="tauri://scale-change",e.WINDOW_THEME_CHANGED="tauri://theme-changed",e.WINDOW_FILE_DROP="tauri://file-drop",e.WINDOW_FILE_DROP_HOVER="tauri://file-drop-hover",e.WINDOW_FILE_DROP_CANCELLED="tauri://file-drop-cancelled",e.MENU="tauri://menu",e))(h||{});async function p(e,i){await u("plugin:event|unlisten",{event:e,eventId:i})}async function b(e,i,n){return u("plugin:event|listen",{event:e,windowLabel:n?.target,handler:s(i)}).then((i=>async()=>p(e,i)))}async function I(e,i,n){return b(e,(n=>{i(n),p(e,n.id).catch((()=>{}))}),n)}async function y(e,i,n){await u("plugin:event|emit",{event:e,windowLabel:n?.target,payload:i})}class g{constructor(e,i){this.type="Logical",this.width=e,this.height=i}}class E{constructor(e,i){this.type="Physical",this.width=e,this.height=i}toLogical(e){return new g(this.width/e,this.height/e)}}class A{constructor(e,i){this.type="Logical",this.x=e,this.y=i}}class O{constructor(e,i){this.type="Physical",this.x=e,this.y=i}toLogical(e){return new A(this.x/e,this.y/e)}}function T(){return window.__TAURI_METADATA__.__windows.map((e=>new N(e.label,{skip:!0})))}e.UserAttentionType=void 0,(d=e.UserAttentionType||(e.UserAttentionType={}))[d.Critical=1]="Critical",d[d.Informational=2]="Informational";const m=["tauri://created","tauri://error"];class R{constructor(e){this.label=e,this.listeners=Object.create(null)}async listen(e,i){return this._handleTauriEvent(e,i)?Promise.resolve((()=>{const n=this.listeners[e];n.splice(n.indexOf(i),1)})):b(e,i,{target:this.label})}async once(e,i){return this._handleTauriEvent(e,i)?Promise.resolve((()=>{const n=this.listeners[e];n.splice(n.indexOf(i),1)})):I(e,i,{target:this.label})}async emit(e,i){if(m.includes(e)){for(const n of this.listeners[e]||[])n({event:e,id:-1,windowLabel:this.label,payload:i});return Promise.resolve()}return y(e,i,{target:this.label})}_handleTauriEvent(e,i){return!!m.includes(e)&&(e in this.listeners?this.listeners[e].push(i):this.listeners[e]=[i],!0)}}class v extends R{async scaleFactor(){return window.__TAURI_INVOKE__("plugin:window|scale_factor",{label:this.label})}async innerPosition(){return window.__TAURI_INVOKE__("plugin:window|inner_position",{label:this.label}).then((({x:e,y:i})=>new O(e,i)))}async outerPosition(){return window.__TAURI_INVOKE__("plugin:window|outer_position",{label:this.label}).then((({x:e,y:i})=>new O(e,i)))}async innerSize(){return window.__TAURI_INVOKE__("plugin:window|inner_size",{label:this.label}).then((({width:e,height:i})=>new E(e,i)))}async outerSize(){return window.__TAURI_INVOKE__("plugin:window|outer_size",{label:this.label}).then((({width:e,height:i})=>new E(e,i)))}async isFullscreen(){return window.__TAURI_INVOKE__("plugin:window|is_fullscreen",{label:this.label})}async isMinimized(){return window.__TAURI_INVOKE__("plugin:window|is_minimized",{label:this.label})}async isMaximized(){return window.__TAURI_INVOKE__("plugin:window|is_maximized",{label:this.label})}async isFocused(){return window.__TAURI_INVOKE__("plugin:window|is_focused",{label:this.label})}async isDecorated(){return window.__TAURI_INVOKE__("plugin:window|is_decorated",{label:this.label})}async isResizable(){return window.__TAURI_INVOKE__("plugin:window|is_resizable",{label:this.label})}async isMaximizable(){return window.__TAURI_INVOKE__("plugin:window|is_maximizable",{label:this.label})}async isMinimizable(){return window.__TAURI_INVOKE__("plugin:window|is_minimizable",{label:this.label})}async isClosable(){return window.__TAURI_INVOKE__("plugin:window|is_closable",{label:this.label})}async isVisible(){return window.__TAURI_INVOKE__("plugin:window|is_visible",{label:this.label})}async title(){return window.__TAURI_INVOKE__("plugin:window|title",{label:this.label})}async theme(){return window.__TAURI_INVOKE__("plugin:window|theme",{label:this.label})}async center(){return window.__TAURI_INVOKE__("plugin:window|center",{label:this.label})}async requestUserAttention(i){let n=null;return i&&(n=i===e.UserAttentionType.Critical?{type:"Critical"}:{type:"Informational"}),window.__TAURI_INVOKE__("plugin:window|request_user_attention",{label:this.label,value:n})}async setResizable(e){return window.__TAURI_INVOKE__("plugin:window|set_resizable",{label:this.label,value:e})}async setMaximizable(e){return window.__TAURI_INVOKE__("plugin:window|set_maximizable",{label:this.label,value:e})}async setMinimizable(e){return window.__TAURI_INVOKE__("plugin:window|set_minimizable",{label:this.label,value:e})}async setClosable(e){return window.__TAURI_INVOKE__("plugin:window|set_closable",{label:this.label,value:e})}async setTitle(e){return window.__TAURI_INVOKE__("plugin:window|set_title",{label:this.label,value:e})}async maximize(){return window.__TAURI_INVOKE__("plugin:window|maximize",{label:this.label})}async unmaximize(){return window.__TAURI_INVOKE__("plugin:window|unmaximize",{label:this.label})}async toggleMaximize(){return window.__TAURI_INVOKE__("plugin:window|toggle_maximize",{label:this.label})}async minimize(){return window.__TAURI_INVOKE__("plugin:window|minimize",{label:this.label})}async unminimize(){return window.__TAURI_INVOKE__("plugin:window|unminimize",{label:this.label})}async show(){return window.__TAURI_INVOKE__("plugin:window|show",{label:this.label})}async hide(){return window.__TAURI_INVOKE__("plugin:window|hide",{label:this.label})}async close(){return window.__TAURI_INVOKE__("plugin:window|close",{label:this.label})}async setDecorations(e){return window.__TAURI_INVOKE__("plugin:window|set_decorations",{label:this.label,value:e})}async setShadow(e){return window.__TAURI_INVOKE__("plugin:window|set_shadow",{label:this.label,value:e})}async setEffects(e){return window.__TAURI_INVOKE__("plugin:window|set_effects",{label:this.label,value:e})}async clearEffects(){return window.__TAURI_INVOKE__("plugin:window|set_effects",{label:this.label,value:null})}async setAlwaysOnTop(e){return window.__TAURI_INVOKE__("plugin:window|set_always_on_top",{label:this.label,value:e})}async setContentProtected(e){return window.__TAURI_INVOKE__("plugin:window|set_content_protected",{label:this.label,value:e})}async setSize(e){if(!e||"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");return window.__TAURI_INVOKE__("plugin:window|set_size",{label:this.label,value:{type:e.type,data:{width:e.width,height:e.height}}})}async setMinSize(e){if(e&&"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");return window.__TAURI_INVOKE__("plugin:window|set_min_size",{label:this.label,value:e?{type:e.type,data:{width:e.width,height:e.height}}:null})}async setMaxSize(e){if(e&&"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");return window.__TAURI_INVOKE__("plugin:window|set_max_size",{label:this.label,value:e?{type:e.type,data:{width:e.width,height:e.height}}:null})}async setPosition(e){if(!e||"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `position` argument must be either a LogicalPosition or a PhysicalPosition instance");return window.__TAURI_INVOKE__("plugin:window|set_position",{label:this.label,value:{type:e.type,data:{x:e.x,y:e.y}}})}async setFullscreen(e){return window.__TAURI_INVOKE__("plugin:window|set_fullscreen",{label:this.label,value:e})}async setFocus(){return window.__TAURI_INVOKE__("plugin:window|set_focus",{label:this.label})}async setIcon(e){return window.__TAURI_INVOKE__("plugin:window|set_icon",{label:this.label,value:"string"==typeof e?e:Array.from(e)})}async setSkipTaskbar(e){return window.__TAURI_INVOKE__("plugin:window|set_skip_taskbar",{label:this.label,value:e})}async setCursorGrab(e){return window.__TAURI_INVOKE__("plugin:window|set_cursor_grab",{label:this.label,value:e})}async setCursorVisible(e){return window.__TAURI_INVOKE__("plugin:window|set_cursor_visible",{label:this.label,value:e})}async setCursorIcon(e){return window.__TAURI_INVOKE__("plugin:window|set_cursor_icon",{label:this.label,value:e})}async setCursorPosition(e){if(!e||"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `position` argument must be either a LogicalPosition or a PhysicalPosition instance");return window.__TAURI_INVOKE__("plugin:window|set_cursor_position",{label:this.label,value:{type:e.type,data:{x:e.x,y:e.y}}})}async setIgnoreCursorEvents(e){return window.__TAURI_INVOKE__("plugin:window|set_ignore_cursor_events",{label:this.label,value:e})}async startDragging(){return window.__TAURI_INVOKE__("plugin:window|start_dragging",{label:this.label})}async onResized(e){return this.listen(h.WINDOW_RESIZED,(i=>{i.payload=K(i.payload),e(i)}))}async onMoved(e){return this.listen(h.WINDOW_MOVED,(i=>{i.payload=D(i.payload),e(i)}))}async onCloseRequested(e){return this.listen(h.WINDOW_CLOSE_REQUESTED,(i=>{const n=new f(i);Promise.resolve(e(n)).then((()=>{if(!n.isPreventDefault())return this.close()}))}))}async onFocusChanged(e){const i=await this.listen(h.WINDOW_FOCUS,(i=>{e({...i,payload:!0})})),n=await this.listen(h.WINDOW_BLUR,(i=>{e({...i,payload:!1})}));return()=>{i(),n()}}async onScaleChanged(e){return this.listen(h.WINDOW_SCALE_FACTOR_CHANGED,e)}async onMenuClicked(e){return this.listen(h.MENU,e)}async onFileDropEvent(e){const i=await this.listen(h.WINDOW_FILE_DROP,(i=>{e({...i,payload:{type:"drop",paths:i.payload}})})),n=await this.listen(h.WINDOW_FILE_DROP_HOVER,(i=>{e({...i,payload:{type:"hover",paths:i.payload}})})),t=await this.listen(h.WINDOW_FILE_DROP_CANCELLED,(i=>{e({...i,payload:{type:"cancel"}})}));return()=>{i(),n(),t()}}async onThemeChanged(e){return this.listen(h.WINDOW_THEME_CHANGED,e)}}class f{constructor(e){this._preventDefault=!1,this.event=e.event,this.windowLabel=e.windowLabel,this.id=e.id}preventDefault(){this._preventDefault=!0}isPreventDefault(){return this._preventDefault}}class N extends v{constructor(e,i={}){super(e),(null==i?void 0:i.skip)||window.__TAURI_INVOKE__("plugin:window|create",{options:{...i,label:e}}).then((async()=>this.emit("tauri://created"))).catch((async e=>this.emit("tauri://error",e)))}static getByLabel(e){return T().some((i=>i.label===e))?new N(e,{skip:!0}):null}static async getFocusedWindow(){for(const e of T())if(await e.isFocused())return e;return null}}var U,W;function V(e){return null===e?null:{name:e.name,scaleFactor:e.scaleFactor,position:D(e.position),size:K(e.size)}}function D(e){return new O(e.x,e.y)}function K(e){return new E(e.width,e.height)}return e.appWindow=void 0,"__TAURI_METADATA__"in window?e.appWindow=new N(window.__TAURI_METADATA__.__currentWindow.label,{skip:!0}):(console.warn('Could not find "window.__TAURI_METADATA__". The "appWindow" value will reference the "main" window label.\nNote that this is not an issue if running this frontend on a browser instead of a Tauri window.'),e.appWindow=new N("main",{skip:!0})),e.Effect=void 0,(U=e.Effect||(e.Effect={})).AppearanceBased="appearanceBased",U.Light="light",U.Dark="dark",U.MediumLight="mediumLight",U.UltraDark="ultraDark",U.Titlebar="titlebar",U.Selection="selection",U.Menu="menu",U.Popover="popover",U.Sidebar="sidebar",U.HeaderView="headerView",U.Sheet="sheet",U.WindowBackground="windowBackground",U.HudWindow="hudWindow",U.FullScreenUI="fullScreenUI",U.Tooltip="tooltip",U.ContentBackground="contentBackground",U.UnderWindowBackground="underWindowBackground",U.UnderPageBackground="underPageBackground",U.Mica="mica",U.Blur="blur",U.Acrylic="acrylic",e.EffectState=void 0,(W=e.EffectState||(e.EffectState={})).FollowsWindowActiveState="followsWindowActiveState",W.Active="active",W.Inactive="inactive",e.CloseRequestedEvent=f,e.LogicalPosition=A,e.LogicalSize=g,e.PhysicalPosition=O,e.PhysicalSize=E,e.WebviewWindow=N,e.WebviewWindowHandle=R,e.WindowManager=v,e.availableMonitors=async function(){return window.__TAURI_INVOKE__("plugin:window|available_monitors").then((e=>e.map(V)))},e.currentMonitor=async function(){return window.__TAURI_INVOKE__("plugin:window|current_monitor").then(V)},e.getAll=T,e.getCurrent=function(){return new N(window.__TAURI_METADATA__.__currentWindow.label,{skip:!0})},e.primaryMonitor=async function(){return window.__TAURI_INVOKE__("plugin:window|primary_monitor").then(V)},e}({});Object.defineProperty(window.__TAURI__,"window",{value:__TAURI_WINDOW__})}
>>>>>>> v2
