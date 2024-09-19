use std::collections::HashSet;

use anyhow::Result;
use geojson::{feature::Id, GeoJson};
use graph::RoadID;

use crate::{MapModel, Route};

impl MapModel {
    pub fn add_route(&mut self, route: Route) -> Result<usize> {
        // Check for overlaps
        let used_roads: HashSet<RoadID> = self
            .routes
            .values()
            .flat_map(|route| route.roads.clone())
            .collect();
        if route.roads.iter().any(|r| used_roads.contains(r)) {
            bail!("Another route already crosses the same road");
        }

        let id = self.id_counter;
        self.routes.insert(id, route);
        self.id_counter += 1;
        Ok(id)
    }

    pub fn delete_route(&mut self, id: usize) -> Result<()> {
        if self.routes.remove(&id).is_some() {
            return Ok(());
        }
        bail!("Unknown route {id}");
    }

    pub fn edit_route_details(&mut self, id: usize, name: String, notes: String) -> Result<()> {
        let Some(route) = self.routes.get_mut(&id) else {
            bail!("Unknown route {id}");
        };
        route.name = name;
        route.notes = notes;
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
