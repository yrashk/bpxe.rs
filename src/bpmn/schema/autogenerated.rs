// This file is generated from BPMN 2.0 schema using `codegen.sh` script
use derive_more::AsRef;
use dyn_clone::DynClone;
use std::fmt::Debug;
use strong_xml::XmlRead;
use tia::Tia;
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
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:definitions")]
pub struct Definitions {
    #[xml(attr = "id")]
    #[tia("DefinitionsType",rg*="id")]
    pub id: Option<Id>,
    #[xml(attr = "name")]
    #[tia("DefinitionsType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "targetNamespace")]
    #[tia("DefinitionsType",rg*="target_namespace")]
    pub target_namespace: URI,
    #[xml(attr = "expressionLanguage")]
    #[tia("DefinitionsType",rg*="expression_language")]
    pub expression_language: Option<URI>,
    #[xml(attr = "typeLanguage")]
    #[tia("DefinitionsType",rg*="type_language")]
    pub type_language: Option<URI>,
    #[xml(attr = "exporter")]
    #[tia("DefinitionsType",rg*="exporter")]
    pub exporter: Option<String>,
    #[xml(attr = "exporterVersion")]
    #[tia("DefinitionsType",rg*="exporter_version")]
    pub exporter_version: Option<String>,
    #[xml(child = "bpmn:import")]
    #[tia("DefinitionsType",rg*="imports")]
    pub imports: Vec<Import>,
    #[xml(child = "bpmn:extension")]
    #[tia("DefinitionsType",rg*="extensions")]
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
    #[tia("DefinitionsType",rg*="root_elements")]
    pub root_elements: Vec<RootElement>,
    #[xml(child = "bpmn:relationship")]
    #[tia("DefinitionsType",rg*="relationships")]
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
        None
    }
}
// Traits

//

/// Schema for `definitions`
pub trait DefinitionsType: Downcast + Debug + Send + DynClone {
    /// Get value of attribute `id`
    fn id(&self) -> &Option<Id>;
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `targetNamespace`
    fn target_namespace(&self) -> &URI;
    /// Get value of attribute `expressionLanguage`
    fn expression_language(&self) -> &Option<URI>;
    /// Get value of attribute `typeLanguage`
    fn type_language(&self) -> &Option<URI>;
    /// Get value of attribute `exporter`
    fn exporter(&self) -> &Option<String>;
    /// Get value of attribute `exporterVersion`
    fn exporter_version(&self) -> &Option<String>;
    /// Get value of `import` child
    fn imports(&self) -> &Vec<Import>;
    /// Get value of `extension` child
    fn extensions(&self) -> &Vec<Extension>;
    /// Get value of `rootElement` child
    fn root_elements(&self) -> &Vec<RootElement>;
    /// Get value of `relationship` child
    fn relationships(&self) -> &Vec<Relationship>;
}
dyn_clone::clone_trait_object!(DefinitionsType);
impl_downcast!(DefinitionsType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:import")]
pub struct Import {
    #[xml(attr = "namespace")]
    #[tia("ImportType",rg*="namespace")]
    pub namespace: URI,
    #[xml(attr = "location")]
    #[tia("ImportType",rg*="location")]
    pub location: String,
    #[xml(attr = "importType")]
    #[tia("ImportType",rg*="import_type")]
    pub import_type: URI,
}
impl DocumentElement for Import {
    fn element(&self) -> Element {
        Element::Import
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for Import {}
// Traits

//

/// Schema for `import`
pub trait ImportType: Downcast + Debug + Send + DynClone {
    /// Get value of attribute `namespace`
    fn namespace(&self) -> &URI;
    /// Get value of attribute `location`
    fn location(&self) -> &String;
    /// Get value of attribute `importType`
    fn import_type(&self) -> &URI;
}
dyn_clone::clone_trait_object!(ImportType);
impl_downcast!(ImportType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:activity")]
pub enum Activity {}
impl DocumentElementContainer for Activity {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
/// Schema for `activity`
pub trait ActivityType: FlowNodeType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `isForCompensation`
    fn is_for_compensation(&self) -> &Option<bool>;
    /// Get value of attribute `startQuantity`
    fn start_quantity(&self) -> &Option<Integer>;
    /// Get value of attribute `completionQuantity`
    fn completion_quantity(&self) -> &Option<Integer>;
    /// Get value of attribute `default`
    fn default(&self) -> &Option<String>;
    /// Get value of `ioSpecification` child
    fn io_specification(&self) -> &Option<InputOutputSpecification>;
    /// Get value of `property` child
    fn properies(&self) -> &Vec<Property>;
    /// Get value of `dataInputAssociation` child
    fn data_input_associations(&self) -> &Vec<DataInputAssociation>;
    /// Get value of `dataOutputAssociation` child
    fn data_output_associations(&self) -> &Vec<DataOutputAssociation>;
    /// Get value of `resourceRole` child
    fn resource_roles(&self) -> &Vec<ResourceRole>;
    /// Get value of `loopCharacteristics` child
    fn loop_characteristics(&self) -> &Option<LoopCharacteristics>;
}
dyn_clone::clone_trait_object!(ActivityType);
impl_downcast!(ActivityType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:adHocSubProcess")]
pub struct AdHocSubProcess {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "triggeredByEvent")]
    #[tia("SubProcessType",rg*="triggered_byevent")]
    pub triggered_byevent: Option<bool>,
    #[xml(child = "bpmn:laneSet")]
    #[tia("SubProcessType",rg*="lane_sets")]
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
    #[tia("SubProcessType",rg*="flow_elements")]
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("SubProcessType",rg*="artifacts")]
    pub artifacts: Vec<Artifact>,
    #[xml(attr = "cancelRemainingInstances")]
    #[tia("AdHocSubProcessType",rg*="cancel_remaining_instances")]
    pub cancel_remaining_instances: Option<bool>,
    #[xml(attr = "ordering")]
    #[tia("AdHocSubProcessType",rg*="ordering")]
    pub ordering: Option<String>,
    #[xml(child = "bpmn:completionCondition")]
    #[tia("AdHocSubProcessType",rg*="completion_condition")]
    pub completion_condition: Option<Expression>,
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
        None
    }
}
// Traits

//

/// Schema for `adHocSubProcess`
pub trait AdHocSubProcessType: SubProcessType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `cancelRemainingInstances`
    fn cancel_remaining_instances(&self) -> &Option<bool>;
    /// Get value of attribute `ordering`
    fn ordering(&self) -> &Option<String>;
    /// Get value of `completionCondition` child
    fn completion_condition(&self) -> &Option<Expression>;
}
dyn_clone::clone_trait_object!(AdHocSubProcessType);
impl_downcast!(AdHocSubProcessType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
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
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            Artifact::Association(e) => e.find_by_id(id),
            Artifact::Group(e) => e.find_by_id(id),
            Artifact::TextAnnotation(e) => e.find_by_id(id),

            _ => None,
        }
    }
}
/// Schema for `artifact`
pub trait ArtifactType: BaseElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ArtifactType);
impl_downcast!(ArtifactType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:assignment")]
pub struct Assignment {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:from")]
    #[tia("AssignmentType",rg*="from")]
    pub from: Expression,
    #[xml(child = "bpmn:to")]
    #[tia("AssignmentType",rg*="to")]
    pub to: Expression,
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
        None
    }
}
// Traits

//

/// Schema for `assignment`
pub trait AssignmentType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of `from` child
    fn from(&self) -> &Expression;
    /// Get value of `to` child
    fn to(&self) -> &Expression;
}
dyn_clone::clone_trait_object!(AssignmentType);
impl_downcast!(AssignmentType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:association")]
pub struct Association {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "sourceRef")]
    #[tia("AssociationType",rg*="source_ref")]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    #[tia("AssociationType",rg*="target_ref")]
    pub target_ref: String,
    #[xml(attr = "associationDirection")]
    #[tia("AssociationType",rg*="association_direction")]
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

        None
    }
}
// Traits
impl ArtifactType for Association {}
//

/// Schema for `association`
pub trait AssociationType: ArtifactType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `sourceRef`
    fn source_ref(&self) -> &String;
    /// Get value of attribute `targetRef`
    fn target_ref(&self) -> &String;
    /// Get value of attribute `associationDirection`
    fn association_direction(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(AssociationType);
impl_downcast!(AssociationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:auditing")]
pub struct Auditing {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
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

        None
    }
}
// Traits

//

/// Schema for `auditing`
pub trait AuditingType: BaseElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(AuditingType);
impl_downcast!(AuditingType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:baseElement")]
pub enum BaseElement {}
impl DocumentElementContainer for BaseElement {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
/// Schema for `baseElement`
pub trait BaseElementType: Downcast + Debug + Send + DynClone {
    /// Get value of attribute `id`
    fn id(&self) -> &Option<Id>;
    /// Get value of `documentation` child
    fn documentations(&self) -> &Vec<Documentation>;
    /// Get value of `extensionElements` child
    fn extension_elements(&self) -> &Option<ExtensionElements>;
}
dyn_clone::clone_trait_object!(BaseElementType);
impl_downcast!(BaseElementType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:baseElementWithMixedContent")]
pub enum BaseElementWithMixedContent {}
impl DocumentElementContainer for BaseElementWithMixedContent {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
/// Schema for `baseElementWithMixedContent`
pub trait BaseElementWithMixedContentType: Downcast + Debug + Send + DynClone {
    /// Get value of attribute `id`
    fn id(&self) -> &Option<Id>;
    /// Get value of `documentation` child
    fn documentations(&self) -> &Vec<Documentation>;
    /// Get value of `extensionElements` child
    fn extension_elements(&self) -> &Option<ExtensionElements>;
}
dyn_clone::clone_trait_object!(BaseElementWithMixedContentType);
impl_downcast!(BaseElementWithMixedContentType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:boundaryEvent")]
pub struct BoundaryEvent {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    #[tia("EventType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(attr = "parallelMultiple")]
    #[tia("CatchEventType",rg*="parallel_multiple")]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    #[tia("CatchEventType",rg*="data_outputs")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("CatchEventType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:outputSet")]
    #[tia("CatchEventType",rg*="output_set")]
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
    #[tia("CatchEventType",rg*="event_definitions")]
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    #[tia("CatchEventType",rg*="event_definition_refs")]
    pub event_definition_refs: Vec<EventDefinitionRef>,
    #[xml(attr = "cancelActivity")]
    #[tia("BoundaryEventType",rg*="cancel_activity")]
    pub cancel_activity: Option<bool>,
    #[xml(attr = "attachedToRef")]
    #[tia("BoundaryEventType",rg*="attached_toref")]
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

        None
    }
}
// Traits

//

/// Schema for `boundaryEvent`
pub trait BoundaryEventType: CatchEventType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `cancelActivity`
    fn cancel_activity(&self) -> &Option<bool>;
    /// Get value of attribute `attachedToRef`
    fn attached_toref(&self) -> &String;
}
dyn_clone::clone_trait_object!(BoundaryEventType);
impl_downcast!(BoundaryEventType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:businessRuleTask")]
pub struct BusinessRuleTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    #[tia("BusinessRuleTaskType",rg*="implementation")]
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

        None
    }
}
// Traits
impl TaskType for BusinessRuleTask {}
//

/// Schema for `businessRuleTask`
pub trait BusinessRuleTaskType: TaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `implementation`
    fn implementation(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(BusinessRuleTaskType);
impl_downcast!(BusinessRuleTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:callableElement")]
pub struct CallableElement {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings")]
    pub io_bindings: Vec<InputOutputBinding>,
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
        None
    }
}
// Traits
impl RootElementType for CallableElement {}
//

/// Schema for `callableElement`
pub trait CallableElementType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `supportedInterfaceRef` child
    fn supported_interface_refs(&self) -> &Vec<SupportedInterfaceRef>;
    /// Get value of `ioSpecification` child
    fn io_specification(&self) -> &Option<InputOutputSpecification>;
    /// Get value of `ioBinding` child
    fn io_bindings(&self) -> &Vec<InputOutputBinding>;
}
dyn_clone::clone_trait_object!(CallableElementType);
impl_downcast!(CallableElementType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:callActivity")]
pub struct CallActivity {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "calledElement")]
    #[tia("CallActivityType",rg*="called_element")]
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

        None
    }
}
// Traits

//

/// Schema for `callActivity`
pub trait CallActivityType: ActivityType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `calledElement`
    fn called_element(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(CallActivityType);
impl_downcast!(CallActivityType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:callChoreography")]
pub struct CallChoreography {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "initiatingParticipantRef")]
    #[tia("ChoreographyActivityType",rg*="initiating_participant_ref")]
    pub initiating_participant_ref: String,
    #[xml(attr = "loopType")]
    #[tia("ChoreographyActivityType",rg*="loop_type")]
    pub loop_type: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    #[tia("ChoreographyActivityType",rg*="participant_refs")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("ChoreographyActivityType",rg*="correlation_keys")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(attr = "calledChoreographyRef")]
    #[tia("CallChoreographyType",rg*="called_choreography_ref")]
    pub called_choreography_ref: Option<String>,
    #[xml(child = "bpmn:participantAssociation")]
    #[tia("CallChoreographyType",rg*="participant_associations")]
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
        None
    }
}
// Traits

//

