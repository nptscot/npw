# Network Planning Workspace: User guide

## Overview

The Network Planning Workspace (NPW) is designed to enable local authorities to plan a cycle network for a local authority area, using segregated infrastructure on key routes and ensuring local places are properly connected without severance.

As a user of the NPW tool, you are able to 'draw on' the map to create the network, starting from a primary 'tier' forming the backbone of the network, followed by a secondary tier and then local connections and long-distance routes between settlements.

While drawing on each segment of the network, the NPW will automatically try to determine the most appropriate achievable infrastructure within the street space available. Where infrastructure cannot meet best practice, you will need to resolve deliverability problems and sub-optimal Level of Service issues. As the network is built up, metrics to determine the performance of the network are calculated and displayed.

Users are able to kick-start the creation of their network from a range of potential starting points, such as all existing on-street infrastructure meeting a quality standard.

A range of contextual data layers are available, covering transport and socio-demographic information. These should be consulted to aid the creation of the network, to help ensure your proposals support active travel comprehensively in the area.

Upon completion of the network, a final report can be created, and the data exported.

## Structure of the tool

The NPW is intended to guide users through the process of creating a network, using a step-by-step approach. However, it does not force users to stick to a linear approach rigidly.

The stages, each of which starts with a main screen, are:

_Opening:_

