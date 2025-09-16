use geospec::*;

#[test]
fn test_encode_decode_mask() {
    let mask = encode_mask(&["length", "radius", "volume"]);
    let decoded = decode_mask(mask);
    assert!(decoded.contains(&"length"));
    assert!(decoded.contains(&"radius"));
    assert!(decoded.contains(&"volume"));
    assert!(!decoded.contains(&"width"));
}

#[test]
fn test_quaternion_normalization() {
    let q = [2.0, 0.0, 0.0, 0.0];
    let norm = normalize_quaternion(q);
    assert!((norm[0] - 1.0).abs() < 1e-9);
    assert_eq!(norm[1], 0.0);
}

#[test]
fn test_bubble_from_geospec() {
    let mask = encode_mask(&["length", "width"]);
    let values = vec![10.0, 20.0];
    let bubble = bubble_from_geospec(mask, &values);
    assert_eq!(bubble, 10.0); // half of max(10,20)
}

#[test]
fn test_shape_from_spec_generic() {
    let mask = encode_mask(&["length", "width"]);
    let values = vec![5.0, 2.0];
    let shape = shape_from_spec(mask, &values, None);

    assert_eq!(shape.r#type, "generic");
    assert_eq!(shape.dimensions.get("length"), Some(&5.0));
    assert_eq!(shape.dimensions.get("width"), Some(&2.0));
    assert!(shape.bubble > 0.0);
    assert!((shape.orientation[0] - 1.0).abs() < 1e-9);
}

#[test]
fn test_shape_from_spec_pipe() {
    let mask = encode_mask(&["innerDiameter", "outerDiameter", "length"]);
    let values = vec![5.0, 10.0, 100.0];
    let shape = shape_from_spec(mask, &values, None);

    assert_eq!(shape.r#type, "pipe");
}

#[test]
fn test_shape_from_spec_rod() {
    let mask = encode_mask(&["radius", "length"]);
    let values = vec![2.0, 50.0];
    let shape = shape_from_spec(mask, &values, None);

    assert_eq!(shape.r#type, "rod");
}

#[test]
fn test_shape_from_spec_sphere() {
    let mask = encode_mask(&["radius"]);
    let values = vec![10.0];
    let shape = shape_from_spec(mask, &values, None);

    assert_eq!(shape.r#type, "sphere");
}
