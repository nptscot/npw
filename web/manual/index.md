# Network Planning Workspace User Manual

[Back to NPW](./)

Not ready yet

## Technical details

These are some notes about how NPW works (not how to use it). Maybe this will be a separate appendix, or maybe the user guide will have collapsible/optional sections. An appropriate tone/style will happen later.

### The editable network

NPW models a network, consisting of road segments leading between exactly two intersections. The network is built from OpenStreetMap (OSM). Note that OSM is constantly updated, but changes there are not reflected in NPW until the development team regenerates NPW base data. The OSM data in NPW is from **TODO** .

Only roads that should be candidates for a cycling network are included in NPW.

- Motorways are excluded (**TODO verify**), because they will never be appropriate for cycling infrastructure. (We may need to support new off-road segments that could be parallel.)
- Most OSM highway types for roads are included
- Anything currently under construction is excluded
- Steps/stairs are excluded, because unless this becomes an appropriate ramp, this can never be acceptable infrastructure
- When OSM has pedestrian-oriented infrastructure, it's only included sometimes, depending on tagging. (**TODO get specific**)

Currently all routes in NPW must follow one of these existing road segments. You cannot draw new road segments. It's impossible to model a new bridge over a canal or through a park. If you intend to create a new off-road route parallel to an existing road, then for now, draw along that road.

A simplifying assumption throughout NPW is that all infrastructure is **bidirectional** . When you draw something along a road, it's assumed that cyclists will be able to travel either direction along it, **even when the road is one-way for drivers** . NPW does not care about the difference beteen two segregated cycletracks on both sides of the road, versus one segregated bidirectional track on one side of the road.

### Starting a network

You can start with a completely blank network, or you can initially seed your network from a few different sources.

You can include existing infrastructure from OSM. There is lots of existing infrastructure mapped in OSM that doesn't form part of an appropriate network, so there are different options to filter for only the appropriate things. **TODO describe the two options** . If you start from these options, you should audit everything to make sure it really is compliant. If something is incorrect or missing, you should **TODO do something** .

You can also start with the **core network** , which is a suggested network automatically produced by the NPT team. **TODO link to its methodology** . There are many caveats with this network, so you should not just blindly trust it.

### Drawing routes

To draw a route, you pick a sequence of two or more waypoints on the map. The route between them is the most direct (shortest distance). If the route alignment is incorrect, you can drag one of the small white dots and create a new waypoint. All of the waypoints "snap" to an intersection. When you draw in the primary or secondary tier, by default, these waypoints try to snap to major junctions, shown as small grey dots. You can adjust this by zooming in and dragging a waypoint.

The route you draw will be split into sections when:

- the route crosses another existing route
- the auto-recommended or manual infrastructure type changes
- the infrastructure type does or does not fit in the available streetspace
- the tier changes (based on whether the road is inside a settlement or not)

After saving a route, these sections become independent. You can edit each one, but the original longer route you drew is lost.

### Level of Service and automatic infrastructure types

By default, each route section is assigned the most appropriate infrastructure type, based on the "cheapest" type that achieves a high Level of Service (LoS). The LoS is only focused on safety and separation from motor vehicles, not comfort or speed or separation from pedestrians. LoS depends on four things -- the infrastructure type, the marked speed limit of the road, an estimate of the traffic volume along that road (which has many caveats), and whether the road is within a settlement or not. Many of the rules come from table 3.2 of [Cycling by Design](https://www.transport.gov.scot/media/50323/cycling-by-design-update-2019-final-document-15-september-2021-1.pdf).

- **Mixed traffic** follows the CbD table; speed and traffic matter
- **Segregated cycletracks** follow the table too, for "Cycle Track at Carriageway Level".
- **Painted cycle lanestraffic** follow the CbD table; speed and traffic matter
- Mixed traffic or segregated tracks **with traffic measures** always achieve high LoS, by definition. This is used to model external plans that NPW doesn't know about to reduce traffic volume and/or speed to the level that makes mixed traffic or a segregated track appropriate.
- **Off-road** routes are high LoS by definition, because they are separated from motor vehicles. NPW's LoS definition does not account for path surface or lighting.
- **Shared footways** are low LoS when they are within a settlement, and high when they're out.

The automatic infrastructure type recommendation follows this order:

1.  If the segment is off-road, then use that type. This is automatically detected from existing roads, based on OSM highway type and naming, and may have bugs.
2.  If mixed traffic is appropriate based on speed and volume, use that
3.  If the road is outside a settlement, use a shared footway
4.  Otherwise, use a segregated cycle track

Note there may be two cases where the automatic recommendation has problems. When speed and volume are every high, even a segregated track doesn't achieve high LoS according to CbD guidance. In tat case, you must accept this lower LoS, realign the route, or upgrade to the stronger segregated + traffic measures type.

Another problem may be that a segregated track does not fit the available streetspace. This is a rough guess; it's subject to problems with the data and methodology, and may have both false positives and negatives. When it is a real problem, you can switch to mixed traffic + traffic measures, realign the route, or pick a smaller infrastructure and accept lower LoS.

### Layers

**TODO** . For each layer, list the data sources, limits, etc. Maybe some of this info should get inlined as help in the app too. For example for greenspaces, which are included? How do we determine attractive streets?

### Reachability

Some of the metrics determine if a population zone, town centre, or POI (school, hospital or GP, or greenspace) are **reachable** from your network. **TODO** discuss what this means, using the audit doc. Be careful about disconnectd networks.

### Cycling demand

Discuss how the demand network is built -- the OD data source, the methods, the lack of gradient data, the difference from what's shown on the NPT site.

Commute data from...

Utility trips from...

School trips excluded, because the zone-to-point isn't public data.

### Other metrics

Anything else not covered, like mesh density
