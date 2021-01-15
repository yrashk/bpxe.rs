// This file is generated from BPMN 2.0 schema using `codegen.sh` script
use std::borrow::Cow;
use strong_xml::XmlRead;

/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:definitions")]
pub struct Definitions<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "targetNamespace")]
    pub target_namespace: URI,
    #[xml(attr = "expressionLanguage")]
    pub expression_language: Option<URI>,
    #[xml(attr = "typeLanguage")]
    pub type_language: Option<URI>,
    #[xml(attr = "exporter")]
    pub exporter: Option<Cow<'a, str>>,
    #[xml(attr = "exporterVersion")]
    pub exporter_version: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:import")]
    pub imports: Vec<Import<'a>>,
    #[xml(child = "bpmn:extension")]
    pub extensions: Vec<Extension<'a>>,
    #[xml(
        child = "bpmn:category",
        child = "bpmn:collaboration",
        child = "bpmn:correlationProperty",
        child = "bpmn:dataStore",
        child = "bpmn:endPoint",
        child = "bpmn:error",
        child = "bpmn:escalation",
        child = "bpmn:eventDefinition",
        child = "bpmn:globalBusinessRuleTask",
        child = "bpmn:globalManualTask",
        child = "bpmn:globalScriptTask",
        child = "bpmn:globalTask",
        child = "bpmn:globalUserTask",
        child = "bpmn:interface",
        child = "bpmn:itemDefinition",
        child = "bpmn:message",
        child = "bpmn:partnerEntity",
        child = "bpmn:partnerRole",
        child = "bpmn:process",
        child = "bpmn:resource",
        child = "bpmn:signal"
    )]
    pub root_elements: Vec<RootElement<'a>>,
    #[xml(child = "bpmn:relationship")]
    pub relationships: Vec<Relationship<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:import")]
pub struct Import<'a> {
    #[xml(attr = "namespace")]
    pub namespace: URI,
    #[xml(attr = "location")]
    pub location: Cow<'a, str>,
    #[xml(attr = "importType")]
    pub import_type: URI,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:activity")]
pub struct Activity<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:adHocSubProcess")]
pub struct AdHocSubProcess<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
    #[xml(attr = "triggeredByEvent")]
    pub triggered_byevent: Option<bool>,
    #[xml(child = "bpmn:laneSet")]
    pub lane_sets: Vec<LaneSet<'a>>,
    #[xml(
        child = "bpmn:adHocSubProcess",
        child = "bpmn:boundaryEvent",
        child = "bpmn:businessRuleTask",
        child = "bpmn:callActivity",
        child = "bpmn:callChoreography",
        child = "bpmn:choreographyTask",
        child = "bpmn:complexGateway",
        child = "bpmn:dataObject",
        child = "bpmn:dataObjectReference",
        child = "bpmn:dataStoreReference",
        child = "bpmn:endEvent",
        child = "bpmn:event",
        child = "bpmn:eventBasedGateway",
        child = "bpmn:exclusiveGateway",
        child = "bpmn:implicitThrowEvent",
        child = "bpmn:inclusiveGateway",
        child = "bpmn:intermediateCatchEvent",
        child = "bpmn:intermediateThrowEvent",
        child = "bpmn:manualTask",
        child = "bpmn:parallelGateway",
        child = "bpmn:receiveTask",
        child = "bpmn:scriptTask",
        child = "bpmn:sendTask",
        child = "bpmn:sequenceFlow",
        child = "bpmn:serviceTask",
        child = "bpmn:startEvent",
        child = "bpmn:subChoreography",
        child = "bpmn:subProcess",
        child = "bpmn:task",
        child = "bpmn:transaction",
        child = "bpmn:userTask"
    )]
    pub flow_elements: Vec<FlowElement<'a>>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact<'a>>,
    #[xml(attr = "cancelRemainingInstances")]
    pub cancel_remaining_instances: Option<bool>,
    #[xml(attr = "ordering")]
    pub ordering: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:completionCondition")]
    pub completion_condition: Option<CompletionCondition<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:completionCondition")]
pub struct CompletionCondition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:artifact")]
pub enum Artifact<'a> {
    #[xml(tag = "bpmn:association")]
    Association(Association<'a>),
    #[xml(tag = "bpmn:group")]
    Group(Group<'a>),
    #[xml(tag = "bpmn:textAnnotation")]
    TextAnnotation(TextAnnotation<'a>),
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:assignment")]
pub struct Assignment<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(child = "bpmn:from")]
    pub from: From<'a>,
    #[xml(child = "bpmn:to")]
    pub to: To<'a>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:from")]
pub struct From<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:to")]
pub struct To<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:association")]
pub struct Association<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "sourceRef")]
    pub source_ref: Cow<'a, str>,
    #[xml(attr = "targetRef")]
    pub target_ref: Cow<'a, str>,
    #[xml(attr = "associationDirection")]
    pub association_direction: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:auditing")]
pub struct Auditing<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:baseElement")]
pub struct BaseElement<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:baseElementWithMixedContent")]
pub struct BaseElementWithMixedContent<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:boundaryEvent")]
pub struct BoundaryEvent<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(attr = "parallelMultiple")]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    pub data_outputs: Vec<DataOutput<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:outputSet")]
    pub output_set: Option<OutputSet<'a>>,
    #[xml(
        child = "bpmn:cancelEventDefinition",
        child = "bpmn:compensateEventDefinition",
        child = "bpmn:conditionalEventDefinition",
        child = "bpmn:errorEventDefinition",
        child = "bpmn:escalationEventDefinition",
        child = "bpmn:linkEventDefinition",
        child = "bpmn:messageEventDefinition",
        child = "bpmn:signalEventDefinition",
        child = "bpmn:terminateEventDefinition",
        child = "bpmn:timerEventDefinition"
    )]
    pub event_definitions: Vec<EventDefinition<'a>>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef<'a>>,
    #[xml(attr = "cancelActivity")]
    pub cancel_activity: Option<bool>,
    #[xml(attr = "attachedToRef")]
    pub attached_toref: Cow<'a, str>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:businessRuleTask")]
pub struct BusinessRuleTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
    #[xml(attr = "implementation")]
    pub implementation: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:callableElement")]
pub struct CallableElement<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef<'a>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:supportedInterfaceRef")]
pub struct SupportedInterfaceRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:callActivity")]
pub struct CallActivity<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
    #[xml(attr = "calledElement")]
    pub called_element: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:callChoreography")]
pub struct CallChoreography<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "initiatingParticipantRef")]
    pub initiating_participant_ref: Cow<'a, str>,
    #[xml(attr = "loopType")]
    pub loop_type: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef<'a>>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey<'a>>,
    #[xml(attr = "calledChoreographyRef")]
    pub called_choreography_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:participantAssociation")]
    pub participant_associations: Vec<ParticipantAssociation<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:callConversation")]
pub struct CallConversation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef<'a>>,
    #[xml(child = "bpmn:messageFlowRef")]
    pub message_flow_refs: Vec<MessageFlowRef<'a>>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey<'a>>,
    #[xml(attr = "calledCollaborationRef")]
    pub called_collaboration_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:participantAssociation")]
    pub participant_associations: Vec<ParticipantAssociation<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:cancelEventDefinition")]
