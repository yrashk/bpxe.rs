// This file is generated from BPMN 2.0 schema using `codegen.sh` script
use strong_xml::XmlRead;

/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:definitions")]
pub struct Definitions {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "targetNamespace")]
    pub target_namespace: URI,
    #[xml(attr = "expressionLanguage")]
    pub expression_language: Option<URI>,
    #[xml(attr = "typeLanguage")]
    pub type_language: Option<URI>,
    #[xml(attr = "exporter")]
    pub exporter: Option<String>,
    #[xml(attr = "exporterVersion")]
    pub exporter_version: Option<String>,
    #[xml(child = "bpmn:import")]
    pub imports: Vec<Import>,
    #[xml(child = "bpmn:extension")]
    pub extensions: Vec<Extension>,
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
    pub root_elements: Vec<RootElement>,
    #[xml(child = "bpmn:relationship")]
    pub relationships: Vec<Relationship>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:import")]
pub struct Import {
    #[xml(attr = "namespace")]
    pub namespace: URI,
    #[xml(attr = "location")]
    pub location: String,
    #[xml(attr = "importType")]
    pub import_type: URI,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:activity")]
pub struct Activity {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:adHocSubProcess")]
pub struct AdHocSubProcess {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "triggeredByEvent")]
    pub triggered_byevent: Option<bool>,
    #[xml(child = "bpmn:laneSet")]
    pub lane_sets: Vec<LaneSet>,
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
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact>,
    #[xml(attr = "cancelRemainingInstances")]
    pub cancel_remaining_instances: Option<bool>,
    #[xml(attr = "ordering")]
    pub ordering: Option<String>,
    #[xml(child = "bpmn:completionCondition")]
    pub completion_condition: Option<CompletionCondition>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:completionCondition")]
pub struct CompletionCondition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:artifact")]
pub enum Artifact {
    #[xml(tag = "bpmn:association")]
    Association(Association),
    #[xml(tag = "bpmn:group")]
    Group(Group),
    #[xml(tag = "bpmn:textAnnotation")]
    TextAnnotation(TextAnnotation),
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:assignment")]
pub struct Assignment {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:from")]
    pub from: From,
    #[xml(child = "bpmn:to")]
    pub to: To,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:from")]
pub struct From {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:to")]
pub struct To {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:association")]
pub struct Association {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "sourceRef")]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    pub target_ref: String,
    #[xml(attr = "associationDirection")]
    pub association_direction: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:auditing")]
pub struct Auditing {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:baseElement")]
pub struct BaseElement {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:baseElementWithMixedContent")]
pub struct BaseElementWithMixedContent {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:boundaryEvent")]
pub struct BoundaryEvent {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(attr = "parallelMultiple")]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:outputSet")]
    pub output_set: Option<OutputSet>,
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
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef>,
    #[xml(attr = "cancelActivity")]
    pub cancel_activity: Option<bool>,
    #[xml(attr = "attachedToRef")]
    pub attached_toref: String,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:businessRuleTask")]
pub struct BusinessRuleTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    pub implementation: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:callableElement")]
pub struct CallableElement {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:supportedInterfaceRef")]
pub struct SupportedInterfaceRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:callActivity")]
pub struct CallActivity {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "calledElement")]
    pub called_element: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:callChoreography")]
pub struct CallChoreography {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "initiatingParticipantRef")]
    pub initiating_participant_ref: String,
    #[xml(attr = "loopType")]
    pub loop_type: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(attr = "calledChoreographyRef")]
    pub called_choreography_ref: Option<String>,
    #[xml(child = "bpmn:participantAssociation")]
    pub participant_associations: Vec<ParticipantAssociation>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:callConversation")]
pub struct CallConversation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:messageFlowRef")]
    pub message_flow_refs: Vec<MessageFlowRef>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(attr = "calledCollaborationRef")]
    pub called_collaboration_ref: Option<String>,
    #[xml(child = "bpmn:participantAssociation")]
    pub participant_associations: Vec<ParticipantAssociation>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:cancelEventDefinition")]
pub struct CancelEventDefinition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:catchEvent")]
pub struct CatchEvent {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(attr = "parallelMultiple")]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:outputSet")]
    pub output_set: Option<OutputSet>,
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
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:eventDefinitionRef")]
pub struct EventDefinitionRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:category")]
pub struct Category {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:categoryValue")]
    pub category_values: Vec<CategoryValue>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:categoryValue")]
pub struct CategoryValue {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "value")]
    pub value: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:choreography")]
pub struct Choreography {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "isClosed")]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    pub participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    pub message_flows: Vec<MessageFlow>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    pub conversation_nodes: Vec<ConversationNode>,
    #[xml(child = "bpmn:conversationAssociation")]
    pub conversation_associations: Vec<ConversationAssociation>,
    #[xml(child = "bpmn:participantAssociation")]
    pub participant_associations: Vec<ParticipantAssociation>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    pub message_flow_associations: Vec<MessageFlowAssociation>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(child = "bpmn:choreographyRef")]
    pub choreography_refs: Vec<ChoreographyRef>,
    #[xml(child = "bpmn:conversationLink")]
    pub conversation_links: Vec<ConversationLink>,
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
    pub flow_elements: Vec<FlowElement>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:choreographyActivity")]
pub struct ChoreographyActivity {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "initiatingParticipantRef")]
    pub initiating_participant_ref: String,
    #[xml(attr = "loopType")]
    pub loop_type: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:participantRef")]
pub struct ParticipantRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:choreographyTask")]
pub struct ChoreographyTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "initiatingParticipantRef")]
    pub initiating_participant_ref: String,
    #[xml(attr = "loopType")]
    pub loop_type: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(child = "bpmn:messageFlowRef")]
    pub message_flow_ref: MessageFlowRef,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messageFlowRef")]
