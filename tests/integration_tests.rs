use geospec::*;

#[test]
fn test_encode_decode() {
    let mask = encode_mask(&["length", "radius"]);
    let decoded = decode_mask(mask);
    assert!(decoded.contains(&"length"));
    assert!(decoded.contains(&"radius"));
}

#[test]
fn test_quaternion_identity() {
    let q = normalize_quaternion([0.0, 0.0, 0.0, 0.0]);
    assert_eq!(q, [1.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_shape_inference() {
    let mask = encode_mask(&["radius"]);
    let values = vec![5.0];
    let shape = shape_from_spec(mask, &values, None);
    assert_eq!(shape.r#type, "sphere");
}
