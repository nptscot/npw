var et=Object.defineProperty;var nt=(t,e,n)=>e in t?et(t,e,{enumerable:!0,configurable:!0,writable:!0,value:n}):t[e]=n;var j=(t,e,n)=>(nt(t,typeof e!="symbol"?e+"":e,n),n);(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))s(o);new MutationObserver(o=>{for(const r of o)if(r.type==="childList")for(const i of r.addedNodes)i.tagName==="LINK"&&i.rel==="modulepreload"&&s(i)}).observe(document,{childList:!0,subtree:!0});function n(o){const r={};return o.integrity&&(r.integrity=o.integrity),o.referrerPolicy&&(r.referrerPolicy=o.referrerPolicy),o.crossOrigin==="use-credentials"?r.credentials="include":o.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function s(o){if(o.ep)return;o.ep=!0;const r=n(o);fetch(o.href,r)}})();function E(){}function st(t,e){for(const n in e)t[n]=e[n];return t}function mt(t){return!!t&&(typeof t=="object"||typeof t=="function")&&typeof t.then=="function"}function U(t){return t()}function K(){return Object.create(null)}function x(t){t.forEach(U)}function R(t){return typeof t=="function"}function wt(t,e){return t!=t?e==e:t!==e||t&&typeof t=="object"||typeof t=="function"}let L;function $t(t,e){return t===e?!0:(L||(L=document.createElement("a")),L.href=e,t===L.href)}function ot(t){return Object.keys(t).length===0}function V(t,...e){if(t==null){for(const s of e)s(void 0);return E}const n=t.subscribe(...e);return n.unsubscribe?()=>n.unsubscribe():n}function bt(t){let e;return V(t,n=>e=n)(),e}function xt(t,e,n){t.$$.on_destroy.push(V(e,n))}function vt(t,e,n,s){if(t){const o=Y(t,e,n,s);return t[0](o)}}function Y(t,e,n,s){return t[1]&&s?st(n.ctx.slice(),t[1](s(e))):n.ctx}function St(t,e,n,s){if(t[2]&&s){const o=t[2](s(n));if(e.dirty===void 0)return o;if(typeof o=="object"){const r=[],i=Math.max(e.dirty.length,o.length);for(let l=0;l<i;l+=1)r[l]=e.dirty[l]|o[l];return r}return e.dirty|o}return e.dirty}function Et(t,e,n,s,o,r){if(o){const i=Y(e,n,s,r);t.p(i,o)}}function Ot(t){if(t.ctx.length>32){const e=[],n=t.ctx.length/32;for(let s=0;s<n;s++)e[s]=-1;return e}return-1}function kt(t){const e={};for(const n in t)n[0]!=="$"&&(e[n]=t[n]);return e}function Pt(t){const e={};for(const n in t)e[n]=!0;return e}function Lt(t){return t??""}function Nt(t,e,n){return t.set(n),e}function It(t){return t&&R(t.destroy)?t.destroy:E}function At(t,e){t.appendChild(e)}function Ct(t,e,n){t.insertBefore(e,n||null)}function rt(t){t.parentNode&&t.parentNode.removeChild(t)}function Mt(t,e){for(let n=0;n<t.length;n+=1)t[n]&&t[n].d(e)}function jt(t){return document.createElement(t)}function qt(t){return document.createElementNS("http://www.w3.org/2000/svg",t)}function G(t){return document.createTextNode(t)}function Dt(){return G(" ")}function Ft(){return G("")}function Bt(t,e,n,s){return t.addEventListener(e,n,s),()=>t.removeEventListener(e,n,s)}function Rt(t){return function(e){return e.preventDefault(),t.call(this,e)}}function Wt(t){return function(e){return e.stopPropagation(),t.call(this,e)}}function it(t,e,n){n==null?t.removeAttribute(e):t.getAttribute(e)!==n&&t.setAttribute(e,n)}const ut=["width","height"];function zt(t,e){const n=Object.getOwnPropertyDescriptors(t.__proto__);for(const s in e)e[s]==null?t.removeAttribute(s):s==="style"?t.style.cssText=e[s]:s==="__value"?t.value=t[s]=e[s]:n[s]&&n[s].set&&ut.indexOf(s)===-1?t[s]=e[s]:it(t,s,e[s])}function Jt(t){return t===""?null:+t}function ct(t){return Array.from(t.childNodes)}function Kt(t,e){e=""+e,t.data!==e&&(t.data=e)}function Tt(t,e){t.value=e??""}function Ut(t,e,n,s){n==null?t.style.removeProperty(e):t.style.setProperty(e,n,s?"important":"")}function Vt(t,e,n){for(let s=0;s<t.options.length;s+=1){const o=t.options[s];if(o.__value===e){o.selected=!0;return}}(!n||e!==void 0)&&(t.selectedIndex=-1)}function Yt(t){const e=t.querySelector(":checked");return e&&e.__value}function Gt(t,e,n){t.classList.toggle(e,!!n)}function at(t,e,{bubbles:n=!1,cancelable:s=!1}={}){return new CustomEvent(t,{detail:e,bubbles:n,cancelable:s})}let O;function S(t){O=t}function v(){if(!O)throw new Error("Function called outside component initialization");return O}function Ht(t){v().$$.on_mount.push(t)}function Qt(t){v().$$.after_update.push(t)}function Xt(t){v().$$.on_destroy.push(t)}function Zt(){const t=v();return(e,n,{cancelable:s=!1}={})=>{const o=t.$$.callbacks[e];if(o){const r=at(e,n,{cancelable:s});return o.slice().forEach(i=>{i.call(t,r)}),!r.defaultPrevented}return!0}}function te(t,e){return v().$$.context.set(t,e),e}function ee(t){return v().$$.context.get(t)}function ne(t,e){const n=t.$$.callbacks[e.type];n&&n.slice().forEach(s=>s.call(this,e))}const $=[],T=[];let b=[];const D=[],H=Promise.resolve();let F=!1;function Q(){F||(F=!0,H.then(X))}function se(){return Q(),H}function B(t){b.push(t)}function oe(t){D.push(t)}const q=new Set;let w=0;function X(){if(w!==0)return;const t=O;do{try{for(;w<$.length;){const e=$[w];w++,S(e),lt(e.$$)}}catch(e){throw $.length=0,w=0,e}for(S(null),$.length=0,w=0;T.length;)T.pop()();for(let e=0;e<b.length;e+=1){const n=b[e];q.has(n)||(q.add(n),n())}b.length=0}while($.length);for(;D.length;)D.pop()();F=!1,q.clear(),S(t)}function lt(t){if(t.fragment!==null){t.update(),x(t.before_update);const e=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,e),t.after_update.forEach(B)}}function ft(t){const e=[],n=[];b.forEach(s=>t.indexOf(s)===-1?e.push(s):n.push(s)),n.forEach(s=>s()),b=e}const N=new Set;let y;function re(){y={r:0,c:[],p:y}}function ie(){y.r||x(y.c),y=y.p}function Z(t,e){t&&t.i&&(N.delete(t),t.i(e))}function dt(t,e,n,s){if(t&&t.o){if(N.has(t))return;N.add(t),y.c.push(()=>{N.delete(t),s&&(n&&t.d(1),s())}),t.o(e)}else s&&s()}function ue(t){return(t==null?void 0:t.length)!==void 0?t:Array.from(t)}function ce(t,e){dt(t,1,1,()=>{e.delete(t.key)})}function ae(t,e,n,s,o,r,i,l,g,u,m,d){let a=t.length,h=r.length,f=a;const I={};for(;f--;)I[t[f].key]=f;const k=[],A=new Map,C=new Map,W=[];for(f=h;f--;){const c=d(o,r,f),_=n(c);let p=i.get(_);p?s&&W.push(()=>p.p(c,e)):(p=u(_,c),p.c()),A.set(_,k[f]=p),_ in I&&C.set(_,Math.abs(f-I[_]))}const z=new Set,J=new Set;function M(c){Z(c,1),c.m(l,m),i.set(c.key,c),m=c.first,h--}for(;a&&h;){const c=k[h-1],_=t[a-1],p=c.key,P=_.key;c===_?(m=c.first,a--,h--):A.has(P)?!i.has(p)||z.has(p)?M(c):J.has(P)?a--:C.get(p)>C.get(P)?(J.add(p),M(c)):(z.add(P),a--):(g(_,i),a--)}for(;a--;){const c=t[a];A.has(c.key)||g(c,i)}for(;h;)M(k[h-1]);return x(W),k}function le(t,e,n){const s=t.$$.props[e];s!==void 0&&(t.$$.bound[s]=n,n(t.$$.ctx[s]))}function fe(t){t&&t.c()}function ht(t,e,n){const{fragment:s,after_update:o}=t.$$;s&&s.m(e,n),B(()=>{const r=t.$$.on_mount.map(U).filter(R);t.$$.on_destroy?t.$$.on_destroy.push(...r):x(r),t.$$.on_mount=[]}),o.forEach(B)}function _t(t,e){const n=t.$$;n.fragment!==null&&(ft(n.after_update),x(n.on_destroy),n.fragment&&n.fragment.d(e),n.on_destroy=n.fragment=null,n.ctx=[])}function pt(t,e){t.$$.dirty[0]===-1&&($.push(t),Q(),t.$$.dirty.fill(0)),t.$$.dirty[e/31|0]|=1<<e%31}function de(t,e,n,s,o,r,i=null,l=[-1]){const g=O;S(t);const u=t.$$={fragment:null,ctx:[],props:r,update:E,not_equal:o,bound:K(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(e.context||(g?g.$$.context:[])),callbacks:K(),dirty:l,skip_bound:!1,root:e.target||g.$$.root};i&&i(u.root);let m=!1;if(u.ctx=n?n(t,e.props||{},(d,a,...h)=>{const f=h.length?h[0]:a;return u.ctx&&o(u.ctx[d],u.ctx[d]=f)&&(!u.skip_bound&&u.bound[d]&&u.bound[d](f),m&&pt(t,d)),a}):[],u.update(),m=!0,x(u.before_update),u.fragment=s?s(u.ctx):!1,e.target){if(e.hydrate){const d=ct(e.target);u.fragment&&u.fragment.l(d),d.forEach(rt)}else u.fragment&&u.fragment.c();e.intro&&Z(t.$$.fragment),ht(t,e.target,e.anchor),X()}S(g)}class he{constructor(){j(this,"$$");j(this,"$$set")}$destroy(){_t(this,1),this.$destroy=E}$on(e,n){if(!R(n))return E;const s=this.$$.callbacks[e]||(this.$$.callbacks[e]=[]);return s.push(n),()=>{const o=s.indexOf(n);o!==-1&&s.splice(o,1)}}$set(e){this.$$set&&!ot(e)&&(this.$$.skip_bound=!0,this.$$set(e),this.$$.skip_bound=!1)}}const gt="4";typeof window<"u"&&(window.__svelte||(window.__svelte={v:new Set})).v.add(gt);function _e(t,e){try{window.localStorage.setItem(t,e)}catch(n){console.log(`Couldn't set local storage for ${t}: ${n}`),window.alert("Your changes couldn't be saved because you've run out of local storage. Please fix this problem on the next page and try again."),window.location.href="local_storage.html"}}function pe(t,e){return`npw/${t}/${e}`}function ge(t){let e=`npw/${t}/`,n=[];for(let s=0;s<window.localStorage.length;s++){let o=window.localStorage.key(s);if(o.startsWith(e)){let r=o.slice(e.length);if(r=="last-opened-file")continue;try{let i=JSON.parse(window.localStorage.getItem(o));n.push([r,tt(i)])}catch{}}}return n.sort(),n}function ye(){let t=new Map;for(let e=0;e<window.localStorage.length;e++){let n=window.localStorage.key(e);if(n.startsWith("npw/")&&!n.endsWith("/last-opened-file"))try{let s=JSON.parse(window.localStorage.getItem(n)||""),o=tt(s),[r,i,l]=n.split("/");t.has(i)||t.set(i,[]),t.get(i).push([l,o])}catch{}}for(let e of t.values())e.sort();return t}function tt(t){try{return`${t.features.length} routes`}catch{return"outdated file"}}function me(){let t=[];for(let e=0;e<window.localStorage.length;e++){let n=window.localStorage.key(e);t.push([n,window.localStorage.getItem(n).length])}return t.sort((e,n)=>n[1]-e[1]),t}export{Gt as $,mt as A,v as B,S as C,re as D,ie as E,X as F,T as G,le as H,oe as I,vt as J,Et as K,Ot as L,St as M,ne as N,xt as O,Zt as P,Xt as Q,Ut as R,he as S,It as T,Wt as U,Rt as V,R as W,Nt as X,Ft as Y,Pt as Z,qt as _,jt as a,$t as a0,Lt as a1,ae as a2,ce as a3,st as a4,V as a5,bt as a6,O as a7,zt as a8,Qt as a9,kt as aa,Jt as ab,ge as ac,pe as ad,_e as ae,te as af,ee as ag,se as ah,me as ai,Dt as b,fe as c,Tt as d,ue as e,it as f,B as g,Ct as h,de as i,At as j,Vt as k,Bt as l,ht as m,Z as n,dt as o,rt as p,Mt as q,_t as r,wt as s,G as t,x as u,ye as v,Ht as w,Kt as x,E as y,Yt as z};