pub struct MessageFlowRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:collaboration")]
pub struct Collaboration {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "isClosed")]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    pub participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    pub message_flows: Vec<MessageFlow>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    pub conversation_nodes: Vec<ConversationNode>,
    #[xml(child = "bpmn:conversationAssociation")]
    pub conversation_associations: Vec<ConversationAssociation>,
    #[xml(child = "bpmn:participantAssociation")]
    pub participant_associations: Vec<ParticipantAssociation>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    pub message_flow_associations: Vec<MessageFlowAssociation>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(child = "bpmn:choreographyRef")]
    pub choreography_refs: Vec<ChoreographyRef>,
    #[xml(child = "bpmn:conversationLink")]
    pub conversation_links: Vec<ConversationLink>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:choreographyRef")]
pub struct ChoreographyRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:compensateEventDefinition")]
pub struct CompensateEventDefinition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "waitForCompletion")]
    pub wait_for_completion: Option<bool>,
    #[xml(attr = "activityRef")]
    pub activity_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:complexBehaviorDefinition")]
pub struct ComplexBehaviorDefinition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:condition")]
    pub condition: Condition,
    #[xml(child = "bpmn:event")]
    pub event: Option<Event>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:condition")]
pub struct Condition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "language")]
    pub language: Option<URI>,
    #[xml(attr = "evaluatesToTypeRef")]
    pub evaluates_totype_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:event")]
pub struct Event {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInput")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:inputSet")]
    pub input_set: Option<InputSet>,
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
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:complexGateway")]
pub struct ComplexGateway {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "gatewayDirection")]
    pub gateway_direction: Option<String>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:activationCondition")]
    pub activation_condition: Option<ActivationCondition>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:activationCondition")]
pub struct ActivationCondition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conditionalEventDefinition")]
pub struct ConditionalEventDefinition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:condition")]
    pub condition: Condition,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conversation")]
pub struct Conversation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:messageFlowRef")]
    pub message_flow_refs: Vec<MessageFlowRef>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conversationAssociation")]
pub struct ConversationAssociation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "innerConversationNodeRef")]
    pub inner_conversation_node_ref: String,
    #[xml(attr = "outerConversationNodeRef")]
    pub outer_conversation_node_ref: String,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conversationLink")]
pub struct ConversationLink {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "sourceRef")]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    pub target_ref: String,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:conversationNode")]
pub enum ConversationNode {
    #[xml(tag = "bpmn:callConversation")]
    CallConversation(CallConversation),
    #[xml(tag = "bpmn:conversation")]
    Conversation(Conversation),
    #[xml(tag = "bpmn:subConversation")]
    SubConversation(SubConversation),
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationKey")]
pub struct CorrelationKey {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:correlationPropertyRef")]
    pub correlation_property_refs: Vec<CorrelationPropertyRef>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationPropertyRef")]
pub struct CorrelationPropertyRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationProperty")]
pub struct CorrelationProperty {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "type")]
    pub typ: Option<String>,
    #[xml(child = "bpmn:correlationPropertyRetrievalExpression")]
    pub correlation_property_retrieval_expressions: Vec<CorrelationPropertyRetrievalExpression>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationPropertyBinding")]
pub struct CorrelationPropertyBinding {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "correlationPropertyRef")]
    pub correlation_property_ref: String,
    #[xml(child = "bpmn:dataPath")]
    pub data_path: DataPath,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataPath")]
pub struct DataPath {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "language")]
    pub language: Option<URI>,
    #[xml(attr = "evaluatesToTypeRef")]
    pub evaluates_totype_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationPropertyRetrievalExpression")]
pub struct CorrelationPropertyRetrievalExpression {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "messageRef")]
    pub message_ref: String,
    #[xml(child = "bpmn:messagePath")]
    pub message_path: MessagePath,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messagePath")]
pub struct MessagePath {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "language")]
    pub language: Option<URI>,
    #[xml(attr = "evaluatesToTypeRef")]
    pub evaluates_totype_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationSubscription")]
pub struct CorrelationSubscription {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "correlationKeyRef")]
    pub correlation_key_ref: String,
    #[xml(child = "bpmn:correlationPropertyBinding")]
    pub correlation_property_bindings: Vec<CorrelationPropertyBinding>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataAssociation")]