pub struct CancelEventDefinition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:catchEvent")]
pub struct CatchEvent<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(attr = "parallelMultiple")]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    pub data_outputs: Vec<DataOutput<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:outputSet")]
    pub output_set: Option<OutputSet<'a>>,
    #[xml(
        child = "bpmn:cancelEventDefinition",
        child = "bpmn:compensateEventDefinition",
        child = "bpmn:conditionalEventDefinition",
        child = "bpmn:errorEventDefinition",
        child = "bpmn:escalationEventDefinition",
        child = "bpmn:linkEventDefinition",
        child = "bpmn:messageEventDefinition",
        child = "bpmn:signalEventDefinition",
        child = "bpmn:terminateEventDefinition",
        child = "bpmn:timerEventDefinition"
    )]
    pub event_definitions: Vec<EventDefinition<'a>>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:eventDefinitionRef")]
pub struct EventDefinitionRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:category")]
pub struct Category<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:categoryValue")]
    pub category_values: Vec<CategoryValue<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:categoryValue")]
pub struct CategoryValue<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "value")]
    pub value: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:choreography")]
pub struct Choreography<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "isClosed")]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    pub participants: Vec<Participant<'a>>,
    #[xml(child = "bpmn:messageFlow")]
    pub message_flows: Vec<MessageFlow<'a>>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact<'a>>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    pub conversation_nodes: Vec<ConversationNode<'a>>,
    #[xml(child = "bpmn:conversationAssociation")]
    pub conversation_associations: Vec<ConversationAssociation<'a>>,
    #[xml(child = "bpmn:participantAssociation")]
    pub participant_associations: Vec<ParticipantAssociation<'a>>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    pub message_flow_associations: Vec<MessageFlowAssociation<'a>>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey<'a>>,
    #[xml(child = "bpmn:choreographyRef")]
    pub choreography_refs: Vec<ChoreographyRef<'a>>,
    #[xml(child = "bpmn:conversationLink")]
    pub conversation_links: Vec<ConversationLink<'a>>,
    #[xml(
        child = "bpmn:adHocSubProcess",
        child = "bpmn:boundaryEvent",
        child = "bpmn:businessRuleTask",
        child = "bpmn:callActivity",
        child = "bpmn:callChoreography",
        child = "bpmn:choreographyTask",
        child = "bpmn:complexGateway",
        child = "bpmn:dataObject",
        child = "bpmn:dataObjectReference",
        child = "bpmn:dataStoreReference",
        child = "bpmn:endEvent",
        child = "bpmn:event",
        child = "bpmn:eventBasedGateway",
        child = "bpmn:exclusiveGateway",
        child = "bpmn:implicitThrowEvent",
        child = "bpmn:inclusiveGateway",
        child = "bpmn:intermediateCatchEvent",
        child = "bpmn:intermediateThrowEvent",
        child = "bpmn:manualTask",
        child = "bpmn:parallelGateway",
        child = "bpmn:receiveTask",
        child = "bpmn:scriptTask",
        child = "bpmn:sendTask",
        child = "bpmn:sequenceFlow",
        child = "bpmn:serviceTask",
        child = "bpmn:startEvent",
        child = "bpmn:subChoreography",
        child = "bpmn:subProcess",
        child = "bpmn:task",
        child = "bpmn:transaction",
        child = "bpmn:userTask"
    )]
    pub flow_elements: Vec<FlowElement<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:choreographyActivity")]
pub struct ChoreographyActivity<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "initiatingParticipantRef")]
    pub initiating_participant_ref: Cow<'a, str>,
    #[xml(attr = "loopType")]
    pub loop_type: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef<'a>>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:participantRef")]
pub struct ParticipantRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:choreographyTask")]
pub struct ChoreographyTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "initiatingParticipantRef")]
    pub initiating_participant_ref: Cow<'a, str>,
    #[xml(attr = "loopType")]
    pub loop_type: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef<'a>>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey<'a>>,
    #[xml(child = "bpmn:messageFlowRef")]
    pub message_flow_ref: MessageFlowRef<'a>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messageFlowRef")]
pub struct MessageFlowRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:collaboration")]
pub struct Collaboration<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "isClosed")]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    pub participants: Vec<Participant<'a>>,
    #[xml(child = "bpmn:messageFlow")]
    pub message_flows: Vec<MessageFlow<'a>>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact<'a>>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    pub conversation_nodes: Vec<ConversationNode<'a>>,
    #[xml(child = "bpmn:conversationAssociation")]
    pub conversation_associations: Vec<ConversationAssociation<'a>>,
    #[xml(child = "bpmn:participantAssociation")]
    pub participant_associations: Vec<ParticipantAssociation<'a>>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    pub message_flow_associations: Vec<MessageFlowAssociation<'a>>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey<'a>>,
    #[xml(child = "bpmn:choreographyRef")]
    pub choreography_refs: Vec<ChoreographyRef<'a>>,
    #[xml(child = "bpmn:conversationLink")]
    pub conversation_links: Vec<ConversationLink<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:choreographyRef")]
pub struct ChoreographyRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:compensateEventDefinition")]
pub struct CompensateEventDefinition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "waitForCompletion")]
    pub wait_for_completion: Option<bool>,
    #[xml(attr = "activityRef")]
    pub activity_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:complexBehaviorDefinition")]
pub struct ComplexBehaviorDefinition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(child = "bpmn:condition")]
    pub condition: Condition<'a>,
    #[xml(child = "bpmn:event")]
    pub event: Option<Event<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:condition")]
pub struct Condition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "language")]
    pub language: Option<URI>,
    #[xml(attr = "evaluatesToTypeRef")]
    pub evaluates_totype_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:event")]
pub struct Event<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInput")]
    pub data_inputs: Vec<DataInput<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:inputSet")]
    pub input_set: Option<InputSet<'a>>,
    #[xml(
        child = "bpmn:cancelEventDefinition",
        child = "bpmn:compensateEventDefinition",
        child = "bpmn:conditionalEventDefinition",
        child = "bpmn:errorEventDefinition",
        child = "bpmn:escalationEventDefinition",
        child = "bpmn:linkEventDefinition",
        child = "bpmn:messageEventDefinition",
        child = "bpmn:signalEventDefinition",
        child = "bpmn:terminateEventDefinition",
        child = "bpmn:timerEventDefinition"
    )]
    pub event_definitions: Vec<EventDefinition<'a>>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:complexGateway")]
pub struct ComplexGateway<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "gatewayDirection")]
    pub gateway_direction: Option<Cow<'a, str>>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:activationCondition")]
    pub activation_condition: Option<ActivationCondition<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:activationCondition")]
pub struct ActivationCondition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conditionalEventDefinition")]
pub struct ConditionalEventDefinition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(child = "bpmn:condition")]
    pub condition: Condition<'a>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conversation")]
pub struct Conversation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef<'a>>,
    #[xml(child = "bpmn:messageFlowRef")]
    pub message_flow_refs: Vec<MessageFlowRef<'a>>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conversationAssociation")]
pub struct ConversationAssociation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "innerConversationNodeRef")]
    pub inner_conversation_node_ref: Cow<'a, str>,
    #[xml(attr = "outerConversationNodeRef")]
    pub outer_conversation_node_ref: Cow<'a, str>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conversationLink")]
pub struct ConversationLink<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "sourceRef")]
    pub source_ref: Cow<'a, str>,
    #[xml(attr = "targetRef")]
    pub target_ref: Cow<'a, str>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conversationNode")]