/// Schema for `callChoreography`
pub trait CallChoreographyType:
    ChoreographyActivityType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `calledChoreographyRef`
    fn called_choreography_ref(&self) -> &Option<String>;
    /// Get value of `participantAssociation` child
    fn participant_associations(&self) -> &Vec<ParticipantAssociation>;
}
dyn_clone::clone_trait_object!(CallChoreographyType);
impl_downcast!(CallChoreographyType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:callConversation")]
pub struct CallConversation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ConversationNodeType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    #[tia("ConversationNodeType",rg*="participant_refs")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:messageFlowRef")]
    #[tia("ConversationNodeType",rg*="message_flow_refs")]
    pub message_flow_refs: Vec<MessageFlowRef>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("ConversationNodeType",rg*="correlation_keys")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(attr = "calledCollaborationRef")]
    #[tia("CallConversationType",rg*="called_collaboration_ref")]
    pub called_collaboration_ref: Option<String>,
    #[xml(child = "bpmn:participantAssociation")]
    #[tia("CallConversationType",rg*="participant_associations")]
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
        None
    }
}
// Traits

//

/// Schema for `callConversation`
pub trait CallConversationType: ConversationNodeType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `calledCollaborationRef`
    fn called_collaboration_ref(&self) -> &Option<String>;
    /// Get value of `participantAssociation` child
    fn participant_associations(&self) -> &Vec<ParticipantAssociation>;
}
dyn_clone::clone_trait_object!(CallConversationType);
impl_downcast!(CallConversationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:cancelEventDefinition")]
pub struct CancelEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
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

        None
    }
}
// Traits
impl EventDefinitionType for CancelEventDefinition {}
impl RootElementType for CancelEventDefinition {}
//

/// Schema for `cancelEventDefinition`
pub trait CancelEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
}
dyn_clone::clone_trait_object!(CancelEventDefinitionType);
impl_downcast!(CancelEventDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:catchEvent")]
pub enum CatchEvent {}
impl DocumentElementContainer for CatchEvent {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
/// Schema for `catchEvent`
pub trait CatchEventType: EventType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `parallelMultiple`
    fn parallel_multiple(&self) -> &Option<bool>;
    /// Get value of `dataOutput` child
    fn data_outputs(&self) -> &Vec<DataOutput>;
    /// Get value of `dataOutputAssociation` child
    fn data_output_associations(&self) -> &Vec<DataOutputAssociation>;
    /// Get value of `outputSet` child
    fn output_set(&self) -> &Option<OutputSet>;
    /// Get value of `eventDefinition` child
    fn event_definitions(&self) -> &Vec<EventDefinition>;
    /// Get value of `eventDefinitionRef` child
    fn event_definition_refs(&self) -> &Vec<EventDefinitionRef>;
}
dyn_clone::clone_trait_object!(CatchEventType);
impl_downcast!(CatchEventType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:category")]
pub struct Category {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CategoryType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:categoryValue")]
    #[tia("CategoryType",rg*="category_values")]
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
        None
    }
}
// Traits
impl RootElementType for Category {}
//

/// Schema for `category`
pub trait CategoryType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `categoryValue` child
    fn category_values(&self) -> &Vec<CategoryValue>;
}
dyn_clone::clone_trait_object!(CategoryType);
impl_downcast!(CategoryType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:categoryValue")]
pub struct CategoryValue {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "value")]
    #[tia("CategoryValueType",rg*="value")]
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

        None
    }
}
// Traits

//

/// Schema for `categoryValue`
pub trait CategoryValueType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `value`
    fn value(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(CategoryValueType);
impl_downcast!(CategoryValueType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:choreography")]
pub struct Choreography {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CollaborationType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "isClosed")]
    #[tia("CollaborationType",rg*="is_closed")]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    #[tia("CollaborationType",rg*="participants")]
    pub participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    #[tia("CollaborationType",rg*="message_flows")]
    pub message_flows: Vec<MessageFlow>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("CollaborationType",rg*="artifacts")]
    pub artifacts: Vec<Artifact>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    #[tia("CollaborationType",rg*="conversation_nodes")]
    pub conversation_nodes: Vec<ConversationNode>,
    #[xml(child = "bpmn:conversationAssociation")]
    #[tia("CollaborationType",rg*="conversation_associations")]
    pub conversation_associations: Vec<ConversationAssociation>,
    #[xml(child = "bpmn:participantAssociation")]
    #[tia("CollaborationType",rg*="participant_associations")]
    pub participant_associations: Vec<ParticipantAssociation>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    #[tia("CollaborationType",rg*="message_flow_associations")]
    pub message_flow_associations: Vec<MessageFlowAssociation>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("CollaborationType",rg*="correlation_keys")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(child = "bpmn:choreographyRef")]
    #[tia("CollaborationType",rg*="choreography_refs")]
    pub choreography_refs: Vec<ChoreographyRef>,
    #[xml(child = "bpmn:conversationLink")]
    #[tia("CollaborationType",rg*="conversation_links")]
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
    #[tia("ChoreographyType",rg*="flow_elements")]
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
        None
    }
}
// Traits
impl RootElementType for Choreography {}
//

/// Schema for `choreography`
pub trait ChoreographyType: CollaborationType + Downcast + Debug + Send + DynClone {
    /// Get value of `flowElement` child
    fn flow_elements(&self) -> &Vec<FlowElement>;
}
dyn_clone::clone_trait_object!(ChoreographyType);
impl_downcast!(ChoreographyType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:choreographyActivity")]
pub enum ChoreographyActivity {}
impl DocumentElementContainer for ChoreographyActivity {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
/// Schema for `choreographyActivity`
pub trait ChoreographyActivityType: FlowNodeType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `initiatingParticipantRef`
    fn initiating_participant_ref(&self) -> &String;
    /// Get value of attribute `loopType`
    fn loop_type(&self) -> &Option<String>;
    /// Get value of `participantRef` child
    fn participant_refs(&self) -> &Vec<ParticipantRef>;
    /// Get value of `correlationKey` child
    fn correlation_keys(&self) -> &Vec<CorrelationKey>;
}
dyn_clone::clone_trait_object!(ChoreographyActivityType);
impl_downcast!(ChoreographyActivityType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:choreographyTask")]
pub struct ChoreographyTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "initiatingParticipantRef")]
    #[tia("ChoreographyActivityType",rg*="initiating_participant_ref")]
    pub initiating_participant_ref: String,
    #[xml(attr = "loopType")]
    #[tia("ChoreographyActivityType",rg*="loop_type")]
    pub loop_type: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    #[tia("ChoreographyActivityType",rg*="participant_refs")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("ChoreographyActivityType",rg*="correlation_keys")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(child = "bpmn:messageFlowRef")]
    #[tia("ChoreographyTaskType",rg*="message_flow_ref")]
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
        None
    }
}
// Traits

//

/// Schema for `choreographyTask`
pub trait ChoreographyTaskType:
    ChoreographyActivityType + Downcast + Debug + Send + DynClone
{
    /// Get value of `messageFlowRef` child
    fn message_flow_ref(&self) -> &MessageFlowRef;
}
dyn_clone::clone_trait_object!(ChoreographyTaskType);
impl_downcast!(ChoreographyTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:collaboration")]
pub struct Collaboration {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CollaborationType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "isClosed")]
    #[tia("CollaborationType",rg*="is_closed")]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    #[tia("CollaborationType",rg*="participants")]
    pub participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    #[tia("CollaborationType",rg*="message_flows")]
    pub message_flows: Vec<MessageFlow>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("CollaborationType",rg*="artifacts")]
    pub artifacts: Vec<Artifact>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    #[tia("CollaborationType",rg*="conversation_nodes")]
    pub conversation_nodes: Vec<ConversationNode>,
    #[xml(child = "bpmn:conversationAssociation")]
    #[tia("CollaborationType",rg*="conversation_associations")]
    pub conversation_associations: Vec<ConversationAssociation>,
    #[xml(child = "bpmn:participantAssociation")]
    #[tia("CollaborationType",rg*="participant_associations")]
    pub participant_associations: Vec<ParticipantAssociation>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    #[tia("CollaborationType",rg*="message_flow_associations")]
    pub message_flow_associations: Vec<MessageFlowAssociation>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("CollaborationType",rg*="correlation_keys")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(child = "bpmn:choreographyRef")]
    #[tia("CollaborationType",rg*="choreography_refs")]
    pub choreography_refs: Vec<ChoreographyRef>,
    #[xml(child = "bpmn:conversationLink")]
    #[tia("CollaborationType",rg*="conversation_links")]
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
        None
    }
}
// Traits
impl RootElementType for Collaboration {}
//

/// Schema for `collaboration`
pub trait CollaborationType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `isClosed`
    fn is_closed(&self) -> &Option<bool>;
    /// Get value of `participant` child
    fn participants(&self) -> &Vec<Participant>;
    /// Get value of `messageFlow` child
    fn message_flows(&self) -> &Vec<MessageFlow>;
    /// Get value of `artifact` child
    fn artifacts(&self) -> &Vec<Artifact>;
    /// Get value of `conversationNode` child
    fn conversation_nodes(&self) -> &Vec<ConversationNode>;
    /// Get value of `conversationAssociation` child
    fn conversation_associations(&self) -> &Vec<ConversationAssociation>;
    /// Get value of `participantAssociation` child
    fn participant_associations(&self) -> &Vec<ParticipantAssociation>;
    /// Get value of `messageFlowAssociation` child
    fn message_flow_associations(&self) -> &Vec<MessageFlowAssociation>;
    /// Get value of `correlationKey` child
    fn correlation_keys(&self) -> &Vec<CorrelationKey>;
    /// Get value of `choreographyRef` child
    fn choreography_refs(&self) -> &Vec<ChoreographyRef>;
    /// Get value of `conversationLink` child
    fn conversation_links(&self) -> &Vec<ConversationLink>;
}
dyn_clone::clone_trait_object!(CollaborationType);
impl_downcast!(CollaborationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:compensateEventDefinition")]
pub struct CompensateEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "waitForCompletion")]
    #[tia("CompensateEventDefinitionType",rg*="wait_for_completion")]
    pub wait_for_completion: Option<bool>,
    #[xml(attr = "activityRef")]
    #[tia("CompensateEventDefinitionType",rg*="activity_ref")]
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

        None
    }
}
// Traits
impl EventDefinitionType for CompensateEventDefinition {}
impl RootElementType for CompensateEventDefinition {}
//

/// Schema for `compensateEventDefinition`
pub trait CompensateEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `waitForCompletion`
    fn wait_for_completion(&self) -> &Option<bool>;
    /// Get value of attribute `activityRef`
    fn activity_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(CompensateEventDefinitionType);
impl_downcast!(CompensateEventDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:complexBehaviorDefinition")]
pub struct ComplexBehaviorDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:condition")]
    #[tia("ComplexBehaviorDefinitionType",rg*="condition")]
    pub condition: FormalExpression,
    #[xml(child = "bpmn:event")]
    #[tia("ComplexBehaviorDefinitionType",rg*="event")]
    pub event: Option<ImplicitThrowEvent>,
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
        None
    }
}
// Traits

//

/// Schema for `complexBehaviorDefinition`
pub trait ComplexBehaviorDefinitionType:
    BaseElementType + Downcast + Debug + Send + DynClone
{
    /// Get value of `condition` child
    fn condition(&self) -> &FormalExpression;
    /// Get value of `event` child
    fn event(&self) -> &Option<ImplicitThrowEvent>;
}
dyn_clone::clone_trait_object!(ComplexBehaviorDefinitionType);
impl_downcast!(ComplexBehaviorDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:complexGateway")]
pub struct ComplexGateway {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "gatewayDirection")]
    #[tia("GatewayType",rg*="gateway_direction")]
    pub gateway_direction: Option<String>,
    #[xml(attr = "default")]
    #[tia("ComplexGatewayType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:activationCondition")]
    #[tia("ComplexGatewayType",rg*="activation_condition")]
    pub activation_condition: Option<Expression>,
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
        None
    }
}
// Traits

//

/// Schema for `complexGateway`
pub trait ComplexGatewayType: GatewayType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `default`
    fn default(&self) -> &Option<String>;
    /// Get value of `activationCondition` child
    fn activation_condition(&self) -> &Option<Expression>;
}
dyn_clone::clone_trait_object!(ComplexGatewayType);
impl_downcast!(ComplexGatewayType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conditionalEventDefinition")]
pub struct ConditionalEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:condition")]
    #[tia("ConditionalEventDefinitionType",rg*="condition")]
    pub condition: Expression,
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
        None
    }
}
// Traits
impl EventDefinitionType for ConditionalEventDefinition {}
impl RootElementType for ConditionalEventDefinition {}
//

/// Schema for `conditionalEventDefinition`
pub trait ConditionalEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of `condition` child
    fn condition(&self) -> &Expression;
}
dyn_clone::clone_trait_object!(ConditionalEventDefinitionType);
impl_downcast!(ConditionalEventDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conversation")]
pub struct Conversation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ConversationNodeType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    #[tia("ConversationNodeType",rg*="participant_refs")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:messageFlowRef")]
    #[tia("ConversationNodeType",rg*="message_flow_refs")]
    pub message_flow_refs: Vec<MessageFlowRef>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("ConversationNodeType",rg*="correlation_keys")]
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

        None
    }
}
// Traits

//

/// Schema for `conversation`
pub trait ConversationType: ConversationNodeType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ConversationType);
impl_downcast!(ConversationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conversationAssociation")]
pub struct ConversationAssociation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "innerConversationNodeRef")]
    #[tia("ConversationAssociationType",rg*="inner_conversation_node_ref")]
    pub inner_conversation_node_ref: String,
    #[xml(attr = "outerConversationNodeRef")]
    #[tia("ConversationAssociationType",rg*="outer_conversation_node_ref")]
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

        None
    }
}
// Traits

//

/// Schema for `conversationAssociation`
pub trait ConversationAssociationType:
    BaseElementType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `innerConversationNodeRef`
    fn inner_conversation_node_ref(&self) -> &String;
    /// Get value of attribute `outerConversationNodeRef`
    fn outer_conversation_node_ref(&self) -> &String;
}
dyn_clone::clone_trait_object!(ConversationAssociationType);
impl_downcast!(ConversationAssociationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:conversationLink")]
pub struct ConversationLink {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ConversationLinkType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "sourceRef")]
    #[tia("ConversationLinkType",rg*="source_ref")]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    #[tia("ConversationLinkType",rg*="target_ref")]
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

        None
    }
}
// Traits