pub struct DataAssociation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:sourceRef")]
    pub source_refs: Vec<SourceRef>,
    #[xml(child = "bpmn:targetRef")]
    pub target_ref: TargetRef,
    #[xml(child = "bpmn:transformation")]
    pub transformation: Option<Transformation>,
    #[xml(child = "bpmn:assignment")]
    pub assignments: Vec<Assignment>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:sourceRef")]
pub struct SourceRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:targetRef")]
pub struct TargetRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:transformation")]
pub struct Transformation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "language")]
    pub language: Option<URI>,
    #[xml(attr = "evaluatesToTypeRef")]
    pub evaluates_totype_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataInput")]
pub struct DataInput {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "isCollection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataInputAssociation")]
pub struct DataInputAssociation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:sourceRef")]
    pub source_refs: Vec<SourceRef>,
    #[xml(child = "bpmn:targetRef")]
    pub target_ref: TargetRef,
    #[xml(child = "bpmn:transformation")]
    pub transformation: Option<Transformation>,
    #[xml(child = "bpmn:assignment")]
    pub assignments: Vec<Assignment>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataObject")]
pub struct DataObject {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "isCollection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataObjectReference")]
pub struct DataObjectReference {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "dataObjectRef")]
    pub data_object_ref: Option<String>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataOutput")]
pub struct DataOutput {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "isCollection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataOutputAssociation")]
pub struct DataOutputAssociation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:sourceRef")]
    pub source_refs: Vec<SourceRef>,
    #[xml(child = "bpmn:targetRef")]
    pub target_ref: TargetRef,
    #[xml(child = "bpmn:transformation")]
    pub transformation: Option<Transformation>,
    #[xml(child = "bpmn:assignment")]
    pub assignments: Vec<Assignment>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataState")]
pub struct DataState {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataStore")]
pub struct DataStore {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "capacity")]
    pub capacity: Option<Integer>,
    #[xml(attr = "isUnlimited")]
    pub is_unlimited: Option<bool>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<String>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataStoreReference")]
pub struct DataStoreReference {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "dataStoreRef")]
    pub data_store_ref: Option<String>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:documentation")]
pub struct Documentation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(attr = "textFormat")]
    pub text_format: Option<String>,
    #[xml(text, cdata)]
    content: String,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:endEvent")]
pub struct EndEvent {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInput")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:inputSet")]
    pub input_set: Option<InputSet>,
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
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:endPoint")]
pub struct EndPoint {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:error")]
pub struct Error {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "errorCode")]
    pub error_code: Option<String>,
    #[xml(attr = "structureRef")]
    pub structure_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:errorEventDefinition")]
pub struct ErrorEventDefinition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "errorRef")]
    pub error_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:escalation")]
pub struct Escalation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "escalationCode")]
    pub escalation_code: Option<String>,
    #[xml(attr = "structureRef")]
    pub structure_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:escalationEventDefinition")]
pub struct EscalationEventDefinition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "escalationRef")]
    pub escalation_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:eventBasedGateway")]
pub struct EventBasedGateway {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "gatewayDirection")]
    pub gateway_direction: Option<String>,
    #[xml(attr = "instantiate")]
    pub instantiate: Option<bool>,
    #[xml(attr = "eventGatewayType")]
    pub event_gateway_type: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:eventDefinition")]
pub enum EventDefinition {
    #[xml(tag = "bpmn:cancelEventDefinition")]
    CancelEventDefinition(CancelEventDefinition),
    #[xml(tag = "bpmn:compensateEventDefinition")]
    CompensateEventDefinition(CompensateEventDefinition),
    #[xml(tag = "bpmn:conditionalEventDefinition")]
    ConditionalEventDefinition(ConditionalEventDefinition),
    #[xml(tag = "bpmn:errorEventDefinition")]
    ErrorEventDefinition(ErrorEventDefinition),
    #[xml(tag = "bpmn:escalationEventDefinition")]
    EscalationEventDefinition(EscalationEventDefinition),
    #[xml(tag = "bpmn:linkEventDefinition")]
    LinkEventDefinition(LinkEventDefinition),
    #[xml(tag = "bpmn:messageEventDefinition")]
    MessageEventDefinition(MessageEventDefinition),
    #[xml(tag = "bpmn:signalEventDefinition")]
    SignalEventDefinition(SignalEventDefinition),
    #[xml(tag = "bpmn:terminateEventDefinition")]
    TerminateEventDefinition(TerminateEventDefinition),
    #[xml(tag = "bpmn:timerEventDefinition")]
    TimerEventDefinition(TimerEventDefinition),
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:exclusiveGateway")]
pub struct ExclusiveGateway {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "gatewayDirection")]
    pub gateway_direction: Option<String>,
    #[xml(attr = "default")]
    pub default: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:expression")]
