import{S as pe,i as ue,s as fe,e as G,l as _e,a as p,b as f,t as S,c as A,d as E,f as $,g as de,h as he,j as y,k as r,m as oe,n as j,o as ae,p as N,q as z,r as k,u as J,v as F,w as me,x as $e,y as ie,z as ce,A as ge}from"./files-aca523be.js";import{M as ve,m as be,l as we,N as ye,G as ke,F as Ce,h as Le,L as Se,P as Ae}from"./stores-febbc656.js";const je="/npw/demo/assets/boundaries-c02eb905.geojson";function Ne(o,e,t){const n=o.slice();return n[7]=e[t][0],n[8]=e[t][1],n}function ze(o,e,t){const n=o.slice();return n[11]=e[t][0],n[12]=e[t][1],n}function se(o,e,t){const n=o.slice();return n[15]=e[t],n}function re(o){let e,t=o[15]+"",n,a;return{c(){e=p("option"),n=S(t),e.__value=a=o[15],E(e,e.__value)},m(l,s){y(l,e,s),r(e,n)},p(l,s){s&4&&t!==(t=l[15]+"")&&ie(n,t),s&4&&a!==(a=l[15])&&(e.__value=a,E(e,e.__value))},d(l){l&&k(e)}}}function Fe(o){let e,t,n,a,l;return{c(){e=p("p"),t=p("a"),t.textContent=`${o[11]}`,n=S(`
              (`),a=S(o[12]),l=S(")"),$(t,"href",`npw.html?boundary=${o[7]}&file=${o[11]}`)},m(s,c){y(s,e,c),r(e,t),r(e,n),r(e,a),r(e,l)},p:ce,d(s){s&&k(e)}}}function Me(o){let e,t,n,a,l=G(o[8]),s=[];for(let c=0;c<l.length;c+=1)s[c]=Fe(ze(o,l,c));return{c(){e=p("div"),t=p("h4"),t.textContent=`${o[7]}`,n=f();for(let c=0;c<s.length;c+=1)s[c].c();a=f(),$(e,"class","file-group svelte-6ynzbr")},m(c,M){y(c,e,M),r(e,t),r(e,n);for(let _=0;_<s.length;_+=1)s[_]&&s[_].m(e,null);r(e,a)},p:ce,d(c){c&&k(e),J(s,c)}}}function Pe(o){let e,t=o[6].name+"",n;return{c(){e=p("p"),n=S(t)},m(a,l){y(a,e,l),r(e,n)},p(a,l){l&64&&t!==(t=a[6].name+"")&&ie(n,t)},d(a){a&&k(e)}}}function Ie(o){let e,t;return e=new Ae({props:{openOn:"hover",$$slots:{default:[Pe,({props:n})=>({6:n}),({props:n})=>n?64:0]},$$scope:{ctx:o}}}),{c(){A(e.$$.fragment)},m(n,a){j(e,n,a),t=!0},p(n,a){const l={};a&262208&&(l.$$scope={dirty:a,ctx:n}),e.$set(l)},i(n){t||(N(e.$$.fragment,n),t=!0)},o(n){z(e.$$.fragment,n),t=!1},d(n){F(e,n)}}}function Te(o){let e,t,n,a;return e=new Ce({props:{paint:{"fill-color":"rgb(200, 100, 240)","fill-outline-color":"rgb(200, 100, 240)","fill-opacity":Le(0,.5)},beforeId:"Road labels",manageHoverState:!0,hoverCursor:"pointer",$$slots:{default:[Ie]},$$scope:{ctx:o}}}),e.$on("click",Oe),n=new Se({props:{paint:{"line-color":"rgb(200, 100, 240)","line-width":2.5},beforeId:"Road labels"}}),{c(){A(e.$$.fragment),t=f(),A(n.$$.fragment)},m(l,s){j(e,l,s),y(l,t,s),j(n,l,s),a=!0},p(l,s){const c={};s&262144&&(c.$$scope={dirty:s,ctx:l}),e.$set(c)},i(l){a||(N(e.$$.fragment,l),N(n.$$.fragment,l),a=!0)},o(l){z(e.$$.fragment,l),z(n.$$.fragment,l),a=!1},d(l){l&&k(t),F(e,l),F(n,l)}}}function De(o){let e,t,n,a;return e=new ye({props:{showCompass:!1,position:"top-right"}}),n=new ke({props:{data:o[0],generateId:!0,$$slots:{default:[Te]},$$scope:{ctx:o}}}),{c(){A(e.$$.fragment),t=f(),A(n.$$.fragment)},m(l,s){j(e,l,s),y(l,t,s),j(n,l,s),a=!0},p(l,s){const c={};s&1&&(c.data=l[0]),s&262144&&(c.$$scope={dirty:s,ctx:l}),n.$set(c)},i(l){a||(N(e.$$.fragment,l),N(n.$$.fragment,l),a=!0)},o(l){z(e.$$.fragment,l),z(n.$$.fragment,l),a=!1},d(l){l&&k(t),F(e,l),F(n,l)}}}function He(o){let e,t,n,a,l,s,c,M,_,T,R,K,C,d,P,U,D,Q,b,V,H,X,Y,Z,W,x,O,ee,q,g,I,B,te,L=G(o[2]),u=[];for(let i=0;i<L.length;i+=1)u[i]=re(se(o,L,i));let le=G(_e()),w=[];for(let i=0;i<le.length;i+=1)w[i]=Me(Ne(o,le,i));return g=new ve({props:{style:`https://api.maptiler.com/maps/streets-v2/style.json?key=${be}`,bounds:[-8.943,54.631,-.901,59.489],$$slots:{default:[De]},$$scope:{ctx:o}}}),g.$on("error",o[5]),{c(){e=p("div"),t=p("div"),n=p("p"),n.innerHTML=`<a href="https://www.npt.scot/"><img src="${we}" alt="NPT logo"/></a>`,a=f(),l=p("h1"),l.textContent="Network Planning Workspace",s=f(),c=p("p"),c.textContent=`The NPW is designed to enable local authorities to plan a cycle network
      for the area, using segregated infrastructure on key routes and ensuring
      local places are properly connected without severance.`,M=f(),_=p("h3"),_.innerHTML='<label class="ds_label" for="component">Select an area</label>',T=f(),R=p("p"),R.textContent="Select your area from this list, or click on the map, to start.",K=f(),C=p("div"),d=p("select"),P=p("option");for(let i=0;i<u.length;i+=1)u[i].c();U=f(),D=p("span"),Q=f(),b=p("button"),V=S("Start"),X=f(),Y=p("hr"),Z=f(),W=p("h3"),W.textContent="Or continue with a previously opened file",x=f(),O=p("div");for(let i=0;i<w.length;i+=1)w[i].c();ee=f(),q=p("div"),A(g.$$.fragment),P.__value="",E(P,P.__value),$(d,"class","ds_select"),$(d,"id","component"),$(d,"name","component"),o[1]===void 0&&de(()=>o[3].call(d)),$(D,"class","ds_select-arrow"),$(D,"aria-hidden","true"),$(C,"class","ds_select-wrapper"),$(b,"class","ds_button"),b.disabled=H=o[1]=="",he(O,"columns","2"),$(t,"class","controls svelte-6ynzbr"),$(q,"class","map svelte-6ynzbr"),$(e,"class","container svelte-6ynzbr")},m(i,v){y(i,e,v),r(e,t),r(t,n),r(t,a),r(t,l),r(t,s),r(t,c),r(t,M),r(t,_),r(t,T),r(t,R),r(t,K),r(t,C),r(C,d),r(d,P);for(let h=0;h<u.length;h+=1)u[h]&&u[h].m(d,null);oe(d,o[1],!0),r(C,U),r(C,D),r(t,Q),r(t,b),r(b,V),r(t,X),r(t,Y),r(t,Z),r(t,W),r(t,x),r(t,O);for(let h=0;h<w.length;h+=1)w[h]&&w[h].m(O,null);r(e,ee),r(e,q),j(g,q,null),I=!0,B||(te=[ae(d,"change",o[3]),ae(b,"click",o[4])],B=!0)},p(i,[v]){if(v&4){L=G(i[2]);let m;for(m=0;m<L.length;m+=1){const ne=se(i,L,m);u[m]?u[m].p(ne,v):(u[m]=re(ne),u[m].c(),u[m].m(d,null))}for(;m<u.length;m+=1)u[m].d(1);u.length=L.length}v&6&&oe(d,i[1]),(!I||v&6&&H!==(H=i[1]==""))&&(b.disabled=H);const h={};v&262145&&(h.$$scope={dirty:v,ctx:i}),g.$set(h)},i(i){I||(N(g.$$.fragment,i),I=!0)},o(i){z(g.$$.fragment,i),I=!1},d(i){i&&k(e),J(u,i),J(w,i),F(g),B=!1,me(te)}}}function Oe(o){let e=o.detail.features[0].properties;window.location.href=`npw.html?boundary=LAD_${e.name}`}function qe(o,e,t){let n={type:"FeatureCollection",features:[]},a="",l=[];$e(async()=>{window.DS.initAll();let _=await fetch(je);t(0,n=await _.json());for(let T of n.features)l.push(T.properties.name);l.sort(),t(2,l)});function s(){a=ge(this),t(1,a),t(2,l)}return[n,a,l,s,()=>window.location.href=`npw.html?boundary=LAD_${a}`,_=>{console.log(_.detail.error)}]}class Ge extends pe{constructor(e){super(),ue(this,e,qe,He,fe,{})}}new Ge({target:document.getElementById("app")});