//

/// Schema for `conversationLink`
pub trait ConversationLinkType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `sourceRef`
    fn source_ref(&self) -> &String;
    /// Get value of attribute `targetRef`
    fn target_ref(&self) -> &String;
}
dyn_clone::clone_trait_object!(ConversationLinkType);
impl_downcast!(ConversationLinkType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
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
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            ConversationNode::CallConversation(e) => e.find_by_id(id),
            ConversationNode::Conversation(e) => e.find_by_id(id),
            ConversationNode::SubConversation(e) => e.find_by_id(id),

            _ => None,
        }
    }
}
/// Schema for `conversationNode`
pub trait ConversationNodeType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `participantRef` child
    fn participant_refs(&self) -> &Vec<ParticipantRef>;
    /// Get value of `messageFlowRef` child
    fn message_flow_refs(&self) -> &Vec<MessageFlowRef>;
    /// Get value of `correlationKey` child
    fn correlation_keys(&self) -> &Vec<CorrelationKey>;
}
dyn_clone::clone_trait_object!(ConversationNodeType);
impl_downcast!(ConversationNodeType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationKey")]
pub struct CorrelationKey {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CorrelationKeyType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:correlationPropertyRef")]
    #[tia("CorrelationKeyType",rg*="correlation_property_refs")]
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
        None
    }
}
// Traits

//

/// Schema for `correlationKey`
pub trait CorrelationKeyType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `correlationPropertyRef` child
    fn correlation_property_refs(&self) -> &Vec<CorrelationPropertyRef>;
}
dyn_clone::clone_trait_object!(CorrelationKeyType);
impl_downcast!(CorrelationKeyType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationProperty")]
pub struct CorrelationProperty {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CorrelationPropertyType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "type")]
    #[tia("CorrelationPropertyType",rg*="typ")]
    pub typ: Option<String>,
    #[xml(child = "bpmn:correlationPropertyRetrievalExpression")]
    #[tia("CorrelationPropertyType",rg*="correlation_property_retrieval_expressions")]
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
        None
    }
}
// Traits
impl RootElementType for CorrelationProperty {}
//

/// Schema for `correlationProperty`
pub trait CorrelationPropertyType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `type`
    fn typ(&self) -> &Option<String>;
    /// Get value of `correlationPropertyRetrievalExpression` child
    fn correlation_property_retrieval_expressions(
        &self,
    ) -> &Vec<CorrelationPropertyRetrievalExpression>;
}
dyn_clone::clone_trait_object!(CorrelationPropertyType);
impl_downcast!(CorrelationPropertyType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationPropertyBinding")]
pub struct CorrelationPropertyBinding {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "correlationPropertyRef")]
    #[tia("CorrelationPropertyBindingType",rg*="correlation_property_ref")]
    pub correlation_property_ref: String,
    #[xml(child = "bpmn:dataPath")]
    #[tia("CorrelationPropertyBindingType",rg*="data_path")]
    pub data_path: FormalExpression,
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
        None
    }
}
// Traits

//

/// Schema for `correlationPropertyBinding`
pub trait CorrelationPropertyBindingType:
    BaseElementType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `correlationPropertyRef`
    fn correlation_property_ref(&self) -> &String;
    /// Get value of `dataPath` child
    fn data_path(&self) -> &FormalExpression;
}
dyn_clone::clone_trait_object!(CorrelationPropertyBindingType);
impl_downcast!(CorrelationPropertyBindingType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationPropertyRetrievalExpression")]
pub struct CorrelationPropertyRetrievalExpression {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "messageRef")]
    #[tia("CorrelationPropertyRetrievalExpressionType",rg*="message_ref")]
    pub message_ref: String,
    #[xml(child = "bpmn:messagePath")]
    #[tia("CorrelationPropertyRetrievalExpressionType",rg*="message_path")]
    pub message_path: FormalExpression,
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
        None
    }
}
// Traits

//

/// Schema for `correlationPropertyRetrievalExpression`
pub trait CorrelationPropertyRetrievalExpressionType:
    BaseElementType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `messageRef`
    fn message_ref(&self) -> &String;
    /// Get value of `messagePath` child
    fn message_path(&self) -> &FormalExpression;
}
dyn_clone::clone_trait_object!(CorrelationPropertyRetrievalExpressionType);
impl_downcast!(CorrelationPropertyRetrievalExpressionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:correlationSubscription")]
pub struct CorrelationSubscription {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "correlationKeyRef")]
    #[tia("CorrelationSubscriptionType",rg*="correlation_key_ref")]
    pub correlation_key_ref: String,
    #[xml(child = "bpmn:correlationPropertyBinding")]
    #[tia("CorrelationSubscriptionType",rg*="correlation_property_bindings")]
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
        None
    }
}
// Traits

//

/// Schema for `correlationSubscription`
pub trait CorrelationSubscriptionType:
    BaseElementType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `correlationKeyRef`
    fn correlation_key_ref(&self) -> &String;
    /// Get value of `correlationPropertyBinding` child
    fn correlation_property_bindings(&self) -> &Vec<CorrelationPropertyBinding>;
}
dyn_clone::clone_trait_object!(CorrelationSubscriptionType);
impl_downcast!(CorrelationSubscriptionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataAssociation")]
pub struct DataAssociation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:sourceRef")]
    #[tia("DataAssociationType",rg*="source_refs")]
    pub source_refs: Vec<SourceRef>,
    #[xml(child = "bpmn:targetRef")]
    #[tia("DataAssociationType",rg*="target_ref")]
    pub target_ref: TargetRef,
    #[xml(child = "bpmn:transformation")]
    #[tia("DataAssociationType",rg*="transformation")]
    pub transformation: Option<FormalExpression>,
    #[xml(child = "bpmn:assignment")]
    #[tia("DataAssociationType",rg*="assignments")]
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
        None
    }
}
// Traits

//

/// Schema for `dataAssociation`
pub trait DataAssociationType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of `sourceRef` child
    fn source_refs(&self) -> &Vec<SourceRef>;
    /// Get value of `targetRef` child
    fn target_ref(&self) -> &TargetRef;
    /// Get value of `transformation` child
    fn transformation(&self) -> &Option<FormalExpression>;
    /// Get value of `assignment` child
    fn assignments(&self) -> &Vec<Assignment>;
}
dyn_clone::clone_trait_object!(DataAssociationType);
impl_downcast!(DataAssociationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataInput")]
pub struct DataInput {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("DataInputType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("DataInputType",rg*="item_subject_ref")]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "isCollection")]
    #[tia("DataInputType",rg*="is_collection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    #[tia("DataInputType",rg*="data_state")]
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
        None
    }
}
// Traits

//

/// Schema for `dataInput`
pub trait DataInputType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `itemSubjectRef`
    fn item_subject_ref(&self) -> &Option<String>;
    /// Get value of attribute `isCollection`
    fn is_collection(&self) -> &Option<bool>;
    /// Get value of `dataState` child
    fn data_state(&self) -> &Option<DataState>;
}
dyn_clone::clone_trait_object!(DataInputType);
impl_downcast!(DataInputType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataInputAssociation")]
pub struct DataInputAssociation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:sourceRef")]
    #[tia("DataAssociationType",rg*="source_refs")]
    pub source_refs: Vec<SourceRef>,
    #[xml(child = "bpmn:targetRef")]
    #[tia("DataAssociationType",rg*="target_ref")]
    pub target_ref: TargetRef,
    #[xml(child = "bpmn:transformation")]
    #[tia("DataAssociationType",rg*="transformation")]
    pub transformation: Option<FormalExpression>,
    #[xml(child = "bpmn:assignment")]
    #[tia("DataAssociationType",rg*="assignments")]
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

        None
    }
}
// Traits

//

/// Schema for `dataInputAssociation`
pub trait DataInputAssociationType:
    DataAssociationType + Downcast + Debug + Send + DynClone
{
}
dyn_clone::clone_trait_object!(DataInputAssociationType);
impl_downcast!(DataInputAssociationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataObject")]
pub struct DataObject {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("DataObjectType",rg*="item_subject_ref")]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "isCollection")]
    #[tia("DataObjectType",rg*="is_collection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    #[tia("DataObjectType",rg*="data_state")]
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
        None
    }
}
// Traits

//

/// Schema for `dataObject`
pub trait DataObjectType: FlowElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `itemSubjectRef`
    fn item_subject_ref(&self) -> &Option<String>;
    /// Get value of attribute `isCollection`
    fn is_collection(&self) -> &Option<bool>;
    /// Get value of `dataState` child
    fn data_state(&self) -> &Option<DataState>;
}
dyn_clone::clone_trait_object!(DataObjectType);
impl_downcast!(DataObjectType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataObjectReference")]
pub struct DataObjectReference {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("DataObjectReferenceType",rg*="item_subject_ref")]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "dataObjectRef")]
    #[tia("DataObjectReferenceType",rg*="data_object_ref")]
    pub data_object_ref: Option<String>,
    #[xml(child = "bpmn:dataState")]
    #[tia("DataObjectReferenceType",rg*="data_state")]
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
        None
    }
}
// Traits

//

/// Schema for `dataObjectReference`
pub trait DataObjectReferenceType: FlowElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `itemSubjectRef`
    fn item_subject_ref(&self) -> &Option<String>;
    /// Get value of attribute `dataObjectRef`
    fn data_object_ref(&self) -> &Option<String>;
    /// Get value of `dataState` child
    fn data_state(&self) -> &Option<DataState>;
}
dyn_clone::clone_trait_object!(DataObjectReferenceType);
impl_downcast!(DataObjectReferenceType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataOutput")]
pub struct DataOutput {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("DataOutputType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("DataOutputType",rg*="item_subject_ref")]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "isCollection")]
    #[tia("DataOutputType",rg*="is_collection")]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    #[tia("DataOutputType",rg*="data_state")]
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
        None
    }
}
// Traits

//

/// Schema for `dataOutput`
pub trait DataOutputType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `itemSubjectRef`
    fn item_subject_ref(&self) -> &Option<String>;
    /// Get value of attribute `isCollection`
    fn is_collection(&self) -> &Option<bool>;
    /// Get value of `dataState` child
    fn data_state(&self) -> &Option<DataState>;
}
dyn_clone::clone_trait_object!(DataOutputType);
impl_downcast!(DataOutputType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataOutputAssociation")]
pub struct DataOutputAssociation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:sourceRef")]
    #[tia("DataAssociationType",rg*="source_refs")]
    pub source_refs: Vec<SourceRef>,
    #[xml(child = "bpmn:targetRef")]
    #[tia("DataAssociationType",rg*="target_ref")]
    pub target_ref: TargetRef,
    #[xml(child = "bpmn:transformation")]
    #[tia("DataAssociationType",rg*="transformation")]
    pub transformation: Option<FormalExpression>,
    #[xml(child = "bpmn:assignment")]
    #[tia("DataAssociationType",rg*="assignments")]
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

        None
    }
}
// Traits

//

/// Schema for `dataOutputAssociation`
pub trait DataOutputAssociationType:
    DataAssociationType + Downcast + Debug + Send + DynClone
{
}
dyn_clone::clone_trait_object!(DataOutputAssociationType);
impl_downcast!(DataOutputAssociationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataState")]
pub struct DataState {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("DataStateType",rg*="name")]
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

        None
    }
}
// Traits

//

/// Schema for `dataState`
pub trait DataStateType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(DataStateType);
impl_downcast!(DataStateType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataStore")]
pub struct DataStore {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("DataStoreType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "capacity")]
    #[tia("DataStoreType",rg*="capacity")]
    pub capacity: Option<Integer>,
    #[xml(attr = "isUnlimited")]
    #[tia("DataStoreType",rg*="is_unlimited")]
    pub is_unlimited: Option<bool>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("DataStoreType",rg*="item_subject_ref")]
    pub item_subject_ref: Option<String>,
    #[xml(child = "bpmn:dataState")]
    #[tia("DataStoreType",rg*="data_state")]
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
        None
    }
}
// Traits
impl RootElementType for DataStore {}
//

/// Schema for `dataStore`
pub trait DataStoreType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `capacity`
    fn capacity(&self) -> &Option<Integer>;
    /// Get value of attribute `isUnlimited`
    fn is_unlimited(&self) -> &Option<bool>;
    /// Get value of attribute `itemSubjectRef`
    fn item_subject_ref(&self) -> &Option<String>;
    /// Get value of `dataState` child
    fn data_state(&self) -> &Option<DataState>;
}
dyn_clone::clone_trait_object!(DataStoreType);
impl_downcast!(DataStoreType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:dataStoreReference")]
pub struct DataStoreReference {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("DataStoreReferenceType",rg*="item_subject_ref")]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "dataStoreRef")]
    #[tia("DataStoreReferenceType",rg*="data_store_ref")]
    pub data_store_ref: Option<String>,
    #[xml(child = "bpmn:dataState")]
    #[tia("DataStoreReferenceType",rg*="data_state")]
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
        None
    }
}
// Traits

//

/// Schema for `dataStoreReference`
pub trait DataStoreReferenceType: FlowElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `itemSubjectRef`
    fn item_subject_ref(&self) -> &Option<String>;
    /// Get value of attribute `dataStoreRef`
    fn data_store_ref(&self) -> &Option<String>;
    /// Get value of `dataState` child
    fn data_state(&self) -> &Option<DataState>;
}
dyn_clone::clone_trait_object!(DataStoreReferenceType);
impl_downcast!(DataStoreReferenceType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:documentation")]
pub struct Documentation {
    #[xml(attr = "id")]
    #[tia("DocumentationType",rg*="id")]
    pub id: Option<Id>,
    #[xml(attr = "textFormat")]
    #[tia("DocumentationType",rg*="text_format")]
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

        None
    }
}
// Traits