pub struct Expression {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:extension")]
pub struct Extension {
    #[xml(attr = "definition")]
    pub definition: Option<String>,
    #[xml(attr = "mustUnderstand")]
    pub must_understand: Option<bool>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:extensionElements")]
pub struct ExtensionElements {
    #[xml(text, cdata)]
    content: String,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:flowElement")]
pub enum FlowElement {
    #[xml(tag = "bpmn:adHocSubProcess")]
    AdHocSubProcess(AdHocSubProcess),
    #[xml(tag = "bpmn:boundaryEvent")]
    BoundaryEvent(BoundaryEvent),
    #[xml(tag = "bpmn:businessRuleTask")]
    BusinessRuleTask(BusinessRuleTask),
    #[xml(tag = "bpmn:callActivity")]
    CallActivity(CallActivity),
    #[xml(tag = "bpmn:callChoreography")]
    CallChoreography(CallChoreography),
    #[xml(tag = "bpmn:choreographyTask")]
    ChoreographyTask(ChoreographyTask),
    #[xml(tag = "bpmn:complexGateway")]
    ComplexGateway(ComplexGateway),
    #[xml(tag = "bpmn:dataObject")]
    DataObject(DataObject),
    #[xml(tag = "bpmn:dataObjectReference")]
    DataObjectReference(DataObjectReference),
    #[xml(tag = "bpmn:dataStoreReference")]
    DataStoreReference(DataStoreReference),
    #[xml(tag = "bpmn:endEvent")]
    EndEvent(EndEvent),
    #[xml(tag = "bpmn:event")]
    Event(Event),
    #[xml(tag = "bpmn:eventBasedGateway")]
    EventBasedGateway(EventBasedGateway),
    #[xml(tag = "bpmn:exclusiveGateway")]
    ExclusiveGateway(ExclusiveGateway),
    #[xml(tag = "bpmn:implicitThrowEvent")]
    ImplicitThrowEvent(ImplicitThrowEvent),
    #[xml(tag = "bpmn:inclusiveGateway")]
    InclusiveGateway(InclusiveGateway),
    #[xml(tag = "bpmn:intermediateCatchEvent")]
    IntermediateCatchEvent(IntermediateCatchEvent),
    #[xml(tag = "bpmn:intermediateThrowEvent")]
    IntermediateThrowEvent(IntermediateThrowEvent),
    #[xml(tag = "bpmn:manualTask")]
    ManualTask(ManualTask),
    #[xml(tag = "bpmn:parallelGateway")]
    ParallelGateway(ParallelGateway),
    #[xml(tag = "bpmn:receiveTask")]
    ReceiveTask(ReceiveTask),
    #[xml(tag = "bpmn:scriptTask")]
    ScriptTask(ScriptTask),
    #[xml(tag = "bpmn:sendTask")]
    SendTask(SendTask),
    #[xml(tag = "bpmn:sequenceFlow")]
    SequenceFlow(SequenceFlow),
    #[xml(tag = "bpmn:serviceTask")]
    ServiceTask(ServiceTask),
    #[xml(tag = "bpmn:startEvent")]
    StartEvent(StartEvent),
    #[xml(tag = "bpmn:subChoreography")]
    SubChoreography(SubChoreography),
    #[xml(tag = "bpmn:subProcess")]
    SubProcess(SubProcess),
    #[xml(tag = "bpmn:task")]
    Task(Task),
    #[xml(tag = "bpmn:transaction")]
    Transaction(Transaction),
    #[xml(tag = "bpmn:userTask")]
    UserTask(UserTask),
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:categoryValueRef")]
pub struct CategoryValueRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:flowNode")]
pub struct FlowNode {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:incoming")]
pub struct Incoming {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outgoing")]
pub struct Outgoing {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:formalExpression")]
pub struct FormalExpression {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "language")]
    pub language: Option<URI>,
    #[xml(attr = "evaluatesToTypeRef")]
    pub evaluates_totype_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:gateway")]
pub struct Gateway {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "gatewayDirection")]
    pub gateway_direction: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalBusinessRuleTask")]
pub struct GlobalBusinessRuleTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(attr = "implementation")]
    pub implementation: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalChoreographyTask")]
pub struct GlobalChoreographyTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "isClosed")]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    pub participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    pub message_flows: Vec<MessageFlow>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    pub conversation_nodes: Vec<ConversationNode>,
    #[xml(child = "bpmn:conversationAssociation")]
    pub conversation_associations: Vec<ConversationAssociation>,
    #[xml(child = "bpmn:participantAssociation")]
    pub participant_associations: Vec<ParticipantAssociation>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    pub message_flow_associations: Vec<MessageFlowAssociation>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(child = "bpmn:choreographyRef")]
    pub choreography_refs: Vec<ChoreographyRef>,
    #[xml(child = "bpmn:conversationLink")]
    pub conversation_links: Vec<ConversationLink>,
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
    pub flow_elements: Vec<FlowElement>,
    #[xml(attr = "initiatingParticipantRef")]
    pub initiating_participant_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalConversation")]
pub struct GlobalConversation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "isClosed")]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    pub participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    pub message_flows: Vec<MessageFlow>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    pub conversation_nodes: Vec<ConversationNode>,
    #[xml(child = "bpmn:conversationAssociation")]
    pub conversation_associations: Vec<ConversationAssociation>,
    #[xml(child = "bpmn:participantAssociation")]
    pub participant_associations: Vec<ParticipantAssociation>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    pub message_flow_associations: Vec<MessageFlowAssociation>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(child = "bpmn:choreographyRef")]
    pub choreography_refs: Vec<ChoreographyRef>,
    #[xml(child = "bpmn:conversationLink")]
    pub conversation_links: Vec<ConversationLink>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalManualTask")]
