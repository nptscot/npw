import{S as Ce,i as Oe,s as Me,e as n,a as s,c as Se,g as o,m as Le,k as We,n as Ne,o as i,q as Pe,u as De,t as $e}from"./index-39859ba5.js";import"./Modal.svelte_svelte_type_style_lang-14adb101.js";import{B as _e}from"./all.min-d4de7573.js";function He(d){let a;return{c(){a=$e("Back to NPW")},m(r,l){o(r,a,l)},d(r){r&&i(a)}}}function Ie(d){let a,r,l,E,u,G,p,R,h,U,c,J,f,K,m,Q,b,V,y,X,w,Z,g,ee,k,te,x,oe,v,ie,T,ne,C,se,O,ae,M,le,S,re,L,de,W,ue,N,pe,P,he,D,ce,$,fe,_,me,H,be,I,ye,A,we,q,ge,B,ke,Y,xe,j,ve,z,F;return l=new _e({props:{$$slots:{default:[He]},$$scope:{ctx:d}}}),l.$on("click",d[0]),{c(){a=n("h1"),a.textContent="Network Planning Workspace User Manual",r=s(),Se(l.$$.fragment),E=s(),u=n("p"),u.textContent="Not ready yet",G=s(),p=n("h2"),p.textContent="Technical details",R=s(),h=n("p"),h.textContent=`These are some notes about how NPW works (not how to use it). Maybe this will
  be a separate appendix, or maybe the user guide will have collapsible/optional
  sections. An appropriate tone/style will happen later.`,U=s(),c=n("h3"),c.textContent="The editable network",J=s(),f=n("p"),f.innerHTML=`NPW models a network, consisting of road segments leading between exactly two
  intersections. The network is built from OpenStreetMap (OSM). Note that OSM is
  constantly updated, but changes there are not reflected in NPW until the
  development team regenerates NPW base data. The OSM data in NPW is from <b>TODO</b>
  .`,K=s(),m=n("p"),m.textContent=`Only roads that should be candidates for a cycling network are included in
  NPW.`,Q=s(),b=n("ul"),b.innerHTML=`<li>Motorways are excluded (
    <b>TODO verify</b>
    ), because they will never be appropriate for cycling infrastructure. (We
    may need to support new off-road segments that could be parallel.)</li> <li>Most OSM highway types for roads are included</li> <li>Anything currently under construction is excluded</li> <li>Steps/stairs are excluded, because unless this becomes an appropriate ramp,
    this can never be acceptable infrastructure</li> <li>When OSM has pedestrian-oriented infrastructure, it&#39;s only included
    sometimes, depending on tagging. (
    <b>TODO get specific</b>
    )</li>`,V=s(),y=n("p"),y.textContent=`Currently all routes in NPW must follow one of these existing road segments.
  You cannot draw new road segments. It's impossible to model a new bridge over
  a canal or through a park. If you intend to create a new off-road route
  parallel to an existing road, then for now, draw along that road.`,X=s(),w=n("p"),w.innerHTML=`A simplifying assumption throughout NPW is that all infrastructure is <b>bidirectional</b>
  . When you draw something along a road, it&#39;s assumed that cyclists will be
  able to travel either direction along it,
  <b>even when the road is one-way for drivers</b>
  . NPW does not care about the difference beteen two segregated cycletracks on
  both sides of the road, versus one segregated bidirectional track on one side
  of the road.`,Z=s(),g=n("h3"),g.textContent="Starting a network",ee=s(),k=n("p"),k.textContent=`You can start with a completely blank network, or you can initially seed your
  network from a few different sources.`,te=s(),x=n("p"),x.innerHTML=`You can include existing infrastructure from OSM. There is lots of existing
  infrastructure mapped in OSM that doesn&#39;t form part of an appropriate network,
  so there are different options to filter for only the appropriate things. <b>TODO describe the two options</b>
  . If you start from these options, you should audit everything to make sure it
  really is compliant. If something is incorrect or missing, you should
  <b>TODO do something</b>
  .`,oe=s(),v=n("p"),v.innerHTML=`You can also start with the <b>core network</b>
  , which is a suggested network automatically produced by the NPT team.
  <b>TODO link to its methodology</b>
  . There are many caveats with this network, so you should not just blindly
  trust it.`,ie=s(),T=n("h3"),T.textContent="Drawing routes",ne=s(),C=n("p"),C.textContent=`To draw a route, you pick a sequence of two or more waypoints on the map. The
  route between them is the most direct (shortest distance). If the route
  alignment is incorrect, you can drag one of the small white dots and create a
  new waypoint. All of the waypoints "snap" to an intersection. When you draw in
  the primary or secondary tier, by default, these waypoints try to snap to
  major junctions, shown as small grey dots. You can adjust this by zooming in
  and dragging a waypoint.`,se=s(),O=n("p"),O.textContent="The route you draw will be split into sections when:",ae=s(),M=n("ul"),M.innerHTML="<li>the route crosses another existing route</li> <li>the auto-recommended or manual infrastructure type changes</li> <li>the infrastructure type does or does not fit in the available streetspace</li> <li>the tier changes (based on whether the road is inside a settlement or not)</li>",le=s(),S=n("p"),S.textContent=`After saving a route, these sections become independent. You can edit each
  one, but the original longer route you drew is lost.`,re=s(),L=n("h3"),L.textContent="Level of Service and automatic infrastructure types",de=s(),W=n("p"),W.innerHTML=`By default, each route section is assigned the most appropriate infrastructure
  type, based on the &quot;cheapest&quot; type that achieves a high Level of Service
  (LoS). The LoS is only focused on safety and separation from motor vehicles,
  not comfort or speed or separation from pedestrians. LoS depends on four
  things -- the infrastructure type, the marked speed limit of the road, an
  estimate of the traffic volume along that road (which has many caveats), and
  whether the road is within a settlement or not. Many of the rules come from
  table 3.2 of <a href="https://www.transport.gov.scot/media/50323/cycling-by-design-update-2019-final-document-15-september-2021-1.pdf" target="_blank">Cycling by Design</a>
  .`,ue=s(),N=n("ul"),N.innerHTML=`<li><b>Mixed traffic</b>
     follows the CbD table; speed and traffic matter</li> <li><b>Segregated cycletracks</b>
     follow the table too, for &quot;Cycle Track at Carriageway Level&quot;.</li> <li><b>Painted cycle lanestraffic</b>
     follow the CbD table; speed and traffic matter</li> <li>Mixed traffic or segregated tracks <b>with traffic measures</b>
     always achieve high LoS, by definition. This is used to model external plans
    that NPW doesn&#39;t know about to reduce traffic volume and/or speed to the level
    that makes mixed traffic or a segregated track appropriate.</li> <li><b>Off-road</b>
     routes are high LoS by definition, because they are separated from motor vehicles.
    NPW&#39;s LoS definition does not account for path surface or lighting.</li> <li><b>Shared footways</b>
     are low LoS when they are within a settlement, and high when they&#39;re out.</li>`,pe=s(),P=n("p"),P.textContent="The automatic infrastructure type recommendation follows this order:",he=s(),D=n("ol"),D.innerHTML=`<li>If the segment is off-road, then use that type. This is automatically
    detected from existing roads, based on OSM highway type and naming, and may
    have bugs.</li> <li>If mixed traffic is appropriate based on speed and volume, use that</li> <li>If the road is outside a settlement, use a shared footway</li> <li>Otherwise, use a segregated cycle track</li>`,ce=s(),$=n("p"),$.textContent=`Note there may be two cases where the automatic recommendation has problems.
  When speed and volume are every high, even a segregated track doesn't achieve
  high LoS according to CbD guidance. In tat case, you must accept this lower
  LoS, realign the route, or upgrade to the stronger segregated + traffic
  measures type.`,fe=s(),_=n("p"),_.textContent=`Another problem may be that a segregated track does not fit the available
  streetspace. This is a rough guess; it's subject to problems with the data and
  methodology, and may have both false positives and negatives. When it is a
  real problem, you can switch to mixed traffic + traffic measures, realign the
  route, or pick a smaller infrastructure and accept lower LoS.`,me=s(),H=n("h3"),H.textContent="Layers",be=s(),I=n("p"),I.innerHTML=`<b>TODO</b>
  . For each layer, list the data sources, limits, etc. Maybe some of this info
  should get inlined as help in the app too. For example for greenspaces, which
  are included? How do we determine attractive streets?`,ye=s(),A=n("h3"),A.textContent="Reachability",we=s(),q=n("p"),q.innerHTML=`Some of the metrics determine if a population zone, town centre, or POI
  (school, hospital or GP, or greenspace) are <b>reachable</b>
  from your network.
  <b>TODO</b>
   discuss what this means, using the audit doc. Be careful about disconnectd networks.`,ge=s(),B=n("h3"),B.textContent="Cycling demand",ke=s(),Y=n("p"),Y.textContent=`Discuss how the demand network is built -- the OD data source, the methods,
  the lack of gradient data, the difference from what's shown on the NPT site.`,xe=s(),j=n("h3"),j.textContent="Other metrics",ve=s(),z=n("p"),z.textContent="Anything else not covered, like mesh density"},m(e,t){o(e,a,t),o(e,r,t),Le(l,e,t),o(e,E,t),o(e,u,t),o(e,G,t),o(e,p,t),o(e,R,t),o(e,h,t),o(e,U,t),o(e,c,t),o(e,J,t),o(e,f,t),o(e,K,t),o(e,m,t),o(e,Q,t),o(e,b,t),o(e,V,t),o(e,y,t),o(e,X,t),o(e,w,t),o(e,Z,t),o(e,g,t),o(e,ee,t),o(e,k,t),o(e,te,t),o(e,x,t),o(e,oe,t),o(e,v,t),o(e,ie,t),o(e,T,t),o(e,ne,t),o(e,C,t),o(e,se,t),o(e,O,t),o(e,ae,t),o(e,M,t),o(e,le,t),o(e,S,t),o(e,re,t),o(e,L,t),o(e,de,t),o(e,W,t),o(e,ue,t),o(e,N,t),o(e,pe,t),o(e,P,t),o(e,he,t),o(e,D,t),o(e,ce,t),o(e,$,t),o(e,fe,t),o(e,_,t),o(e,me,t),o(e,H,t),o(e,be,t),o(e,I,t),o(e,ye,t),o(e,A,t),o(e,we,t),o(e,q,t),o(e,ge,t),o(e,B,t),o(e,ke,t),o(e,Y,t),o(e,xe,t),o(e,j,t),o(e,ve,t),o(e,z,t),F=!0},p(e,[t]){const Te={};t&2&&(Te.$$scope={dirty:t,ctx:e}),l.$set(Te)},i(e){F||(We(l.$$.fragment,e),F=!0)},o(e){Ne(l.$$.fragment,e),F=!1},d(e){e&&(i(a),i(r),i(E),i(u),i(G),i(p),i(R),i(h),i(U),i(c),i(J),i(f),i(K),i(m),i(Q),i(b),i(V),i(y),i(X),i(w),i(Z),i(g),i(ee),i(k),i(te),i(x),i(oe),i(v),i(ie),i(T),i(ne),i(C),i(se),i(O),i(ae),i(M),i(le),i(S),i(re),i(L),i(de),i(W),i(ue),i(N),i(pe),i(P),i(he),i(D),i(ce),i($),i(fe),i(_),i(me),i(H),i(be),i(I),i(ye),i(A),i(we),i(q),i(ge),i(B),i(ke),i(Y),i(xe),i(j),i(ve),i(z)),Pe(l,e)}}}function Ae(d){return De(async()=>{window.DS.initAll()}),[()=>window.location.href="./"]}class qe extends Ce{constructor(a){super(),Oe(this,a,Ae,Ie,Me,{})}}new qe({target:document.getElementById("app")});