//

/// Schema for `documentation`
pub trait DocumentationType: Downcast + Debug + Send + DynClone {
    /// Get value of attribute `id`
    fn id(&self) -> &Option<Id>;
    /// Get value of attribute `textFormat`
    fn text_format(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(DocumentationType);
impl_downcast!(DocumentationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:endEvent")]
pub struct EndEvent {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    #[tia("EventType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInput")]
    #[tia("ThrowEventType",rg*="data_inputs")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ThrowEventType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:inputSet")]
    #[tia("ThrowEventType",rg*="input_set")]
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
    #[tia("ThrowEventType",rg*="event_definitions")]
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    #[tia("ThrowEventType",rg*="event_definition_refs")]
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

        None
    }
}
// Traits

//

/// Schema for `endEvent`
pub trait EndEventType: ThrowEventType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(EndEventType);
impl_downcast!(EndEventType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:endPoint")]
pub struct EndPoint {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
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

        None
    }
}
// Traits
impl RootElementType for EndPoint {}
//

/// Schema for `endPoint`
pub trait EndPointType: RootElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(EndPointType);
impl_downcast!(EndPointType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:error")]
pub struct Error {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ErrorType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "errorCode")]
    #[tia("ErrorType",rg*="error_code")]
    pub error_code: Option<String>,
    #[xml(attr = "structureRef")]
    #[tia("ErrorType",rg*="structure_ref")]
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

        None
    }
}
// Traits
impl RootElementType for Error {}
//

/// Schema for `error`
pub trait ErrorType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `errorCode`
    fn error_code(&self) -> &Option<String>;
    /// Get value of attribute `structureRef`
    fn structure_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(ErrorType);
impl_downcast!(ErrorType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:errorEventDefinition")]
pub struct ErrorEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "errorRef")]
    #[tia("ErrorEventDefinitionType",rg*="error_ref")]
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

        None
    }
}
// Traits
impl EventDefinitionType for ErrorEventDefinition {}
impl RootElementType for ErrorEventDefinition {}
//

/// Schema for `errorEventDefinition`
pub trait ErrorEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `errorRef`
    fn error_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(ErrorEventDefinitionType);
impl_downcast!(ErrorEventDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:escalation")]
pub struct Escalation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("EscalationType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "escalationCode")]
    #[tia("EscalationType",rg*="escalation_code")]
    pub escalation_code: Option<String>,
    #[xml(attr = "structureRef")]
    #[tia("EscalationType",rg*="structure_ref")]
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

        None
    }
}
// Traits
impl RootElementType for Escalation {}
//

/// Schema for `escalation`
pub trait EscalationType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `escalationCode`
    fn escalation_code(&self) -> &Option<String>;
    /// Get value of attribute `structureRef`
    fn structure_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(EscalationType);
impl_downcast!(EscalationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:escalationEventDefinition")]
pub struct EscalationEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "escalationRef")]
    #[tia("EscalationEventDefinitionType",rg*="escalation_ref")]
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

        None
    }
}
// Traits
impl EventDefinitionType for EscalationEventDefinition {}
impl RootElementType for EscalationEventDefinition {}
//

/// Schema for `escalationEventDefinition`
pub trait EscalationEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `escalationRef`
    fn escalation_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(EscalationEventDefinitionType);
impl_downcast!(EscalationEventDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:event")]
pub enum Event {}
impl DocumentElementContainer for Event {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
/// Schema for `event`
pub trait EventType: FlowNodeType + Downcast + Debug + Send + DynClone {
    /// Get value of `property` child
    fn properies(&self) -> &Vec<Property>;
}
dyn_clone::clone_trait_object!(EventType);
impl_downcast!(EventType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:eventBasedGateway")]
pub struct EventBasedGateway {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "gatewayDirection")]
    #[tia("GatewayType",rg*="gateway_direction")]
    pub gateway_direction: Option<String>,
    #[xml(attr = "instantiate")]
    #[tia("EventBasedGatewayType",rg*="instantiate")]
    pub instantiate: Option<bool>,
    #[xml(attr = "eventGatewayType")]
    #[tia("EventBasedGatewayType",rg*="event_gateway_type")]
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

        None
    }
}
// Traits

//

/// Schema for `eventBasedGateway`
pub trait EventBasedGatewayType: GatewayType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `instantiate`
    fn instantiate(&self) -> &Option<bool>;
    /// Get value of attribute `eventGatewayType`
    fn event_gateway_type(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(EventBasedGatewayType);
impl_downcast!(EventBasedGatewayType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
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
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
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

            _ => None,
        }
    }
}
/// Schema for `eventDefinition`
pub trait EventDefinitionType: RootElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(EventDefinitionType);
impl_downcast!(EventDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:exclusiveGateway")]
pub struct ExclusiveGateway {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "gatewayDirection")]
    #[tia("GatewayType",rg*="gateway_direction")]
    pub gateway_direction: Option<String>,
    #[xml(attr = "default")]
    #[tia("ExclusiveGatewayType",rg*="default")]
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

        None
    }
}
// Traits

//

/// Schema for `exclusiveGateway`
pub trait ExclusiveGatewayType: GatewayType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `default`
    fn default(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(ExclusiveGatewayType);
impl_downcast!(ExclusiveGatewayType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:expression")]
pub struct Expression {
    #[xml(attr = "id")]
    #[tia("BaseElementWithMixedContentType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementWithMixedContentType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementWithMixedContentType",rg*="extension_elements")]
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

        None
    }
}
// Traits

//

/// Schema for `expression`
pub trait ExpressionType:
    BaseElementWithMixedContentType + Downcast + Debug + Send + DynClone
{
}
dyn_clone::clone_trait_object!(ExpressionType);
impl_downcast!(ExpressionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:extension")]
pub struct Extension {
    #[xml(attr = "definition")]
    #[tia("ExtensionType",rg*="definition")]
    pub definition: Option<String>,
    #[xml(attr = "mustUnderstand")]
    #[tia("ExtensionType",rg*="must_understand")]
    pub must_understand: Option<bool>,
    #[xml(child = "bpmn:documentation")]
    #[tia("ExtensionType",rg*="documentations")]
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
        None
    }
}
// Traits

//

/// Schema for `extension`
pub trait ExtensionType: Downcast + Debug + Send + DynClone {
    /// Get value of attribute `definition`
    fn definition(&self) -> &Option<String>;
    /// Get value of attribute `mustUnderstand`
    fn must_understand(&self) -> &Option<bool>;
    /// Get value of `documentation` child
    fn documentations(&self) -> &Vec<Documentation>;
}
dyn_clone::clone_trait_object!(ExtensionType);
impl_downcast!(ExtensionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
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
impl DocumentElementContainer for ExtensionElements {}
// Traits

//

/// Schema for `extensionElements`
pub trait ExtensionElementsType: Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ExtensionElementsType);
impl_downcast!(ExtensionElementsType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
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
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
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

            _ => None,
        }
    }
}
/// Schema for `flowElement`
pub trait FlowElementType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `auditing` child
    fn auditing(&self) -> &Option<Auditing>;
    /// Get value of `monitoring` child
    fn monitoring(&self) -> &Option<Monitoring>;
    /// Get value of `categoryValueRef` child
    fn category_value_refs(&self) -> &Vec<CategoryValueRef>;
}
dyn_clone::clone_trait_object!(FlowElementType);
impl_downcast!(FlowElementType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:flowNode")]
pub enum FlowNode {}
impl DocumentElementContainer for FlowNode {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
/// Schema for `flowNode`
pub trait FlowNodeType: FlowElementType + Downcast + Debug + Send + DynClone {
    /// Get value of `incoming` child
    fn incomings(&self) -> &Vec<Incoming>;
    /// Get value of `outgoing` child
    fn outgoings(&self) -> &Vec<Outgoing>;
}
dyn_clone::clone_trait_object!(FlowNodeType);
impl_downcast!(FlowNodeType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:formalExpression")]
pub struct FormalExpression {
    #[xml(attr = "id")]
    #[tia("BaseElementWithMixedContentType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementWithMixedContentType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementWithMixedContentType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "language")]
    #[tia("FormalExpressionType",rg*="language")]
    pub language: Option<URI>,
    #[xml(attr = "evaluatesToTypeRef")]
    #[tia("FormalExpressionType",rg*="evaluates_totype_ref")]
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

        None
    }
}
// Traits
impl ExpressionType for FormalExpression {}
//

/// Schema for `formalExpression`
pub trait FormalExpressionType: ExpressionType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `language`
    fn language(&self) -> &Option<URI>;
    /// Get value of attribute `evaluatesToTypeRef`
    fn evaluates_totype_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(FormalExpressionType);
impl_downcast!(FormalExpressionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:gateway")]
pub struct Gateway {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "gatewayDirection")]
    #[tia("GatewayType",rg*="gateway_direction")]
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

        None
    }
}
// Traits

//

/// Schema for `gateway`
pub trait GatewayType: FlowNodeType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `gatewayDirection`
    fn gateway_direction(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(GatewayType);
impl_downcast!(GatewayType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalBusinessRuleTask")]
pub struct GlobalBusinessRuleTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings")]
    pub io_bindings: Vec<InputOutputBinding>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("GlobalTaskType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(attr = "implementation")]
    #[tia("GlobalBusinessRuleTaskType",rg*="implementation")]
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

        None
    }
}
// Traits
impl RootElementType for GlobalBusinessRuleTask {}
//

/// Schema for `globalBusinessRuleTask`
pub trait GlobalBusinessRuleTaskType: GlobalTaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `implementation`
    fn implementation(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(GlobalBusinessRuleTaskType);
impl_downcast!(GlobalBusinessRuleTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalChoreographyTask")]
pub struct GlobalChoreographyTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CollaborationType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "isClosed")]
    #[tia("CollaborationType",rg*="is_closed")]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    #[tia("CollaborationType",rg*="participants")]
    pub participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    #[tia("CollaborationType",rg*="message_flows")]
    pub message_flows: Vec<MessageFlow>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("CollaborationType",rg*="artifacts")]
    pub artifacts: Vec<Artifact>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    #[tia("CollaborationType",rg*="conversation_nodes")]
    pub conversation_nodes: Vec<ConversationNode>,
    #[xml(child = "bpmn:conversationAssociation")]
    #[tia("CollaborationType",rg*="conversation_associations")]
    pub conversation_associations: Vec<ConversationAssociation>,
    #[xml(child = "bpmn:participantAssociation")]
    #[tia("CollaborationType",rg*="participant_associations")]
    pub participant_associations: Vec<ParticipantAssociation>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    #[tia("CollaborationType",rg*="message_flow_associations")]
    pub message_flow_associations: Vec<MessageFlowAssociation>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("CollaborationType",rg*="correlation_keys")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(child = "bpmn:choreographyRef")]
    #[tia("CollaborationType",rg*="choreography_refs")]
    pub choreography_refs: Vec<ChoreographyRef>,
    #[xml(child = "bpmn:conversationLink")]
    #[tia("CollaborationType",rg*="conversation_links")]
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
    #[tia("ChoreographyType",rg*="flow_elements")]
    pub flow_elements: Vec<FlowElement>,
    #[xml(attr = "initiatingParticipantRef")]
    #[tia("GlobalChoreographyTaskType",rg*="initiating_participant_ref")]
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

        None
    }
}
// Traits
impl RootElementType for GlobalChoreographyTask {}
//

/// Schema for `globalChoreographyTask`
pub trait GlobalChoreographyTaskType:
    ChoreographyType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `initiatingParticipantRef`
    fn initiating_participant_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(GlobalChoreographyTaskType);
impl_downcast!(GlobalChoreographyTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalConversation")]
pub struct GlobalConversation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CollaborationType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "isClosed")]
    #[tia("CollaborationType",rg*="is_closed")]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    #[tia("CollaborationType",rg*="participants")]
    pub participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    #[tia("CollaborationType",rg*="message_flows")]
    pub message_flows: Vec<MessageFlow>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("CollaborationType",rg*="artifacts")]
    pub artifacts: Vec<Artifact>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    #[tia("CollaborationType",rg*="conversation_nodes")]
    pub conversation_nodes: Vec<ConversationNode>,
    #[xml(child = "bpmn:conversationAssociation")]
    #[tia("CollaborationType",rg*="conversation_associations")]
    pub conversation_associations: Vec<ConversationAssociation>,
    #[xml(child = "bpmn:participantAssociation")]
    #[tia("CollaborationType",rg*="participant_associations")]
    pub participant_associations: Vec<ParticipantAssociation>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    #[tia("CollaborationType",rg*="message_flow_associations")]
    pub message_flow_associations: Vec<MessageFlowAssociation>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("CollaborationType",rg*="correlation_keys")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(child = "bpmn:choreographyRef")]
    #[tia("CollaborationType",rg*="choreography_refs")]
    pub choreography_refs: Vec<ChoreographyRef>,
    #[xml(child = "bpmn:conversationLink")]
    #[tia("CollaborationType",rg*="conversation_links")]
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

        None
    }
}
// Traits
impl RootElementType for GlobalConversation {}
//

/// Schema for `globalConversation`
pub trait GlobalConversationType: CollaborationType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(GlobalConversationType);
impl_downcast!(GlobalConversationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalManualTask")]
pub struct GlobalManualTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings")]
    pub io_bindings: Vec<InputOutputBinding>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("GlobalTaskType",rg*="resource_roles")]
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

        None
    }
}
// Traits
impl RootElementType for GlobalManualTask {}
//