pub struct GlobalManualTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalScriptTask")]
pub struct GlobalScriptTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(attr = "scriptLanguage")]
    pub script_language: Option<URI>,
    #[xml(child = "bpmn:script")]
    pub script: Option<Script>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalTask")]
pub struct GlobalTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalUserTask")]
pub struct GlobalUserTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(attr = "implementation")]
    pub implementation: Option<String>,
    #[xml(child = "bpmn:rendering")]
    pub renderings: Vec<Rendering>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:group")]
pub struct Group {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "categoryValueRef")]
    pub category_value_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:humanPerformer")]
pub struct HumanPerformer {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:resourceRef")]
    pub resource_ref: ResourceRef,
    #[xml(child = "bpmn:resourceParameterBinding")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    pub resource_assignment_expression: Option<ResourceAssignmentExpression>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:implicitThrowEvent")]
pub struct ImplicitThrowEvent {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInput")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:inputSet")]
    pub input_set: Option<InputSet>,
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
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:inclusiveGateway")]
pub struct InclusiveGateway {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "gatewayDirection")]
    pub gateway_direction: Option<String>,
    #[xml(attr = "default")]
    pub default: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:inputSet")]
pub struct InputSet {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:dataInputRefs")]
    pub data_input_refss: Vec<DataInputRefs>,
    #[xml(child = "bpmn:optionalInputRefs")]
    pub optional_input_refss: Vec<OptionalInputRefs>,
    #[xml(child = "bpmn:whileExecutingInputRefs")]
    pub while_executing_input_refss: Vec<WhileExecutingInputRefs>,
    #[xml(child = "bpmn:outputSetRefs")]
    pub output_set_refss: Vec<OutputSetRefs>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataInputRefs")]
pub struct DataInputRefs {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:optionalInputRefs")]
pub struct OptionalInputRefs {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:whileExecutingInputRefs")]
pub struct WhileExecutingInputRefs {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outputSetRefs")]
pub struct OutputSetRefs {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:interface")]
pub struct Interface {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: String,
    #[xml(attr = "implementationRef")]
    pub implementation_ref: Option<String>,
    #[xml(child = "bpmn:operation")]
    pub operations: Vec<Operation>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:intermediateCatchEvent")]
pub struct IntermediateCatchEvent {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(attr = "parallelMultiple")]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:outputSet")]
    pub output_set: Option<OutputSet>,
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
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:intermediateThrowEvent")]
pub struct IntermediateThrowEvent {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInput")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:inputSet")]
    pub input_set: Option<InputSet>,
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
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:ioBinding")]
pub struct IoBinding {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "operationRef")]
    pub operation_ref: String,
    #[xml(attr = "inputDataRef")]
    pub input_data_ref: String,
    #[xml(attr = "outputDataRef")]
    pub output_data_ref: String,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:ioSpecification")]
pub struct IoSpecification {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:dataInput")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataOutput")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:inputSet")]
    pub input_sets: Vec<InputSet>,
    #[xml(child = "bpmn:outputSet")]
    pub output_sets: Vec<OutputSet>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:itemDefinition")]
pub struct ItemDefinition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "structureRef")]
    pub structure_ref: Option<String>,
    #[xml(attr = "isCollection")]
    pub is_collection: Option<bool>,
    #[xml(attr = "itemKind")]
    pub item_kind: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:lane")]
pub struct Lane {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "partitionElementRef")]
    pub partition_element_ref: Option<String>,
    #[xml(child = "bpmn:partitionElement")]
    pub partition_element: Option<PartitionElement>,
    #[xml(child = "bpmn:flowNodeRef")]
    pub flow_node_refs: Vec<FlowNodeRef>,
    #[xml(child = "bpmn:childLaneSet")]
    pub child_lane_set: Option<ChildLaneSet>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:partitionElement")]
pub struct PartitionElement {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:flowNodeRef")]
pub struct FlowNodeRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:childLaneSet")]
pub struct ChildLaneSet {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:lane")]
    pub lanes: Vec<Lane>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:laneSet")]
pub struct LaneSet {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:lane")]
    pub lanes: Vec<Lane>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:linkEventDefinition")]
pub struct LinkEventDefinition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: String,
    #[xml(child = "bpmn:source")]
    pub sources: Vec<Source>,
    #[xml(child = "bpmn:target")]
    pub target: Option<Target>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:source")]
pub struct Source {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:target")]
pub struct Target {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:loopCharacteristics")]
pub enum LoopCharacteristics {
    #[xml(tag = "bpmn:multiInstanceLoopCharacteristics")]
    MultiInstanceLoopCharacteristics(MultiInstanceLoopCharacteristics),
    #[xml(tag = "bpmn:standardLoopCharacteristics")]
    StandardLoopCharacteristics(StandardLoopCharacteristics),
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:manualTask")]
pub struct ManualTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:message")]
pub struct Message {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "itemRef")]
    pub item_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messageEventDefinition")]
pub struct MessageEventDefinition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "messageRef")]
    pub message_ref: Option<String>,
    #[xml(child = "bpmn:operationRef")]
    pub operation_ref: Option<OperationRef>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:operationRef")]
pub struct OperationRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messageFlow")]
pub struct MessageFlow {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "sourceRef")]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    pub target_ref: String,
    #[xml(attr = "messageRef")]
    pub message_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messageFlowAssociation")]
pub struct MessageFlowAssociation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "innerMessageFlowRef")]
    pub inner_message_flow_ref: String,
    #[xml(attr = "outerMessageFlowRef")]
    pub outer_message_flow_ref: String,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:monitoring")]
pub struct Monitoring {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:multiInstanceLoopCharacteristics")]
pub struct MultiInstanceLoopCharacteristics {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "isSequential")]
    pub is_sequential: Option<bool>,
    #[xml(attr = "behavior")]
    pub behavior: Option<String>,
    #[xml(attr = "oneBehaviorEventRef")]
    pub one_behavior_event_ref: Option<String>,
    #[xml(attr = "noneBehaviorEventRef")]
    pub none_behavior_event_ref: Option<String>,
    #[xml(child = "bpmn:loopCardinality")]
    pub loop_cardinality: Option<LoopCardinality>,
    #[xml(child = "bpmn:loopDataInputRef")]
    pub loop_data_input_ref: Option<LoopDataInputRef>,
    #[xml(child = "bpmn:loopDataOutputRef")]
    pub loop_data_output_ref: Option<LoopDataOutputRef>,
    #[xml(child = "bpmn:inputDataItem")]
    pub input_data_item: Option<InputDataItem>,
    #[xml(child = "bpmn:outputDataItem")]
    pub output_data_item: Option<OutputDataItem>,
    #[xml(child = "bpmn:complexBehaviorDefinition")]
    pub complex_behavior_definitions: Vec<ComplexBehaviorDefinition>,
    #[xml(child = "bpmn:completionCondition")]
    pub completion_condition: Option<CompletionCondition>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:loopCardinality")]
pub struct LoopCardinality {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:loopDataInputRef")]
pub struct LoopDataInputRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:loopDataOutputRef")]
pub struct LoopDataOutputRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:inputDataItem")]
pub struct InputDataItem {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "isCollection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outputDataItem")]
pub struct OutputDataItem {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "isCollection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:operation")]
pub struct Operation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: String,
    #[xml(attr = "implementationRef")]
    pub implementation_ref: Option<String>,
    #[xml(child = "bpmn:inMessageRef")]
    pub in_message_ref: InMessageRef,
    #[xml(child = "bpmn:outMessageRef")]
    pub out_message_ref: Option<OutMessageRef>,
    #[xml(child = "bpmn:errorRef")]
    pub error_refs: Vec<ErrorRef>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:inMessageRef")]