pub enum ConversationNode<'a> {
    #[xml(tag = "bpmn:callConversation")]
    CallConversation(CallConversation<'a>),
    #[xml(tag = "bpmn:conversation")]
    Conversation(Conversation<'a>),
    #[xml(tag = "bpmn:subConversation")]
    SubConversation(SubConversation<'a>),
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationKey")]
pub struct CorrelationKey<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:correlationPropertyRef")]
    pub correlation_property_refs: Vec<CorrelationPropertyRef<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationPropertyRef")]
pub struct CorrelationPropertyRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationProperty")]
pub struct CorrelationProperty<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "type")]
    pub typ: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:correlationPropertyRetrievalExpression")]
    pub correlation_property_retrieval_expressions: Vec<CorrelationPropertyRetrievalExpression<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationPropertyBinding")]
pub struct CorrelationPropertyBinding<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "correlationPropertyRef")]
    pub correlation_property_ref: Cow<'a, str>,
    #[xml(child = "bpmn:dataPath")]
    pub data_path: DataPath<'a>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataPath")]
pub struct DataPath<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "language")]
    pub language: Option<URI>,
    #[xml(attr = "evaluatesToTypeRef")]
    pub evaluates_totype_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationPropertyRetrievalExpression")]
pub struct CorrelationPropertyRetrievalExpression<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "messageRef")]
    pub message_ref: Cow<'a, str>,
    #[xml(child = "bpmn:messagePath")]
    pub message_path: MessagePath<'a>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messagePath")]
pub struct MessagePath<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "language")]
    pub language: Option<URI>,
    #[xml(attr = "evaluatesToTypeRef")]
    pub evaluates_totype_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationSubscription")]
pub struct CorrelationSubscription<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "correlationKeyRef")]
    pub correlation_key_ref: Cow<'a, str>,
    #[xml(child = "bpmn:correlationPropertyBinding")]
    pub correlation_property_bindings: Vec<CorrelationPropertyBinding<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataAssociation")]
pub struct DataAssociation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(child = "bpmn:sourceRef")]
    pub source_refs: Vec<SourceRef<'a>>,
    #[xml(child = "bpmn:targetRef")]
    pub target_ref: TargetRef<'a>,
    #[xml(child = "bpmn:transformation")]
    pub transformation: Option<Transformation<'a>>,
    #[xml(child = "bpmn:assignment")]
    pub assignments: Vec<Assignment<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:sourceRef")]
pub struct SourceRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:targetRef")]
pub struct TargetRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:transformation")]
pub struct Transformation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "language")]
    pub language: Option<URI>,
    #[xml(attr = "evaluatesToTypeRef")]
    pub evaluates_totype_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataInput")]
pub struct DataInput<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<Cow<'a, str>>,
    #[xml(attr = "isCollection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataInputAssociation")]
pub struct DataInputAssociation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(child = "bpmn:sourceRef")]
    pub source_refs: Vec<SourceRef<'a>>,
    #[xml(child = "bpmn:targetRef")]
    pub target_ref: TargetRef<'a>,
    #[xml(child = "bpmn:transformation")]
    pub transformation: Option<Transformation<'a>>,
    #[xml(child = "bpmn:assignment")]
    pub assignments: Vec<Assignment<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataObject")]
pub struct DataObject<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<Cow<'a, str>>,
    #[xml(attr = "isCollection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataObjectReference")]
pub struct DataObjectReference<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<Cow<'a, str>>,
    #[xml(attr = "dataObjectRef")]
    pub data_object_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataOutput")]
pub struct DataOutput<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<Cow<'a, str>>,
    #[xml(attr = "isCollection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataOutputAssociation")]
pub struct DataOutputAssociation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(child = "bpmn:sourceRef")]
    pub source_refs: Vec<SourceRef<'a>>,
    #[xml(child = "bpmn:targetRef")]
    pub target_ref: TargetRef<'a>,
    #[xml(child = "bpmn:transformation")]
    pub transformation: Option<Transformation<'a>>,
    #[xml(child = "bpmn:assignment")]
    pub assignments: Vec<Assignment<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataState")]
pub struct DataState<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataStore")]
pub struct DataStore<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "capacity")]
    pub capacity: Option<Integer>,
    #[xml(attr = "isUnlimited")]
    pub is_unlimited: Option<bool>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataStoreReference")]
pub struct DataStoreReference<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<Cow<'a, str>>,
    #[xml(attr = "dataStoreRef")]
    pub data_store_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:documentation")]
pub struct Documentation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(attr = "textFormat")]
    pub text_format: Option<Cow<'a, str>>,
    #[xml(text, cdata)]
    content: Cow<'a, str>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:endEvent")]
pub struct EndEvent<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInput")]
    pub data_inputs: Vec<DataInput<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:inputSet")]
    pub input_set: Option<InputSet<'a>>,
    #[xml(
        child = "bpmn:cancelEventDefinition",
        child = "bpmn:compensateEventDefinition",
        child = "bpmn:conditionalEventDefinition",
        child = "bpmn:errorEventDefinition",
        child = "bpmn:escalationEventDefinition",
        child = "bpmn:linkEventDefinition",
        child = "bpmn:messageEventDefinition",
        child = "bpmn:signalEventDefinition",
        child = "bpmn:terminateEventDefinition",
        child = "bpmn:timerEventDefinition"
    )]
    pub event_definitions: Vec<EventDefinition<'a>>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:endPoint")]
pub struct EndPoint<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:error")]
pub struct Error<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "errorCode")]
    pub error_code: Option<Cow<'a, str>>,
    #[xml(attr = "structureRef")]
    pub structure_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:errorEventDefinition")]
pub struct ErrorEventDefinition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "errorRef")]
    pub error_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:escalation")]
pub struct Escalation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "escalationCode")]
    pub escalation_code: Option<Cow<'a, str>>,
    #[xml(attr = "structureRef")]
    pub structure_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:escalationEventDefinition")]
pub struct EscalationEventDefinition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "escalationRef")]
    pub escalation_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:eventBasedGateway")]
pub struct EventBasedGateway<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "gatewayDirection")]
    pub gateway_direction: Option<Cow<'a, str>>,
    #[xml(attr = "instantiate")]
    pub instantiate: Option<bool>,
    #[xml(attr = "eventGatewayType")]
    pub event_gateway_type: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:eventDefinition")]