/// Schema for `globalManualTask`
pub trait GlobalManualTaskType: GlobalTaskType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(GlobalManualTaskType);
impl_downcast!(GlobalManualTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalScriptTask")]
pub struct GlobalScriptTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings")]
    pub io_bindings: Vec<InputOutputBinding>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("GlobalTaskType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(attr = "scriptLanguage")]
    #[tia("GlobalScriptTaskType",rg*="script_language")]
    pub script_language: Option<URI>,
    #[xml(child = "bpmn:script")]
    #[tia("GlobalScriptTaskType",rg*="script")]
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
        None
    }
}
// Traits
impl RootElementType for GlobalScriptTask {}
//

/// Schema for `globalScriptTask`
pub trait GlobalScriptTaskType: GlobalTaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `scriptLanguage`
    fn script_language(&self) -> &Option<URI>;
    /// Get value of `script` child
    fn script(&self) -> &Option<Script>;
}
dyn_clone::clone_trait_object!(GlobalScriptTaskType);
impl_downcast!(GlobalScriptTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalTask")]
pub struct GlobalTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings")]
    pub io_bindings: Vec<InputOutputBinding>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("GlobalTaskType",rg*="resource_roles")]
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
        None
    }
}
// Traits
impl RootElementType for GlobalTask {}
//

/// Schema for `globalTask`
pub trait GlobalTaskType: CallableElementType + Downcast + Debug + Send + DynClone {
    /// Get value of `resourceRole` child
    fn resource_roles(&self) -> &Vec<ResourceRole>;
}
dyn_clone::clone_trait_object!(GlobalTaskType);
impl_downcast!(GlobalTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:globalUserTask")]
pub struct GlobalUserTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings")]
    pub io_bindings: Vec<InputOutputBinding>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("GlobalTaskType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(attr = "implementation")]
    #[tia("GlobalUserTaskType",rg*="implementation")]
    pub implementation: Option<String>,
    #[xml(child = "bpmn:rendering")]
    #[tia("GlobalUserTaskType",rg*="renderings")]
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
        None
    }
}
// Traits
impl RootElementType for GlobalUserTask {}
//

/// Schema for `globalUserTask`
pub trait GlobalUserTaskType: GlobalTaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `implementation`
    fn implementation(&self) -> &Option<String>;
    /// Get value of `rendering` child
    fn renderings(&self) -> &Vec<Rendering>;
}
dyn_clone::clone_trait_object!(GlobalUserTaskType);
impl_downcast!(GlobalUserTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:group")]
pub struct Group {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "categoryValueRef")]
    #[tia("GroupType",rg*="category_value_ref")]
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

        None
    }
}
// Traits
impl ArtifactType for Group {}
//

/// Schema for `group`
pub trait GroupType: ArtifactType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `categoryValueRef`
    fn category_value_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(GroupType);
impl_downcast!(GroupType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:humanPerformer")]
pub struct HumanPerformer {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ResourceRoleType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:resourceRef")]
    #[tia("ResourceRoleType",rg*="resource_ref")]
    pub resource_ref: ResourceRef,
    #[xml(child = "bpmn:resourceParameterBinding")]
    #[tia("ResourceRoleType",rg*="resource_parameter_bindings")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    #[tia("ResourceRoleType",rg*="resource_assignment_expression")]
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

        None
    }
}
// Traits
impl PerformerType for HumanPerformer {}
//

/// Schema for `humanPerformer`
pub trait HumanPerformerType: PerformerType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(HumanPerformerType);
impl_downcast!(HumanPerformerType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:implicitThrowEvent")]
pub struct ImplicitThrowEvent {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    #[tia("EventType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInput")]
    #[tia("ThrowEventType",rg*="data_inputs")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ThrowEventType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:inputSet")]
    #[tia("ThrowEventType",rg*="input_set")]
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
    #[tia("ThrowEventType",rg*="event_definitions")]
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    #[tia("ThrowEventType",rg*="event_definition_refs")]
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

        None
    }
}
// Traits

//

/// Schema for `implicitThrowEvent`
pub trait ImplicitThrowEventType: ThrowEventType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ImplicitThrowEventType);
impl_downcast!(ImplicitThrowEventType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:inclusiveGateway")]
pub struct InclusiveGateway {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "gatewayDirection")]
    #[tia("GatewayType",rg*="gateway_direction")]
    pub gateway_direction: Option<String>,
    #[xml(attr = "default")]
    #[tia("InclusiveGatewayType",rg*="default")]
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

        None
    }
}
// Traits

//

/// Schema for `inclusiveGateway`
pub trait InclusiveGatewayType: GatewayType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `default`
    fn default(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(InclusiveGatewayType);
impl_downcast!(InclusiveGatewayType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:inputSet")]
pub struct InputSet {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("InputSetType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:dataInputRefs")]
    #[tia("InputSetType",rg*="data_input_refss")]
    pub data_input_refss: Vec<DataInputRefs>,
    #[xml(child = "bpmn:optionalInputRefs")]
    #[tia("InputSetType",rg*="optional_input_refss")]
    pub optional_input_refss: Vec<OptionalInputRefs>,
    #[xml(child = "bpmn:whileExecutingInputRefs")]
    #[tia("InputSetType",rg*="while_executing_input_refss")]
    pub while_executing_input_refss: Vec<WhileExecutingInputRefs>,
    #[xml(child = "bpmn:outputSetRefs")]
    #[tia("InputSetType",rg*="output_set_refss")]
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
        None
    }
}
// Traits

//

/// Schema for `inputSet`
pub trait InputSetType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `dataInputRefs` child
    fn data_input_refss(&self) -> &Vec<DataInputRefs>;
    /// Get value of `optionalInputRefs` child
    fn optional_input_refss(&self) -> &Vec<OptionalInputRefs>;
    /// Get value of `whileExecutingInputRefs` child
    fn while_executing_input_refss(&self) -> &Vec<WhileExecutingInputRefs>;
    /// Get value of `outputSetRefs` child
    fn output_set_refss(&self) -> &Vec<OutputSetRefs>;
}
dyn_clone::clone_trait_object!(InputSetType);
impl_downcast!(InputSetType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:interface")]
pub struct Interface {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("InterfaceType",rg*="name")]
    pub name: String,
    #[xml(attr = "implementationRef")]
    #[tia("InterfaceType",rg*="implementation_ref")]
    pub implementation_ref: Option<String>,
    #[xml(child = "bpmn:operation")]
    #[tia("InterfaceType",rg*="operations")]
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
        None
    }
}
// Traits
impl RootElementType for Interface {}
//

/// Schema for `interface`
pub trait InterfaceType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &String;
    /// Get value of attribute `implementationRef`
    fn implementation_ref(&self) -> &Option<String>;
    /// Get value of `operation` child
    fn operations(&self) -> &Vec<Operation>;
}
dyn_clone::clone_trait_object!(InterfaceType);
impl_downcast!(InterfaceType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:intermediateCatchEvent")]
pub struct IntermediateCatchEvent {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    #[tia("EventType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(attr = "parallelMultiple")]
    #[tia("CatchEventType",rg*="parallel_multiple")]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    #[tia("CatchEventType",rg*="data_outputs")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("CatchEventType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:outputSet")]
    #[tia("CatchEventType",rg*="output_set")]
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
    #[tia("CatchEventType",rg*="event_definitions")]
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    #[tia("CatchEventType",rg*="event_definition_refs")]
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

        None
    }
}
// Traits

//

/// Schema for `intermediateCatchEvent`
pub trait IntermediateCatchEventType: CatchEventType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(IntermediateCatchEventType);
impl_downcast!(IntermediateCatchEventType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:intermediateThrowEvent")]
pub struct IntermediateThrowEvent {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    #[tia("EventType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInput")]
    #[tia("ThrowEventType",rg*="data_inputs")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ThrowEventType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:inputSet")]
    #[tia("ThrowEventType",rg*="input_set")]
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
    #[tia("ThrowEventType",rg*="event_definitions")]
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    #[tia("ThrowEventType",rg*="event_definition_refs")]
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

        None
    }
}
// Traits

//

/// Schema for `intermediateThrowEvent`
pub trait IntermediateThrowEventType: ThrowEventType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(IntermediateThrowEventType);
impl_downcast!(IntermediateThrowEventType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:ioBinding")]
pub struct InputOutputBinding {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "operationRef")]
    #[tia("InputOutputBindingType",rg*="operation_ref")]
    pub operation_ref: String,
    #[xml(attr = "inputDataRef")]
    #[tia("InputOutputBindingType",rg*="input_data_ref")]
    pub input_data_ref: String,
    #[xml(attr = "outputDataRef")]
    #[tia("InputOutputBindingType",rg*="output_data_ref")]
    pub output_data_ref: String,
}
impl DocumentElement for InputOutputBinding {
    fn element(&self) -> Element {
        Element::IoBinding
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for InputOutputBinding {
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }
}
// Traits

//

/// Schema for `ioBinding`
pub trait InputOutputBindingType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `operationRef`
    fn operation_ref(&self) -> &String;
    /// Get value of attribute `inputDataRef`
    fn input_data_ref(&self) -> &String;
    /// Get value of attribute `outputDataRef`
    fn output_data_ref(&self) -> &String;
}
dyn_clone::clone_trait_object!(InputOutputBindingType);
impl_downcast!(InputOutputBindingType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:ioSpecification")]
pub struct InputOutputSpecification {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:dataInput")]
    #[tia("InputOutputSpecificationType",rg*="data_inputs")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataOutput")]
    #[tia("InputOutputSpecificationType",rg*="data_outputs")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:inputSet")]
    #[tia("InputOutputSpecificationType",rg*="input_sets")]
    pub input_sets: Vec<InputSet>,
    #[xml(child = "bpmn:outputSet")]
    #[tia("InputOutputSpecificationType",rg*="output_sets")]
    pub output_sets: Vec<OutputSet>,
}
impl DocumentElement for InputOutputSpecification {
    fn element(&self) -> Element {
        Element::IoSpecification
    }
}
#[allow(unused_variables)]
impl DocumentElementContainer for InputOutputSpecification {
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
        None
    }
}
// Traits

//

/// Schema for `ioSpecification`
pub trait InputOutputSpecificationType:
    BaseElementType + Downcast + Debug + Send + DynClone
{
    /// Get value of `dataInput` child
    fn data_inputs(&self) -> &Vec<DataInput>;
    /// Get value of `dataOutput` child
    fn data_outputs(&self) -> &Vec<DataOutput>;
    /// Get value of `inputSet` child
    fn input_sets(&self) -> &Vec<InputSet>;
    /// Get value of `outputSet` child
    fn output_sets(&self) -> &Vec<OutputSet>;
}
dyn_clone::clone_trait_object!(InputOutputSpecificationType);
impl_downcast!(InputOutputSpecificationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:itemDefinition")]
pub struct ItemDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "structureRef")]
    #[tia("ItemDefinitionType",rg*="structure_ref")]
    pub structure_ref: Option<String>,
    #[xml(attr = "isCollection")]
    #[tia("ItemDefinitionType",rg*="is_collection")]
    pub is_collection: Option<bool>,
    #[xml(attr = "itemKind")]
    #[tia("ItemDefinitionType",rg*="item_kind")]
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

        None
    }
}
// Traits
impl RootElementType for ItemDefinition {}
//

/// Schema for `itemDefinition`
pub trait ItemDefinitionType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `structureRef`
    fn structure_ref(&self) -> &Option<String>;
    /// Get value of attribute `isCollection`
    fn is_collection(&self) -> &Option<bool>;
    /// Get value of attribute `itemKind`
    fn item_kind(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(ItemDefinitionType);
impl_downcast!(ItemDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:lane")]
pub struct Lane {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("LaneType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "partitionElementRef")]
    #[tia("LaneType",rg*="partition_element_ref")]
    pub partition_element_ref: Option<String>,
    #[xml(child = "bpmn:partitionElement")]
    #[tia("LaneType",rg*="partition_element")]
    pub partition_element: Option<BaseElement>,
    #[xml(child = "bpmn:flowNodeRef")]
    #[tia("LaneType",rg*="flow_node_refs")]
    pub flow_node_refs: Vec<FlowNodeRef>,
    #[xml(child = "bpmn:childLaneSet")]
    #[tia("LaneType",rg*="child_lane_set")]
    pub child_lane_set: Option<LaneSet>,
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
        None
    }
}
// Traits

//

/// Schema for `lane`
pub trait LaneType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `partitionElementRef`
    fn partition_element_ref(&self) -> &Option<String>;
    /// Get value of `partitionElement` child
    fn partition_element(&self) -> &Option<BaseElement>;
    /// Get value of `flowNodeRef` child
    fn flow_node_refs(&self) -> &Vec<FlowNodeRef>;
    /// Get value of `childLaneSet` child
    fn child_lane_set(&self) -> &Option<LaneSet>;
}
dyn_clone::clone_trait_object!(LaneType);
impl_downcast!(LaneType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:laneSet")]
pub struct LaneSet {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("LaneSetType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:lane")]
    #[tia("LaneSetType",rg*="lanes")]
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
        None
    }
}
// Traits

//

/// Schema for `laneSet`
pub trait LaneSetType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `lane` child
    fn lanes(&self) -> &Vec<Lane>;
}
dyn_clone::clone_trait_object!(LaneSetType);
impl_downcast!(LaneSetType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:linkEventDefinition")]
pub struct LinkEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("LinkEventDefinitionType",rg*="name")]
    pub name: String,
    #[xml(child = "bpmn:source")]
    #[tia("LinkEventDefinitionType",rg*="sources")]
    pub sources: Vec<Source>,
    #[xml(child = "bpmn:target")]
    #[tia("LinkEventDefinitionType",rg*="target")]
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
        None
    }
}
// Traits
impl EventDefinitionType for LinkEventDefinition {}
impl RootElementType for LinkEventDefinition {}
//