pub struct InMessageRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outMessageRef")]
pub struct OutMessageRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:errorRef")]
pub struct ErrorRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outputSet")]
pub struct OutputSet {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:dataOutputRefs")]
    pub data_output_refss: Vec<DataOutputRefs>,
    #[xml(child = "bpmn:optionalOutputRefs")]
    pub optional_output_refss: Vec<OptionalOutputRefs>,
    #[xml(child = "bpmn:whileExecutingOutputRefs")]
    pub while_executing_output_refss: Vec<WhileExecutingOutputRefs>,
    #[xml(child = "bpmn:inputSetRefs")]
    pub input_set_refss: Vec<InputSetRefs>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataOutputRefs")]
pub struct DataOutputRefs {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:optionalOutputRefs")]
pub struct OptionalOutputRefs {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:whileExecutingOutputRefs")]
pub struct WhileExecutingOutputRefs {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:inputSetRefs")]
pub struct InputSetRefs {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:parallelGateway")]
pub struct ParallelGateway {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "gatewayDirection")]
    pub gateway_direction: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:participant")]
pub struct Participant {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "processRef")]
    pub process_ref: Option<String>,
    #[xml(child = "bpmn:interfaceRef")]
    pub interface_refs: Vec<InterfaceRef>,
    #[xml(child = "bpmn:endPointRef")]
    pub end_point_refs: Vec<EndPointRef>,
    #[xml(child = "bpmn:participantMultiplicity")]
    pub participant_multiplicity: Option<ParticipantMultiplicity>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:interfaceRef")]
pub struct InterfaceRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:endPointRef")]
pub struct EndPointRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:participantAssociation")]
pub struct ParticipantAssociation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:innerParticipantRef")]
    pub inner_participant_ref: InnerParticipantRef,
    #[xml(child = "bpmn:outerParticipantRef")]
    pub outer_participant_ref: OuterParticipantRef,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:innerParticipantRef")]
pub struct InnerParticipantRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outerParticipantRef")]
pub struct OuterParticipantRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:participantMultiplicity")]
pub struct ParticipantMultiplicity {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "minimum")]
    pub minimum: Option<Int>,
    #[xml(attr = "maximum")]
    pub maximum: Option<Int>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:partnerEntity")]
pub struct PartnerEntity {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:partnerRole")]
pub struct PartnerRole {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:performer")]
pub struct Performer {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:resourceRef")]
    pub resource_ref: ResourceRef,
    #[xml(child = "bpmn:resourceParameterBinding")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    pub resource_assignment_expression: Option<ResourceAssignmentExpression>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:potentialOwner")]
pub struct PotentialOwner {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:resourceRef")]
    pub resource_ref: ResourceRef,
    #[xml(child = "bpmn:resourceParameterBinding")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    pub resource_assignment_expression: Option<ResourceAssignmentExpression>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:process")]
pub struct Process {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    pub io_bindings: Vec<IoBinding>,
    #[xml(attr = "processType")]
    pub process_type: Option<String>,
    #[xml(attr = "isClosed")]
    pub is_closed: Option<bool>,
    #[xml(attr = "isExecutable")]
    pub is_executable: Option<bool>,
    #[xml(attr = "definitionalCollaborationRef")]
    pub definitional_collaboration_ref: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:laneSet")]
    pub lane_sets: Vec<LaneSet>,
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
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(child = "bpmn:correlationSubscription")]
    pub correlation_subscriptions: Vec<CorrelationSubscription>,
    #[xml(child = "bpmn:supports")]
    pub supportss: Vec<Supports>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:supports")]
pub struct Supports {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:property")]
pub struct Property {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "itemSubjectRef")]
    pub item_subject_ref: Option<String>,
    #[xml(child = "bpmn:dataState")]
    pub data_state: Option<DataState>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:receiveTask")]
pub struct ReceiveTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    pub implementation: Option<String>,
    #[xml(attr = "instantiate")]
    pub instantiate: Option<bool>,
    #[xml(attr = "messageRef")]
    pub message_ref: Option<String>,
    #[xml(attr = "operationRef")]
    pub operation_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:relationship")]
pub struct Relationship {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "type")]
    pub typ: String,
    #[xml(attr = "direction")]
    pub direction: Option<String>,
    #[xml(child = "bpmn:source")]
    pub sources: Vec<Source>,
    #[xml(child = "bpmn:target")]
    pub targets: Vec<Target>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:rendering")]
pub struct Rendering {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resource")]
pub struct Resource {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: String,
    #[xml(child = "bpmn:resourceParameter")]
    pub resource_parameters: Vec<ResourceParameter>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceAssignmentExpression")]
pub struct ResourceAssignmentExpression {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:expression")]
    pub expression: Expression,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceParameter")]
pub struct ResourceParameter {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "type")]
    pub typ: Option<String>,
    #[xml(attr = "isRequired")]
    pub is_required: Option<bool>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceParameterBinding")]
pub struct ResourceParameterBinding {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "parameterRef")]
    pub parameter_ref: String,
    #[xml(child = "bpmn:expression")]
    pub expression: Expression,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceRole")]
pub struct ResourceRole {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:resourceRef")]
    pub resource_ref: ResourceRef,
    #[xml(child = "bpmn:resourceParameterBinding")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    pub resource_assignment_expression: Option<ResourceAssignmentExpression>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceRef")]
pub struct ResourceRef {
    #[xml(text, cdata)]
    pub content: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:rootElement")]
pub enum RootElement {
    #[xml(tag = "bpmn:category")]
    Category(Category),
    #[xml(tag = "bpmn:collaboration")]
    Collaboration(Collaboration),
    #[xml(tag = "bpmn:correlationProperty")]
    CorrelationProperty(CorrelationProperty),
    #[xml(tag = "bpmn:dataStore")]
    DataStore(DataStore),
    #[xml(tag = "bpmn:endPoint")]
    EndPoint(EndPoint),
    #[xml(tag = "bpmn:error")]
    Error(Error),
    #[xml(tag = "bpmn:escalation")]
    Escalation(Escalation),
    #[xml(tag = "bpmn:eventDefinition")]
    EventDefinition(EventDefinition),
    #[xml(tag = "bpmn:globalBusinessRuleTask")]
    GlobalBusinessRuleTask(GlobalBusinessRuleTask),
    #[xml(tag = "bpmn:globalManualTask")]
    GlobalManualTask(GlobalManualTask),
    #[xml(tag = "bpmn:globalScriptTask")]
    GlobalScriptTask(GlobalScriptTask),
    #[xml(tag = "bpmn:globalTask")]
    GlobalTask(GlobalTask),
    #[xml(tag = "bpmn:globalUserTask")]
    GlobalUserTask(GlobalUserTask),
    #[xml(tag = "bpmn:interface")]
    Interface(Interface),
    #[xml(tag = "bpmn:itemDefinition")]
    ItemDefinition(ItemDefinition),
    #[xml(tag = "bpmn:message")]
    Message(Message),
    #[xml(tag = "bpmn:partnerEntity")]
    PartnerEntity(PartnerEntity),
    #[xml(tag = "bpmn:partnerRole")]
    PartnerRole(PartnerRole),
    #[xml(tag = "bpmn:process")]
    Process(Process),
    #[xml(tag = "bpmn:resource")]
    Resource(Resource),
    #[xml(tag = "bpmn:signal")]
    Signal(Signal),
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:scriptTask")]
pub struct ScriptTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "scriptFormat")]
    pub script_format: Option<String>,
    #[xml(child = "bpmn:script")]
    pub script: Option<Script>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:script")]
pub struct Script {
    #[xml(text, cdata)]
    content: String,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:sendTask")]
pub struct SendTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    pub implementation: Option<String>,
    #[xml(attr = "messageRef")]
    pub message_ref: Option<String>,
    #[xml(attr = "operationRef")]
    pub operation_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:sequenceFlow")]
pub struct SequenceFlow {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(attr = "sourceRef")]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    pub target_ref: String,
    #[xml(attr = "isImmediate")]
    pub is_immediate: Option<bool>,
    #[xml(child = "bpmn:conditionExpression")]
    pub condition_expression: Option<ConditionExpression>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conditionExpression")]
pub struct ConditionExpression {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:serviceTask")]
pub struct ServiceTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    pub implementation: Option<String>,
    #[xml(attr = "operationRef")]
    pub operation_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:signal")]
pub struct Signal {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "structureRef")]
    pub structure_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:signalEventDefinition")]
pub struct SignalEventDefinition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "signalRef")]
    pub signal_ref: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:standardLoopCharacteristics")]
pub struct StandardLoopCharacteristics {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "testBefore")]
    pub test_before: Option<bool>,
    #[xml(attr = "loopMaximum")]
    pub loop_maximum: Option<Integer>,
    #[xml(child = "bpmn:loopCondition")]
    pub loop_condition: Option<LoopCondition>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:loopCondition")]
pub struct LoopCondition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:startEvent")]
pub struct StartEvent {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(attr = "parallelMultiple")]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:outputSet")]
    pub output_set: Option<OutputSet>,
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
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef>,
    #[xml(attr = "isInterrupting")]
    pub is_interrupting: Option<bool>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:subChoreography")]
pub struct SubChoreography {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "initiatingParticipantRef")]
    pub initiating_participant_ref: String,
    #[xml(attr = "loopType")]
    pub loop_type: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey>,
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
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:subConversation")]
pub struct SubConversation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:messageFlowRef")]
    pub message_flow_refs: Vec<MessageFlowRef>,
    #[xml(child = "bpmn:correlationKey")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    pub conversation_nodes: Vec<ConversationNode>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:subProcess")]
pub struct SubProcess {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "triggeredByEvent")]
    pub triggered_byevent: Option<bool>,
    #[xml(child = "bpmn:laneSet")]
    pub lane_sets: Vec<LaneSet>,
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
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:task")]
pub struct Task {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:terminateEventDefinition")]
pub struct TerminateEventDefinition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:textAnnotation")]
pub struct TextAnnotation {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "textFormat")]
    pub text_format: Option<String>,
    #[xml(child = "bpmn:text")]
    pub text: Option<Text>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:text")]
pub struct Text {
    #[xml(text, cdata)]
    content: String,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:throwEvent")]
pub struct ThrowEvent {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInput")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:inputSet")]
    pub input_set: Option<InputSet>,
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
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    pub event_definition_refs: Vec<EventDefinitionRef>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:timerEventDefinition")]
pub struct TimerEventDefinition {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:timeDate")]
    pub time_date: Option<TimeDate>,
    #[xml(child = "bpmn:timeDuration")]
    pub time_duration: Option<TimeDuration>,
    #[xml(child = "bpmn:timeCycle")]
    pub time_cycle: Option<TimeCycle>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:timeDate")]
pub struct TimeDate {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:timeDuration")]
pub struct TimeDuration {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:timeCycle")]
pub struct TimeCycle {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:transaction")]
pub struct Transaction {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "triggeredByEvent")]
    pub triggered_byevent: Option<bool>,
    #[xml(child = "bpmn:laneSet")]
    pub lane_sets: Vec<LaneSet>,
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
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    pub artifacts: Vec<Artifact>,
    #[xml(attr = "method")]
    pub method: Option<String>,
}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:userTask")]
pub struct UserTask {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    pub io_specification: Option<IoSpecification>,
    #[xml(child = "bpmn:property")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    pub implementation: Option<String>,
    #[xml(child = "bpmn:rendering")]
    pub renderings: Vec<Rendering>,
}