1. **Introduction page**, which welcomes you to the tool, and requests an area to be selected.
2. **Project selection**, where you can begin work in the selected geographical area on a new project or continue an existing project. (Note that, at present, work is saved only within your computer's browser rather than to a web service.)
3. **New network design** (if starting a new project), where you can choose either to start from a blank map, or kick-start your network creation from a range of potential pre-created datasets.

_Network planning by tier:_

4. **NPW workflow overview.** This is the main area to which you will return regularly. The screen will guide you step-by-step through the creation of tiers of your network, and acts as a portal to each of those sections.
5. **Tier 1: Plan primary routes:** Primary routes form the core of a cycle network. They should closely follow the highest demand, most strategic corridors and form key connections over LA boundaries.
6. **Tier 2: Plan secondary routes:** Secondary tier routes should cover medium cycling demand corridors, connect town centres and neighbourhoods. It will be important to avoid severance issues, arising from high traffic speeds and volumes, in creating this tier of the network. Accordingly, the sidebar includes the ability to see these severance issues.
7. **Tier 3: Plan local access to POIs:** The local access tier is the finest grained part of the network and ensures access to key points of interest (POIs), such as railway stations, schools, hospitals and green spaces. Some POIs (shown in red) may be unreachable from the current network due to severance caused by streets with high speeds and volumes, visible on the right panel. POIs unreachable from the current network should be connected by adding new routes.
8. **Tier 4: Plan long distance network:** Long distance routes are primarily inter-settlement links connecting settlements together with main urban areas (that are close enough for everyday work, shopping or social journeys).

_Finish:_

9. **Assess network quality:** Having designed your network, you can now fix any problems and assess its performance. A range of tools are presented for this
10. **Export:** Here you can complete the process, by exporting your finalised network for submission, and printing off a report summarising the network's performance.

More detail on these sections is given later in this manual.

## Interface areas

After the initial screens for starting to plan a network, the main workspace area provides a consistent interface consisting of:

- **Top menu:** This provides a breadcrumb-style navigation, enabling you to see where you are in the overall process in a logical order. The breadcrumb trail menu is designed to be flexible, so that you are able to jump forward or back if desired.
- **Workspace panel**: The left panel is the main section of the interface, facilitating the expected actions that you need to perform at any point. It will usually contain a summary description of what the current screen is for, and will often provide a primary action in blue, to help guide you through.
- **A main map area:** The map shows your network over a background map. The map forms a canvas on which the network is to be created. The section outside of your area is grayed-out. Standard +/- zoom controls are available, and scrollwheel/trackpad pan/zoom is supported. You can jump to a location using the geocoder in the top-left. For convenience, you can also jump directly to key settlements within the area. You can switch the background and view a location using Street View using the controls in the bottom-left. You can report a data issue in the bottom-left, as long as you are sufficiently zoomed-in.
- **Top-right metrics:** This provides a quick summary, which you should monitor regularly, showing the performance of your network. As you make changes, these metrics will be automatically re-computed and displayed. As the network becomes more comprehensive and is designed to meet best practice for design of active travel routes, the metrics will each gradually increase to approach 100%.
- **Background layers:** Contextual data layers are available on the right-hand-side of the map. Each available layer can be clicked on to enable/disable it. A layer may provide a small legend, possibly with filtering and other controls.
- **Show network panel**: This appears at the bottom and enables you to show which dimension of your drawn network should be shown visually, e.g. colouring by infrastructure type, by tier, and so on.
- **Tier-specific metrics:** When in a tier-specific screen, the bottom of the workspace panel will provide key metrics for the performance of your network in respect of that tier. In this area you can also evaluate a specific A-B journey to see how it would use your proposed network.

## The editable network

NPW models a network, consisting of road segments leading between exactly two intersections.

The underlying network data is built using OpenStreetMap (OSM), an open data source of the physical road/street/path network. More details about OSM are given later in this manual. Note that, although OpenStreetMap is a data source being constantly updated, changes there are not reflected in NPW until the development team regenerates NPW base data.

The OSM data in NPW is dated **TODO**.

Only roads that should be candidates for a cycling network are included in NPW.

- Motorways are excluded, because they will never be appropriate for cycling infrastructure.
- Most OSM highway types for roads are included.
- Anything currently under construction is excluded.
- Steps/stairs are excluded, because unless this becomes an appropriate ramp, this can never be acceptable infrastructure.
- When OSM has pedestrian-oriented infrastructure, it is only included sometimes, depending on tagging. **TODO More specific information needed.**

Currently all routes in NPW must follow one of these existing road segments.

A simplifying assumption throughout NPW is that all infrastructure is bidirectional. When you draw something along a road, it is assumed that people cycling will be able to travel either direction along it, even when the road is one-way for drivers. NPW does not presently differentiate between two one-way segregated cycle tracks on both sides of the road versus a single segregated bidirectional track on one side of the road.

## Starting a network

You can start with a completely blank network, or you can initially seed your network from a few different sources:

- All existing physical infrastructure, if it achieves high Level of Service.
- Existing segregated tracks and off-road cycleways.
- All arterial roads, i.e. trunk, A, B and C roads.

In each case, each road/street/path segment will be automatically allocated to a tier (Primary / Secondary / Local access / Long distance). These can be toggled within the network panel at the bottom of the map, which also provides a legend for the colourings.

If you choose more than one of the sources, any overlapping parts will automatically be de-duplicated.

Naturally the existing on-street network contains much existing infrastructure that does not form part of an appropriate network, so you should carefully consider what should or should not be used in your area. Therefore, if you start from any of the above options, you should audit everything to make sure it really matches the expectations given. There is always the possibility of incorrect data or, in the case of the coherent network, limitations arising from use of an algorithm.

If a particular street/path is incorrect or missing, you can give feedback using the button in the bottom-left of the map.

## Coherent network

The coherent network is available as a background layer. It is composed of high-potential, direct routes that strategically connect urban areas. Created through automated analysis by the NPT team, this network emphasises coherence in design to ensure cycling infrastructure is functional, accessible, and efficient. You can trace over it to start your primary and secondary network.

The coherent network serves as a guide for prioritising investment by highlighting routes that maximize coverage and connectivity across urban zones.

Route selection for the coherent network is based on two key attributes: road classification and cycling connectivity potential, both of which influence the "arterialness" score. This score prioritises routes that balance road hierarchy with accessibility for cycling. Using a spatial clustering algorithm (DBSCAN), high-flow areas are identified, forming a focused core network by filtering out redundant or isolated segments. This method ensures that selected routes maximize connectivity and suitability, aligning the network's structure with both directness and density requirements.

Routes are then classified into primary and secondary categories based on road type:

- A roads are classified as primary routes, forming the backbone of the cycling network.
- All other roads, including B roads, minor roads, and off-road paths, are classified as secondary routes, providing supplementary connections and ensuring comprehensive coverage.
- If the road segment is outside a settlement, we always force it to the long distance tier.

The network is adjusted to ensure direct routes with optimal density, following Transport Scotland's specifications (250m in urban centres, 400m in suburban areas). This allows the network to meet a wide range of origin-destination trip needs, improving access and usability.

The [algorithm for the coherent network](https://github.com/nptscot/npt/blob/main/R/cohesive_network.R) is published and available openly.

## Tier 1: Plan primary routes

Primary routes form the core of a cycle network. They should closely follow the highest demand, most strategic corridors and form key connections over LA boundaries.

Often these will be A roads, or key links over bridges, or along existing major cycleway routes.

The most fundamental part of this process, as with other tiers, is drawing lines on the map, representing the on-street infrastructure. Fuller details of the mechanics of how to draw are given later in this manual.

## Tier 2: Plan secondary routes

Secondary tier routes should cover medium cycling demand corridors, connect town centres and neighbourhoods.

To see severance issues (due to high traffic speeds and volumes), turn on the layer on the right.

## Tier 3: Plan local access to POIs

The local access tier is the finest grained part of the network and ensures access to key points of interest (POIs), such as railway stations, schools, hospitals and green spaces.

Some POIs (shown in red) may be unreachable from the current network due to severance caused by streets with high speeds and volumes, visible on the right panel. POIs unreachable from the current network should be connected by adding new routes.

Generally speaking, if you have drawn a reasonably comprehensive network of primary and secondary routes, POIs will have good connectivity in a natural way to that network, because they are connected to those main routes by local streets.

Therefore, if you are finding that a large proportion of POIs are showing in red, your main task will generally be to improve the backbone of the network, not spend time drawing in a lot of local connections that cross severances like main roads.

When trying to resolve a missing connection, a suggested route is shown dashed. You can choose to accept that proposal, or draw on manually in the usual way.

Multiple types of POIs are included: railway stations, schools, hospitals and green spaces. You can optionally filter to a specific type on the main screen, or just work randomly amongst different types.

At all times, a Street View button is available in the bottom-left, to help judge the suitability of local streets.

## Tier 4: Plan long distance network

Long distance routes are primarily inter-settlement links connecting settlements together with main urban areas (that are close enough for everyday work, shopping or social journeys).

TODO: Need something outlining best practice and expectations here.

If drawing over a major road between settlements, the tool will automatically assign the route to the long-distance tier.

## Drawing routes

To draw a route, you pick a sequence of two or more waypoints on the map. The route between them is the most direct (shortest distance).

You should aim to create the routes as logical segments between major junctions.

Start by clicking on a map to begin the line, ensuring that if you are joining to another line, you connect exactly at its endpoint. The line is now 'open' and ready for a next point. Move along the proposed road/street/path, and click again to set another point. You can continue clicking to draw further waypoints. When you have drawn a line that makes a logical segment, you need to 'close' that line, which you do either by clicking on the final point, or pressing Finish on the left.

While the line being drawn is still open (i.e. incomplete) you can pan, zoom in/out and drag the map in the normal way. This means you can move quite some distance away from the point you started from.

During drawing, you can also click the Undo button to undo the last operation, e.g. removing a waypoint you clicked in.

If the route alignment is incorrect, you can drag one of the small white dots and create a new waypoint.

All of the waypoints 'snap' to an intersection, in order to enable you to join up sections of the network easily without leaving gaps. When you draw in the primary or secondary tier, by default, these waypoints try to snap to major junctions, shown as small grey dots. You can adjust this by zooming in to the map more closely and dragging a waypoint.

The route you draw will be split into sections when either:

- the route crosses another existing route;
- the auto-recommended or manual infrastructure type changes;
- the infrastructure type does or does not fit in the available streetspace;
- the Level of Service changes, based on the infrastructure type;
- the tier changes, based on whether the road is inside a settlement or not (e.g. if drawing over a major road between settlements, the tool will automatically assign this to the long-distance tier.

After saving a route, these sections become independent. You can edit each one, but the original longer line you drew is lost.

Where the appropriate infrastructure type needed in a section would not actually fit, this will still be proposed but the NPW will warn you to deal with it. You are expected to to deal manually with deliverability problems and LoS problems. For instance, even segregated infrastructure on a high traffic/volume street may not give a high LoS and the stronger infrastructure type to include traffic mitigation measures would be needed.

While drawing, and as the number of drawn routes increases, you should carefully review the network to check that places are properly connected to each other. For instance, at a junction the end point of one line should always be directly connected to the start point of another line, at exactly the same point – not a few metres away. Otherwise you will be creating gaps in the network, which people cannot cycle over, thus lowering the scoring of the performance of the network.

## Level of Service and automatic infrastructure types

By default, each route section is assigned the most appropriate infrastructure type, based on the "cheapest" type that achieves a high Level of Service (LoS).

The LoS is only focused on safety and separation from motor vehicles, not comfort or speed or separation from pedestrians.

LoS depends on four things: the infrastructure type, the marked speed limit of the road, an estimate of the traffic volume along that road (which has various caveats), and whether the road is within a settlement or not

Many of the rules come from table 3.2 of [Cycling by Design](https://www.transport.gov.scot/media/50323/cycling-by-design-update-2019-final-document-15-september-2021-1.pdf).

- **Segregated cycle tracks** follow the table in Cycling by Design, for "Cycle Track at Carriageway Level".
- **Off-road** routes are high LoS by definition, because they are separated from motor vehicles. NPW's LoS definition does not account for path surface or lighting.
- **Painted cycle lanes** follow the table in Cycling by Design; speed and traffic matter
- **Mixed traffic** or **Segregated tracks with traffic measures** always achieve high LoS, by definition. The NPW assumes that transport professionals will include measures at time of implementation that would reduce traffic volume and/or speed to a level that would make mixed traffic or a segregated track appropriate.
- **Shared footways** are scored low LoS when they are within a settlement, and high when they are between settlements. TODO: Subject to change
- **Mixed traffic** follows the CbD table; speed and traffic levels matter.

The automatic infrastructure type recommendation follows this order:

- If the segment is off-road, then use that type. This is automatically detected from existing roads, based on OSM highway type and naming, and may have bugs;
- If mixed traffic is appropriate based on speed and volume, use that;
- If the road is outside a settlement, use a shared footway;
- Otherwise, use a segregated cycle track.

Note there may be two cases where the automatic recommendation has problems:

- When speed and volume are very high, even a segregated track does not achieve high LoS according to CbD guidance. In that case, you must accept this lower LoS, realign the route, or upgrade to the stronger 'Segregated + traffic measures' type.
- Another problem may be that a segregated track does not fit the available streetspace. This is a rough estimate. It is subject to problems with the data and methodology, and may have both false positives and negatives. When it is a real problem, you can switch to the 'Mixed traffic + traffic measures' type, realign the route, or pick a lesser infrastructure type and accept a lower LoS.

## Contextual data layers

When designing a network, one should always be taking into account a range of attributes of the roads/streets/paths being used, whether physical attributes or socio-demographic aspects of an area.

We therefore present a range of data layers on the right-hand side panel, for ease of access. These can be toggled on or off. These match the layers presented in more detail in the [Network Planning Tool (NPT)](https://www.npt.scot/).

Only one contextual layer can be shown at a time, to avoid the situation of an excessively visually-busy map screen.

The available layers are essentially in three categories:

_Infrastructure-related:_

- **Coherent network:** Automatically-generated coherent network in urban areas connecting key origins and destinations with relatively direct routes. A fuller description is provided elsewhere in this manual.
- **Existing infrastructure:** The type of infrastructure currently present on street, e.g. segregated track, shared-use footway, etc.
- **Estimated traffic volume:** The estimated traffic volume, based on an [algorithm](https://github.com/nptscot/osmactive/blob/main/code/traffic-volumes.R).
- **Speed limit:** The known speed limit. This uses the standard national speed limits (e.g. 30 mph for local streets) except where exceptions have been marked (e.g. 20 mph). For service roads we assume 10 mph.
- **Gradient:** The average gradient of the road is shown as a percentage. Steeper roads are a barrier to cycling and affect route choice and the uptake of cycling in the scenarios. Please note in some locations where the network does follow the land contours, e.g. some bridges, the gradient will incorrectly show flat sections of network as steep.
- **Street space:** This provides an analysis of whether a segregated track would fit within the carriageway, or carriageway with verges, or with the footway also included. This dataset uses an Ordnance Survey dataset to determine the edge-to-edge (i.e. from property to opposite property) road width and carriageway-only road width.
- **Attractive streets:** This experimental layer aims to determine the attractiveness of a street based on its proximity to green space. This uses the Ordnance Survey's Open Greenspace dataset. If a road center-line is within 10m of any green space polygon (without any filtering), it is considered attractive. The algorithm is thus currently very simplistic, so should not be relied upon beyond providing a basic indication.
- **NPT full network:** The route network estimates the number of cycle trips on each road. It is designed to emphasise roads with lots of cycling potential and so aid planners in deciding where cycling infrastructure is needed. Full details are given in the [NPT manual](https://npt.scot/manual/#routenetwork).

_Socio-demographic:_

- **Population:** This provides the population density. An option is available to style these based on level of cycling demand.
- **Deprived population (SIMD):** This provides data from the [Scottish Index of Multiple Deprivation 2020](https://simd.scot/). Again, this layer can be styled based on demand (rather than the SIMD level).

_Evaluation-based (i.e. based on what you have drawn):_

- **Level of service:** This applies the resulting Level of Service (LoS) of the network you have drawn to every road/street/path on the map. As you change the map, the effect of your network will be reflected here.
- **Severances:** This evaluates, for each road/street/path, whether a user can or cannot cycle to it without having reached a severance (e.g. a major road without cycle infrastructure).

Fuller details of each of the infrastructure and socio-demographic layers can be seen in the [NPT manual](https://npt.scot/manual/).

Some of the layers can be styled by level of potential/future demand, colouring based on the estimated cycle flows.

## Reachability

Some of the metrics determine if a population zone, town centre, or POI (school, hospital or GP, or greenspace) are reachable from your network.

**TODO** discuss what this means, using the audit doc. Be careful about disconnectd networks.

## Cycling demand

**TODO** Discuss how the demand network is built -- the OD data source, the methods, the lack of gradient data, the difference from what's shown on the NPT site.

**TODO** Commute data from…

**TODO** Utility trips from…

**TODO** School trips excluded, because the zone-to-point isn't public data \[don't think this is still the case?\]

You may not be able to achieve 100% on the cycling demand coverage metrics in the tool. In some cases, there is high demand on parallel roads, like with dual carriageways or a more suitable street parallel to a main road. Even though you have provided an adequate route, this score literally counts coverage on a fixed set of roads.

## Other metrics

**TODO** Anything else not covered, like mesh density

## Known limitations

- **New bridges or links:** You cannot draw new road segments in places where there is no existing infrastructure. It's impossible to model a new bridge over a canal or through a park. If you intend to create a new off-road route parallel to an existing road, then for now, draw along that road.
- **Junctions:** There is currently no modelling of the effect of junctions or the ability to specify an improvement at a junction. At present it is assumed that a designer will incorporate appropriate junction treatments. We hope to address this in a later phase of work.
- **Bidirectionality**: A simplifying assumption throughout NPW is that all infrastructure is bidirectional. When you draw something along a road, it is assumed that people cycling will be able to travel either direction along it, even when the road is one-way for drivers. NPW does not presently differentiate between two one-way segregated cycle tracks on both sides of the road versus a single segregated bidirectional track on one side of the road.
- **Paths alongside motorways:** Similar to the note above about new bridges or links, there is currently no way to represent a proposed new path alongside an existing motorway, because motorway data is not imported (and thus its attributes cannot be changed) and it is not currently possible to draw on new links.
- **Gradients:** Currently a basic interpretation of land height data is used. This means that where the network does follow the land contours, e.g. some bridges, the gradient will incorrectly show flat sections of network as steep. Similarly, paths alongside waterways could end up with oscillating results due to the resolution of the underlying height readings. For the coherent network, which uses CycleStreets routing, a full-scale implementation of gradients has been used and so that dataset is not been subject to these distortions.
- **Attractive streets:** This experimental layer aims to determine the attractiveness of a street based on its proximity to green space, but the algorithm is currently very simplistic. This uses the Ordnance Survey's Open Greenspace dataset. If a road center-line is within 10m of any green space polygon (without any filtering), it is considered attractive. We hope in future to create a more comprehensive analysis.
- **Out-of-boundary drawing:** Currently the NPW disallows drawing outside the boundary of settlements. It is recognised that this can cause issues when a boundary line forms a concave shape, and that the edges of this need a route going into another local authority's boundary. In this scenario, for now, local authority officers should communicate with each other to ensure their collective submissions ensure routes across these areas.
- **Saving work:** Currently work is saved only within your computer's browser rather than to a web service. This means that if you want to switch to another computer, you need to export the data first, then load it into your other computer's browser on the project selection screen.
- **Mobile phone compatibility:** No attempt has been made to adapt the layout of the site to work well on phones, as the ability to plan a network would be hard to do efficiently on a small screen. It is assumed at present that transport professionals will be using a desktop computer or a large tablet. However, we have not actively disabled any functionality on smaller devices, and we are using web standards that should be well-supported on any device.

## OpenStreetMap

[OpenStreetMap](https://www.openstreetmap.org/#map=7/56.514/-3.834) (OSM) is a free, editable map of the whole world that is being built by volunteers largely from scratch, together with data provided by companies and governmental organisations and released with an open-content license.

OSM is widely considered the best available data source for planning of active travel networks, as it contains the most comprehensive network of paths beyond just the street network.

OSM consists only of objective, factual data (e.g. that a particular street exists in a particular location, with particular attributes such as width, surface, etc.), as distinct from objective data (e.g. that a street is considered nice to ride on by some people).

In modelling the world, OSM consists of any real-world feature that physically exists.

The OSM data model fundamentally consists of:

- Geometries, which outline the location of each feature, e.g. the shape of the road based on longitude-latitude co-ordinates. These are either points (Nodes) or lines (Ways). An enclosed area, e.g. a pedestrian area, is just a Way where start and end points are the same location.
- Tags, which are attributes attached to each geometry, e.g. surface type, width, access conditions, presence of particular types of cycle infrastructure. These are represented as key/value pairs. For instance, a C road which is lit at night would include the tags: highway=tertiary, lit=yes.
- Relations, which are used when it is necessary to group together pieces of real world infrastructure. Examples would include a signed cycle route (which consists of a set of roads/streets/paths), or a bus route (which consists of sections of roads/streets which the bus travels through, and the number of the route).

Tags that are most relevant to planning of active travel include:

- [highway=…](https://wiki.openstreetmap.org/wiki/Key:highway#Values), representing the highway type, e.g. trunk, primary, cycleway, footway.
- [access=…](https://wiki.openstreetmap.org/wiki/Key:access), concerned with who can use a piece of infrastructure, e.g. that a pedestrian may or may not be allowed to use a cycleway, or that motor vehicles are not permitted on a track, or that all users are not allowed within a particular private area. Access for particular types of user is often inferred, e.g. a motorway has implied bicycle=no, foot=no, because motorways never permit these types of users.
- [bicycle=yes/no/…](https://wiki.openstreetmap.org/wiki/Key:bicycle), [foot=yes/no/…](https://wiki.openstreetmap.org/wiki/Key:foot), representing more detailed overrides for particular types of street/path infrastructure. This enables for instance, disambiguation between shared-use and dedicated cycle infrastructure.
- [width=…](https://wiki.openstreetmap.org/wiki/Key:width), representing the width of a piece of infrastructure. A variant is [est_width](https://wiki.openstreetmap.org/wiki/Key:est_width) for estimated width.
- [oneway=yes/no](https://wiki.openstreetmap.org/wiki/Key:oneway), representing limited directional movement, which could include a cycle contraflow exempting cycles from such a restriction.
- [surface=…](https://wiki.openstreetmap.org/wiki/Key:surface), describing the physical surface.
- [lit=yes/no](https://wiki.openstreetmap.org/wiki/Key:lit), for whether a location is lit at night.
- [maxspeed=…](https://wiki.openstreetmap.org/wiki/Key:maxspeed), providing the maximum legal speed, though this is usually implied by the highway tag type. For residential streets, the speed limit in the UK is 30 mph but areas such as Edinburgh have the maxspeed value set to 20 mph on the large number of streets where this applies instead.
- [lcn=yes, ncn=yes](https://wiki.openstreetmap.org/wiki/Cycle_routes), as part of a Relation, specifying that a particular road/street/path is part of the local cycle network (e.g. blue signs on street) or the National Cycle Network (i.e. Sustrans routes).

These attributes are used by high-quality routing engines, such as the [CycleStreets](https://www.cyclestreets.net/) routing engine which has been used in the up-front network analysis in the NPT, to determine the level of service on any particular route. For instance, there is a big difference between a Dutch-style 5m-wide segregated route, and a narrow shared-use pavement, or a narrow 1.2m-wide cycle lane on a busy road.

There are various ways to obtain source OSM data:

- Visually, by simply accessing the public map web interface at [www.openstreetmap.org](http://www.openstreetmap.org)
- Downloading GIS data from one of many organisations such as [GeoFabrik](https://download.geofabrik.de/europe/united-kingdom.html), whose website provides free downloads updated on a daily basis, segmented by area. The data has been converted to Shapefile format and then grouped by interest (e.g. roads, water features, natural features, etc.) to match more standard GIS expectations.
- Using a querying interface to obtain selected parts of the data, such as [Overpass Turbo](https://overpass-turbo.eu/)
- Via a wide range of third-party apps, websites, QA tools, and other systems.

## Technical overview

NPW is an entirely self-contained web application, running entirely within your web browser, like a normal website. No downloading of software is required.

We have aimed to make the system as user-friendly and intuitive as possible, for what is arguably a complex task that in the past has perhaps been done with the assistance of highly-trained GIS professionals.

The web application essentially consists of two halves:

- A [user interface](https://github.com/nptscot/npw/tree/main/web) codebase, which users interact with. This uses HTML/Javascript/CSS, as all websites do. As this is a complex web application, we have used the [Svelte framework](https://svelte.dev/) as a means of structuring the system code into logical self-contained highly modular components. These interact with each other, and maintain overall state in a consistent manner. Use of this framework results in a far lower chance of interaction bugs compared to many Javascript-based interactions.
- A [backend](https://github.com/nptscot/npw/tree/main/backend/) codebase, used for computation of network aspects. This uses the programming language Rust, together with WebAssembly (WASM). This means that intensive calculations, such as determining the mesh density of the network, can be done in the background using compiled and thus highly-efficient code directly within the browser. No external web service (API) is involved.

In addition, we make heavy use of the [MapLibre](https://maplibre.org/) mapping framework. This provides the map canvas and handles all the interactions there, such as line drawing or display of points. MapLibre uses the Mapbox Vector Tiles format, a system of presenting large geographical datasets in an efficient manner.

The visual design and user experience design makes use of the [Scottish Government Design System](https://designsystem.gov.scot/), based partly on the [GOV.UK Design System](https://design-system.service.gov.uk/), whose principles of development we have adhered to strongly. For instance, we have used the standardised page elements such as buttons and text styles that local government officers will likely be familiar with.

In following these design principles, we have adhered to the workflow concepts of a single main action to ensure that the intended action at any point is always clear, and that users are taken through in a step-wise manner.

The entire codebase of the system is open source, and [available on Github](https://github.com/nptscot/npw/). Contributions of code, bug reports, design, tests, and documentation, are all warmly welcomed.