/// Schema for `linkEventDefinition`
pub trait LinkEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `name`
    fn name(&self) -> &String;
    /// Get value of `source` child
    fn sources(&self) -> &Vec<Source>;
    /// Get value of `target` child
    fn target(&self) -> &Option<Target>;
}
dyn_clone::clone_trait_object!(LinkEventDefinitionType);
impl_downcast!(LinkEventDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:loopCharacteristics")]
pub enum LoopCharacteristics {
    #[xml(tag = "bpmn:multiInstanceLoopCharacteristics")]
    MultiInstanceLoopCharacteristics(MultiInstanceLoopCharacteristics),
    #[xml(tag = "bpmn:standardLoopCharacteristics")]
    StandardLoopCharacteristics(StandardLoopCharacteristics),
}
impl DocumentElementContainer for LoopCharacteristics {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            LoopCharacteristics::MultiInstanceLoopCharacteristics(e) => e.find_by_id(id),
            LoopCharacteristics::StandardLoopCharacteristics(e) => e.find_by_id(id),

            _ => None,
        }
    }
}
/// Schema for `loopCharacteristics`
pub trait LoopCharacteristicsType: BaseElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(LoopCharacteristicsType);
impl_downcast!(LoopCharacteristicsType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:manualTask")]
pub struct ManualTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics")]
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

        None
    }
}
// Traits
impl TaskType for ManualTask {}
//

/// Schema for `manualTask`
pub trait ManualTaskType: TaskType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ManualTaskType);
impl_downcast!(ManualTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:message")]
pub struct Message {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("MessageType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "itemRef")]
    #[tia("MessageType",rg*="item_ref")]
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

        None
    }
}
// Traits
impl RootElementType for Message {}
//

/// Schema for `message`
pub trait MessageType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `itemRef`
    fn item_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(MessageType);
impl_downcast!(MessageType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messageEventDefinition")]
pub struct MessageEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "messageRef")]
    #[tia("MessageEventDefinitionType",rg*="message_ref")]
    pub message_ref: Option<String>,
    #[xml(child = "bpmn:operationRef")]
    #[tia("MessageEventDefinitionType",rg*="operation_ref")]
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
        None
    }
}
// Traits
impl EventDefinitionType for MessageEventDefinition {}
impl RootElementType for MessageEventDefinition {}
//

/// Schema for `messageEventDefinition`
pub trait MessageEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `messageRef`
    fn message_ref(&self) -> &Option<String>;
    /// Get value of `operationRef` child
    fn operation_ref(&self) -> &Option<OperationRef>;
}
dyn_clone::clone_trait_object!(MessageEventDefinitionType);
impl_downcast!(MessageEventDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messageFlow")]
pub struct MessageFlow {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("MessageFlowType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "sourceRef")]
    #[tia("MessageFlowType",rg*="source_ref")]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    #[tia("MessageFlowType",rg*="target_ref")]
    pub target_ref: String,
    #[xml(attr = "messageRef")]
    #[tia("MessageFlowType",rg*="message_ref")]
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

        None
    }
}
// Traits

//

/// Schema for `messageFlow`
pub trait MessageFlowType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `sourceRef`
    fn source_ref(&self) -> &String;
    /// Get value of attribute `targetRef`
    fn target_ref(&self) -> &String;
    /// Get value of attribute `messageRef`
    fn message_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(MessageFlowType);
impl_downcast!(MessageFlowType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:messageFlowAssociation")]
pub struct MessageFlowAssociation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "innerMessageFlowRef")]
    #[tia("MessageFlowAssociationType",rg*="inner_message_flow_ref")]
    pub inner_message_flow_ref: String,
    #[xml(attr = "outerMessageFlowRef")]
    #[tia("MessageFlowAssociationType",rg*="outer_message_flow_ref")]
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

        None
    }
}
// Traits

//

/// Schema for `messageFlowAssociation`
pub trait MessageFlowAssociationType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `innerMessageFlowRef`
    fn inner_message_flow_ref(&self) -> &String;
    /// Get value of attribute `outerMessageFlowRef`
    fn outer_message_flow_ref(&self) -> &String;
}
dyn_clone::clone_trait_object!(MessageFlowAssociationType);
impl_downcast!(MessageFlowAssociationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:monitoring")]
pub struct Monitoring {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
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

        None
    }
}
// Traits

//

/// Schema for `monitoring`
pub trait MonitoringType: BaseElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(MonitoringType);
impl_downcast!(MonitoringType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:multiInstanceLoopCharacteristics")]
pub struct MultiInstanceLoopCharacteristics {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "isSequential")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="is_sequential")]
    pub is_sequential: Option<bool>,
    #[xml(attr = "behavior")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="behavior")]
    pub behavior: Option<String>,
    #[xml(attr = "oneBehaviorEventRef")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="one_behavior_event_ref")]
    pub one_behavior_event_ref: Option<String>,
    #[xml(attr = "noneBehaviorEventRef")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="none_behavior_event_ref")]
    pub none_behavior_event_ref: Option<String>,
    #[xml(child = "bpmn:loopCardinality")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="loop_cardinality")]
    pub loop_cardinality: Option<Expression>,
    #[xml(child = "bpmn:loopDataInputRef")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="loop_data_input_ref")]
    pub loop_data_input_ref: Option<LoopDataInputRef>,
    #[xml(child = "bpmn:loopDataOutputRef")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="loop_data_output_ref")]
    pub loop_data_output_ref: Option<LoopDataOutputRef>,
    #[xml(child = "bpmn:inputDataItem")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="input_data_item")]
    pub input_data_item: Option<DataInput>,
    #[xml(child = "bpmn:outputDataItem")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="output_data_item")]
    pub output_data_item: Option<DataOutput>,
    #[xml(child = "bpmn:complexBehaviorDefinition")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="complex_behavior_definitions")]
    pub complex_behavior_definitions: Vec<ComplexBehaviorDefinition>,
    #[xml(child = "bpmn:completionCondition")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="completion_condition")]
    pub completion_condition: Option<Expression>,
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
        None
    }
}
// Traits
impl LoopCharacteristicsType for MultiInstanceLoopCharacteristics {}
//

/// Schema for `multiInstanceLoopCharacteristics`
pub trait MultiInstanceLoopCharacteristicsType:
    LoopCharacteristicsType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `isSequential`
    fn is_sequential(&self) -> &Option<bool>;
    /// Get value of attribute `behavior`
    fn behavior(&self) -> &Option<String>;
    /// Get value of attribute `oneBehaviorEventRef`
    fn one_behavior_event_ref(&self) -> &Option<String>;
    /// Get value of attribute `noneBehaviorEventRef`
    fn none_behavior_event_ref(&self) -> &Option<String>;
    /// Get value of `loopCardinality` child
    fn loop_cardinality(&self) -> &Option<Expression>;
    /// Get value of `loopDataInputRef` child
    fn loop_data_input_ref(&self) -> &Option<LoopDataInputRef>;
    /// Get value of `loopDataOutputRef` child
    fn loop_data_output_ref(&self) -> &Option<LoopDataOutputRef>;
    /// Get value of `inputDataItem` child
    fn input_data_item(&self) -> &Option<DataInput>;
    /// Get value of `outputDataItem` child
    fn output_data_item(&self) -> &Option<DataOutput>;
    /// Get value of `complexBehaviorDefinition` child
    fn complex_behavior_definitions(&self) -> &Vec<ComplexBehaviorDefinition>;
    /// Get value of `completionCondition` child
    fn completion_condition(&self) -> &Option<Expression>;
}
dyn_clone::clone_trait_object!(MultiInstanceLoopCharacteristicsType);
impl_downcast!(MultiInstanceLoopCharacteristicsType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:operation")]
pub struct Operation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("OperationType",rg*="name")]
    pub name: String,
    #[xml(attr = "implementationRef")]
    #[tia("OperationType",rg*="implementation_ref")]
    pub implementation_ref: Option<String>,
    #[xml(child = "bpmn:inMessageRef")]
    #[tia("OperationType",rg*="in_message_ref")]
    pub in_message_ref: InMessageRef,
    #[xml(child = "bpmn:outMessageRef")]
    #[tia("OperationType",rg*="out_message_ref")]
    pub out_message_ref: Option<OutMessageRef>,
    #[xml(child = "bpmn:errorRef")]
    #[tia("OperationType",rg*="error_refs")]
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
        None
    }
}
// Traits

//

/// Schema for `operation`
pub trait OperationType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &String;
    /// Get value of attribute `implementationRef`
    fn implementation_ref(&self) -> &Option<String>;
    /// Get value of `inMessageRef` child
    fn in_message_ref(&self) -> &InMessageRef;
    /// Get value of `outMessageRef` child
    fn out_message_ref(&self) -> &Option<OutMessageRef>;
    /// Get value of `errorRef` child
    fn error_refs(&self) -> &Vec<ErrorRef>;
}
dyn_clone::clone_trait_object!(OperationType);
impl_downcast!(OperationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:outputSet")]
pub struct OutputSet {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("OutputSetType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:dataOutputRefs")]
    #[tia("OutputSetType",rg*="data_output_refss")]
    pub data_output_refss: Vec<DataOutputRefs>,
    #[xml(child = "bpmn:optionalOutputRefs")]
    #[tia("OutputSetType",rg*="optional_output_refss")]
    pub optional_output_refss: Vec<OptionalOutputRefs>,
    #[xml(child = "bpmn:whileExecutingOutputRefs")]
    #[tia("OutputSetType",rg*="while_executing_output_refss")]
    pub while_executing_output_refss: Vec<WhileExecutingOutputRefs>,
    #[xml(child = "bpmn:inputSetRefs")]
    #[tia("OutputSetType",rg*="input_set_refss")]
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
        None
    }
}
// Traits

//

/// Schema for `outputSet`
pub trait OutputSetType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `dataOutputRefs` child
    fn data_output_refss(&self) -> &Vec<DataOutputRefs>;
    /// Get value of `optionalOutputRefs` child
    fn optional_output_refss(&self) -> &Vec<OptionalOutputRefs>;
    /// Get value of `whileExecutingOutputRefs` child
    fn while_executing_output_refss(&self) -> &Vec<WhileExecutingOutputRefs>;
    /// Get value of `inputSetRefs` child
    fn input_set_refss(&self) -> &Vec<InputSetRefs>;
}
dyn_clone::clone_trait_object!(OutputSetType);
impl_downcast!(OutputSetType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:parallelGateway")]
pub struct ParallelGateway {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "gatewayDirection")]
    #[tia("GatewayType",rg*="gateway_direction")]
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

        None
    }
}
// Traits

//

/// Schema for `parallelGateway`
pub trait ParallelGatewayType: GatewayType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ParallelGatewayType);
impl_downcast!(ParallelGatewayType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:participant")]
pub struct Participant {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ParticipantType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "processRef")]
    #[tia("ParticipantType",rg*="process_ref")]
    pub process_ref: Option<String>,
    #[xml(child = "bpmn:interfaceRef")]
    #[tia("ParticipantType",rg*="interface_refs")]
    pub interface_refs: Vec<InterfaceRef>,
    #[xml(child = "bpmn:endPointRef")]
    #[tia("ParticipantType",rg*="end_point_refs")]
    pub end_point_refs: Vec<EndPointRef>,
    #[xml(child = "bpmn:participantMultiplicity")]
    #[tia("ParticipantType",rg*="participant_multiplicity")]
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
        None
    }
}
// Traits

//

/// Schema for `participant`
pub trait ParticipantType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `processRef`
    fn process_ref(&self) -> &Option<String>;
    /// Get value of `interfaceRef` child
    fn interface_refs(&self) -> &Vec<InterfaceRef>;
    /// Get value of `endPointRef` child
    fn end_point_refs(&self) -> &Vec<EndPointRef>;
    /// Get value of `participantMultiplicity` child
    fn participant_multiplicity(&self) -> &Option<ParticipantMultiplicity>;
}
dyn_clone::clone_trait_object!(ParticipantType);
impl_downcast!(ParticipantType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:participantAssociation")]
pub struct ParticipantAssociation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:innerParticipantRef")]
    #[tia("ParticipantAssociationType",rg*="inner_participant_ref")]
    pub inner_participant_ref: InnerParticipantRef,
    #[xml(child = "bpmn:outerParticipantRef")]
    #[tia("ParticipantAssociationType",rg*="outer_participant_ref")]
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
        None
    }
}
// Traits

//

/// Schema for `participantAssociation`
pub trait ParticipantAssociationType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of `innerParticipantRef` child
    fn inner_participant_ref(&self) -> &InnerParticipantRef;
    /// Get value of `outerParticipantRef` child
    fn outer_participant_ref(&self) -> &OuterParticipantRef;
}
dyn_clone::clone_trait_object!(ParticipantAssociationType);
impl_downcast!(ParticipantAssociationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:participantMultiplicity")]
pub struct ParticipantMultiplicity {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "minimum")]
    #[tia("ParticipantMultiplicityType",rg*="minimum")]
    pub minimum: Option<Int>,
    #[xml(attr = "maximum")]
    #[tia("ParticipantMultiplicityType",rg*="maximum")]
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

        None
    }
}
// Traits

//

