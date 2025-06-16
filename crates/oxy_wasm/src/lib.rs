use csv::WriterBuilder;
use rstar::{RTree, RTreeObject, PointDistance};
use rstar::primitives::{PointWithData, RectangleWithData};
use flatgeobuf::{Geom, GeomType};
use packed_simd::f64x4;
use serde_wasm_bindgen::SerdeWasmBindgen;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
struct ActivistEvent {
    id: String,
    org: String,
    date: f64,
    sentiment: f32,
    momentum: f32,
    geometry: Vec<f32>,
}

#[wasm_bindgen]
pub struct GeoProcessor {
    events: Vec<ActivistEvent>,
    spatial_index: RTree<RectangleWithData<ActivistEvent>>,
}

#[wasm_bindgen]
impl GeoProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        GeoProcessor {
            events: Vec::new(),
            spatial_index: RTree::new(),
        }
    }

    #[wasm_bindgen]
    pub fn load_data(&mut self, data: &[u8]) -> Result<GeoProcessor, JsValue> {
        let reader = std::io::Cursor::new(data);
        let mut events = Vec::new();
        let mut spatial_index = RTree::new();

        for feature in flatgeobuf::read(reader).map_err(|e| JsValue::from(e))? {
            if let Some(Geom::Point(point)) = feature.geometry.as_point() {
                let event = ActivistEvent {
                    id: feature.id.clone(),
                    org: feature.properties.get("org").unwrap_or("").to_string(),
                    date: feature
                        .properties
                        .get("date")
                        .unwrap_or(&0.0)
                        .parse()
                        .unwrap_or(0.0),
                    sentiment: feature
                        .properties
                        .get("sentiment")
                        .unwrap_or(&0.0)
                        .parse()
                        .unwrap_or(0.0) as f32,
                    momentum: feature
                        .properties
                        .get("momentum")
                        .unwrap_or(&0.0)
                        .parse()
                        .unwrap_or(0.0) as f32,
                    geometry: point.clone(),
                };
                events.push(event);
                spatial_index.insert(RectangleWithData {
                    mbr: [point[0], point[1], point[0], point[1]],
                    data: event.clone(),
                });
            }
        }

        self.events = events;
        self.spatial_index = spatial_index;

        Ok(GeoProcessor {
            events: self.events.clone(),
            spatial_index: self.spatial_index.clone(),
        })
    }

    #[wasm_bindgen]
    pub fn apply_filters(
        &self,
        org: &str,
        date_range: [f64; 2],
        sentiment_range: [f64; 2],
    ) -> JsValue {
        let mut filtered_events = Vec::new();

        for event in &self.events {
            if event.org == org
                && event.date >= date_range[0]
                && event.date <= date_range[1]
                && event.sentiment as f64 >= sentiment_range[0]
                && event.sentiment as f64 <= sentiment_range[1]
            {
                filtered_events.push(event);
            }
        }

        let geojson = serde_json::to_string(&filtered_events).unwrap();
        JsValue::from(geojson)
    }

    #[wasm_bindgen]
    pub fn export_csv(&self) -> Vec<u8> {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.write_record(&["id", "org", "date", "sentiment", "momentum", "geometry"])
            .unwrap();

        for event in &self.events {
            wtr.serialize((
                event.id.clone(),
                event.org.clone(),
                event.date,
                event.sentiment,
                event.momentum,
                event
                    .geometry
                    .iter()
                    .map(|&f| f.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ))
            .unwrap();
        }

        wtr.finish().into_inner()
    }

    #[wasm_bindgen]
    pub fn memory_footprint(&self) -> u32 {
        self.events.len() as u32 * std::mem::size_of::<ActivistEvent>() as u32
    }
}
