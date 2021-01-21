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
fn serialize_deserialize_serde() {
    let sample = include_str!("fixtures/sample_ns.bpmn");
    let definitions = bpxe::bpmn::parse(&sample).unwrap();
    let yaml = serde_yaml::to_string(&definitions).unwrap();
    let definitions_1: Definitions = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(definitions_1, definitions);
}
