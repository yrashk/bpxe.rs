use bpxe;

#[test]
fn parse_sample() {
    let sample = include_str!("fixtures/sample.bpmn");
    let definitions = bpxe::bpmn::parse(sample).unwrap();
    assert!(definitions.root_elements.len() > 0);
}

#[test]
fn normalization() {
    let sample = include_str!("fixtures/sample_ns.bpmn");
    let normalized = bpxe::bpmn::normalize(sample).unwrap();
    let definitions = bpxe::bpmn::parse(&normalized).unwrap();
    assert!(definitions.root_elements.len() > 0);
}