pub enum EventDefinition<'a> {
    #[xml(tag = "bpmn:cancelEventDefinition")]
    CancelEventDefinition(CancelEventDefinition<'a>),
    #[xml(tag = "bpmn:compensateEventDefinition")]
    CompensateEventDefinition(CompensateEventDefinition<'a>),
    #[xml(tag = "bpmn:conditionalEventDefinition")]
    ConditionalEventDefinition(ConditionalEventDefinition<'a>),
    #[xml(tag = "bpmn:errorEventDefinition")]
    ErrorEventDefinition(ErrorEventDefinition<'a>),
    #[xml(tag = "bpmn:escalationEventDefinition")]
    EscalationEventDefinition(EscalationEventDefinition<'a>),
    #[xml(tag = "bpmn:linkEventDefinition")]
    LinkEventDefinition(LinkEventDefinition<'a>),
    #[xml(tag = "bpmn:messageEventDefinition")]
    MessageEventDefinition(MessageEventDefinition<'a>),
    #[xml(tag = "bpmn:signalEventDefinition")]
    SignalEventDefinition(SignalEventDefinition<'a>),
    #[xml(tag = "bpmn:terminateEventDefinition")]
    TerminateEventDefinition(TerminateEventDefinition<'a>),
    #[xml(tag = "bpmn:timerEventDefinition")]
    TimerEventDefinition(TimerEventDefinition<'a>),
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:exclusiveGateway")]
pub struct ExclusiveGateway<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "gatewayDirection")]
    pub gateway_direction: Option<Cow<'a, str>>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:expression")]
pub struct Expression<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:extension")]
pub struct Extension<'a> {
    #[xml(attr = "definition")]
    pub definition: Option<Cow<'a, str>>,
    #[xml(attr = "mustUnderstand")]
    pub must_understand: Option<bool>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:extensionElements")]
pub struct ExtensionElements<'a> {
    #[xml(text, cdata)]
    content: Cow<'a, str>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:flowElement")]
pub enum FlowElement<'a> {
    #[xml(tag = "bpmn:adHocSubProcess")]
    AdHocSubProcess(AdHocSubProcess<'a>),
    #[xml(tag = "bpmn:boundaryEvent")]
    BoundaryEvent(BoundaryEvent<'a>),
    #[xml(tag = "bpmn:businessRuleTask")]
    BusinessRuleTask(BusinessRuleTask<'a>),
    #[xml(tag = "bpmn:callActivity")]
    CallActivity(CallActivity<'a>),
    #[xml(tag = "bpmn:callChoreography")]
    CallChoreography(CallChoreography<'a>),
    #[xml(tag = "bpmn:choreographyTask")]
    ChoreographyTask(ChoreographyTask<'a>),
    #[xml(tag = "bpmn:complexGateway")]
    ComplexGateway(ComplexGateway<'a>),
    #[xml(tag = "bpmn:dataObject")]
    DataObject(DataObject<'a>),
    #[xml(tag = "bpmn:dataObjectReference")]
    DataObjectReference(DataObjectReference<'a>),
    #[xml(tag = "bpmn:dataStoreReference")]
    DataStoreReference(DataStoreReference<'a>),
    #[xml(tag = "bpmn:endEvent")]
    EndEvent(EndEvent<'a>),
    #[xml(tag = "bpmn:event")]
    Event(Event<'a>),
    #[xml(tag = "bpmn:eventBasedGateway")]
    EventBasedGateway(EventBasedGateway<'a>),
    #[xml(tag = "bpmn:exclusiveGateway")]
    ExclusiveGateway(ExclusiveGateway<'a>),
    #[xml(tag = "bpmn:implicitThrowEvent")]
    ImplicitThrowEvent(ImplicitThrowEvent<'a>),
    #[xml(tag = "bpmn:inclusiveGateway")]
    InclusiveGateway(InclusiveGateway<'a>),
    #[xml(tag = "bpmn:intermediateCatchEvent")]
    IntermediateCatchEvent(IntermediateCatchEvent<'a>),
    #[xml(tag = "bpmn:intermediateThrowEvent")]
    IntermediateThrowEvent(IntermediateThrowEvent<'a>),
    #[xml(tag = "bpmn:manualTask")]
    ManualTask(ManualTask<'a>),
    #[xml(tag = "bpmn:parallelGateway")]
    ParallelGateway(ParallelGateway<'a>),
    #[xml(tag = "bpmn:receiveTask")]
    ReceiveTask(ReceiveTask<'a>),
    #[xml(tag = "bpmn:scriptTask")]
    ScriptTask(ScriptTask<'a>),
    #[xml(tag = "bpmn:sendTask")]
    SendTask(SendTask<'a>),
    #[xml(tag = "bpmn:sequenceFlow")]
    SequenceFlow(SequenceFlow<'a>),
    #[xml(tag = "bpmn:serviceTask")]
    ServiceTask(ServiceTask<'a>),
    #[xml(tag = "bpmn:startEvent")]
    StartEvent(StartEvent<'a>),
    #[xml(tag = "bpmn:subChoreography")]
    SubChoreography(SubChoreography<'a>),
    #[xml(tag = "bpmn:subProcess")]
    SubProcess(SubProcess<'a>),
    #[xml(tag = "bpmn:task")]
    Task(Task<'a>),
    #[xml(tag = "bpmn:transaction")]
    Transaction(Transaction<'a>),
    #[xml(tag = "bpmn:userTask")]
    UserTask(UserTask<'a>),
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:categoryValueRef")]
pub struct CategoryValueRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:flowNode")]
pub struct FlowNode<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:incoming")]
pub struct Incoming<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outgoing")]
pub struct Outgoing<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:formalExpression")]
pub struct FormalExpression<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "language")]
    pub language: Option<URI>,
    #[xml(attr = "evaluatesToTypeRef")]
    pub evaluates_totype_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:gateway")]
pub struct Gateway<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "gatewayDirection")]
    pub gateway_direction: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalBusinessRuleTask")]
pub struct GlobalBusinessRuleTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef<'a>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(attr = "implementation")]
    pub implementation: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalChoreographyTask")]
pub struct GlobalChoreographyTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "isClosed")]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    pub participants: Vec<Participant<'a>>,
    #[xml(child = "bpmn:messageFlow")]
    pub message_flows: Vec<MessageFlow<'a>>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact<'a>>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    pub conversation_nodes: Vec<ConversationNode<'a>>,
    #[xml(child = "bpmn:conversationAssociation")]
    pub conversation_associations: Vec<ConversationAssociation<'a>>,
    #[xml(child = "bpmn:participantAssociation")]
    pub participant_associations: Vec<ParticipantAssociation<'a>>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    pub message_flow_associations: Vec<MessageFlowAssociation<'a>>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey<'a>>,
    #[xml(child = "bpmn:choreographyRef")]
    pub choreography_refs: Vec<ChoreographyRef<'a>>,
    #[xml(child = "bpmn:conversationLink")]
    pub conversation_links: Vec<ConversationLink<'a>>,
    #[xml(
        child = "bpmn:adHocSubProcess",
        child = "bpmn:boundaryEvent",
        child = "bpmn:businessRuleTask",
        child = "bpmn:callActivity",
        child = "bpmn:callChoreography",
        child = "bpmn:choreographyTask",
        child = "bpmn:complexGateway",
        child = "bpmn:dataObject",
        child = "bpmn:dataObjectReference",
        child = "bpmn:dataStoreReference",
        child = "bpmn:endEvent",
        child = "bpmn:event",
        child = "bpmn:eventBasedGateway",
        child = "bpmn:exclusiveGateway",
        child = "bpmn:implicitThrowEvent",
        child = "bpmn:inclusiveGateway",
        child = "bpmn:intermediateCatchEvent",
        child = "bpmn:intermediateThrowEvent",
        child = "bpmn:manualTask",
        child = "bpmn:parallelGateway",
        child = "bpmn:receiveTask",
        child = "bpmn:scriptTask",
        child = "bpmn:sendTask",
        child = "bpmn:sequenceFlow",
        child = "bpmn:serviceTask",
        child = "bpmn:startEvent",
        child = "bpmn:subChoreography",
        child = "bpmn:subProcess",
        child = "bpmn:task",
        child = "bpmn:transaction",
        child = "bpmn:userTask"
    )]
    pub flow_elements: Vec<FlowElement<'a>>,
    #[xml(attr = "initiatingParticipantRef")]
    pub initiating_participant_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalConversation")]
pub struct GlobalConversation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "isClosed")]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    pub participants: Vec<Participant<'a>>,
    #[xml(child = "bpmn:messageFlow")]
    pub message_flows: Vec<MessageFlow<'a>>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact<'a>>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    pub conversation_nodes: Vec<ConversationNode<'a>>,
    #[xml(child = "bpmn:conversationAssociation")]
    pub conversation_associations: Vec<ConversationAssociation<'a>>,
    #[xml(child = "bpmn:participantAssociation")]
    pub participant_associations: Vec<ParticipantAssociation<'a>>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    pub message_flow_associations: Vec<MessageFlowAssociation<'a>>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey<'a>>,
    #[xml(child = "bpmn:choreographyRef")]
    pub choreography_refs: Vec<ChoreographyRef<'a>>,
    #[xml(child = "bpmn:conversationLink")]
    pub conversation_links: Vec<ConversationLink<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalManualTask")]
pub struct GlobalManualTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef<'a>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalScriptTask")]
pub struct GlobalScriptTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef<'a>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(attr = "scriptLanguage")]
    pub script_language: Option<URI>,
    #[xml(child = "bpmn:script")]
    pub script: Option<Script<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalTask")]
pub struct GlobalTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef<'a>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalUserTask")]
pub struct GlobalUserTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef<'a>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(attr = "implementation")]
    pub implementation: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:rendering")]
    pub renderings: Vec<Rendering<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:group")]
pub struct Group<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "categoryValueRef")]
    pub category_value_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:humanPerformer")]
pub struct HumanPerformer<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:resourceRef")]
    pub resource_ref: ResourceRef<'a>,
    #[xml(child = "bpmn:resourceParameterBinding")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding<'a>>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    pub resource_assignment_expression: Option<ResourceAssignmentExpression<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:implicitThrowEvent")]
pub struct ImplicitThrowEvent<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInput")]
    pub data_inputs: Vec<DataInput<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:inputSet")]
    pub input_set: Option<InputSet<'a>>,
    #[xml(
        child = "bpmn:cancelEventDefinition",
        child = "bpmn:compensateEventDefinition",
        child = "bpmn:conditionalEventDefinition",
        child = "bpmn:errorEventDefinition",
        child = "bpmn:escalationEventDefinition",
        child = "bpmn:linkEventDefinition",
        child = "bpmn:messageEventDefinition",
        child = "bpmn:signalEventDefinition",
        child = "bpmn:terminateEventDefinition",
        child = "bpmn:timerEventDefinition"
    )]
    pub event_definitions: Vec<EventDefinition<'a>>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:inclusiveGateway")]
pub struct InclusiveGateway<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "gatewayDirection")]
    pub gateway_direction: Option<Cow<'a, str>>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:inputSet")]
pub struct InputSet<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:dataInputRefs")]
    pub data_input_refss: Vec<DataInputRefs<'a>>,
    #[xml(child = "bpmn:optionalInputRefs")]
    pub optional_input_refss: Vec<OptionalInputRefs<'a>>,
    #[xml(child = "bpmn:whileExecutingInputRefs")]
    pub while_executing_input_refss: Vec<WhileExecutingInputRefs<'a>>,
    #[xml(child = "bpmn:outputSetRefs")]
    pub output_set_refss: Vec<OutputSetRefs<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataInputRefs")]
pub struct DataInputRefs<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:optionalInputRefs")]
pub struct OptionalInputRefs<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:whileExecutingInputRefs")]
pub struct WhileExecutingInputRefs<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outputSetRefs")]
pub struct OutputSetRefs<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:interface")]
pub struct Interface<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(attr = "implementationRef")]
    pub implementation_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:operation")]
    pub operations: Vec<Operation<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:intermediateCatchEvent")]
pub struct IntermediateCatchEvent<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(attr = "parallelMultiple")]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    pub data_outputs: Vec<DataOutput<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:outputSet")]
    pub output_set: Option<OutputSet<'a>>,
    #[xml(
        child = "bpmn:cancelEventDefinition",
        child = "bpmn:compensateEventDefinition",
        child = "bpmn:conditionalEventDefinition",
        child = "bpmn:errorEventDefinition",
        child = "bpmn:escalationEventDefinition",
        child = "bpmn:linkEventDefinition",
        child = "bpmn:messageEventDefinition",
        child = "bpmn:signalEventDefinition",
        child = "bpmn:terminateEventDefinition",
        child = "bpmn:timerEventDefinition"
    )]
    pub event_definitions: Vec<EventDefinition<'a>>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:intermediateThrowEvent")]
pub struct IntermediateThrowEvent<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInput")]
    pub data_inputs: Vec<DataInput<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:inputSet")]
    pub input_set: Option<InputSet<'a>>,
    #[xml(
        child = "bpmn:cancelEventDefinition",
        child = "bpmn:compensateEventDefinition",
        child = "bpmn:conditionalEventDefinition",
        child = "bpmn:errorEventDefinition",
        child = "bpmn:escalationEventDefinition",
        child = "bpmn:linkEventDefinition",
        child = "bpmn:messageEventDefinition",
        child = "bpmn:signalEventDefinition",
        child = "bpmn:terminateEventDefinition",
        child = "bpmn:timerEventDefinition"
    )]
    pub event_definitions: Vec<EventDefinition<'a>>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:ioBinding")]
pub struct IoBinding<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "operationRef")]
    pub operation_ref: Cow<'a, str>,
    #[xml(attr = "inputDataRef")]
    pub input_data_ref: Cow<'a, str>,
    #[xml(attr = "outputDataRef")]
    pub output_data_ref: Cow<'a, str>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:ioSpecification")]
pub struct IoSpecification<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(child = "bpmn:dataInput")]
    pub data_inputs: Vec<DataInput<'a>>,
    #[xml(child = "bpmn:dataOutput")]
    pub data_outputs: Vec<DataOutput<'a>>,
    #[xml(child = "bpmn:inputSet")]
    pub input_sets: Vec<InputSet<'a>>,
    #[xml(child = "bpmn:outputSet")]
    pub output_sets: Vec<OutputSet<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:itemDefinition")]
pub struct ItemDefinition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "structureRef")]
    pub structure_ref: Option<Cow<'a, str>>,
    #[xml(attr = "isCollection")]
    pub is_collection: Option<bool>,
    #[xml(attr = "itemKind")]
    pub item_kind: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:lane")]
pub struct Lane<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "partitionElementRef")]
    pub partition_element_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:partitionElement")]
    pub partition_element: Option<PartitionElement<'a>>,
    #[xml(child = "bpmn:flowNodeRef")]
    pub flow_node_refs: Vec<FlowNodeRef<'a>>,
    #[xml(child = "bpmn:childLaneSet")]
    pub child_lane_set: Option<ChildLaneSet<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:partitionElement")]
pub struct PartitionElement<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:flowNodeRef")]
pub struct FlowNodeRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:childLaneSet")]
pub struct ChildLaneSet<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:lane")]
    pub lanes: Vec<Lane<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:laneSet")]
pub struct LaneSet<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:lane")]
    pub lanes: Vec<Lane<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:linkEventDefinition")]
pub struct LinkEventDefinition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(child = "bpmn:source")]
    pub sources: Vec<Source<'a>>,
    #[xml(child = "bpmn:target")]
    pub target: Option<Target<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:source")]
pub struct Source<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:target")]
pub struct Target<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:loopCharacteristics")]
pub enum LoopCharacteristics<'a> {
    #[xml(tag = "bpmn:multiInstanceLoopCharacteristics")]
    MultiInstanceLoopCharacteristics(MultiInstanceLoopCharacteristics<'a>),
    #[xml(tag = "bpmn:standardLoopCharacteristics")]
    StandardLoopCharacteristics(StandardLoopCharacteristics<'a>),
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:manualTask")]
pub struct ManualTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:message")]
pub struct Message<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "itemRef")]
    pub item_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messageEventDefinition")]
pub struct MessageEventDefinition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "messageRef")]
    pub message_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:operationRef")]
    pub operation_ref: Option<OperationRef<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:operationRef")]
pub struct OperationRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messageFlow")]
pub struct MessageFlow<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "sourceRef")]
    pub source_ref: Cow<'a, str>,
    #[xml(attr = "targetRef")]
    pub target_ref: Cow<'a, str>,
    #[xml(attr = "messageRef")]
    pub message_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messageFlowAssociation")]
pub struct MessageFlowAssociation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "innerMessageFlowRef")]
    pub inner_message_flow_ref: Cow<'a, str>,
    #[xml(attr = "outerMessageFlowRef")]
    pub outer_message_flow_ref: Cow<'a, str>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:monitoring")]
pub struct Monitoring<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:multiInstanceLoopCharacteristics")]
pub struct MultiInstanceLoopCharacteristics<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "isSequential")]
    pub is_sequential: Option<bool>,
    #[xml(attr = "behavior")]
    pub behavior: Option<Cow<'a, str>>,
    #[xml(attr = "oneBehaviorEventRef")]
    pub one_behavior_event_ref: Option<Cow<'a, str>>,
    #[xml(attr = "noneBehaviorEventRef")]
    pub none_behavior_event_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:loopCardinality")]
    pub loop_cardinality: Option<LoopCardinality<'a>>,
    #[xml(child = "bpmn:loopDataInputRef")]
    pub loop_data_input_ref: Option<LoopDataInputRef<'a>>,
    #[xml(child = "bpmn:loopDataOutputRef")]
    pub loop_data_output_ref: Option<LoopDataOutputRef<'a>>,
    #[xml(child = "bpmn:inputDataItem")]
    pub input_data_item: Option<InputDataItem<'a>>,
    #[xml(child = "bpmn:outputDataItem")]
    pub output_data_item: Option<OutputDataItem<'a>>,
    #[xml(child = "bpmn:complexBehaviorDefinition")]
    pub complex_behavior_definitions: Vec<ComplexBehaviorDefinition<'a>>,
    #[xml(child = "bpmn:completionCondition")]
    pub completion_condition: Option<CompletionCondition<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:loopCardinality")]
pub struct LoopCardinality<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:loopDataInputRef")]
pub struct LoopDataInputRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:loopDataOutputRef")]
pub struct LoopDataOutputRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:inputDataItem")]
pub struct InputDataItem<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<Cow<'a, str>>,
    #[xml(attr = "isCollection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outputDataItem")]
pub struct OutputDataItem<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<Cow<'a, str>>,
    #[xml(attr = "isCollection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:operation")]
pub struct Operation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(attr = "implementationRef")]
    pub implementation_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:inMessageRef")]
    pub in_message_ref: InMessageRef<'a>,
    #[xml(child = "bpmn:outMessageRef")]
    pub out_message_ref: Option<OutMessageRef<'a>>,
    #[xml(child = "bpmn:errorRef")]
    pub error_refs: Vec<ErrorRef<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:inMessageRef")]
