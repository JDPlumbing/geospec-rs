pub const PARAMS: [&str; 21] = [
    "length", "width", "height", "depth", "radius",
    "innerDiameter", "outerDiameter", "wallThickness", "angle", "curvature",
    "majorRadius", "minorRadius", "vertexCount", "edgeLength", "apothem",
    "surfaceArea", "volume", "crossSectionArea", "taperRatio", "aspectRatio",
    "slendernessRatio",
];

/// Encode a list of parameter names into a 32-bit mask.
pub fn encode_mask(params: &[&str]) -> u32 {
    let mut mask = 0u32;
    for p in params {
        if let Some(idx) = PARAMS.iter().position(|&name| name == *p) {
            mask |= 1 << idx;
        }
    }
    mask
}

/// Decode a mask back into parameter names.
pub fn decode_mask(mask: u32) -> Vec<&'static str> {
    PARAMS.iter()
        .enumerate()
        .filter_map(|(idx, &p)| if (mask & (1 << idx)) != 0 { Some(p) } else { None })
        .collect()
}

/// Normalize a quaternion to length 1.
pub fn normalize_quaternion(q: [f64; 4]) -> [f64; 4] {
    let mag = (q[0]*q[0] + q[1]*q[1] + q[2]*q[2] + q[3]*q[3]).sqrt();
    if mag == 0.0 {
        [1.0, 0.0, 0.0, 0.0]
    } else {
        [q[0]/mag, q[1]/mag, q[2]/mag, q[3]/mag]
    }
}

/// Compute bubble radius from values.
pub fn bubble_from_geospec(mask: u32, values: &[f64]) -> f64 {
    if mask == 0 || values.is_empty() {
        return 0.0;
    }
    let max_val = values.iter().map(|v| v.abs()).fold(0.0, f64::max);
    max_val * 0.5
}

/// Shape type & bubble inference
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ShapeSpec {
    pub r#type: String,
    pub dimensions: std::collections::HashMap<&'static str, f64>,
    pub bubble: f64,
    pub orientation: [f64; 4],
}

pub fn shape_from_spec(mask: u32, values: &[f64], orientation: Option<[f64; 4]>) -> ShapeSpec {
    let keys = decode_mask(mask);
    let mut dims = std::collections::HashMap::new();
    for (i, k) in keys.iter().enumerate() {
        if let Some(&val) = values.get(i) {
            dims.insert(*k, val);
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
