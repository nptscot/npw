import{S as O,i as P,s as R,e as q,a as r,b as _,q as T,f as L,d as W,g as K,h as t,r as I,u as A,k as N,n as Y,af as D,O as H,T as J}from"./files-7254d1b1.js";import{d as Q}from"./index-7a434c42.js";function E(l,e,i){const n=l.slice();return n[6]=e[i][0],n[7]=e[i][1],n}function G(l){let e,i,n=l[6]+"",u,b,f,p=$(l[7])+"",m,o,h,w,z,d,y,M,B;function v(){return l[4](l[6])}function F(){return l[5](l[6])}return{c(){e=r("tr"),i=r("td"),u=T(n),b=_(),f=r("td"),m=T(p),o=_(),h=r("td"),w=r("button"),w.textContent="Download",z=_(),d=r("button"),d.textContent="Delete",y=_(),L(w,"class","secondary"),L(d,"class","secondary")},m(k,g){K(k,e,g),t(e,i),t(i,u),t(e,b),t(e,f),t(f,m),t(e,o),t(e,h),t(h,w),t(h,z),t(h,d),t(e,y),M||(B=[H(w,"click",v),H(d,"click",F)],M=!0)},p(k,g){l=k,g&1&&n!==(n=l[6]+"")&&I(u,n),g&1&&p!==(p=$(l[7])+"")&&I(m,p)},d(k){k&&N(e),M=!1,J(B)}}}function U(l){let e,i,n,u,b,f,p,m,o=$(l[1])+"",h,w,z,d,y,M,B,v,F,k,g,S=q(l[0]),c=[];for(let a=0;a<S.length;a+=1)c[a]=G(E(l,S,a));return{c(){e=r("main"),i=r("h1"),i.textContent="Clean local storage",n=_(),u=r("a"),u.textContent="Back to NPW",b=_(),f=r("p"),f.innerHTML=`All files are stored in your browser&#39;s local storage. There&#39;s a 5MB size
    limit, and you&#39;ve been redirected to this page because the last action you
    took exceeds this limit. You must delete some other files first to continue. <b>Before you delete a file, you should download a copy.</b>`,p=_(),m=r("p"),h=T(o),w=T(" / 5 MB is used right now"),z=_(),d=r("progress"),M=_(),B=r("table"),v=r("table"),F=r("thead"),F.innerHTML="<tr><th>Filename</th> <th>Size</th> <th>Actions</th></tr>",k=_(),g=r("tbody");for(let a=0;a<c.length;a+=1)c[a].c();L(u,"href","index.html"),d.value=y=100*l[1]/(1024*1024*5),L(d,"max","100"),W(d,"width","100%"),L(e,"class","container")},m(a,C){K(a,e,C),t(e,i),t(e,n),t(e,u),t(e,b),t(e,f),t(e,p),t(e,m),t(m,h),t(m,w),t(e,z),t(e,d),t(e,M),t(e,B),t(B,v),t(v,F),t(v,k),t(v,g);for(let s=0;s<c.length;s+=1)c[s]&&c[s].m(g,null)},p(a,[C]){if(C&2&&o!==(o=$(a[1])+"")&&I(h,o),C&2&&y!==(y=100*a[1]/(1024*1024*5))&&(d.value=y),C&13){S=q(a[0]);let s;for(s=0;s<S.length;s+=1){const j=E(a,S,s);c[s]?c[s].p(j,C):(c[s]=G(j),c[s].c(),c[s].m(g,null))}for(;s<c.length;s+=1)c[s].d(1);c.length=S.length}},i:A,o:A,d(a){a&&N(e),Y(c,a)}}}function $(l){if(l==0)return"0 bytes";let e=1024,i=["bytes","KB","MB"],n=Math.floor(Math.log(l)/Math.log(e));return parseFloat((l/Math.pow(e,n)).toFixed(1))+" "+i[n]}function V(l,e,i){let n,u=D();function b(o){Q(`${o}.geojson`,window.localStorage.getItem(o))}function f(o){window.confirm(`Really delete ${o}?`)&&(window.localStorage.removeItem(o),i(0,u=D()))}const p=o=>b(o),m=o=>f(o);return l.$$.update=()=>{l.$$.dirty&1&&i(1,n=u.reduce((o,h)=>o+h[1],0))},[u,n,b,f,p,m]}class X extends O{constructor(e){super(),P(this,e,V,U,R,{})}}new X({target:document.getElementById("app")});