pub struct InMessageRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outMessageRef")]
pub struct OutMessageRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:errorRef")]
pub struct ErrorRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outputSet")]
pub struct OutputSet<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:dataOutputRefs")]
    pub data_output_refss: Vec<DataOutputRefs<'a>>,
    #[xml(child = "bpmn:optionalOutputRefs")]
    pub optional_output_refss: Vec<OptionalOutputRefs<'a>>,
    #[xml(child = "bpmn:whileExecutingOutputRefs")]
    pub while_executing_output_refss: Vec<WhileExecutingOutputRefs<'a>>,
    #[xml(child = "bpmn:inputSetRefs")]
    pub input_set_refss: Vec<InputSetRefs<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataOutputRefs")]
pub struct DataOutputRefs<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:optionalOutputRefs")]
pub struct OptionalOutputRefs<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:whileExecutingOutputRefs")]
pub struct WhileExecutingOutputRefs<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:inputSetRefs")]
pub struct InputSetRefs<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:parallelGateway")]
pub struct ParallelGateway<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "gatewayDirection")]
    pub gateway_direction: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:participant")]
pub struct Participant<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "processRef")]
    pub process_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:interfaceRef")]
    pub interface_refs: Vec<InterfaceRef<'a>>,
    #[xml(child = "bpmn:endPointRef")]
    pub end_point_refs: Vec<EndPointRef<'a>>,
    #[xml(child = "bpmn:participantMultiplicity")]
    pub participant_multiplicity: Option<ParticipantMultiplicity<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:interfaceRef")]
