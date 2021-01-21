use bpxe;

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
