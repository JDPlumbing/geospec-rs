use geospec::*;
use serde_json::json;

#[test]
fn sphere_surface_and_volume() {
    let s = Sphere { radius: 2.0 };
    assert!((s.surface_area() - 4.0 * std::f64::consts::PI * 4.0).abs() < 1e-6);
    assert!((s.volume() - (4.0/3.0) * std::f64::consts::PI * 8.0).abs() < 1e-6);

    let json = s.as_json();
    assert_eq!(json["type"], "sphere");
    assert!(json["surface_area"].as_f64().unwrap() > 0.0);
    assert!(json["volume"].as_f64().unwrap() > 0.0);
}

#[test]
fn box_surface_and_volume() {
    let b = BoxShape { length: 2.0, width: 3.0, height: 4.0 };
    assert_eq!(b.surface_area(), 2.0*(6.0+8.0+12.0));
    assert_eq!(b.volume(), 24.0);

    let json = b.as_json();
    assert_eq!(json["type"], "box");
    assert_eq!(json["volume"], 24.0);
}

#[test]
fn cylinder_inference() {
    let input = json!({
        "type": "cylinder",
        "radius": 1.0,
        "height": 2.0
    });
    let out = infer_from_json(&input).unwrap();
    assert_eq!(out["type"], "cylinder");
    assert!(out["surface_area"].as_f64().unwrap() > 0.0);
    assert!(out["volume"].as_f64().unwrap() > 0.0);
}

#[test]
fn cone_inference() {
    let input = json!({
        "type": "cone",
        "radius": 3.0,
        "height": 4.0
    });
    let out = infer_from_json(&input).unwrap();
    assert_eq!(out["type"], "cone");
    assert!(out["volume"].as_f64().unwrap() > 0.0);
}