pub struct InterfaceRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:endPointRef")]
pub struct EndPointRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:participantAssociation")]
pub struct ParticipantAssociation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(child = "bpmn:innerParticipantRef")]
    pub inner_participant_ref: InnerParticipantRef<'a>,
    #[xml(child = "bpmn:outerParticipantRef")]
    pub outer_participant_ref: OuterParticipantRef<'a>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:innerParticipantRef")]
pub struct InnerParticipantRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outerParticipantRef")]
pub struct OuterParticipantRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:participantMultiplicity")]
pub struct ParticipantMultiplicity<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "minimum")]
    pub minimum: Option<Int>,
    #[xml(attr = "maximum")]
    pub maximum: Option<Int>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:partnerEntity")]
pub struct PartnerEntity<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:partnerRole")]
pub struct PartnerRole<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:performer")]
pub struct Performer<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:resourceRef")]
    pub resource_ref: ResourceRef<'a>,
    #[xml(child = "bpmn:resourceParameterBinding")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding<'a>>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    pub resource_assignment_expression: Option<ResourceAssignmentExpression<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:potentialOwner")]
pub struct PotentialOwner<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:resourceRef")]
    pub resource_ref: ResourceRef<'a>,
    #[xml(child = "bpmn:resourceParameterBinding")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding<'a>>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    pub resource_assignment_expression: Option<ResourceAssignmentExpression<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:process")]
pub struct Process<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef<'a>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding<'a>>,
    #[xml(attr = "processType")]
    pub process_type: Option<Cow<'a, str>>,
    #[xml(attr = "isClosed")]
    pub is_closed: Option<bool>,
    #[xml(attr = "isExecutable")]
    pub is_executable: Option<bool>,
    #[xml(attr = "definitionalCollaborationRef")]
    pub definitional_collaboration_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:laneSet")]
    pub lane_sets: Vec<LaneSet<'a>>,
    #[xml(
        child = "bpmn:adHocSubProcess",
        child = "bpmn:boundaryEvent",
        child = "bpmn:businessRuleTask",
        child = "bpmn:callActivity",
        child = "bpmn:callChoreography",
        child = "bpmn:choreographyTask",
        child = "bpmn:complexGateway",
        child = "bpmn:dataObject",
        child = "bpmn:dataObjectReference",
        child = "bpmn:dataStoreReference",
        child = "bpmn:endEvent",
        child = "bpmn:event",
        child = "bpmn:eventBasedGateway",
        child = "bpmn:exclusiveGateway",
        child = "bpmn:implicitThrowEvent",
        child = "bpmn:inclusiveGateway",
        child = "bpmn:intermediateCatchEvent",
        child = "bpmn:intermediateThrowEvent",
        child = "bpmn:manualTask",
        child = "bpmn:parallelGateway",
        child = "bpmn:receiveTask",
        child = "bpmn:scriptTask",
        child = "bpmn:sendTask",
        child = "bpmn:sequenceFlow",
        child = "bpmn:serviceTask",
        child = "bpmn:startEvent",
        child = "bpmn:subChoreography",
        child = "bpmn:subProcess",
        child = "bpmn:task",
        child = "bpmn:transaction",
        child = "bpmn:userTask"
    )]
    pub flow_elements: Vec<FlowElement<'a>>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(child = "bpmn:correlationSubscription")]
    pub correlation_subscriptions: Vec<CorrelationSubscription<'a>>,
    #[xml(child = "bpmn:supports")]
    pub supportss: Vec<Supports<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:supports")]
