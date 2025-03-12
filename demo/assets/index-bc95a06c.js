import{S as V,i as X,s as Y,e as H,l as Z,a as p,b as h,c as z,d as A,f as d,g,h as c,m as F,t as D,j as I,k as b,n as q,o as M,p as ee,q as S,r as W,u as Q}from"./files-7254d1b1.js";import{M as te,m as le,G as ne,F as oe,h as re,L as ae,P as se}from"./stores-e85215f3.js";const ie="/npw/demo/assets/boundaries-c02eb905.geojson";function ce(r,e,l){const t=r.slice();return t[4]=e[l][0],t[5]=e[l][1],t}function pe(r,e,l){const t=r.slice();return t[8]=e[l][0],t[9]=e[l][1],t}function K(r,e,l){const t=r.slice();return t[12]=e[l],t}function U(r){let e,l,t=r[12]+"",o,n;return{c(){e=p("li"),l=p("a"),o=S(t),d(l,"href",n="npw.html?boundary=LAD_"+r[12])},m(a,i){g(a,e,i),c(e,l),c(l,o)},p(a,i){i&2&&t!==(t=a[12]+"")&&W(o,t),i&2&&n!==(n="npw.html?boundary=LAD_"+a[12])&&d(l,"href",n)},d(a){a&&b(e)}}}function fe(r){let e,l,t,o,n;return{c(){e=p("p"),l=p("a"),l.textContent=`${r[8]}`,t=S(`
            (`),o=S(r[9]),n=S(")"),d(l,"href",`npw.html?boundary=${r[4]}&file=${r[8]}`)},m(a,i){g(a,e,i),c(e,l),c(e,t),c(e,o),c(e,n)},p:Q,d(a){a&&b(e)}}}function ue(r){let e,l,t,o,n=H(r[5]),a=[];for(let i=0;i<n.length;i+=1)a[i]=fe(pe(r,n,i));return{c(){e=p("div"),l=p("h2"),l.textContent=`${r[4]}`,t=h();for(let i=0;i<a.length;i+=1)a[i].c();o=h(),d(e,"class","group svelte-6955zw")},m(i,v){g(i,e,v),c(e,l),c(e,t);for(let w=0;w<a.length;w+=1)a[w]&&a[w].m(e,null);c(e,o)},p:Q,d(i){i&&b(e),q(a,i)}}}function _e(r){let e,l=r[3].name+"",t;return{c(){e=p("p"),t=S(l)},m(o,n){g(o,e,n),c(e,t)},p(o,n){n&8&&l!==(l=o[3].name+"")&&W(t,l)},d(o){o&&b(e)}}}function he(r){let e,l;return e=new se({props:{openOn:"hover",$$slots:{default:[_e,({props:t})=>({3:t}),({props:t})=>t?8:0]},$$scope:{ctx:r}}}),{c(){z(e.$$.fragment)},m(t,o){F(e,t,o),l=!0},p(t,o){const n={};o&32776&&(n.$$scope={dirty:o,ctx:t}),e.$set(n)},i(t){l||(D(e.$$.fragment,t),l=!0)},o(t){I(e.$$.fragment,t),l=!1},d(t){M(e,t)}}}function me(r){let e,l,t,o;return e=new oe({props:{paint:{"fill-color":"rgb(200, 100, 240)","fill-outline-color":"rgb(200, 100, 240)","fill-opacity":re(0,.5)},beforeId:"Road labels",manageHoverState:!0,hoverCursor:"pointer",$$slots:{default:[he]},$$scope:{ctx:r}}}),e.$on("click",ge),t=new ae({props:{paint:{"line-color":"rgb(200, 100, 240)","line-width":2.5},beforeId:"Road labels",manageHoverState:!0}}),{c(){z(e.$$.fragment),l=h(),z(t.$$.fragment)},m(n,a){F(e,n,a),g(n,l,a),F(t,n,a),o=!0},p(n,a){const i={};a&32768&&(i.$$scope={dirty:a,ctx:n}),e.$set(i)},i(n){o||(D(e.$$.fragment,n),D(t.$$.fragment,n),o=!0)},o(n){I(e.$$.fragment,n),I(t.$$.fragment,n),o=!1},d(n){n&&b(l),M(e,n),M(t,n)}}}function $e(r){let e,l;return e=new ne({props:{data:r[0],generateId:!0,$$slots:{default:[me]},$$scope:{ctx:r}}}),{c(){z(e.$$.fragment)},m(t,o){F(e,t,o),l=!0},p(t,o){const n={};o&1&&(n.data=t[0]),o&32768&&(n.$$scope={dirty:o,ctx:t}),e.$set(n)},i(t){l||(D(e.$$.fragment,t),l=!0)},o(t){I(e.$$.fragment,t),l=!1},d(t){M(e,t)}}}function de(r){let e,l,t,o,n,a,i,v,w,G,R,N,T,j,B,O,P,L,y,m,x,C=H(r[1]),f=[];for(let s=0;s<C.length;s+=1)f[s]=U(K(r,C,s));let E=H(Z()),k=[];for(let s=0;s<E.length;s+=1)k[s]=ue(ce(r,E,s));return m=new te({props:{style:`https://api.maptiler.com/maps/streets-v2/style.json?key=${le}`,standardControls:!0,bounds:[-8.943,54.631,-.901,59.489],$$slots:{default:[$e]},$$scope:{ctx:r}}}),m.$on("error",r[2]),{c(){e=p("div"),l=p("h2"),l.textContent="Network Planning Workspace",t=h(),o=p("p"),o.innerHTML=`This is an
    <a href="https://github.com/nptscot/npw" target="_blank">open source project</a>
    project developed by
    <a href="https://github.com/dabreegster/" target="_blank">Dustin Carlino</a>
    .`,n=h(),a=p("p"),a.textContent="Choose a boundary below or on the map to begin sketching:",i=h(),v=p("ul");for(let s=0;s<f.length;s+=1)f[s].c();w=h(),G=p("hr"),R=h(),N=p("p"),N.textContent="Or continue with a previously opened file:",T=h(),j=p("div");for(let s=0;s<k.length;s+=1)k[s].c();B=h(),O=p("style"),O.textContent=`.group {
      border: 1px solid black;
      padding: 4px;
      margin-bottom: 8px;
      break-inside: avoid-column;
    }`,P=h(),L=p("div"),y=p("div"),z(m.$$.fragment),A(v,"columns","3"),A(j,"columns","2"),d(j,"class","svelte-6955zw"),d(e,"class","left pico svelte-6955zw"),A(y,"position","relative"),A(y,"width","100%"),A(y,"height","100%"),d(y,"class","svelte-6955zw"),d(L,"class","map svelte-6955zw")},m(s,$){g(s,e,$),c(e,l),c(e,t),c(e,o),c(e,n),c(e,a),c(e,i),c(e,v);for(let u=0;u<f.length;u+=1)f[u]&&f[u].m(v,null);c(e,w),c(e,G),c(e,R),c(e,N),c(e,T),c(e,j);for(let u=0;u<k.length;u+=1)k[u]&&k[u].m(j,null);c(e,B),c(e,O),g(s,P,$),g(s,L,$),c(L,y),F(m,y,null),x=!0},p(s,[$]){if($&2){C=H(s[1]);let _;for(_=0;_<C.length;_+=1){const J=K(s,C,_);f[_]?f[_].p(J,$):(f[_]=U(J),f[_].c(),f[_].m(v,null))}for(;_<f.length;_+=1)f[_].d(1);f.length=C.length}const u={};$&32769&&(u.$$scope={dirty:$,ctx:s}),m.$set(u)},i(s){x||(D(m.$$.fragment,s),x=!0)},o(s){I(m.$$.fragment,s),x=!1},d(s){s&&(b(e),b(P),b(L)),q(f,s),q(k,s),M(m)}}}function ge(r){let e=r.detail.features[0].properties;window.location.href=`npw.html?boundary=LAD_${e.name}`}function be(r,e,l){let t={type:"FeatureCollection",features:[]},o=[];return ee(async()=>{let a=await fetch(ie);l(0,t=await a.json());for(let i of t.features)o.push(i.properties.name);o.sort(),l(1,o)}),[t,o,a=>{console.log(a.detail.error)}]}class ve extends V{constructor(e){super(),X(this,e,be,de,Y,{})}}new ve({target:document.getElementById("app")});