/// Schema for `participantMultiplicity`
pub trait ParticipantMultiplicityType:
    BaseElementType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `minimum`
    fn minimum(&self) -> &Option<Int>;
    /// Get value of attribute `maximum`
    fn maximum(&self) -> &Option<Int>;
}
dyn_clone::clone_trait_object!(ParticipantMultiplicityType);
impl_downcast!(ParticipantMultiplicityType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:partnerEntity")]
pub struct PartnerEntity {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("PartnerEntityType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    #[tia("PartnerEntityType",rg*="participant_refs")]
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
        None
    }
}
// Traits
impl RootElementType for PartnerEntity {}
//

/// Schema for `partnerEntity`
pub trait PartnerEntityType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `participantRef` child
    fn participant_refs(&self) -> &Vec<ParticipantRef>;
}
dyn_clone::clone_trait_object!(PartnerEntityType);
impl_downcast!(PartnerEntityType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:partnerRole")]
pub struct PartnerRole {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("PartnerRoleType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    #[tia("PartnerRoleType",rg*="participant_refs")]
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
        None
    }
}
// Traits
impl RootElementType for PartnerRole {}
//

/// Schema for `partnerRole`
pub trait PartnerRoleType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `participantRef` child
    fn participant_refs(&self) -> &Vec<ParticipantRef>;
}
dyn_clone::clone_trait_object!(PartnerRoleType);
impl_downcast!(PartnerRoleType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:performer")]
pub struct Performer {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ResourceRoleType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:resourceRef")]
    #[tia("ResourceRoleType",rg*="resource_ref")]
    pub resource_ref: ResourceRef,
    #[xml(child = "bpmn:resourceParameterBinding")]
    #[tia("ResourceRoleType",rg*="resource_parameter_bindings")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    #[tia("ResourceRoleType",rg*="resource_assignment_expression")]
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

        None
    }
}
// Traits

//

/// Schema for `performer`
pub trait PerformerType: ResourceRoleType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(PerformerType);
impl_downcast!(PerformerType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:potentialOwner")]
pub struct PotentialOwner {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ResourceRoleType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:resourceRef")]
    #[tia("ResourceRoleType",rg*="resource_ref")]
    pub resource_ref: ResourceRef,
    #[xml(child = "bpmn:resourceParameterBinding")]
    #[tia("ResourceRoleType",rg*="resource_parameter_bindings")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    #[tia("ResourceRoleType",rg*="resource_assignment_expression")]
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

        None
    }
}
// Traits
impl HumanPerformerType for PotentialOwner {}
impl PerformerType for PotentialOwner {}
//

/// Schema for `potentialOwner`
pub trait PotentialOwnerType: HumanPerformerType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(PotentialOwnerType);
impl_downcast!(PotentialOwnerType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:process")]
pub struct Process {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs")]
    pub supported_interface_refs: Vec<SupportedInterfaceRef>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings")]
    pub io_bindings: Vec<InputOutputBinding>,
    #[xml(attr = "processType")]
    #[tia("ProcessType",rg*="process_type")]
    pub process_type: Option<String>,
    #[xml(attr = "isClosed")]
    #[tia("ProcessType",rg*="is_closed")]
    pub is_closed: Option<bool>,
    #[xml(attr = "isExecutable")]
    #[tia("ProcessType",rg*="is_executable")]
    pub is_executable: Option<bool>,
    #[xml(attr = "definitionalCollaborationRef")]
    #[tia("ProcessType",rg*="definitional_collaboration_ref")]
    pub definitional_collaboration_ref: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("ProcessType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("ProcessType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:property")]
    #[tia("ProcessType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:laneSet")]
    #[tia("ProcessType",rg*="lane_sets")]
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
    #[tia("ProcessType",rg*="flow_elements")]
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("ProcessType",rg*="artifacts")]
    pub artifacts: Vec<Artifact>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ProcessType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(child = "bpmn:correlationSubscription")]
    #[tia("ProcessType",rg*="correlation_subscriptions")]
    pub correlation_subscriptions: Vec<CorrelationSubscription>,
    #[xml(child = "bpmn:supports")]
    #[tia("ProcessType",rg*="supportss")]
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
        None
    }
}
// Traits
impl RootElementType for Process {}
//

/// Schema for `process`
pub trait ProcessType: CallableElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `processType`
    fn process_type(&self) -> &Option<String>;
    /// Get value of attribute `isClosed`
    fn is_closed(&self) -> &Option<bool>;
    /// Get value of attribute `isExecutable`
    fn is_executable(&self) -> &Option<bool>;
    /// Get value of attribute `definitionalCollaborationRef`
    fn definitional_collaboration_ref(&self) -> &Option<String>;
    /// Get value of `auditing` child
    fn auditing(&self) -> &Option<Auditing>;
    /// Get value of `monitoring` child
    fn monitoring(&self) -> &Option<Monitoring>;
    /// Get value of `property` child
    fn properies(&self) -> &Vec<Property>;
    /// Get value of `laneSet` child
    fn lane_sets(&self) -> &Vec<LaneSet>;
    /// Get value of `flowElement` child
    fn flow_elements(&self) -> &Vec<FlowElement>;
    /// Get value of `artifact` child
    fn artifacts(&self) -> &Vec<Artifact>;
    /// Get value of `resourceRole` child
    fn resource_roles(&self) -> &Vec<ResourceRole>;
    /// Get value of `correlationSubscription` child
    fn correlation_subscriptions(&self) -> &Vec<CorrelationSubscription>;
    /// Get value of `supports` child
    fn supportss(&self) -> &Vec<Supports>;
}
dyn_clone::clone_trait_object!(ProcessType);
impl_downcast!(ProcessType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:property")]
pub struct Property {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("PropertyType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("PropertyType",rg*="item_subject_ref")]
    pub item_subject_ref: Option<String>,
    #[xml(child = "bpmn:dataState")]
    #[tia("PropertyType",rg*="data_state")]
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
        None
    }
}
// Traits

//

/// Schema for `property`
pub trait PropertyType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `itemSubjectRef`
    fn item_subject_ref(&self) -> &Option<String>;
    /// Get value of `dataState` child
    fn data_state(&self) -> &Option<DataState>;
}
dyn_clone::clone_trait_object!(PropertyType);
impl_downcast!(PropertyType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:receiveTask")]
pub struct ReceiveTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    #[tia("ReceiveTaskType",rg*="implementation")]
    pub implementation: Option<String>,
    #[xml(attr = "instantiate")]
    #[tia("ReceiveTaskType",rg*="instantiate")]
    pub instantiate: Option<bool>,
    #[xml(attr = "messageRef")]
    #[tia("ReceiveTaskType",rg*="message_ref")]
    pub message_ref: Option<String>,
    #[xml(attr = "operationRef")]
    #[tia("ReceiveTaskType",rg*="operation_ref")]
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

        None
    }
}
// Traits
impl TaskType for ReceiveTask {}
//

/// Schema for `receiveTask`
pub trait ReceiveTaskType: TaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `implementation`
    fn implementation(&self) -> &Option<String>;
    /// Get value of attribute `instantiate`
    fn instantiate(&self) -> &Option<bool>;
    /// Get value of attribute `messageRef`
    fn message_ref(&self) -> &Option<String>;
    /// Get value of attribute `operationRef`
    fn operation_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(ReceiveTaskType);
impl_downcast!(ReceiveTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:relationship")]
pub struct Relationship {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "type")]
    #[tia("RelationshipType",rg*="typ")]
    pub typ: String,
    #[xml(attr = "direction")]
    #[tia("RelationshipType",rg*="direction")]
    pub direction: Option<String>,
    #[xml(child = "bpmn:source")]
    #[tia("RelationshipType",rg*="sources")]
    pub sources: Vec<Source>,
    #[xml(child = "bpmn:target")]
    #[tia("RelationshipType",rg*="targets")]
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
        None
    }
}
// Traits

//

/// Schema for `relationship`
pub trait RelationshipType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `type`
    fn typ(&self) -> &String;
    /// Get value of attribute `direction`
    fn direction(&self) -> &Option<String>;
    /// Get value of `source` child
    fn sources(&self) -> &Vec<Source>;
    /// Get value of `target` child
    fn targets(&self) -> &Vec<Target>;
}
dyn_clone::clone_trait_object!(RelationshipType);
impl_downcast!(RelationshipType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:rendering")]
pub struct Rendering {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
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

        None
    }
}
// Traits

//

/// Schema for `rendering`
pub trait RenderingType: BaseElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(RenderingType);
impl_downcast!(RenderingType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resource")]
pub struct Resource {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ResourceType",rg*="name")]
    pub name: String,
    #[xml(child = "bpmn:resourceParameter")]
    #[tia("ResourceType",rg*="resource_parameters")]
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
        None
    }
}
// Traits
impl RootElementType for Resource {}
//

/// Schema for `resource`
pub trait ResourceType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &String;
    /// Get value of `resourceParameter` child
    fn resource_parameters(&self) -> &Vec<ResourceParameter>;
}
dyn_clone::clone_trait_object!(ResourceType);
impl_downcast!(ResourceType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceAssignmentExpression")]
pub struct ResourceAssignmentExpression {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:expression")]
    #[tia("ResourceAssignmentExpressionType",rg*="expression")]
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
        None
    }
}
// Traits

//

/// Schema for `resourceAssignmentExpression`
pub trait ResourceAssignmentExpressionType:
    BaseElementType + Downcast + Debug + Send + DynClone
{
    /// Get value of `expression` child
    fn expression(&self) -> &Expression;
}
dyn_clone::clone_trait_object!(ResourceAssignmentExpressionType);
impl_downcast!(ResourceAssignmentExpressionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceParameter")]
pub struct ResourceParameter {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ResourceParameterType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "type")]
    #[tia("ResourceParameterType",rg*="typ")]
    pub typ: Option<String>,
    #[xml(attr = "isRequired")]
    #[tia("ResourceParameterType",rg*="is_required")]
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

        None
    }
}
// Traits

//

/// Schema for `resourceParameter`
pub trait ResourceParameterType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `type`
    fn typ(&self) -> &Option<String>;
    /// Get value of attribute `isRequired`
    fn is_required(&self) -> &Option<bool>;
}
dyn_clone::clone_trait_object!(ResourceParameterType);
impl_downcast!(ResourceParameterType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceParameterBinding")]
pub struct ResourceParameterBinding {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "parameterRef")]
    #[tia("ResourceParameterBindingType",rg*="parameter_ref")]
    pub parameter_ref: String,
    #[xml(child = "bpmn:expression")]
    #[tia("ResourceParameterBindingType",rg*="expression")]
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
        None
    }
}
// Traits

//

/// Schema for `resourceParameterBinding`
pub trait ResourceParameterBindingType:
    BaseElementType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `parameterRef`
    fn parameter_ref(&self) -> &String;
    /// Get value of `expression` child
    fn expression(&self) -> &Expression;
}
dyn_clone::clone_trait_object!(ResourceParameterBindingType);
impl_downcast!(ResourceParameterBindingType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:resourceRole")]
pub struct ResourceRole {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ResourceRoleType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:resourceRef")]
    #[tia("ResourceRoleType",rg*="resource_ref")]
    pub resource_ref: ResourceRef,
    #[xml(child = "bpmn:resourceParameterBinding")]
    #[tia("ResourceRoleType",rg*="resource_parameter_bindings")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    #[tia("ResourceRoleType",rg*="resource_assignment_expression")]
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
        None
    }
}
// Traits

//

/// Schema for `resourceRole`
pub trait ResourceRoleType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `resourceRef` child
    fn resource_ref(&self) -> &ResourceRef;
    /// Get value of `resourceParameterBinding` child
    fn resource_parameter_bindings(&self) -> &Vec<ResourceParameterBinding>;
    /// Get value of `resourceAssignmentExpression` child
    fn resource_assignment_expression(&self) -> &Option<ResourceAssignmentExpression>;
}
dyn_clone::clone_trait_object!(ResourceRoleType);
impl_downcast!(ResourceRoleType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
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
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
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

            _ => None,
        }
    }
}
/// Schema for `rootElement`
pub trait RootElementType: BaseElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(RootElementType);
impl_downcast!(RootElementType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:scriptTask")]
pub struct ScriptTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "scriptFormat")]
    #[tia("ScriptTaskType",rg*="script_format")]
    pub script_format: Option<String>,
    #[xml(child = "bpmn:script")]
    #[tia("ScriptTaskType",rg*="script")]
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
        None
    }
}
// Traits
impl TaskType for ScriptTask {}
//

/// Schema for `scriptTask`
pub trait ScriptTaskType: TaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `scriptFormat`
    fn script_format(&self) -> &Option<String>;
    /// Get value of `script` child
    fn script(&self) -> &Option<Script>;
}
dyn_clone::clone_trait_object!(ScriptTaskType);
impl_downcast!(ScriptTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
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
impl DocumentElementContainer for Script {}
// Traits

//

/// Schema for `script`
pub trait ScriptType: Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ScriptType);
impl_downcast!(ScriptType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:sendTask")]
pub struct SendTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    #[tia("SendTaskType",rg*="implementation")]
    pub implementation: Option<String>,
    #[xml(attr = "messageRef")]
    #[tia("SendTaskType",rg*="message_ref")]
    pub message_ref: Option<String>,
    #[xml(attr = "operationRef")]
    #[tia("SendTaskType",rg*="operation_ref")]
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

        None
    }
}
// Traits
impl TaskType for SendTask {}
//

/// Schema for `sendTask`
pub trait SendTaskType: TaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `implementation`
    fn implementation(&self) -> &Option<String>;
    /// Get value of attribute `messageRef`
    fn message_ref(&self) -> &Option<String>;
    /// Get value of attribute `operationRef`
    fn operation_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(SendTaskType);
impl_downcast!(SendTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:sequenceFlow")]
pub struct SequenceFlow {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(attr = "sourceRef")]
    #[tia("SequenceFlowType",rg*="source_ref")]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    #[tia("SequenceFlowType",rg*="target_ref")]
    pub target_ref: String,
    #[xml(attr = "isImmediate")]
    #[tia("SequenceFlowType",rg*="is_immediate")]
    pub is_immediate: Option<bool>,
    #[xml(child = "bpmn:conditionExpression")]
    #[tia("SequenceFlowType",rg*="condition_expression")]
    pub condition_expression: Option<Expression>,
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
        None
    }
}
// Traits

