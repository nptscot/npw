import"./files-ef316d1e.js";function _(t,l){let e=document.createElement("a");e.setAttribute("href","data:text/plain;charset=utf-8,"+encodeURIComponent(l)),e.setAttribute("download",t),document.body.appendChild(e),e.click(),document.body.removeChild(e)}function g(t){if(t==null||t==null)throw new Error("Oops, notNull given something null");return t}async function f(t,l){let e=await fetch(t);if(!e.ok)throw new Error(`${t} not OK: ${e.status}`);let h=e.body.getReader(),s=e.headers.get("Content-Length");if(!s)throw new Error(`No Content-Length header from ${t}`);let c=parseInt(s),o=0,a=[];for(;;){let{done:n,value:r}=await h.read();if(n)break;r&&(a.push(r),o+=r.length,l(100*o/c))}let i=new Uint8Array(o),d=0;for(let n of a)i.set(n,d),d+=n.length;return i}export{_ as d,f,g as n};
