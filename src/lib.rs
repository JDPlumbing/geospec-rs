use std::collections::HashMap;

pub const PARAMS: [&str; 21] = [
    "length",          // 0
    "width",           // 1
    "height",          // 2
    "depth",           // 3
    "radius",          // 4
    "innerDiameter",   // 5
    "outerDiameter",   // 6
    "wallThickness",   // 7
    "angle",           // 8
    "curvature",       // 9
    "majorRadius",     // 10
    "minorRadius",     // 11
    "vertexCount",     // 12
    "edgeLength",      // 13
    "apothem",         // 14
    "surfaceArea",     // 15
    "volume",          // 16
    "crossSectionArea",// 17
    "taperRatio",      // 18
    "aspectRatio",     // 19
    "slendernessRatio" // 20
];

/// Encode parameter names into a 32-bit mask.
pub fn encode_mask(params: &[&str]) -> u32 {
    let mut mask = 0u32;
    for p in params {
        if let Some(idx) = PARAMS.iter().position(|&x| x == *p) {
            mask |= 1 << idx;
        }
    }
    mask
}

/// Decode a mask back into parameter names.
pub fn decode_mask(mask: u32) -> Vec<&'static str> {
    let mut active = Vec::new();
    for (idx, p) in PARAMS.iter().enumerate() {
        if (mask & (1 << idx)) != 0 {
            active.push(*p);
        }
    }
    active
}

/// Normalize a quaternion so its length = 1.
pub fn normalize_quaternion(q: [f64; 4]) -> [f64; 4] {
    let [w, x, y, z] = q;
    let mag = (w*w + x*x + y*y + z*z).sqrt();
    if mag == 0.0 {
        [1.0, 0.0, 0.0, 0.0]
    } else {
        [w/mag, x/mag, y/mag, z/mag]
    }
}

/// Compute a bubble radius from a GeoSpec.
pub fn bubble_from_geospec(_mask: u32, values: &[f64]) -> f64 {
    if values.is_empty() { return 0.0; }
    let max_val = values.iter().map(|v| v.abs()).fold(0.0, f64::max);
    max_val * 0.5
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ShapeSpec {
    pub r#type: String,
    pub dimensions: HashMap<String, f64>,  // 👈 String keys (fixes lifetime issue)
    pub bubble: f64,
    pub orientation: [f64; 4],
}

/// Build a shape spec from mask + values + optional quaternion.
pub fn shape_from_spec(mask: u32, values: &[f64], orientation: Option<[f64; 4]>) -> ShapeSpec {
    let keys = decode_mask(mask);
    let mut dims = HashMap::new();

    for (i, k) in keys.iter().enumerate() {
        if let Some(val) = values.get(i) {
            dims.insert(k.to_string(), *val);
        }
    }

    // crude type inference
    let mut r#type = "generic".to_string();
    if keys.contains(&"innerDiameter") && keys.contains(&"outerDiameter") && keys.contains(&"length") {
        r#type = "pipe".to_string();
    } else if keys.contains(&"radius") && keys.contains(&"length") {
        r#type = "rod".to_string();
    } else if keys.len() == 1 && keys[0] == "radius" {
        r#type = "sphere".to_string();
    }

    let bubble = bubble_from_geospec(mask, values);
    let orient = normalize_quaternion(orientation.unwrap_or([1.0, 0.0, 0.0, 0.0]));

    ShapeSpec { r#type, dimensions: dims, bubble, orientation: orient }
}