//

/// Schema for `sequenceFlow`
pub trait SequenceFlowType: FlowElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `sourceRef`
    fn source_ref(&self) -> &String;
    /// Get value of attribute `targetRef`
    fn target_ref(&self) -> &String;
    /// Get value of attribute `isImmediate`
    fn is_immediate(&self) -> &Option<bool>;
    /// Get value of `conditionExpression` child
    fn condition_expression(&self) -> &Option<Expression>;
}
dyn_clone::clone_trait_object!(SequenceFlowType);
impl_downcast!(SequenceFlowType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:serviceTask")]
pub struct ServiceTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    #[tia("ServiceTaskType",rg*="implementation")]
    pub implementation: Option<String>,
    #[xml(attr = "operationRef")]
    #[tia("ServiceTaskType",rg*="operation_ref")]
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

        None
    }
}
// Traits
impl TaskType for ServiceTask {}
//

/// Schema for `serviceTask`
pub trait ServiceTaskType: TaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `implementation`
    fn implementation(&self) -> &Option<String>;
    /// Get value of attribute `operationRef`
    fn operation_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(ServiceTaskType);
impl_downcast!(ServiceTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:signal")]
pub struct Signal {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("SignalType",rg*="name")]
    pub name: Option<String>,
    #[xml(attr = "structureRef")]
    #[tia("SignalType",rg*="structure_ref")]
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

        None
    }
}
// Traits
impl RootElementType for Signal {}
//

/// Schema for `signal`
pub trait SignalType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `structureRef`
    fn structure_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(SignalType);
impl_downcast!(SignalType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:signalEventDefinition")]
pub struct SignalEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "signalRef")]
    #[tia("SignalEventDefinitionType",rg*="signal_ref")]
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

        None
    }
}
// Traits
impl EventDefinitionType for SignalEventDefinition {}
impl RootElementType for SignalEventDefinition {}
//

/// Schema for `signalEventDefinition`
pub trait SignalEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `signalRef`
    fn signal_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(SignalEventDefinitionType);
impl_downcast!(SignalEventDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:standardLoopCharacteristics")]
pub struct StandardLoopCharacteristics {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "testBefore")]
    #[tia("StandardLoopCharacteristicsType",rg*="test_before")]
    pub test_before: Option<bool>,
    #[xml(attr = "loopMaximum")]
    #[tia("StandardLoopCharacteristicsType",rg*="loop_maximum")]
    pub loop_maximum: Option<Integer>,
    #[xml(child = "bpmn:loopCondition")]
    #[tia("StandardLoopCharacteristicsType",rg*="loop_condition")]
    pub loop_condition: Option<Expression>,
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
        None
    }
}
// Traits
impl LoopCharacteristicsType for StandardLoopCharacteristics {}
//

/// Schema for `standardLoopCharacteristics`
pub trait StandardLoopCharacteristicsType:
    LoopCharacteristicsType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `testBefore`
    fn test_before(&self) -> &Option<bool>;
    /// Get value of attribute `loopMaximum`
    fn loop_maximum(&self) -> &Option<Integer>;
    /// Get value of `loopCondition` child
    fn loop_condition(&self) -> &Option<Expression>;
}
dyn_clone::clone_trait_object!(StandardLoopCharacteristicsType);
impl_downcast!(StandardLoopCharacteristicsType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:startEvent")]
pub struct StartEvent {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(child = "bpmn:property")]
    #[tia("EventType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(attr = "parallelMultiple")]
    #[tia("CatchEventType",rg*="parallel_multiple")]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    #[tia("CatchEventType",rg*="data_outputs")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("CatchEventType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:outputSet")]
    #[tia("CatchEventType",rg*="output_set")]
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
    #[tia("CatchEventType",rg*="event_definitions")]
    pub event_definitions: Vec<EventDefinition>,
    #[xml(child = "bpmn:eventDefinitionRef")]
    #[tia("CatchEventType",rg*="event_definition_refs")]
    pub event_definition_refs: Vec<EventDefinitionRef>,
    #[xml(attr = "isInterrupting")]
    #[tia("StartEventType",rg*="is_interrupting")]
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

        None
    }
}
// Traits

//

/// Schema for `startEvent`
pub trait StartEventType: CatchEventType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `isInterrupting`
    fn is_interrupting(&self) -> &Option<bool>;
}
dyn_clone::clone_trait_object!(StartEventType);
impl_downcast!(StartEventType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:subChoreography")]
pub struct SubChoreography {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "initiatingParticipantRef")]
    #[tia("ChoreographyActivityType",rg*="initiating_participant_ref")]
    pub initiating_participant_ref: String,
    #[xml(attr = "loopType")]
    #[tia("ChoreographyActivityType",rg*="loop_type")]
    pub loop_type: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    #[tia("ChoreographyActivityType",rg*="participant_refs")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("ChoreographyActivityType",rg*="correlation_keys")]
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
    #[tia("SubChoreographyType",rg*="flow_elements")]
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("SubChoreographyType",rg*="artifacts")]
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
        None
    }
}
// Traits

//

/// Schema for `subChoreography`
pub trait SubChoreographyType:
    ChoreographyActivityType + Downcast + Debug + Send + DynClone
{
    /// Get value of `flowElement` child
    fn flow_elements(&self) -> &Vec<FlowElement>;
    /// Get value of `artifact` child
    fn artifacts(&self) -> &Vec<Artifact>;
}
dyn_clone::clone_trait_object!(SubChoreographyType);
impl_downcast!(SubChoreographyType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:subConversation")]
pub struct SubConversation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ConversationNodeType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:participantRef")]
    #[tia("ConversationNodeType",rg*="participant_refs")]
    pub participant_refs: Vec<ParticipantRef>,
    #[xml(child = "bpmn:messageFlowRef")]
    #[tia("ConversationNodeType",rg*="message_flow_refs")]
    pub message_flow_refs: Vec<MessageFlowRef>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("ConversationNodeType",rg*="correlation_keys")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    #[tia("SubConversationType",rg*="conversation_nodes")]
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
        None
    }
}
// Traits

//

/// Schema for `subConversation`
pub trait SubConversationType: ConversationNodeType + Downcast + Debug + Send + DynClone {
    /// Get value of `conversationNode` child
    fn conversation_nodes(&self) -> &Vec<ConversationNode>;
}
dyn_clone::clone_trait_object!(SubConversationType);
impl_downcast!(SubConversationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:subProcess")]
pub struct SubProcess {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "triggeredByEvent")]
    #[tia("SubProcessType",rg*="triggered_byevent")]
    pub triggered_byevent: Option<bool>,
    #[xml(child = "bpmn:laneSet")]
    #[tia("SubProcessType",rg*="lane_sets")]
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
    #[tia("SubProcessType",rg*="flow_elements")]
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("SubProcessType",rg*="artifacts")]
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
        None
    }
}
// Traits

//

/// Schema for `subProcess`
pub trait SubProcessType: ActivityType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `triggeredByEvent`
    fn triggered_byevent(&self) -> &Option<bool>;
    /// Get value of `laneSet` child
    fn lane_sets(&self) -> &Vec<LaneSet>;
    /// Get value of `flowElement` child
    fn flow_elements(&self) -> &Vec<FlowElement>;
    /// Get value of `artifact` child
    fn artifacts(&self) -> &Vec<Artifact>;
}
dyn_clone::clone_trait_object!(SubProcessType);
impl_downcast!(SubProcessType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:task")]
pub struct Task {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics")]
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

        None
    }
}
// Traits

//

/// Schema for `task`
pub trait TaskType: ActivityType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(TaskType);
impl_downcast!(TaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:terminateEventDefinition")]
pub struct TerminateEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
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

        None
    }
}
// Traits
impl EventDefinitionType for TerminateEventDefinition {}
impl RootElementType for TerminateEventDefinition {}
//

/// Schema for `terminateEventDefinition`
pub trait TerminateEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
}
dyn_clone::clone_trait_object!(TerminateEventDefinitionType);
impl_downcast!(TerminateEventDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:textAnnotation")]
pub struct TextAnnotation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "textFormat")]
    #[tia("TextAnnotationType",rg*="text_format")]
    pub text_format: Option<String>,
    #[xml(child = "bpmn:text")]
    #[tia("TextAnnotationType",rg*="text")]
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
        None
    }
}
// Traits
impl ArtifactType for TextAnnotation {}
//

/// Schema for `textAnnotation`
pub trait TextAnnotationType: ArtifactType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `textFormat`
    fn text_format(&self) -> &Option<String>;
    /// Get value of `text` child
    fn text(&self) -> &Option<Text>;
}
dyn_clone::clone_trait_object!(TextAnnotationType);
impl_downcast!(TextAnnotationType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
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
impl DocumentElementContainer for Text {}
// Traits

//

/// Schema for `text`
pub trait TextType: Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(TextType);
impl_downcast!(TextType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]
#[xml(tag = "bpmn:throwEvent")]
pub enum ThrowEvent {}
impl DocumentElementContainer for ThrowEvent {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
/// Schema for `throwEvent`
pub trait ThrowEventType: EventType + Downcast + Debug + Send + DynClone {
    /// Get value of `dataInput` child
    fn data_inputs(&self) -> &Vec<DataInput>;
    /// Get value of `dataInputAssociation` child
    fn data_input_associations(&self) -> &Vec<DataInputAssociation>;
    /// Get value of `inputSet` child
    fn input_set(&self) -> &Option<InputSet>;
    /// Get value of `eventDefinition` child
    fn event_definitions(&self) -> &Vec<EventDefinition>;
    /// Get value of `eventDefinitionRef` child
    fn event_definition_refs(&self) -> &Vec<EventDefinitionRef>;
}
dyn_clone::clone_trait_object!(ThrowEventType);
impl_downcast!(ThrowEventType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:timerEventDefinition")]
pub struct TimerEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:timeDate")]
    #[tia("TimerEventDefinitionType",rg*="time_date")]
    pub time_date: Option<Expression>,
    #[xml(child = "bpmn:timeDuration")]
    #[tia("TimerEventDefinitionType",rg*="time_duration")]
    pub time_duration: Option<Expression>,
    #[xml(child = "bpmn:timeCycle")]
    #[tia("TimerEventDefinitionType",rg*="time_cycle")]
    pub time_cycle: Option<Expression>,
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
        None
    }
}
// Traits
impl EventDefinitionType for TimerEventDefinition {}
impl RootElementType for TimerEventDefinition {}
//

/// Schema for `timerEventDefinition`
pub trait TimerEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of `timeDate` child
    fn time_date(&self) -> &Option<Expression>;
    /// Get value of `timeDuration` child
    fn time_duration(&self) -> &Option<Expression>;
    /// Get value of `timeCycle` child
    fn time_cycle(&self) -> &Option<Expression>;
}
dyn_clone::clone_trait_object!(TimerEventDefinitionType);
impl_downcast!(TimerEventDefinitionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:transaction")]
pub struct Transaction {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "triggeredByEvent")]
    #[tia("SubProcessType",rg*="triggered_byevent")]
    pub triggered_byevent: Option<bool>,
    #[xml(child = "bpmn:laneSet")]
    #[tia("SubProcessType",rg*="lane_sets")]
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
    #[tia("SubProcessType",rg*="flow_elements")]
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("SubProcessType",rg*="artifacts")]
    pub artifacts: Vec<Artifact>,
    #[xml(attr = "method")]
    #[tia("TransactionType",rg*="method")]
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

        None
    }
}
// Traits

//

/// Schema for `transaction`
pub trait TransactionType: SubProcessType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `method`
    fn method(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(TransactionType);
impl_downcast!(TransactionType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[xml(tag = "bpmn:userTask")]
pub struct UserTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id")]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs")]
    pub category_value_refs: Vec<CategoryValueRef>,
    #[xml(child = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings")]
    pub incomings: Vec<Incoming>,
    #[xml(child = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings")]
    pub outgoings: Vec<Outgoing>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation")]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity")]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity")]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default")]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    #[tia("UserTaskType",rg*="implementation")]
    pub implementation: Option<String>,
    #[xml(child = "bpmn:rendering")]
    #[tia("UserTaskType",rg*="renderings")]
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
        None
    }
}
// Traits
impl TaskType for UserTask {}
//

/// Schema for `userTask`
pub trait UserTaskType: TaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `implementation`
    fn implementation(&self) -> &Option<String>;
    /// Get value of `rendering` child
    fn renderings(&self) -> &Vec<Rendering>;
}
dyn_clone::clone_trait_object!(UserTaskType);
impl_downcast!(UserTaskType);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for SupportedInterfaceRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for EventDefinitionRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for ParticipantRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for MessageFlowRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for ChoreographyRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for CorrelationPropertyRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for SourceRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for TargetRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for CategoryValueRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for Incoming {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for Outgoing {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for DataInputRefs {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for OptionalInputRefs {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for WhileExecutingInputRefs {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for OutputSetRefs {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for FlowNodeRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for Source {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for Target {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for OperationRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for LoopDataInputRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for LoopDataOutputRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for InMessageRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for OutMessageRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for ErrorRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for DataOutputRefs {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for OptionalOutputRefs {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for WhileExecutingOutputRefs {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for InputSetRefs {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for InterfaceRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for EndPointRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for InnerParticipantRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for OuterParticipantRef {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for Supports {}
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(AsRef, Hash, Default, Clone, XmlRead, PartialEq, Debug)]
#[as_ref(forward)]
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
impl DocumentElementContainer for ResourceRef {}
