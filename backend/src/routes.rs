use std::collections::HashSet;

use anyhow::Result;
use geojson::{feature::Id, GeoJson};
use graph::RoadID;

use crate::{MapModel, Route};

impl MapModel {
    /// Returns the route ID
    pub fn set_route(&mut self, edit_id: Option<usize>, route: Route) -> Result<usize> {
        if let Some(id) = edit_id {
            if self.routes.remove(&id).is_none() {
                bail!("Unknown route {id}");
            }
        }

        // Check for overlaps
        let used_roads: HashSet<RoadID> = self
            .routes
            .values()
            .flat_map(|route| route.roads.clone())
            .collect();
        if route.roads.iter().any(|r| used_roads.contains(r)) {
            bail!("Another route already crosses the same road");
        }

        let id = match edit_id {
            Some(id) => id,
            None => {
                let id = self.id_counter;
                self.id_counter += 1;
                id
            }
        };
        self.routes.insert(id, route);
        Ok(id)
    }

    pub fn delete_route(&mut self, id: usize) -> Result<()> {
        if self.routes.remove(&id).is_some() {
            return Ok(());
        }
        bail!("Unknown route {id}");
    }

    pub fn to_routes_gj(&self) -> GeoJson {
        let mut features = Vec::new();
        for (id, route) in &self.routes {
            let mut f = route.feature.clone();
            f.id = Some(Id::Number((*id).into()));
            f.set_property("name", route.name.clone());
            f.set_property("notes", route.notes.clone());
            f.set_property(
                "infra_type",
                serde_json::to_value(&route.infra_type).unwrap(),
            );
            features.push(f);
        }
        GeoJson::from(features)
    }
}
