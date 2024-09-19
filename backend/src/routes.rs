use std::collections::HashSet;

use anyhow::Result;
use geojson::{feature::Id, GeoJson};
use graph::RoadID;

use crate::{MapModel, Route};

impl MapModel {
    pub fn add_route(&mut self, route: Route) -> Result<()> {
        // Check for overlaps
        let used_roads: HashSet<RoadID> = self
            .routes
            .values()
            .flat_map(|route| route.roads.clone())
            .collect();
        if route.roads.iter().any(|r| used_roads.contains(r)) {
            bail!("Another route already crosses the same road");
        }

        self.routes.insert(self.id_counter, route);
        self.id_counter += 1;

        Ok(())
    }

    pub fn to_routes_gj(&self) -> GeoJson {
        let mut features = Vec::new();
        for (id, route) in &self.routes {
            let mut f = route.feature.clone();
            f.id = Some(Id::Number((*id).into()));
            f.set_property("name", route.name.clone());
            f.set_property("notes", route.notes.clone());
            features.push(f);
        }
        GeoJson::from(features)
    }
}
