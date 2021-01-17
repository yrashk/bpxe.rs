// This file is generated from BPMN 2.0 schema using `codegen.sh` script
use strong_xml::XmlRead;
#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Definitions,
    Import,
    Activity,
    AdHocSubProcess,
    CompletionCondition,
    Artifact,
    Assignment,
    From,
    To,
    Association,
    Auditing,
    BaseElement,
    BaseElementWithMixedContent,
    BoundaryEvent,
    BusinessRuleTask,
    CallableElement,
    SupportedInterfaceRef,
    CallActivity,
    CallChoreography,
    CallConversation,
    CancelEventDefinition,
    CatchEvent,
    EventDefinitionRef,
    Category,
    CategoryValue,
    Choreography,
    ChoreographyActivity,
    ParticipantRef,
    ChoreographyTask,
    MessageFlowRef,
    Collaboration,
    ChoreographyRef,
    CompensateEventDefinition,
    ComplexBehaviorDefinition,
    Condition,
    Event,
    ComplexGateway,
    ActivationCondition,
    ConditionalEventDefinition,
    Conversation,
    ConversationAssociation,
    ConversationLink,
    ConversationNode,
    CorrelationKey,
    CorrelationPropertyRef,
    CorrelationProperty,
    CorrelationPropertyBinding,
    DataPath,
    CorrelationPropertyRetrievalExpression,
    MessagePath,
    CorrelationSubscription,
    DataAssociation,
    SourceRef,
    TargetRef,
    Transformation,
    DataInput,
    DataInputAssociation,
    DataObject,
    DataObjectReference,
    DataOutput,
    DataOutputAssociation,
    DataState,
    DataStore,
    DataStoreReference,
    Documentation,
    EndEvent,
    EndPoint,
    Error,
    ErrorEventDefinition,
    Escalation,
    EscalationEventDefinition,
    EventBasedGateway,
    EventDefinition,
    ExclusiveGateway,
    Expression,
    Extension,
    ExtensionElements,
    FlowElement,
    CategoryValueRef,
    FlowNode,
    Incoming,
    Outgoing,
    FormalExpression,
    Gateway,
    GlobalBusinessRuleTask,
    GlobalChoreographyTask,
    GlobalConversation,
    GlobalManualTask,
    GlobalScriptTask,
    GlobalTask,
    GlobalUserTask,
    Group,
    HumanPerformer,
    ImplicitThrowEvent,
    InclusiveGateway,
    InputSet,
    DataInputRefs,
    OptionalInputRefs,
    WhileExecutingInputRefs,
    OutputSetRefs,
    Interface,
    IntermediateCatchEvent,
    IntermediateThrowEvent,
    IoBinding,
    IoSpecification,
    ItemDefinition,
    Lane,
    PartitionElement,
    FlowNodeRef,
    ChildLaneSet,
    LaneSet,
    LinkEventDefinition,
    Source,
    Target,
    LoopCharacteristics,
    ManualTask,
    Message,
    MessageEventDefinition,
    OperationRef,
    MessageFlow,
    MessageFlowAssociation,
    Monitoring,
    MultiInstanceLoopCharacteristics,
    LoopCardinality,
    LoopDataInputRef,
    LoopDataOutputRef,
    InputDataItem,
    OutputDataItem,
    Operation,
    InMessageRef,
    OutMessageRef,
    ErrorRef,
    OutputSet,
    DataOutputRefs,
    OptionalOutputRefs,
    WhileExecutingOutputRefs,
    InputSetRefs,
    ParallelGateway,
    Participant,
    InterfaceRef,
    EndPointRef,
    ParticipantAssociation,
    InnerParticipantRef,
    OuterParticipantRef,
    ParticipantMultiplicity,
    PartnerEntity,
    PartnerRole,
    Performer,
    PotentialOwner,
    Process,
    Supports,
    Property,
    ReceiveTask,
    Relationship,
    Rendering,
    Resource,
    ResourceAssignmentExpression,
    ResourceParameter,
    ResourceParameterBinding,
    ResourceRole,
    ResourceRef,
    RootElement,
    ScriptTask,
    Script,
    SendTask,
    SequenceFlow,
    ConditionExpression,
    ServiceTask,
    Signal,
    SignalEventDefinition,
    StandardLoopCharacteristics,
    LoopCondition,
    StartEvent,
    SubChoreography,
    SubConversation,
    SubProcess,
    Task,
    TerminateEventDefinition,
    TextAnnotation,
    Text,
    ThrowEvent,
    TimerEventDefinition,
    TimeDate,
    TimeDuration,
    TimeCycle,
    Transaction,
    UserTask,
}
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
impl DocumentElement for Definitions {
    fn element(&self) -> Element {
        Element::Definitions
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Definitions {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.imports.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.extensions.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.root_elements.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.relationships.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Import {
    fn element(&self) -> Element {
        Element::Import
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Import {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for Activity {
    fn element(&self) -> Element {
        Element::Activity
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Activity {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.io_specification.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.properies.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.data_input_associations.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.data_output_associations.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.resource_roles.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.loop_characteristics.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for AdHocSubProcess {
    fn element(&self) -> Element {
        Element::AdHocSubProcess
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for AdHocSubProcess {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.completion_condition.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for CompletionCondition {
    fn element(&self) -> Element {
        Element::CompletionCondition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CompletionCondition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElementContainer for Artifact {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            Artifact::Association(e) => e.find_by_id(id),
            Artifact::Group(e) => e.find_by_id(id),
            Artifact::TextAnnotation(e) => e.find_by_id(id),
        }
    }
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
impl DocumentElement for Assignment {
    fn element(&self) -> Element {
        Element::Assignment
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Assignment {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.from.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.to.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for From {
    fn element(&self) -> Element {
        Element::From
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for From {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for To {
    fn element(&self) -> Element {
        Element::To
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for To {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Association {
    fn element(&self) -> Element {
        Element::Association
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Association {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Auditing {
    fn element(&self) -> Element {
        Element::Auditing
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Auditing {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for BaseElement {
    fn element(&self) -> Element {
        Element::BaseElement
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for BaseElement {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.documentations.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.extension_elements.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for BaseElementWithMixedContent {
    fn element(&self) -> Element {
        Element::BaseElementWithMixedContent
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for BaseElementWithMixedContent {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.documentations.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.extension_elements.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for BoundaryEvent {
    fn element(&self) -> Element {
        Element::BoundaryEvent
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for BoundaryEvent {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for BusinessRuleTask {
    fn element(&self) -> Element {
        Element::BusinessRuleTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for BusinessRuleTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for CallableElement {
    fn element(&self) -> Element {
        Element::CallableElement
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CallableElement {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.supported_interface_refs.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.io_specification.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.io_bindings.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for SupportedInterfaceRef {
    fn element(&self) -> Element {
        Element::SupportedInterfaceRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for SupportedInterfaceRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for CallActivity {
    fn element(&self) -> Element {
        Element::CallActivity
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CallActivity {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for CallChoreography {
    fn element(&self) -> Element {
        Element::CallChoreography
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CallChoreography {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.participant_associations.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for CallConversation {
    fn element(&self) -> Element {
        Element::CallConversation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CallConversation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.participant_associations.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for CancelEventDefinition {
    fn element(&self) -> Element {
        Element::CancelEventDefinition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CancelEventDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for CatchEvent {
    fn element(&self) -> Element {
        Element::CatchEvent
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CatchEvent {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_outputs.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.data_output_associations.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.output_set.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.event_definitions.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.event_definition_refs.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for EventDefinitionRef {
    fn element(&self) -> Element {
        Element::EventDefinitionRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for EventDefinitionRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for Category {
    fn element(&self) -> Element {
        Element::Category
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Category {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.category_values.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for CategoryValue {
    fn element(&self) -> Element {
        Element::CategoryValue
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CategoryValue {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Choreography {
    fn element(&self) -> Element {
        Element::Choreography
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Choreography {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.flow_elements.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for ChoreographyActivity {
    fn element(&self) -> Element {
        Element::ChoreographyActivity
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ChoreographyActivity {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.participant_refs.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.correlation_keys.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for ParticipantRef {
    fn element(&self) -> Element {
        Element::ParticipantRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ParticipantRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for ChoreographyTask {
    fn element(&self) -> Element {
        Element::ChoreographyTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ChoreographyTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.message_flow_ref.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for MessageFlowRef {
    fn element(&self) -> Element {
        Element::MessageFlowRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for MessageFlowRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for Collaboration {
    fn element(&self) -> Element {
        Element::Collaboration
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Collaboration {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.participants.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.message_flows.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.artifacts.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.conversation_nodes.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.conversation_associations.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.participant_associations.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.message_flow_associations.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.correlation_keys.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.choreography_refs.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.conversation_links.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for ChoreographyRef {
    fn element(&self) -> Element {
        Element::ChoreographyRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ChoreographyRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for CompensateEventDefinition {
    fn element(&self) -> Element {
        Element::CompensateEventDefinition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CompensateEventDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for ComplexBehaviorDefinition {
    fn element(&self) -> Element {
        Element::ComplexBehaviorDefinition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ComplexBehaviorDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.condition.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.event.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Condition {
    fn element(&self) -> Element {
        Element::Condition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Condition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Event {
    fn element(&self) -> Element {
        Element::Event
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Event {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for ComplexGateway {
    fn element(&self) -> Element {
        Element::ComplexGateway
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ComplexGateway {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.activation_condition.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for ActivationCondition {
    fn element(&self) -> Element {
        Element::ActivationCondition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ActivationCondition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for ConditionalEventDefinition {
    fn element(&self) -> Element {
        Element::ConditionalEventDefinition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ConditionalEventDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.condition.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Conversation {
    fn element(&self) -> Element {
        Element::Conversation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Conversation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for ConversationAssociation {
    fn element(&self) -> Element {
        Element::ConversationAssociation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ConversationAssociation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for ConversationLink {
    fn element(&self) -> Element {
        Element::ConversationLink
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ConversationLink {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElementContainer for ConversationNode {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            ConversationNode::CallConversation(e) => e.find_by_id(id),
            ConversationNode::Conversation(e) => e.find_by_id(id),
            ConversationNode::SubConversation(e) => e.find_by_id(id),
        }
    }
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
impl DocumentElement for CorrelationKey {
    fn element(&self) -> Element {
        Element::CorrelationKey
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CorrelationKey {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.correlation_property_refs.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for CorrelationPropertyRef {
    fn element(&self) -> Element {
        Element::CorrelationPropertyRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CorrelationPropertyRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for CorrelationProperty {
    fn element(&self) -> Element {
        Element::CorrelationProperty
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CorrelationProperty {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self
            .correlation_property_retrieval_expressions
            .find_by_id(id)
        {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for CorrelationPropertyBinding {
    fn element(&self) -> Element {
        Element::CorrelationPropertyBinding
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CorrelationPropertyBinding {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_path.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for DataPath {
    fn element(&self) -> Element {
        Element::DataPath
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataPath {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for CorrelationPropertyRetrievalExpression {
    fn element(&self) -> Element {
        Element::CorrelationPropertyRetrievalExpression
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CorrelationPropertyRetrievalExpression {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.message_path.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for MessagePath {
    fn element(&self) -> Element {
        Element::MessagePath
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for MessagePath {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for CorrelationSubscription {
    fn element(&self) -> Element {
        Element::CorrelationSubscription
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CorrelationSubscription {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.correlation_property_bindings.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for DataAssociation {
    fn element(&self) -> Element {
        Element::DataAssociation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataAssociation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.source_refs.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.target_ref.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.transformation.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.assignments.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for SourceRef {
    fn element(&self) -> Element {
        Element::SourceRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for SourceRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for TargetRef {
    fn element(&self) -> Element {
        Element::TargetRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for TargetRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for Transformation {
    fn element(&self) -> Element {
        Element::Transformation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Transformation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for DataInput {
    fn element(&self) -> Element {
        Element::DataInput
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataInput {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for DataInputAssociation {
    fn element(&self) -> Element {
        Element::DataInputAssociation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataInputAssociation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for DataObject {
    fn element(&self) -> Element {
        Element::DataObject
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataObject {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for DataObjectReference {
    fn element(&self) -> Element {
        Element::DataObjectReference
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataObjectReference {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for DataOutput {
    fn element(&self) -> Element {
        Element::DataOutput
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataOutput {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for DataOutputAssociation {
    fn element(&self) -> Element {
        Element::DataOutputAssociation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataOutputAssociation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for DataState {
    fn element(&self) -> Element {
        Element::DataState
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataState {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for DataStore {
    fn element(&self) -> Element {
        Element::DataStore
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataStore {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for DataStoreReference {
    fn element(&self) -> Element {
        Element::DataStoreReference
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataStoreReference {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Documentation {
    fn element(&self) -> Element {
        Element::Documentation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Documentation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for EndEvent {
    fn element(&self) -> Element {
        Element::EndEvent
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for EndEvent {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for EndPoint {
    fn element(&self) -> Element {
        Element::EndPoint
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for EndPoint {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Error {
    fn element(&self) -> Element {
        Element::Error
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Error {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for ErrorEventDefinition {
    fn element(&self) -> Element {
        Element::ErrorEventDefinition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ErrorEventDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Escalation {
    fn element(&self) -> Element {
        Element::Escalation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Escalation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for EscalationEventDefinition {
    fn element(&self) -> Element {
        Element::EscalationEventDefinition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for EscalationEventDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for EventBasedGateway {
    fn element(&self) -> Element {
        Element::EventBasedGateway
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for EventBasedGateway {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElementContainer for EventDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            EventDefinition::CancelEventDefinition(e) => e.find_by_id(id),
            EventDefinition::CompensateEventDefinition(e) => e.find_by_id(id),
            EventDefinition::ConditionalEventDefinition(e) => e.find_by_id(id),
            EventDefinition::ErrorEventDefinition(e) => e.find_by_id(id),
            EventDefinition::EscalationEventDefinition(e) => e.find_by_id(id),
            EventDefinition::LinkEventDefinition(e) => e.find_by_id(id),
            EventDefinition::MessageEventDefinition(e) => e.find_by_id(id),
            EventDefinition::SignalEventDefinition(e) => e.find_by_id(id),
            EventDefinition::TerminateEventDefinition(e) => e.find_by_id(id),
            EventDefinition::TimerEventDefinition(e) => e.find_by_id(id),
        }
    }
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
impl DocumentElement for ExclusiveGateway {
    fn element(&self) -> Element {
        Element::ExclusiveGateway
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ExclusiveGateway {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Expression {
    fn element(&self) -> Element {
        Element::Expression
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Expression {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Extension {
    fn element(&self) -> Element {
        Element::Extension
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Extension {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(e) = self.documentations.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for ExtensionElements {
    fn element(&self) -> Element {
        Element::ExtensionElements
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ExtensionElements {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElementContainer for FlowElement {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            FlowElement::AdHocSubProcess(e) => e.find_by_id(id),
            FlowElement::BoundaryEvent(e) => e.find_by_id(id),
            FlowElement::BusinessRuleTask(e) => e.find_by_id(id),
            FlowElement::CallActivity(e) => e.find_by_id(id),
            FlowElement::CallChoreography(e) => e.find_by_id(id),
            FlowElement::ChoreographyTask(e) => e.find_by_id(id),
            FlowElement::ComplexGateway(e) => e.find_by_id(id),
            FlowElement::DataObject(e) => e.find_by_id(id),
            FlowElement::DataObjectReference(e) => e.find_by_id(id),
            FlowElement::DataStoreReference(e) => e.find_by_id(id),
            FlowElement::EndEvent(e) => e.find_by_id(id),
            FlowElement::Event(e) => e.find_by_id(id),
            FlowElement::EventBasedGateway(e) => e.find_by_id(id),
            FlowElement::ExclusiveGateway(e) => e.find_by_id(id),
            FlowElement::ImplicitThrowEvent(e) => e.find_by_id(id),
            FlowElement::InclusiveGateway(e) => e.find_by_id(id),
            FlowElement::IntermediateCatchEvent(e) => e.find_by_id(id),
            FlowElement::IntermediateThrowEvent(e) => e.find_by_id(id),
            FlowElement::ManualTask(e) => e.find_by_id(id),
            FlowElement::ParallelGateway(e) => e.find_by_id(id),
            FlowElement::ReceiveTask(e) => e.find_by_id(id),
            FlowElement::ScriptTask(e) => e.find_by_id(id),
            FlowElement::SendTask(e) => e.find_by_id(id),
            FlowElement::SequenceFlow(e) => e.find_by_id(id),
            FlowElement::ServiceTask(e) => e.find_by_id(id),
            FlowElement::StartEvent(e) => e.find_by_id(id),
            FlowElement::SubChoreography(e) => e.find_by_id(id),
            FlowElement::SubProcess(e) => e.find_by_id(id),
            FlowElement::Task(e) => e.find_by_id(id),
            FlowElement::Transaction(e) => e.find_by_id(id),
            FlowElement::UserTask(e) => e.find_by_id(id),
        }
    }
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
impl DocumentElement for CategoryValueRef {
    fn element(&self) -> Element {
        Element::CategoryValueRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for CategoryValueRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for FlowNode {
    fn element(&self) -> Element {
        Element::FlowNode
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for FlowNode {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.incomings.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.outgoings.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Incoming {
    fn element(&self) -> Element {
        Element::Incoming
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Incoming {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for Outgoing {
    fn element(&self) -> Element {
        Element::Outgoing
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Outgoing {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for FormalExpression {
    fn element(&self) -> Element {
        Element::FormalExpression
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for FormalExpression {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Gateway {
    fn element(&self) -> Element {
        Element::Gateway
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Gateway {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for GlobalBusinessRuleTask {
    fn element(&self) -> Element {
        Element::GlobalBusinessRuleTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for GlobalBusinessRuleTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for GlobalChoreographyTask {
    fn element(&self) -> Element {
        Element::GlobalChoreographyTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for GlobalChoreographyTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for GlobalConversation {
    fn element(&self) -> Element {
        Element::GlobalConversation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for GlobalConversation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for GlobalManualTask {
    fn element(&self) -> Element {
        Element::GlobalManualTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for GlobalManualTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for GlobalScriptTask {
    fn element(&self) -> Element {
        Element::GlobalScriptTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for GlobalScriptTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.script.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for GlobalTask {
    fn element(&self) -> Element {
        Element::GlobalTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for GlobalTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.resource_roles.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for GlobalUserTask {
    fn element(&self) -> Element {
        Element::GlobalUserTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for GlobalUserTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.renderings.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Group {
    fn element(&self) -> Element {
        Element::Group
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Group {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for HumanPerformer {
    fn element(&self) -> Element {
        Element::HumanPerformer
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for HumanPerformer {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for ImplicitThrowEvent {
    fn element(&self) -> Element {
        Element::ImplicitThrowEvent
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ImplicitThrowEvent {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for InclusiveGateway {
    fn element(&self) -> Element {
        Element::InclusiveGateway
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for InclusiveGateway {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for InputSet {
    fn element(&self) -> Element {
        Element::InputSet
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for InputSet {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_input_refss.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.optional_input_refss.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.while_executing_input_refss.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.output_set_refss.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for DataInputRefs {
    fn element(&self) -> Element {
        Element::DataInputRefs
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataInputRefs {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for OptionalInputRefs {
    fn element(&self) -> Element {
        Element::OptionalInputRefs
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for OptionalInputRefs {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for WhileExecutingInputRefs {
    fn element(&self) -> Element {
        Element::WhileExecutingInputRefs
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for WhileExecutingInputRefs {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for OutputSetRefs {
    fn element(&self) -> Element {
        Element::OutputSetRefs
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for OutputSetRefs {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for Interface {
    fn element(&self) -> Element {
        Element::Interface
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Interface {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.operations.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for IntermediateCatchEvent {
    fn element(&self) -> Element {
        Element::IntermediateCatchEvent
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for IntermediateCatchEvent {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for IntermediateThrowEvent {
    fn element(&self) -> Element {
        Element::IntermediateThrowEvent
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for IntermediateThrowEvent {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for IoBinding {
    fn element(&self) -> Element {
        Element::IoBinding
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for IoBinding {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for IoSpecification {
    fn element(&self) -> Element {
        Element::IoSpecification
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for IoSpecification {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_inputs.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.data_outputs.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.input_sets.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.output_sets.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for ItemDefinition {
    fn element(&self) -> Element {
        Element::ItemDefinition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ItemDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Lane {
    fn element(&self) -> Element {
        Element::Lane
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Lane {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.partition_element.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.flow_node_refs.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.child_lane_set.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for PartitionElement {
    fn element(&self) -> Element {
        Element::PartitionElement
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for PartitionElement {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.documentations.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.extension_elements.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for FlowNodeRef {
    fn element(&self) -> Element {
        Element::FlowNodeRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for FlowNodeRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for ChildLaneSet {
    fn element(&self) -> Element {
        Element::ChildLaneSet
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ChildLaneSet {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.lanes.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for LaneSet {
    fn element(&self) -> Element {
        Element::LaneSet
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for LaneSet {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.lanes.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for LinkEventDefinition {
    fn element(&self) -> Element {
        Element::LinkEventDefinition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for LinkEventDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.sources.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.target.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Source {
    fn element(&self) -> Element {
        Element::Source
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Source {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for Target {
    fn element(&self) -> Element {
        Element::Target
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Target {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElementContainer for LoopCharacteristics {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            LoopCharacteristics::MultiInstanceLoopCharacteristics(e) => e.find_by_id(id),
            LoopCharacteristics::StandardLoopCharacteristics(e) => e.find_by_id(id),
        }
    }
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
impl DocumentElement for ManualTask {
    fn element(&self) -> Element {
        Element::ManualTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ManualTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Message {
    fn element(&self) -> Element {
        Element::Message
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Message {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for MessageEventDefinition {
    fn element(&self) -> Element {
        Element::MessageEventDefinition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for MessageEventDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.operation_ref.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for OperationRef {
    fn element(&self) -> Element {
        Element::OperationRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for OperationRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for MessageFlow {
    fn element(&self) -> Element {
        Element::MessageFlow
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for MessageFlow {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for MessageFlowAssociation {
    fn element(&self) -> Element {
        Element::MessageFlowAssociation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for MessageFlowAssociation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Monitoring {
    fn element(&self) -> Element {
        Element::Monitoring
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Monitoring {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for MultiInstanceLoopCharacteristics {
    fn element(&self) -> Element {
        Element::MultiInstanceLoopCharacteristics
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for MultiInstanceLoopCharacteristics {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.loop_cardinality.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.loop_data_input_ref.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.loop_data_output_ref.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.input_data_item.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.output_data_item.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.complex_behavior_definitions.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.completion_condition.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for LoopCardinality {
    fn element(&self) -> Element {
        Element::LoopCardinality
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for LoopCardinality {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for LoopDataInputRef {
    fn element(&self) -> Element {
        Element::LoopDataInputRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for LoopDataInputRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for LoopDataOutputRef {
    fn element(&self) -> Element {
        Element::LoopDataOutputRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for LoopDataOutputRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for InputDataItem {
    fn element(&self) -> Element {
        Element::InputDataItem
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for InputDataItem {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for OutputDataItem {
    fn element(&self) -> Element {
        Element::OutputDataItem
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for OutputDataItem {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Operation {
    fn element(&self) -> Element {
        Element::Operation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Operation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.in_message_ref.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.out_message_ref.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.error_refs.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for InMessageRef {
    fn element(&self) -> Element {
        Element::InMessageRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for InMessageRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for OutMessageRef {
    fn element(&self) -> Element {
        Element::OutMessageRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for OutMessageRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for ErrorRef {
    fn element(&self) -> Element {
        Element::ErrorRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ErrorRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for OutputSet {
    fn element(&self) -> Element {
        Element::OutputSet
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for OutputSet {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_output_refss.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.optional_output_refss.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.while_executing_output_refss.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.input_set_refss.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for DataOutputRefs {
    fn element(&self) -> Element {
        Element::DataOutputRefs
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for DataOutputRefs {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for OptionalOutputRefs {
    fn element(&self) -> Element {
        Element::OptionalOutputRefs
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for OptionalOutputRefs {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for WhileExecutingOutputRefs {
    fn element(&self) -> Element {
        Element::WhileExecutingOutputRefs
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for WhileExecutingOutputRefs {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for InputSetRefs {
    fn element(&self) -> Element {
        Element::InputSetRefs
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for InputSetRefs {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for ParallelGateway {
    fn element(&self) -> Element {
        Element::ParallelGateway
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ParallelGateway {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Participant {
    fn element(&self) -> Element {
        Element::Participant
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Participant {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.interface_refs.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.end_point_refs.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.participant_multiplicity.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for InterfaceRef {
    fn element(&self) -> Element {
        Element::InterfaceRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for InterfaceRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for EndPointRef {
    fn element(&self) -> Element {
        Element::EndPointRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for EndPointRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for ParticipantAssociation {
    fn element(&self) -> Element {
        Element::ParticipantAssociation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ParticipantAssociation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.inner_participant_ref.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.outer_participant_ref.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for InnerParticipantRef {
    fn element(&self) -> Element {
        Element::InnerParticipantRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for InnerParticipantRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for OuterParticipantRef {
    fn element(&self) -> Element {
        Element::OuterParticipantRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for OuterParticipantRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for ParticipantMultiplicity {
    fn element(&self) -> Element {
        Element::ParticipantMultiplicity
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ParticipantMultiplicity {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for PartnerEntity {
    fn element(&self) -> Element {
        Element::PartnerEntity
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for PartnerEntity {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.participant_refs.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for PartnerRole {
    fn element(&self) -> Element {
        Element::PartnerRole
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for PartnerRole {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.participant_refs.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Performer {
    fn element(&self) -> Element {
        Element::Performer
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Performer {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for PotentialOwner {
    fn element(&self) -> Element {
        Element::PotentialOwner
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for PotentialOwner {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Process {
    fn element(&self) -> Element {
        Element::Process
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Process {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.auditing.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.monitoring.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.properies.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.lane_sets.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.flow_elements.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.artifacts.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.resource_roles.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.correlation_subscriptions.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.supportss.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Supports {
    fn element(&self) -> Element {
        Element::Supports
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Supports {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for Property {
    fn element(&self) -> Element {
        Element::Property
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Property {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for ReceiveTask {
    fn element(&self) -> Element {
        Element::ReceiveTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ReceiveTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Relationship {
    fn element(&self) -> Element {
        Element::Relationship
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Relationship {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.sources.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.targets.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Rendering {
    fn element(&self) -> Element {
        Element::Rendering
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Rendering {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Resource {
    fn element(&self) -> Element {
        Element::Resource
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Resource {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.resource_parameters.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for ResourceAssignmentExpression {
    fn element(&self) -> Element {
        Element::ResourceAssignmentExpression
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ResourceAssignmentExpression {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.expression.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for ResourceParameter {
    fn element(&self) -> Element {
        Element::ResourceParameter
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ResourceParameter {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for ResourceParameterBinding {
    fn element(&self) -> Element {
        Element::ResourceParameterBinding
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ResourceParameterBinding {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.expression.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for ResourceRole {
    fn element(&self) -> Element {
        Element::ResourceRole
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ResourceRole {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.resource_ref.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.resource_parameter_bindings.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.resource_assignment_expression.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for ResourceRef {
    fn element(&self) -> Element {
        Element::ResourceRef
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ResourceRef {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElementContainer for RootElement {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            RootElement::Category(e) => e.find_by_id(id),
            RootElement::Collaboration(e) => e.find_by_id(id),
            RootElement::CorrelationProperty(e) => e.find_by_id(id),
            RootElement::DataStore(e) => e.find_by_id(id),
            RootElement::EndPoint(e) => e.find_by_id(id),
            RootElement::Error(e) => e.find_by_id(id),
            RootElement::Escalation(e) => e.find_by_id(id),
            RootElement::EventDefinition(e) => e.find_by_id(id),
            RootElement::GlobalBusinessRuleTask(e) => e.find_by_id(id),
            RootElement::GlobalManualTask(e) => e.find_by_id(id),
            RootElement::GlobalScriptTask(e) => e.find_by_id(id),
            RootElement::GlobalTask(e) => e.find_by_id(id),
            RootElement::GlobalUserTask(e) => e.find_by_id(id),
            RootElement::Interface(e) => e.find_by_id(id),
            RootElement::ItemDefinition(e) => e.find_by_id(id),
            RootElement::Message(e) => e.find_by_id(id),
            RootElement::PartnerEntity(e) => e.find_by_id(id),
            RootElement::PartnerRole(e) => e.find_by_id(id),
            RootElement::Process(e) => e.find_by_id(id),
            RootElement::Resource(e) => e.find_by_id(id),
            RootElement::Signal(e) => e.find_by_id(id),
        }
    }
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
impl DocumentElement for ScriptTask {
    fn element(&self) -> Element {
        Element::ScriptTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ScriptTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.script.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Script {
    fn element(&self) -> Element {
        Element::Script
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Script {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for SendTask {
    fn element(&self) -> Element {
        Element::SendTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for SendTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for SequenceFlow {
    fn element(&self) -> Element {
        Element::SequenceFlow
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for SequenceFlow {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.condition_expression.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for ConditionExpression {
    fn element(&self) -> Element {
        Element::ConditionExpression
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ConditionExpression {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for ServiceTask {
    fn element(&self) -> Element {
        Element::ServiceTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ServiceTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Signal {
    fn element(&self) -> Element {
        Element::Signal
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Signal {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for SignalEventDefinition {
    fn element(&self) -> Element {
        Element::SignalEventDefinition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for SignalEventDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for StandardLoopCharacteristics {
    fn element(&self) -> Element {
        Element::StandardLoopCharacteristics
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for StandardLoopCharacteristics {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.loop_condition.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for LoopCondition {
    fn element(&self) -> Element {
        Element::LoopCondition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for LoopCondition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for StartEvent {
    fn element(&self) -> Element {
        Element::StartEvent
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for StartEvent {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for SubChoreography {
    fn element(&self) -> Element {
        Element::SubChoreography
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for SubChoreography {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.flow_elements.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.artifacts.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for SubConversation {
    fn element(&self) -> Element {
        Element::SubConversation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for SubConversation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.conversation_nodes.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for SubProcess {
    fn element(&self) -> Element {
        Element::SubProcess
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for SubProcess {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.lane_sets.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.flow_elements.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.artifacts.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Task {
    fn element(&self) -> Element {
        Element::Task
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Task {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for TerminateEventDefinition {
    fn element(&self) -> Element {
        Element::TerminateEventDefinition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for TerminateEventDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for TextAnnotation {
    fn element(&self) -> Element {
        Element::TextAnnotation
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for TextAnnotation {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.text.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for Text {
    fn element(&self) -> Element {
        Element::Text
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Text {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        return None;
    }
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
impl DocumentElement for ThrowEvent {
    fn element(&self) -> Element {
        Element::ThrowEvent
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for ThrowEvent {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_inputs.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.data_input_associations.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.input_set.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.event_definitions.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.event_definition_refs.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for TimerEventDefinition {
    fn element(&self) -> Element {
        Element::TimerEventDefinition
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for TimerEventDefinition {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.time_date.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.time_duration.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.time_cycle.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
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
impl DocumentElement for TimeDate {
    fn element(&self) -> Element {
        Element::TimeDate
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for TimeDate {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for TimeDuration {
    fn element(&self) -> Element {
        Element::TimeDuration
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for TimeDuration {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for TimeCycle {
    fn element(&self) -> Element {
        Element::TimeCycle
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for TimeCycle {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for Transaction {
    fn element(&self) -> Element {
        Element::Transaction
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Transaction {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        return None;
    }
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
impl DocumentElement for UserTask {
    fn element(&self) -> Element {
        Element::UserTask
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for UserTask {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.renderings.find_by_id(id) {
            return Some(e);
        }
        return None;
    }
}