pub struct Supports<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:property")]
pub struct Property<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:receiveTask")]
pub struct ReceiveTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
    #[xml(attr = "implementation")]
    pub implementation: Option<Cow<'a, str>>,
    #[xml(attr = "instantiate")]
    pub instantiate: Option<bool>,
    #[xml(attr = "messageRef")]
    pub message_ref: Option<Cow<'a, str>>,
    #[xml(attr = "operationRef")]
    pub operation_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:relationship")]
pub struct Relationship<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "type")]
    pub typ: Cow<'a, str>,
    #[xml(attr = "direction")]
    pub direction: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:source")]
    pub sources: Vec<Source<'a>>,
    #[xml(child = "bpmn:target")]
    pub targets: Vec<Target<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:rendering")]
pub struct Rendering<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resource")]
pub struct Resource<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(child = "bpmn:resourceParameter")]
    pub resource_parameters: Vec<ResourceParameter<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceAssignmentExpression")]
pub struct ResourceAssignmentExpression<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(child = "bpmn:expression")]
    pub expression: Expression<'a>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceParameter")]
pub struct ResourceParameter<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "type")]
    pub typ: Option<Cow<'a, str>>,
    #[xml(attr = "isRequired")]
    pub is_required: Option<bool>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceParameterBinding")]
pub struct ResourceParameterBinding<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "parameterRef")]
    pub parameter_ref: Cow<'a, str>,
    #[xml(child = "bpmn:expression")]
    pub expression: Expression<'a>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceRole")]
pub struct ResourceRole<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:resourceRef")]
    pub resource_ref: ResourceRef<'a>,
    #[xml(child = "bpmn:resourceParameterBinding")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding<'a>>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    pub resource_assignment_expression: Option<ResourceAssignmentExpression<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceRef")]
pub struct ResourceRef<'a> {
    #[xml(text, cdata)]
    pub content: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:rootElement")]
