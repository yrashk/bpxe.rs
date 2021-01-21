use bpxe;
use bpxe::bpmn::schema::Definitions;

#[test]
#[ignore] // FIXME: https://github.com/bpxe/bpxe.rs/issues/1
fn parse_sample() {
    let sample = include_str!("fixtures/sample.bpmn");
    let definitions = bpxe::bpmn::parse(sample).unwrap();
    assert!(definitions.root_elements.len() > 0);
}

#[test]
fn normalization() {
    let sample = include_str!("fixtures/sample_ns.bpmn");
    let definitions = bpxe::bpmn::parse(&sample).unwrap();
    assert!(definitions.root_elements.len() > 0);
}

#[test]
fn serialize_deserialize_serde_yaml() {
    let sample = include_str!("fixtures/sample_ns.bpmn");
    let definitions = bpxe::bpmn::parse(&sample).unwrap();
    let yaml = serde_yaml::to_string(&definitions).unwrap();
    let definitions_1: Definitions = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(definitions_1, definitions);
}

#[test]
fn serialize_deserialize_serde_json() {
    let sample = include_str!("fixtures/sample_ns.bpmn");
    let definitions = bpxe::bpmn::parse(&sample).unwrap();
    let json = serde_json::to_string(&definitions).unwrap();
    let definitions_1: Definitions = serde_json::from_str(&json).unwrap();
    assert_eq!(definitions_1, definitions);
}

#[test]
#[ignore] // FIXME: https://github.com/bpxe/bpxe.rs/issues/3
fn serialize_deserialize_serde_toml() {
    let sample = include_str!("fixtures/sample_ns.bpmn");
    let definitions = bpxe::bpmn::parse(&sample).unwrap();
    let toml = toml::to_string(&definitions).unwrap();
    let definitions_1: Definitions = toml::from_str(&toml).unwrap();
    assert_eq!(definitions_1, definitions);
}

#[test]
fn serialize_deserialize_serde_ron() {
    let sample = include_str!("fixtures/sample_ns.bpmn");
    let definitions = bpxe::bpmn::parse(&sample).unwrap();
    let ron = ron::to_string(&definitions).unwrap();
    let definitions_1: Definitions = ron::from_str(&ron).unwrap();
    assert_eq!(definitions_1, definitions);
}

#[test]
fn serialize_deserialize_serde_message_pack() {
    let sample = include_str!("fixtures/sample_ns.bpmn");
    let definitions = bpxe::bpmn::parse(&sample).unwrap();
    let msgpack = rmp_serde::to_vec(&definitions).unwrap();
    let definitions_1: Definitions = rmp_serde::from_read(std::io::Cursor::new(msgpack)).unwrap();
    assert_eq!(definitions_1, definitions);
}
