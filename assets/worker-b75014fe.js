var Re=Object.defineProperty;var Se=(o,p,w)=>p in o?Re(o,p,{enumerable:!0,configurable:!0,writable:!0,value:w}):o[p]=w;var K=(o,p,w)=>(Se(o,typeof p!="symbol"?p+"":p,w),w);(function(){"use strict";let o;function p(n){const e=o.__externref_table_alloc();return o.__wbindgen_export_2.set(e,n),e}function w(n,e){try{return n.apply(this,e)}catch(t){const r=p(t);o.__wbindgen_exn_store(r)}}const j=typeof TextDecoder<"u"?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};typeof TextDecoder<"u"&&j.decode();let O=null;function A(){return(O===null||O.byteLength===0)&&(O=new Uint8Array(o.memory.buffer)),O}function f(n,e){return n=n>>>0,j.decode(A().subarray(n,n+e))}let g=0;const T=typeof TextEncoder<"u"?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}},Q=typeof T.encodeInto=="function"?function(n,e){return T.encodeInto(n,e)}:function(n,e){const t=T.encode(n);return e.set(t),{read:n.length,written:t.length}};function m(n,e,t){if(t===void 0){const c=T.encode(n),_=e(c.length,1)>>>0;return A().subarray(_,_+c.length).set(c),g=c.length,_}let r=n.length,i=e(r,1)>>>0;const a=A();let s=0;for(;s<r;s++){const c=n.charCodeAt(s);if(c>127)break;a[i+s]=c}if(s!==r){s!==0&&(n=n.slice(s)),i=t(i,r,r=s+n.length*3,1)>>>0;const c=A().subarray(i+s,i+r),_=Q(n,c);s+=_.written,i=t(i,r,s,1)>>>0}return g=s,i}let S=null;function R(){return(S===null||S.buffer.detached===!0||S.buffer.detached===void 0&&S.buffer!==o.memory.buffer)&&(S=new DataView(o.memory.buffer)),S}function J(n){const e=typeof n;if(e=="number"||e=="boolean"||n==null)return`${n}`;if(e=="string")return`"${n}"`;if(e=="symbol"){const i=n.description;return i==null?"Symbol":`Symbol(${i})`}if(e=="function"){const i=n.name;return typeof i=="string"&&i.length>0?`Function(${i})`:"Function"}if(Array.isArray(n)){const i=n.length;let a="[";i>0&&(a+=J(n[0]));for(let s=1;s<i;s++)a+=", "+J(n[s]);return a+="]",a}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let r;if(t&&t.length>1)r=t[1];else return toString.call(n);if(r=="Object")try{return"Object("+JSON.stringify(n)+")"}catch{return"Object"}return n instanceof Error?`${n.name}: ${n.message}
${n.stack}`:r}function y(n){return n==null}function ee(n,e){const t=e(n.length*1,1)>>>0;return A().set(n,t/1),g=n.length,t}function u(n){const e=o.__wbindgen_export_2.get(n);return o.__externref_table_dealloc(n),e}let x=null;function te(){return(x===null||x.byteLength===0)&&(x=new Float64Array(o.memory.buffer)),x}function z(n,e){return n=n>>>0,te().subarray(n/8,n/8+e)}let E=null;function ne(){return(E===null||E.byteLength===0)&&(E=new Uint32Array(o.memory.buffer)),E}function U(n,e){const t=e(n.length*4,4)>>>0;return ne().set(n,t/4),g=n.length,t}const L=typeof FinalizationRegistry>"u"?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(n=>o.__wbg_mapmodel_free(n>>>0,1));class re{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,L.unregister(this),e}free(){const e=this.__destroy_into_raw();o.__wbg_mapmodel_free(e,0)}constructor(e){const t=ee(e,o.__wbindgen_malloc),r=g,i=o.mapmodel_new(t,r);if(i[2])throw u(i[1]);return this.__wbg_ptr=i[0]>>>0,L.register(this,this.__wbg_ptr,this),this}renderStaticRoads(){let e,t;try{const a=o.mapmodel_renderStaticRoads(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}renderDynamicRoads(){let e,t;try{const a=o.mapmodel_renderDynamicRoads(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}getInvertedBoundaryInsideSettlements(){let e,t;try{const a=o.mapmodel_getInvertedBoundaryInsideSettlements(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}getInvertedBoundaryOutsideSettlements(){let e,t;try{const a=o.mapmodel_getInvertedBoundaryOutsideSettlements(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}getInvertedBoundaryForStudyArea(){let e,t;try{const a=o.mapmodel_getInvertedBoundaryForStudyArea(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}getBounds(){const e=o.mapmodel_getBounds(this.__wbg_ptr);var t=z(e[0],e[1]).slice();return o.__wbindgen_free(e[0],e[1]*8,8),t}setRoute(e,t){const r=o.mapmodel_setRoute(this.__wbg_ptr,y(e)?4294967297:e>>>0,t);if(r[1])throw u(r[0])}deleteRoutes(e){const t=U(e,o.__wbindgen_malloc),r=g,i=o.mapmodel_deleteRoutes(this.__wbg_ptr,t,r);if(i[1])throw u(i[0])}changeTier(e,t){const r=U(e,o.__wbindgen_malloc),i=g,a=m(t,o.__wbindgen_malloc,o.__wbindgen_realloc),s=g,c=o.mapmodel_changeTier(this.__wbg_ptr,r,i,a,s);if(c[1])throw u(c[0])}changeInfraType(e,t){const r=U(e,o.__wbindgen_malloc),i=g,a=m(t,o.__wbindgen_malloc,o.__wbindgen_realloc),s=g,c=o.mapmodel_changeInfraType(this.__wbg_ptr,r,i,a,s);if(c[1])throw u(c[0])}clearAllRoutes(){o.mapmodel_clearAllRoutes(this.__wbg_ptr)}autosplitRoute(e,t,r,i,a){let s,c;try{const b=m(i,o.__wbindgen_malloc,o.__wbindgen_realloc),h=g,d=o.mapmodel_autosplitRoute(this.__wbg_ptr,y(e)?4294967297:e>>>0,t,r,b,h,!y(a),y(a)?0:a);var _=d[0],l=d[1];if(d[3])throw _=0,l=0,u(d[2]);return s=_,c=l,f(_,l)}finally{o.__wbindgen_free(s,c,1)}}snapPoint(e,t,r){const i=o.mapmodel_snapPoint(this.__wbg_ptr,e,t,!y(r),y(r)?0:r);var a=z(i[0],i[1]).slice();return o.__wbindgen_free(i[0],i[1]*8,8),a}getAllRoutes(){let e,t;try{const a=o.mapmodel_getAllRoutes(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}getRoute(e){let t,r;try{const s=o.mapmodel_getRoute(this.__wbg_ptr,e);var i=s[0],a=s[1];if(s[3])throw i=0,a=0,u(s[2]);return t=i,r=a,f(i,a)}finally{o.__wbindgen_free(t,r,1)}}evaluateRoute(e){let t,r;try{const s=o.mapmodel_evaluateRoute(this.__wbg_ptr,e);var i=s[0],a=s[1];if(s[3])throw i=0,a=0,u(s[2]);return t=i,r=a,f(i,a)}finally{o.__wbindgen_free(t,r,1)}}debugReachablePath(e,t){let r,i;try{const c=m(e,o.__wbindgen_malloc,o.__wbindgen_realloc),_=g,l=o.mapmodel_debugReachablePath(this.__wbg_ptr,c,_,t);var a=l[0],s=l[1];if(l[3])throw a=0,s=0,u(l[2]);return r=a,i=s,f(a,s)}finally{o.__wbindgen_free(r,i,1)}}debugUnreachablePath(e,t){let r,i;try{const c=m(e,o.__wbindgen_malloc,o.__wbindgen_realloc),_=g,l=o.mapmodel_debugUnreachablePath(this.__wbg_ptr,c,_,t);var a=l[0],s=l[1];if(l[3])throw a=0,s=0,u(l[2]);return r=a,i=s,f(a,s)}finally{o.__wbindgen_free(r,i,1)}}fixUnreachablePOI(e,t){let r,i;try{const c=m(e,o.__wbindgen_malloc,o.__wbindgen_realloc),_=g,l=o.mapmodel_fixUnreachablePOI(this.__wbg_ptr,c,_,t);var a=l[0],s=l[1];if(l[3])throw a=0,s=0,u(l[2]);return r=a,i=s,f(a,s)}finally{o.__wbindgen_free(r,i,1)}}evaluateOD(e){let t,r;try{const s=o.mapmodel_evaluateOD(this.__wbg_ptr,e);var i=s[0],a=s[1];if(s[3])throw i=0,a=0,u(s[2]);return t=i,r=a,f(i,a)}finally{o.__wbindgen_free(t,r,1)}}recalculateStats(){let e,t;try{const a=o.mapmodel_recalculateStats(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}recalculateSlowStats(){let e,t;try{const a=o.mapmodel_recalculateSlowStats(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}getBaselineStats(){let e,t;try{const a=o.mapmodel_getBaselineStats(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}recalculateODStats(){let e,t;try{const a=o.mapmodel_recalculateODStats(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}loadSavefile(e){const t=m(e,o.__wbindgen_malloc,o.__wbindgen_realloc),r=g,i=o.mapmodel_loadSavefile(this.__wbg_ptr,t,r);if(i[1])throw u(i[0])}getGridMeshDensity(e,t,r){let i,a;try{const _=o.mapmodel_getGridMeshDensity(this.__wbg_ptr,e,t,r);var s=_[0],c=_[1];if(_[3])throw s=0,c=0,u(_[2]);return i=s,a=c,f(s,c)}finally{o.__wbindgen_free(i,a,1)}}importExistingRoutes(e){return o.mapmodel_importExistingRoutes(this.__wbg_ptr,e)>>>0}importCoreNetwork(){return o.mapmodel_importCoreNetwork(this.__wbg_ptr)>>>0}getPOIs(){let e,t;try{const a=o.mapmodel_getPOIs(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}getGreenspaces(){let e,t;try{const a=o.mapmodel_getGreenspaces(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}getTownCentres(){let e,t;try{const a=o.mapmodel_getTownCentres(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}getSettlements(){let e,t;try{const a=o.mapmodel_getSettlements(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}getDataZones(){let e,t;try{const a=o.mapmodel_getDataZones(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}getConnectedComponents(){let e,t;try{const a=o.mapmodel_getConnectedComponents(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}getExtraNodes(e,t,r){let i,a;try{const _=o.mapmodel_getExtraNodes(this.__wbg_ptr,e,t,!y(r),y(r)?0:r);var s=_[0],c=_[1];if(_[3])throw s=0,c=0,u(_[2]);return i=s,a=c,f(s,c)}finally{o.__wbindgen_free(i,a,1)}}getMajorJunctions(){let e,t;try{const a=o.mapmodel_getMajorJunctions(this.__wbg_ptr);var r=a[0],i=a[1];if(a[3])throw r=0,i=0,u(a[2]);return e=r,t=i,f(r,i)}finally{o.__wbindgen_free(e,t,1)}}}async function ie(n,e){if(typeof Response=="function"&&n instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(n,e)}catch(r){if(n.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",r);else throw r}const t=await n.arrayBuffer();return await WebAssembly.instantiate(t,e)}else{const t=await WebAssembly.instantiate(n,e);return t instanceof WebAssembly.Instance?{instance:t,module:n}:t}}function ae(){const n={};return n.wbg={},n.wbg.__wbg_buffer_aa30bbb65cb44323=function(e){return e.buffer},n.wbg.__wbg_call_41c7efaf6b1182f8=function(){return w(function(e,t,r){return e.call(t,r)},arguments)},n.wbg.__wbg_call_c45d13337ffb12ac=function(){return w(function(e,t){return e.call(t)},arguments)},n.wbg.__wbg_debug_6747a9ad816638fa=function(e){console.debug(e)},n.wbg.__wbg_done_362f78ab584a24b5=function(e){return e.done},n.wbg.__wbg_entries_27a445ca6b702f8d=function(e){return Object.entries(e)},n.wbg.__wbg_error_4e9ba4ac0ad780bd=function(e){console.error(e)},n.wbg.__wbg_error_7534b8e9a36f1ab4=function(e,t){let r,i;try{r=e,i=t,console.error(f(e,t))}finally{o.__wbindgen_free(r,i,1)}},n.wbg.__wbg_get_01203e6a4116a116=function(e,t){return e[t>>>0]},n.wbg.__wbg_get_e7114b7bf3d9d5f5=function(){return w(function(e,t){return Reflect.get(e,t)},arguments)},n.wbg.__wbg_getwithrefkey_1dc361bd10053bfe=function(e,t){return e[t]},n.wbg.__wbg_globalThis_856ff24a65e13540=function(){return w(function(){return globalThis.globalThis},arguments)},n.wbg.__wbg_global_fc813a897a497d26=function(){return w(function(){return global.global},arguments)},n.wbg.__wbg_info_352d63effc192484=function(e){console.info(e)},n.wbg.__wbg_instanceof_ArrayBuffer_8b96bf6c71691dc9=function(e){let t;try{t=e instanceof ArrayBuffer}catch{t=!1}return t},n.wbg.__wbg_instanceof_Uint8Array_faa8901ba56cb8e9=function(e){let t;try{t=e instanceof Uint8Array}catch{t=!1}return t},n.wbg.__wbg_isArray_6836d46c89daf1b6=function(e){return Array.isArray(e)},n.wbg.__wbg_iterator_773e0b022e7009f4=function(){return Symbol.iterator},n.wbg.__wbg_length_0a11127664108286=function(e){return e.length},n.wbg.__wbg_length_9aaa2867670f533a=function(e){return e.length},n.wbg.__wbg_log_0c7c294ecbc8af77=function(e){console.log(e)},n.wbg.__wbg_new_8a6f238a6ece86ea=function(){return new Error},n.wbg.__wbg_new_db41cf29086ce106=function(e){return new Uint8Array(e)},n.wbg.__wbg_newnoargs_29f93ce2db72cd07=function(e,t){return new Function(f(e,t))},n.wbg.__wbg_next_95ee887e1f50209d=function(){return w(function(e){return e.next()},arguments)},n.wbg.__wbg_next_b2690a2dab163f0f=function(e){return e.next},n.wbg.__wbg_now_2c95c9de01293173=function(e){return e.now()},n.wbg.__wbg_performance_7a3ffd0b17f663ad=function(e){return e.performance},n.wbg.__wbg_self_799f153b0b6e0183=function(){return w(function(){return self.self},arguments)},n.wbg.__wbg_set_e97d203fd145cdae=function(e,t,r){e.set(t,r>>>0)},n.wbg.__wbg_stack_0ed75d68575b0f3c=function(e,t){const r=t.stack,i=m(r,o.__wbindgen_malloc,o.__wbindgen_realloc),a=g;R().setInt32(e+4*1,a,!0),R().setInt32(e+4*0,i,!0)},n.wbg.__wbg_value_87c720f6568103d1=function(e){return e.value},n.wbg.__wbg_warn_e02514c3c4a0df8e=function(e){console.warn(e)},n.wbg.__wbg_window_cd65fa4478648b49=function(){return w(function(){return window.window},arguments)},n.wbg.__wbindgen_boolean_get=function(e){const t=e;return typeof t=="boolean"?t?1:0:2},n.wbg.__wbindgen_debug_string=function(e,t){const r=J(t),i=m(r,o.__wbindgen_malloc,o.__wbindgen_realloc),a=g;R().setInt32(e+4*1,a,!0),R().setInt32(e+4*0,i,!0)},n.wbg.__wbindgen_error_new=function(e,t){return new Error(f(e,t))},n.wbg.__wbindgen_in=function(e,t){return e in t},n.wbg.__wbindgen_init_externref_table=function(){const e=o.__wbindgen_export_2,t=e.grow(4);e.set(0,void 0),e.set(t+0,void 0),e.set(t+1,null),e.set(t+2,!0),e.set(t+3,!1)},n.wbg.__wbindgen_is_function=function(e){return typeof e=="function"},n.wbg.__wbindgen_is_object=function(e){const t=e;return typeof t=="object"&&t!==null},n.wbg.__wbindgen_is_string=function(e){return typeof e=="string"},n.wbg.__wbindgen_is_undefined=function(e){return e===void 0},n.wbg.__wbindgen_jsval_loose_eq=function(e,t){return e==t},n.wbg.__wbindgen_memory=function(){return o.memory},n.wbg.__wbindgen_number_get=function(e,t){const r=t,i=typeof r=="number"?r:void 0;R().setFloat64(e+8*1,y(i)?0:i,!0),R().setInt32(e+4*0,!y(i),!0)},n.wbg.__wbindgen_string_get=function(e,t){const r=t,i=typeof r=="string"?r:void 0;var a=y(i)?0:m(i,o.__wbindgen_malloc,o.__wbindgen_realloc),s=g;R().setInt32(e+4*1,s,!0),R().setInt32(e+4*0,a,!0)},n.wbg.__wbindgen_string_new=function(e,t){return f(e,t)},n.wbg.__wbindgen_throw=function(e,t){throw new Error(f(e,t))},n}function oe(n,e){return o=n.exports,G.__wbindgen_wasm_module=e,S=null,x=null,E=null,O=null,o.__wbindgen_start(),o}async function G(n){if(o!==void 0)return o;typeof n<"u"&&(Object.getPrototypeOf(n)===Object.prototype?{module_or_path:n}=n:console.warn("using deprecated parameters for the initialization function; pass a single object instead")),typeof n>"u"&&(n=new URL("/npw/assets/backend_bg-823e5265.wasm",self.location));const e=ae();(typeof n=="string"||typeof Request=="function"&&n instanceof Request||typeof URL=="function"&&n instanceof URL)&&(n=fetch(n));const{instance:t,module:r}=await ie(await n,e);return oe(t,r)}/**
 * @license
 * Copyright 2019 Google LLC
 * SPDX-License-Identifier: Apache-2.0
 */const V=Symbol("Comlink.proxy"),se=Symbol("Comlink.endpoint"),ce=Symbol("Comlink.releaseProxy"),B=Symbol("Comlink.finalizer"),I=Symbol("Comlink.thrown"),$=n=>typeof n=="object"&&n!==null||typeof n=="function",_e={canHandle:n=>$(n)&&n[V],serialize(n){const{port1:e,port2:t}=new MessageChannel;return F(n,e),[t,[t]]},deserialize(n){return n.start(),de(n)}},le={canHandle:n=>$(n)&&I in n,serialize({value:n}){let e;return n instanceof Error?e={isError:!0,value:{message:n.message,name:n.name,stack:n.stack}}:e={isError:!1,value:n},[e,[]]},deserialize(n){throw n.isError?Object.assign(new Error(n.value.message),n.value):n.value}},H=new Map([["proxy",_e],["throw",le]]);function ue(n,e){for(const t of n)if(e===t||t==="*"||t instanceof RegExp&&t.test(e))return!0;return!1}function F(n,e=globalThis,t=["*"]){e.addEventListener("message",function r(i){if(!i||!i.data)return;if(!ue(t,i.origin)){console.warn(`Invalid origin '${i.origin}' for comlink proxy`);return}const{id:a,type:s,path:c}=Object.assign({path:[]},i.data),_=(i.data.argumentList||[]).map(v);let l;try{const b=c.slice(0,-1).reduce((d,N)=>d[N],n),h=c.reduce((d,N)=>d[N],n);switch(s){case"GET":l=h;break;case"SET":b[c.slice(-1)[0]]=v(i.data.value),l=!0;break;case"APPLY":l=h.apply(b,_);break;case"CONSTRUCT":{const d=new h(..._);l=ye(d)}break;case"ENDPOINT":{const{port1:d,port2:N}=new MessageChannel;F(n,N),l=he(d,[d])}break;case"RELEASE":l=void 0;break;default:return}}catch(b){l={value:b,[I]:0}}Promise.resolve(l).catch(b=>({value:b,[I]:0})).then(b=>{const[h,d]=D(b);e.postMessage(Object.assign(Object.assign({},h),{id:a}),d),s==="RELEASE"&&(e.removeEventListener("message",r),Z(e),B in n&&typeof n[B]=="function"&&n[B]())}).catch(b=>{const[h,d]=D({value:new TypeError("Unserializable return value"),[I]:0});e.postMessage(Object.assign(Object.assign({},h),{id:a}),d)})}),e.start&&e.start()}function fe(n){return n.constructor.name==="MessagePort"}function Z(n){fe(n)&&n.close()}function de(n,e){const t=new Map;return n.addEventListener("message",function(i){const{data:a}=i;if(!a||!a.id)return;const s=t.get(a.id);if(s)try{s(a)}finally{t.delete(a.id)}}),W(n,t,[],e)}function M(n){if(n)throw new Error("Proxy has been released and is not useable")}function Y(n){return k(n,new Map,{type:"RELEASE"}).then(()=>{Z(n)})}const P=new WeakMap,C="FinalizationRegistry"in globalThis&&new FinalizationRegistry(n=>{const e=(P.get(n)||0)-1;P.set(n,e),e===0&&Y(n)});function ge(n,e){const t=(P.get(e)||0)+1;P.set(e,t),C&&C.register(n,e,n)}function be(n){C&&C.unregister(n)}function W(n,e,t=[],r=function(){}){let i=!1;const a=new Proxy(r,{get(s,c){if(M(i),c===ce)return()=>{be(a),Y(n),e.clear(),i=!0};if(c==="then"){if(t.length===0)return{then:()=>a};const _=k(n,e,{type:"GET",path:t.map(l=>l.toString())}).then(v);return _.then.bind(_)}return W(n,e,[...t,c])},set(s,c,_){M(i);const[l,b]=D(_);return k(n,e,{type:"SET",path:[...t,c].map(h=>h.toString()),value:l},b).then(v)},apply(s,c,_){M(i);const l=t[t.length-1];if(l===se)return k(n,e,{type:"ENDPOINT"}).then(v);if(l==="bind")return W(n,e,t.slice(0,-1));const[b,h]=X(_);return k(n,e,{type:"APPLY",path:t.map(d=>d.toString()),argumentList:b},h).then(v)},construct(s,c){M(i);const[_,l]=X(c);return k(n,e,{type:"CONSTRUCT",path:t.map(b=>b.toString()),argumentList:_},l).then(v)}});return ge(a,n),a}function we(n){return Array.prototype.concat.apply([],n)}function X(n){const e=n.map(D);return[e.map(t=>t[0]),we(e.map(t=>t[1]))]}const q=new WeakMap;function he(n,e){return q.set(n,e),n}function ye(n){return Object.assign(n,{[V]:!0})}function D(n){for(const[e,t]of H)if(t.canHandle(n)){const[r,i]=t.serialize(n);return[{type:"HANDLER",name:e,value:r},i]}return[{type:"RAW",value:n},q.get(n)||[]]}function v(n){switch(n.type){case"HANDLER":return H.get(n.name).deserialize(n.value);case"RAW":return n.value}}function k(n,e,t,r){return new Promise(i=>{const a=me();e.set(a,i),n.start&&n.start(),n.postMessage(Object.assign({id:a},t),r)})}function me(){return new Array(4).fill(0).map(()=>Math.floor(Math.random()*Number.MAX_SAFE_INTEGER).toString(16)).join("-")}class pe{constructor(){K(this,"inner");this.inner=null}async loadFile(e){await G(),this.inner=new re(e)}getBounds(){return this.checkReady(),Array.from(this.inner.getBounds())}getInvertedBoundaryForStudyArea(){return this.checkReady(),JSON.parse(this.inner.getInvertedBoundaryForStudyArea())}getInvertedBoundaryInsideSettlements(){return this.checkReady(),JSON.parse(this.inner.getInvertedBoundaryInsideSettlements())}getInvertedBoundaryOutsideSettlements(){return this.checkReady(),JSON.parse(this.inner.getInvertedBoundaryOutsideSettlements())}renderStaticRoads(){return this.checkReady(),JSON.parse(this.inner.renderStaticRoads())}renderDynamicRoads(){return this.checkReady(),JSON.parse(this.inner.renderDynamicRoads())}getAllRoutes(){return this.checkReady(),JSON.parse(this.inner.getAllRoutes())}getRoute(e){return this.checkReady(),JSON.parse(this.inner.getRoute(e))}setRoute(e,t){this.checkReady(),this.inner.setRoute(e??void 0,t)}deleteRoutes(e){this.checkReady(),this.inner.deleteRoutes(new Uint32Array(e))}changeTier(e,t){this.checkReady(),this.inner.changeTier(new Uint32Array(e),`"${t}"`)}changeInfraType(e,t){this.checkReady(),this.inner.changeInfraType(new Uint32Array(e),`"${t}"`)}clearAllRoutes(){this.checkReady(),this.inner.clearAllRoutes()}autosplitRoute(e,t,r,i,a){return this.checkReady(),JSON.parse(this.inner.autosplitRoute(e??void 0,t,r,`"${i}"`,a))}evaluateRoute(e){return this.checkReady(),JSON.parse(this.inner.evaluateRoute({x1:e.start.lng,y1:e.start.lat,x2:e.end[0],y2:e.end[1],breakdown:e.breakdown}))}evaluateOD(e){return this.checkReady(),JSON.parse(this.inner.evaluateOD(e))}recalculateStats(){return this.checkReady(),JSON.parse(this.inner.recalculateStats())}recalculateSlowStats(){return this.checkReady(),JSON.parse(this.inner.recalculateSlowStats())}getBaselineStats(){return this.checkReady(),JSON.parse(this.inner.getBaselineStats())}recalculateODStats(){return this.checkReady(),JSON.parse(this.inner.recalculateODStats())}getGridMeshDensity(e,t,r){return this.checkReady(),JSON.parse(this.inner.getGridMeshDensity(e,t,r))}importExistingRoutes(e){return this.checkReady(),this.inner.importExistingRoutes(e=="infra-type")}importCoreNetwork(){return this.checkReady(),this.inner.importCoreNetwork()}loadSavefile(e){this.checkReady(),this.inner.loadSavefile(e)}getPOIs(){return this.checkReady(),JSON.parse(this.inner.getPOIs())}getTownCentres(){return this.checkReady(),JSON.parse(this.inner.getTownCentres())}getSettlements(){return this.checkReady(),JSON.parse(this.inner.getSettlements())}getGreenspaces(){return this.checkReady(),JSON.parse(this.inner.getGreenspaces())}getDataZones(){return this.checkReady(),JSON.parse(this.inner.getDataZones())}debugReachablePath(e,t){return this.checkReady(),JSON.parse(this.inner.debugReachablePath(e,t))}debugUnreachablePath(e,t){return this.checkReady(),JSON.parse(this.inner.debugUnreachablePath(e,t))}fixUnreachablePOI(e,t){return this.checkReady(),JSON.parse(this.inner.fixUnreachablePOI(e,t))}getConnectedComponents(){return this.checkReady(),JSON.parse(this.inner.getConnectedComponents())}snapPoint(e,t){this.checkReady();let r=this.inner.snapPoint(e[0],e[1],t);return[r[0],r[1]]}getExtraNodes(e,t,r){return this.checkReady(),JSON.parse(this.inner.getExtraNodes(e,t,r))}getMajorJunctions(){return this.checkReady(),JSON.parse(this.inner.getMajorJunctions())}checkReady(){if(!this.inner)throw new Error("Backend used without a file loaded")}}F(pe)})();