pub enum RootElement<'a> {
    #[xml(tag = "bpmn:category")]
    Category(Category<'a>),
    #[xml(tag = "bpmn:collaboration")]
    Collaboration(Collaboration<'a>),
    #[xml(tag = "bpmn:correlationProperty")]
    CorrelationProperty(CorrelationProperty<'a>),
    #[xml(tag = "bpmn:dataStore")]
    DataStore(DataStore<'a>),
    #[xml(tag = "bpmn:endPoint")]
    EndPoint(EndPoint<'a>),
    #[xml(tag = "bpmn:error")]
    Error(Error<'a>),
    #[xml(tag = "bpmn:escalation")]
    Escalation(Escalation<'a>),
    #[xml(tag = "bpmn:eventDefinition")]
    EventDefinition(EventDefinition<'a>),
    #[xml(tag = "bpmn:globalBusinessRuleTask")]
    GlobalBusinessRuleTask(GlobalBusinessRuleTask<'a>),
    #[xml(tag = "bpmn:globalManualTask")]
    GlobalManualTask(GlobalManualTask<'a>),
    #[xml(tag = "bpmn:globalScriptTask")]
    GlobalScriptTask(GlobalScriptTask<'a>),
    #[xml(tag = "bpmn:globalTask")]
    GlobalTask(GlobalTask<'a>),
    #[xml(tag = "bpmn:globalUserTask")]
    GlobalUserTask(GlobalUserTask<'a>),
    #[xml(tag = "bpmn:interface")]
    Interface(Interface<'a>),
    #[xml(tag = "bpmn:itemDefinition")]
    ItemDefinition(ItemDefinition<'a>),
    #[xml(tag = "bpmn:message")]
    Message(Message<'a>),
    #[xml(tag = "bpmn:partnerEntity")]
    PartnerEntity(PartnerEntity<'a>),
    #[xml(tag = "bpmn:partnerRole")]
    PartnerRole(PartnerRole<'a>),
    #[xml(tag = "bpmn:process")]
    Process(Process<'a>),
    #[xml(tag = "bpmn:resource")]
    Resource(Resource<'a>),
    #[xml(tag = "bpmn:signal")]
    Signal(Signal<'a>),
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:scriptTask")]
pub struct ScriptTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
    #[xml(attr = "scriptFormat")]
    pub script_format: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:script")]
    pub script: Option<Script<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:script")]
pub struct Script<'a> {
    #[xml(text, cdata)]
    content: Cow<'a, str>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:sendTask")]
pub struct SendTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
    #[xml(attr = "implementation")]
    pub implementation: Option<Cow<'a, str>>,
    #[xml(attr = "messageRef")]
    pub message_ref: Option<Cow<'a, str>>,
    #[xml(attr = "operationRef")]
    pub operation_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:sequenceFlow")]
pub struct SequenceFlow<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(attr = "sourceRef")]
    pub source_ref: Cow<'a, str>,
    #[xml(attr = "targetRef")]
    pub target_ref: Cow<'a, str>,
    #[xml(attr = "isImmediate")]
    pub is_immediate: Option<bool>,
    #[xml(child = "bpmn:conditionExpression")]
    pub condition_expression: Option<ConditionExpression<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conditionExpression")]
pub struct ConditionExpression<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:serviceTask")]
pub struct ServiceTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
    #[xml(attr = "implementation")]
    pub implementation: Option<Cow<'a, str>>,
    #[xml(attr = "operationRef")]
    pub operation_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:signal")]
pub struct Signal<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "structureRef")]
    pub structure_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:signalEventDefinition")]
pub struct SignalEventDefinition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "signalRef")]
    pub signal_ref: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:standardLoopCharacteristics")]
pub struct StandardLoopCharacteristics<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "testBefore")]
    pub test_before: Option<bool>,
    #[xml(attr = "loopMaximum")]
    pub loop_maximum: Option<Integer>,
    #[xml(child = "bpmn:loopCondition")]
    pub loop_condition: Option<LoopCondition<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:loopCondition")]
pub struct LoopCondition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:startEvent")]
pub struct StartEvent<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(attr = "parallelMultiple")]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    pub data_outputs: Vec<DataOutput<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:outputSet")]
    pub output_set: Option<OutputSet<'a>>,
    #[xml(
        child = "bpmn:cancelEventDefinition",
        child = "bpmn:compensateEventDefinition",
        child = "bpmn:conditionalEventDefinition",
        child = "bpmn:errorEventDefinition",
        child = "bpmn:escalationEventDefinition",
        child = "bpmn:linkEventDefinition",
        child = "bpmn:messageEventDefinition",
        child = "bpmn:signalEventDefinition",
        child = "bpmn:terminateEventDefinition",
        child = "bpmn:timerEventDefinition"
    )]
    pub event_definitions: Vec<EventDefinition<'a>>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef<'a>>,
    #[xml(attr = "isInterrupting")]
    pub is_interrupting: Option<bool>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:subChoreography")]
pub struct SubChoreography<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "initiatingParticipantRef")]
    pub initiating_participant_ref: Cow<'a, str>,
    #[xml(attr = "loopType")]
    pub loop_type: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef<'a>>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey<'a>>,
    #[xml(
        child = "bpmn:adHocSubProcess",
        child = "bpmn:boundaryEvent",
        child = "bpmn:businessRuleTask",
        child = "bpmn:callActivity",
        child = "bpmn:callChoreography",
        child = "bpmn:choreographyTask",
        child = "bpmn:complexGateway",
        child = "bpmn:dataObject",
        child = "bpmn:dataObjectReference",
        child = "bpmn:dataStoreReference",
        child = "bpmn:endEvent",
        child = "bpmn:event",
        child = "bpmn:eventBasedGateway",
        child = "bpmn:exclusiveGateway",
        child = "bpmn:implicitThrowEvent",
        child = "bpmn:inclusiveGateway",
        child = "bpmn:intermediateCatchEvent",
        child = "bpmn:intermediateThrowEvent",
        child = "bpmn:manualTask",
        child = "bpmn:parallelGateway",
        child = "bpmn:receiveTask",
        child = "bpmn:scriptTask",
        child = "bpmn:sendTask",
        child = "bpmn:sequenceFlow",
        child = "bpmn:serviceTask",
        child = "bpmn:startEvent",
        child = "bpmn:subChoreography",
        child = "bpmn:subProcess",
        child = "bpmn:task",
        child = "bpmn:transaction",
        child = "bpmn:userTask"
    )]
    pub flow_elements: Vec<FlowElement<'a>>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:subConversation")]
pub struct SubConversation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef<'a>>,
    #[xml(child = "bpmn:messageFlowRef")]
    pub message_flow_refs: Vec<MessageFlowRef<'a>>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey<'a>>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    pub conversation_nodes: Vec<ConversationNode<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:subProcess")]
pub struct SubProcess<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
    #[xml(attr = "triggeredByEvent")]
    pub triggered_byevent: Option<bool>,
    #[xml(child = "bpmn:laneSet")]
    pub lane_sets: Vec<LaneSet<'a>>,
    #[xml(
        child = "bpmn:adHocSubProcess",
        child = "bpmn:boundaryEvent",
        child = "bpmn:businessRuleTask",
        child = "bpmn:callActivity",
        child = "bpmn:callChoreography",
        child = "bpmn:choreographyTask",
        child = "bpmn:complexGateway",
        child = "bpmn:dataObject",
        child = "bpmn:dataObjectReference",
        child = "bpmn:dataStoreReference",
        child = "bpmn:endEvent",
        child = "bpmn:event",
        child = "bpmn:eventBasedGateway",
        child = "bpmn:exclusiveGateway",
        child = "bpmn:implicitThrowEvent",
        child = "bpmn:inclusiveGateway",
        child = "bpmn:intermediateCatchEvent",
        child = "bpmn:intermediateThrowEvent",
        child = "bpmn:manualTask",
        child = "bpmn:parallelGateway",
        child = "bpmn:receiveTask",
        child = "bpmn:scriptTask",
        child = "bpmn:sendTask",
        child = "bpmn:sequenceFlow",
        child = "bpmn:serviceTask",
        child = "bpmn:startEvent",
        child = "bpmn:subChoreography",
        child = "bpmn:subProcess",
        child = "bpmn:task",
        child = "bpmn:transaction",
        child = "bpmn:userTask"
    )]
    pub flow_elements: Vec<FlowElement<'a>>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:task")]
pub struct Task<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:terminateEventDefinition")]
pub struct TerminateEventDefinition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:textAnnotation")]
pub struct TextAnnotation<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "textFormat")]
    pub text_format: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:text")]
    pub text: Option<Text<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:text")]
pub struct Text<'a> {
    #[xml(text, cdata)]
    content: Cow<'a, str>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:throwEvent")]
pub struct ThrowEvent<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInput")]
    pub data_inputs: Vec<DataInput<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:inputSet")]
    pub input_set: Option<InputSet<'a>>,
    #[xml(
        child = "bpmn:cancelEventDefinition",
        child = "bpmn:compensateEventDefinition",
        child = "bpmn:conditionalEventDefinition",
        child = "bpmn:errorEventDefinition",
        child = "bpmn:escalationEventDefinition",
        child = "bpmn:linkEventDefinition",
        child = "bpmn:messageEventDefinition",
        child = "bpmn:signalEventDefinition",
        child = "bpmn:terminateEventDefinition",
        child = "bpmn:timerEventDefinition"
    )]
    pub event_definitions: Vec<EventDefinition<'a>>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:timerEventDefinition")]
pub struct TimerEventDefinition<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(child = "bpmn:timeDate")]
    pub time_date: Option<TimeDate<'a>>,
    #[xml(child = "bpmn:timeDuration")]
    pub time_duration: Option<TimeDuration<'a>>,
    #[xml(child = "bpmn:timeCycle")]
    pub time_cycle: Option<TimeCycle<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:timeDate")]
pub struct TimeDate<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:timeDuration")]
pub struct TimeDuration<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:timeCycle")]
pub struct TimeCycle<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:transaction")]
pub struct Transaction<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
    #[xml(attr = "triggeredByEvent")]
    pub triggered_byevent: Option<bool>,
    #[xml(child = "bpmn:laneSet")]
    pub lane_sets: Vec<LaneSet<'a>>,
    #[xml(
        child = "bpmn:adHocSubProcess",
        child = "bpmn:boundaryEvent",
        child = "bpmn:businessRuleTask",
        child = "bpmn:callActivity",
        child = "bpmn:callChoreography",
        child = "bpmn:choreographyTask",
        child = "bpmn:complexGateway",
        child = "bpmn:dataObject",
        child = "bpmn:dataObjectReference",
        child = "bpmn:dataStoreReference",
        child = "bpmn:endEvent",
        child = "bpmn:event",
        child = "bpmn:eventBasedGateway",
        child = "bpmn:exclusiveGateway",
        child = "bpmn:implicitThrowEvent",
        child = "bpmn:inclusiveGateway",
        child = "bpmn:intermediateCatchEvent",
        child = "bpmn:intermediateThrowEvent",
        child = "bpmn:manualTask",
        child = "bpmn:parallelGateway",
        child = "bpmn:receiveTask",
        child = "bpmn:scriptTask",
        child = "bpmn:sendTask",
        child = "bpmn:sequenceFlow",
        child = "bpmn:serviceTask",
        child = "bpmn:startEvent",
        child = "bpmn:subChoreography",
        child = "bpmn:subProcess",
        child = "bpmn:task",
        child = "bpmn:transaction",
        child = "bpmn:userTask"
    )]
    pub flow_elements: Vec<FlowElement<'a>>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact<'a>>,
    #[xml(attr = "method")]
    pub method: Option<Cow<'a, str>>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:userTask")]
pub struct UserTask<'a> {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation<'a>>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements<'a>>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing<'a>>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring<'a>>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef<'a>>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming<'a>>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing<'a>>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification<'a>>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property<'a>>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation<'a>>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation<'a>>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole<'a>>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics<'a>>,
    #[xml(attr = "implementation")]
    pub implementation: Option<Cow<'a, str>>,
    #[xml(child = "bpmn:rendering")]
    pub renderings: Vec<Rendering<'a>>,
}
