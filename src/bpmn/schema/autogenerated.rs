// This file is generated from BPMN 2.0 schema using `codegen.sh` script
use super::*;
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
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
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:definitions")]
pub struct Definitions {
    #[xml(attr = "id")]
    #[tia("DefinitionsType",rg*="id","DefinitionsTypeMut",s)]
    pub id: Option<Id>,
    #[xml(attr = "name")]
    #[tia("DefinitionsType",rg*="name","DefinitionsTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "targetNamespace")]
    #[tia("DefinitionsType",rg*="target_namespace","DefinitionsTypeMut",s)]
    pub target_namespace: URI,
    #[xml(attr = "expressionLanguage")]
    #[tia("DefinitionsType",rg*="expression_language","DefinitionsTypeMut",s)]
    pub expression_language: Option<URI>,
    #[xml(attr = "typeLanguage")]
    #[tia("DefinitionsType",rg*="type_language","DefinitionsTypeMut",s)]
    pub type_language: Option<URI>,
    #[xml(attr = "exporter")]
    #[tia("DefinitionsType",rg*="exporter","DefinitionsTypeMut",s)]
    pub exporter: Option<String>,
    #[xml(attr = "exporterVersion")]
    #[tia("DefinitionsType",rg*="exporter_version","DefinitionsTypeMut",s)]
    pub exporter_version: Option<String>,
    #[xml(child = "bpmn:import")]
    #[tia("DefinitionsType",rg*="imports","DefinitionsTypeMut",s,rmg*="imports_mut")]
    pub imports: Vec<Import>,
    #[xml(child = "bpmn:extension")]
    #[tia("DefinitionsType",rg*="extensions","DefinitionsTypeMut",s,rmg*="extensions_mut")]
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
    #[tia("DefinitionsType",rg*="root_elements","DefinitionsTypeMut",s,rmg*="root_elements_mut")]
    pub root_elements: Vec<RootElement>,
    #[xml(child = "bpmn:relationship")]
    #[tia("DefinitionsType",rg*="relationships","DefinitionsTypeMut",s,rmg*="relationships_mut")]
    pub relationships: Vec<Relationship>,
}
#[cast_to]
impl DocumentElement for Definitions {
    fn element(&self) -> Element {
        Element::Definitions
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Definitions {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.imports.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.extensions.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.root_elements.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.relationships.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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

/// Access to `definitions`
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
/// Mutable access to `definitions`
pub trait DefinitionsTypeMut: Downcast + Debug + Send + DynClone + DefinitionsType {
    /// Set value of attribute `id`
    fn set_id(&mut self, value: Option<Id>);
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `targetNamespace`
    fn set_target_namespace(&mut self, value: URI);
    /// Set value of attribute `expressionLanguage`
    fn set_expression_language(&mut self, value: Option<URI>);
    /// Set value of attribute `typeLanguage`
    fn set_type_language(&mut self, value: Option<URI>);
    /// Set value of attribute `exporter`
    fn set_exporter(&mut self, value: Option<String>);
    /// Set value of attribute `exporterVersion`
    fn set_exporter_version(&mut self, value: Option<String>);
    /// Get a mutable value of `import` child
    fn imports_mut(&mut self) -> &mut Vec<Import>;
    /// Set value of `import` child
    fn set_imports(&mut self, value: Vec<Import>);
    /// Get a mutable value of `extension` child
    fn extensions_mut(&mut self) -> &mut Vec<Extension>;
    /// Set value of `extension` child
    fn set_extensions(&mut self, value: Vec<Extension>);
    /// Get a mutable value of `rootElement` child
    fn root_elements_mut(&mut self) -> &mut Vec<RootElement>;
    /// Set value of `rootElement` child
    fn set_root_elements(&mut self, value: Vec<RootElement>);
    /// Get a mutable value of `relationship` child
    fn relationships_mut(&mut self) -> &mut Vec<Relationship>;
    /// Set value of `relationship` child
    fn set_relationships(&mut self, value: Vec<Relationship>);
}
dyn_clone::clone_trait_object!(DefinitionsTypeMut);
impl_downcast!(DefinitionsTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:import")]
pub struct Import {
    #[xml(attr = "namespace")]
    #[tia("ImportType",rg*="namespace","ImportTypeMut",s)]
    pub namespace: URI,
    #[xml(attr = "location")]
    #[tia("ImportType",rg*="location","ImportTypeMut",s)]
    pub location: String,
    #[xml(attr = "importType")]
    #[tia("ImportType",rg*="import_type","ImportTypeMut",s)]
    pub import_type: URI,
}
#[cast_to]
impl DocumentElement for Import {
    fn element(&self) -> Element {
        Element::Import
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Import {}
// Traits

//

/// Access to `import`
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
/// Mutable access to `import`
pub trait ImportTypeMut: Downcast + Debug + Send + DynClone + ImportType {
    /// Set value of attribute `namespace`
    fn set_namespace(&mut self, value: URI);
    /// Set value of attribute `location`
    fn set_location(&mut self, value: String);
    /// Set value of attribute `importType`
    fn set_import_type(&mut self, value: URI);
}
dyn_clone::clone_trait_object!(ImportTypeMut);
impl_downcast!(ImportTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:activity")]
#[serde(tag = "type")]
pub enum Activity {}
impl Activity {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {}
    }
}
#[cast_to]
impl DocumentElementContainer for Activity {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            _ => None,
        }
    }

    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
#[cast_to]
impl DocumentElement for Activity {
    fn element(&self) -> Element {
        Element::Activity
    }
}
/// Access to `activity`
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
/// Mutable access to `activity`
pub trait ActivityTypeMut:
    FlowNodeTypeMut + Downcast + Debug + Send + DynClone + ActivityType
{
    /// Set value of attribute `isForCompensation`
    fn set_is_for_compensation(&mut self, value: Option<bool>);
    /// Set value of attribute `startQuantity`
    fn set_start_quantity(&mut self, value: Option<Integer>);
    /// Set value of attribute `completionQuantity`
    fn set_completion_quantity(&mut self, value: Option<Integer>);
    /// Set value of attribute `default`
    fn set_default(&mut self, value: Option<String>);
    /// Get a mutable value of `ioSpecification` child
    fn io_specification_mut(&mut self) -> &mut Option<InputOutputSpecification>;
    /// Set value of `ioSpecification` child
    fn set_io_specification(&mut self, value: Option<InputOutputSpecification>);
    /// Get a mutable value of `property` child
    fn properies_mut(&mut self) -> &mut Vec<Property>;
    /// Set value of `property` child
    fn set_properies(&mut self, value: Vec<Property>);
    /// Get a mutable value of `dataInputAssociation` child
    fn data_input_associations_mut(&mut self) -> &mut Vec<DataInputAssociation>;
    /// Set value of `dataInputAssociation` child
    fn set_data_input_associations(&mut self, value: Vec<DataInputAssociation>);
    /// Get a mutable value of `dataOutputAssociation` child
    fn data_output_associations_mut(&mut self) -> &mut Vec<DataOutputAssociation>;
    /// Set value of `dataOutputAssociation` child
    fn set_data_output_associations(&mut self, value: Vec<DataOutputAssociation>);
    /// Get a mutable value of `resourceRole` child
    fn resource_roles_mut(&mut self) -> &mut Vec<ResourceRole>;
    /// Set value of `resourceRole` child
    fn set_resource_roles(&mut self, value: Vec<ResourceRole>);
    /// Get a mutable value of `loopCharacteristics` child
    fn loop_characteristics_mut(&mut self) -> &mut Option<LoopCharacteristics>;
    /// Set value of `loopCharacteristics` child
    fn set_loop_characteristics(&mut self, value: Option<LoopCharacteristics>);
}
dyn_clone::clone_trait_object!(ActivityTypeMut);
impl_downcast!(ActivityTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:adHocSubProcess")]
pub struct AdHocSubProcess {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation","ActivityTypeMut",s)]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity","ActivityTypeMut",s)]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity","ActivityTypeMut",s)]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default","ActivityTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification","ActivityTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies","ActivityTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations","ActivityTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations","ActivityTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles","ActivityTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics","ActivityTypeMut",s,rmg*="loop_characteristics_mut")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "triggeredByEvent")]
    #[tia("SubProcessType",rg*="triggered_byevent","SubProcessTypeMut",s)]
    pub triggered_byevent: Option<bool>,
    #[xml(child = "bpmn:laneSet")]
    #[tia("SubProcessType",rg*="lane_sets","SubProcessTypeMut",s,rmg*="lane_sets_mut")]
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
    #[tia("SubProcessType",rg*="flow_elements","SubProcessTypeMut",s,rmg*="flow_elements_mut")]
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("SubProcessType",rg*="artifacts","SubProcessTypeMut",s,rmg*="artifacts_mut")]
    pub artifacts: Vec<Artifact>,
    #[xml(attr = "cancelRemainingInstances")]
    #[tia("AdHocSubProcessType",rg*="cancel_remaining_instances","AdHocSubProcessTypeMut",s)]
    pub cancel_remaining_instances: Option<bool>,
    #[xml(attr = "ordering")]
    #[tia("AdHocSubProcessType",rg*="ordering","AdHocSubProcessTypeMut",s)]
    pub ordering: Option<String>,
    #[xml(child = "bpmn:completionCondition")]
    #[tia("AdHocSubProcessType",rg*="completion_condition","AdHocSubProcessTypeMut",s,rmg*="completion_condition_mut")]
    pub completion_condition: Option<Expression>,
}
#[cast_to]
impl DocumentElement for AdHocSubProcess {
    fn element(&self) -> Element {
        Element::AdHocSubProcess
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for AdHocSubProcess {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.completion_condition.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {AdHocSubProcess => SubProcessType,SubProcessTypeMut}
castable_to! {AdHocSubProcess => ActivityType,ActivityTypeMut}
castable_to! {AdHocSubProcess => FlowNodeType,FlowNodeTypeMut}
castable_to! {AdHocSubProcess => FlowElementType,FlowElementTypeMut}
castable_to! {AdHocSubProcess => BaseElementType,BaseElementTypeMut}
//

/// Access to `adHocSubProcess`
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
/// Mutable access to `adHocSubProcess`
pub trait AdHocSubProcessTypeMut:
    SubProcessTypeMut + Downcast + Debug + Send + DynClone + AdHocSubProcessType
{
    /// Set value of attribute `cancelRemainingInstances`
    fn set_cancel_remaining_instances(&mut self, value: Option<bool>);
    /// Set value of attribute `ordering`
    fn set_ordering(&mut self, value: Option<String>);
    /// Get a mutable value of `completionCondition` child
    fn completion_condition_mut(&mut self) -> &mut Option<Expression>;
    /// Set value of `completionCondition` child
    fn set_completion_condition(&mut self, value: Option<Expression>);
}
dyn_clone::clone_trait_object!(AdHocSubProcessTypeMut);
impl_downcast!(AdHocSubProcessTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:artifact")]
#[serde(tag = "type")]
pub enum Artifact {
    #[xml(tag = "bpmn:association")]
    Association(Association),
    #[xml(tag = "bpmn:group")]
    Group(Group),
    #[xml(tag = "bpmn:textAnnotation")]
    TextAnnotation(TextAnnotation),
}
impl From<Association> for Artifact {
    fn from(element: Association) -> Self {
        Self::Association(element)
    }
}
impl From<Group> for Artifact {
    fn from(element: Group) -> Self {
        Self::Group(element)
    }
}
impl From<TextAnnotation> for Artifact {
    fn from(element: TextAnnotation) -> Self {
        Self::TextAnnotation(element)
    }
}
impl Artifact {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {
            Artifact::Association(e) => Box::new(e) as Box<dyn DocumentElement>,
            Artifact::Group(e) => Box::new(e) as Box<dyn DocumentElement>,
            Artifact::TextAnnotation(e) => Box::new(e) as Box<dyn DocumentElement>,
        }
    }
}
#[cast_to]
impl DocumentElementContainer for Artifact {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            Artifact::Association(e) => e.find_by_id_mut(id),
            Artifact::Group(e) => e.find_by_id_mut(id),
            Artifact::TextAnnotation(e) => e.find_by_id_mut(id),

            _ => None,
        }
    }

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
#[cast_to]
impl DocumentElement for Artifact {
    fn element(&self) -> Element {
        Element::Artifact
    }
}
/// Access to `artifact`
pub trait ArtifactType: BaseElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ArtifactType);
impl_downcast!(ArtifactType);
/// Mutable access to `artifact`
pub trait ArtifactTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + ArtifactType
{
}
dyn_clone::clone_trait_object!(ArtifactTypeMut);
impl_downcast!(ArtifactTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:assignment")]
pub struct Assignment {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:from")]
    #[tia("AssignmentType",rg*="from","AssignmentTypeMut",s,rmg*="from_mut")]
    pub from: Expression,
    #[xml(child = "bpmn:to")]
    #[tia("AssignmentType",rg*="to","AssignmentTypeMut",s,rmg*="to_mut")]
    pub to: Expression,
}
#[cast_to]
impl DocumentElement for Assignment {
    fn element(&self) -> Element {
        Element::Assignment
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Assignment {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.from.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.to.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {Assignment => BaseElementType,BaseElementTypeMut}
//

/// Access to `assignment`
pub trait AssignmentType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of `from` child
    fn from(&self) -> &Expression;
    /// Get value of `to` child
    fn to(&self) -> &Expression;
}
dyn_clone::clone_trait_object!(AssignmentType);
impl_downcast!(AssignmentType);
/// Mutable access to `assignment`
pub trait AssignmentTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + AssignmentType
{
    /// Get a mutable value of `from` child
    fn from_mut(&mut self) -> &mut Expression;
    /// Set value of `from` child
    fn set_from(&mut self, value: Expression);
    /// Get a mutable value of `to` child
    fn to_mut(&mut self) -> &mut Expression;
    /// Set value of `to` child
    fn set_to(&mut self, value: Expression);
}
dyn_clone::clone_trait_object!(AssignmentTypeMut);
impl_downcast!(AssignmentTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:association")]
pub struct Association {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "sourceRef")]
    #[tia("AssociationType",rg*="source_ref","AssociationTypeMut",s)]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    #[tia("AssociationType",rg*="target_ref","AssociationTypeMut",s)]
    pub target_ref: String,
    #[xml(attr = "associationDirection")]
    #[tia("AssociationType",rg*="association_direction","AssociationTypeMut",s)]
    pub association_direction: Option<String>,
}
#[cast_to]
impl DocumentElement for Association {
    fn element(&self) -> Element {
        Element::Association
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Association {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl ArtifactType for Association {}
#[cast_to]
impl ArtifactTypeMut for Association {}
castable_to! {Association => PartialEq<Association> }
castable_to! {Association => ArtifactType,ArtifactTypeMut}
castable_to! {Association => BaseElementType,BaseElementTypeMut}
//

/// Access to `association`
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
/// Mutable access to `association`
pub trait AssociationTypeMut:
    ArtifactTypeMut + Downcast + Debug + Send + DynClone + AssociationType
{
    /// Set value of attribute `sourceRef`
    fn set_source_ref(&mut self, value: String);
    /// Set value of attribute `targetRef`
    fn set_target_ref(&mut self, value: String);
    /// Set value of attribute `associationDirection`
    fn set_association_direction(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(AssociationTypeMut);
impl_downcast!(AssociationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:auditing")]
pub struct Auditing {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
}
#[cast_to]
impl DocumentElement for Auditing {
    fn element(&self) -> Element {
        Element::Auditing
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Auditing {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {Auditing => BaseElementType,BaseElementTypeMut}
//

/// Access to `auditing`
pub trait AuditingType: BaseElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(AuditingType);
impl_downcast!(AuditingType);
/// Mutable access to `auditing`
pub trait AuditingTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + AuditingType
{
}
dyn_clone::clone_trait_object!(AuditingTypeMut);
impl_downcast!(AuditingTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:baseElement")]
#[serde(tag = "type")]
pub enum BaseElement {}
impl BaseElement {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {}
    }
}
#[cast_to]
impl DocumentElementContainer for BaseElement {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            _ => None,
        }
    }

    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
#[cast_to]
impl DocumentElement for BaseElement {
    fn element(&self) -> Element {
        Element::BaseElement
    }
}
/// Access to `baseElement`
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
/// Mutable access to `baseElement`
pub trait BaseElementTypeMut: Downcast + Debug + Send + DynClone + BaseElementType {
    /// Set value of attribute `id`
    fn set_id(&mut self, value: Option<Id>);
    /// Get a mutable value of `documentation` child
    fn documentations_mut(&mut self) -> &mut Vec<Documentation>;
    /// Set value of `documentation` child
    fn set_documentations(&mut self, value: Vec<Documentation>);
    /// Get a mutable value of `extensionElements` child
    fn extension_elements_mut(&mut self) -> &mut Option<ExtensionElements>;
    /// Set value of `extensionElements` child
    fn set_extension_elements(&mut self, value: Option<ExtensionElements>);
}
dyn_clone::clone_trait_object!(BaseElementTypeMut);
impl_downcast!(BaseElementTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:baseElementWithMixedContent")]
#[serde(tag = "type")]
pub enum BaseElementWithMixedContent {}
impl BaseElementWithMixedContent {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {}
    }
}
#[cast_to]
impl DocumentElementContainer for BaseElementWithMixedContent {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            _ => None,
        }
    }

    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
#[cast_to]
impl DocumentElement for BaseElementWithMixedContent {
    fn element(&self) -> Element {
        Element::BaseElementWithMixedContent
    }
}
/// Access to `baseElementWithMixedContent`
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
/// Mutable access to `baseElementWithMixedContent`
pub trait BaseElementWithMixedContentTypeMut:
    Downcast + Debug + Send + DynClone + BaseElementWithMixedContentType
{
    /// Set value of attribute `id`
    fn set_id(&mut self, value: Option<Id>);
    /// Get a mutable value of `documentation` child
    fn documentations_mut(&mut self) -> &mut Vec<Documentation>;
    /// Set value of `documentation` child
    fn set_documentations(&mut self, value: Vec<Documentation>);
    /// Get a mutable value of `extensionElements` child
    fn extension_elements_mut(&mut self) -> &mut Option<ExtensionElements>;
    /// Set value of `extensionElements` child
    fn set_extension_elements(&mut self, value: Option<ExtensionElements>);
}
dyn_clone::clone_trait_object!(BaseElementWithMixedContentTypeMut);
impl_downcast!(BaseElementWithMixedContentTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:boundaryEvent")]
pub struct BoundaryEvent {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(child = "bpmn:property")]
    #[tia("EventType",rg*="properies","EventTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(attr = "parallelMultiple")]
    #[tia("CatchEventType",rg*="parallel_multiple","CatchEventTypeMut",s)]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    #[tia("CatchEventType",rg*="data_outputs","CatchEventTypeMut",s,rmg*="data_outputs_mut")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("CatchEventType",rg*="data_output_associations","CatchEventTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:outputSet")]
    #[tia("CatchEventType",rg*="output_set","CatchEventTypeMut",s,rmg*="output_set_mut")]
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
    #[tia("CatchEventType",rg*="event_definitions","CatchEventTypeMut",s,rmg*="event_definitions_mut")]
    pub event_definitions: Vec<EventDefinition>,
    #[xml(flatten_text = "bpmn:eventDefinitionRef")]
    #[tia("CatchEventType",rg*="event_definition_refs","CatchEventTypeMut",s,rmg*="event_definition_refs_mut")]
    pub event_definition_refs: Vec<String>,
    #[xml(attr = "cancelActivity")]
    #[tia("BoundaryEventType",rg*="cancel_activity","BoundaryEventTypeMut",s)]
    pub cancel_activity: Option<bool>,
    #[xml(attr = "attachedToRef")]
    #[tia("BoundaryEventType",rg*="attached_toref","BoundaryEventTypeMut",s)]
    pub attached_toref: String,
}
#[cast_to]
impl DocumentElement for BoundaryEvent {
    fn element(&self) -> Element {
        Element::BoundaryEvent
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for BoundaryEvent {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {BoundaryEvent => CatchEventType,CatchEventTypeMut}
castable_to! {BoundaryEvent => EventType,EventTypeMut}
castable_to! {BoundaryEvent => FlowNodeType,FlowNodeTypeMut}
castable_to! {BoundaryEvent => FlowElementType,FlowElementTypeMut}
castable_to! {BoundaryEvent => BaseElementType,BaseElementTypeMut}
//

/// Access to `boundaryEvent`
pub trait BoundaryEventType: CatchEventType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `cancelActivity`
    fn cancel_activity(&self) -> &Option<bool>;
    /// Get value of attribute `attachedToRef`
    fn attached_toref(&self) -> &String;
}
dyn_clone::clone_trait_object!(BoundaryEventType);
impl_downcast!(BoundaryEventType);
/// Mutable access to `boundaryEvent`
pub trait BoundaryEventTypeMut:
    CatchEventTypeMut + Downcast + Debug + Send + DynClone + BoundaryEventType
{
    /// Set value of attribute `cancelActivity`
    fn set_cancel_activity(&mut self, value: Option<bool>);
    /// Set value of attribute `attachedToRef`
    fn set_attached_toref(&mut self, value: String);
}
dyn_clone::clone_trait_object!(BoundaryEventTypeMut);
impl_downcast!(BoundaryEventTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:businessRuleTask")]
pub struct BusinessRuleTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation","ActivityTypeMut",s)]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity","ActivityTypeMut",s)]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity","ActivityTypeMut",s)]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default","ActivityTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification","ActivityTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies","ActivityTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations","ActivityTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations","ActivityTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles","ActivityTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics","ActivityTypeMut",s,rmg*="loop_characteristics_mut")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    #[tia("BusinessRuleTaskType",rg*="implementation","BusinessRuleTaskTypeMut",s)]
    pub implementation: Option<String>,
}
#[cast_to]
impl DocumentElement for BusinessRuleTask {
    fn element(&self) -> Element {
        Element::BusinessRuleTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for BusinessRuleTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl TaskType for BusinessRuleTask {}
#[cast_to]
impl TaskTypeMut for BusinessRuleTask {}
castable_to! {BusinessRuleTask => PartialEq<BusinessRuleTask> }
castable_to! {BusinessRuleTask => TaskType,TaskTypeMut}
castable_to! {BusinessRuleTask => ActivityType,ActivityTypeMut}
castable_to! {BusinessRuleTask => FlowNodeType,FlowNodeTypeMut}
castable_to! {BusinessRuleTask => FlowElementType,FlowElementTypeMut}
castable_to! {BusinessRuleTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `businessRuleTask`
pub trait BusinessRuleTaskType: TaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `implementation`
    fn implementation(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(BusinessRuleTaskType);
impl_downcast!(BusinessRuleTaskType);
/// Mutable access to `businessRuleTask`
pub trait BusinessRuleTaskTypeMut:
    TaskTypeMut + Downcast + Debug + Send + DynClone + BusinessRuleTaskType
{
    /// Set value of attribute `implementation`
    fn set_implementation(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(BusinessRuleTaskTypeMut);
impl_downcast!(BusinessRuleTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:callableElement")]
pub struct CallableElement {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name","CallableElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs","CallableElementTypeMut",s,rmg*="supported_interface_refs_mut")]
    pub supported_interface_refs: Vec<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification","CallableElementTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings","CallableElementTypeMut",s,rmg*="io_bindings_mut")]
    pub io_bindings: Vec<InputOutputBinding>,
}
#[cast_to]
impl DocumentElement for CallableElement {
    fn element(&self) -> Element {
        Element::CallableElement
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for CallableElement {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.io_specification.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.io_bindings.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
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
#[cast_to]
impl RootElementType for CallableElement {}
#[cast_to]
impl RootElementTypeMut for CallableElement {}
castable_to! {CallableElement => PartialEq<CallableElement> }
castable_to! {CallableElement => RootElementType,RootElementTypeMut}
castable_to! {CallableElement => BaseElementType,BaseElementTypeMut}
//

/// Access to `callableElement`
pub trait CallableElementType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `supportedInterfaceRef` child
    fn supported_interface_refs(&self) -> &Vec<String>;
    /// Get value of `ioSpecification` child
    fn io_specification(&self) -> &Option<InputOutputSpecification>;
    /// Get value of `ioBinding` child
    fn io_bindings(&self) -> &Vec<InputOutputBinding>;
}
dyn_clone::clone_trait_object!(CallableElementType);
impl_downcast!(CallableElementType);
/// Mutable access to `callableElement`
pub trait CallableElementTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + CallableElementType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Get a mutable value of `supportedInterfaceRef` child
    fn supported_interface_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `supportedInterfaceRef` child
    fn set_supported_interface_refs(&mut self, value: Vec<String>);
    /// Get a mutable value of `ioSpecification` child
    fn io_specification_mut(&mut self) -> &mut Option<InputOutputSpecification>;
    /// Set value of `ioSpecification` child
    fn set_io_specification(&mut self, value: Option<InputOutputSpecification>);
    /// Get a mutable value of `ioBinding` child
    fn io_bindings_mut(&mut self) -> &mut Vec<InputOutputBinding>;
    /// Set value of `ioBinding` child
    fn set_io_bindings(&mut self, value: Vec<InputOutputBinding>);
}
dyn_clone::clone_trait_object!(CallableElementTypeMut);
impl_downcast!(CallableElementTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:callActivity")]
pub struct CallActivity {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation","ActivityTypeMut",s)]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity","ActivityTypeMut",s)]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity","ActivityTypeMut",s)]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default","ActivityTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification","ActivityTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies","ActivityTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations","ActivityTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations","ActivityTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles","ActivityTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics","ActivityTypeMut",s,rmg*="loop_characteristics_mut")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "calledElement")]
    #[tia("CallActivityType",rg*="called_element","CallActivityTypeMut",s)]
    pub called_element: Option<String>,
}
#[cast_to]
impl DocumentElement for CallActivity {
    fn element(&self) -> Element {
        Element::CallActivity
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for CallActivity {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {CallActivity => ActivityType,ActivityTypeMut}
castable_to! {CallActivity => FlowNodeType,FlowNodeTypeMut}
castable_to! {CallActivity => FlowElementType,FlowElementTypeMut}
castable_to! {CallActivity => BaseElementType,BaseElementTypeMut}
//

/// Access to `callActivity`
pub trait CallActivityType: ActivityType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `calledElement`
    fn called_element(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(CallActivityType);
impl_downcast!(CallActivityType);
/// Mutable access to `callActivity`
pub trait CallActivityTypeMut:
    ActivityTypeMut + Downcast + Debug + Send + DynClone + CallActivityType
{
    /// Set value of attribute `calledElement`
    fn set_called_element(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(CallActivityTypeMut);
impl_downcast!(CallActivityTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:callChoreography")]
pub struct CallChoreography {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "initiatingParticipantRef")]
    #[tia("ChoreographyActivityType",rg*="initiating_participant_ref","ChoreographyActivityTypeMut",s)]
    pub initiating_participant_ref: String,
    #[xml(attr = "loopType")]
    #[tia("ChoreographyActivityType",rg*="loop_type","ChoreographyActivityTypeMut",s)]
    pub loop_type: Option<String>,
    #[xml(flatten_text = "bpmn:participantRef")]
    #[tia("ChoreographyActivityType",rg*="participant_refs","ChoreographyActivityTypeMut",s,rmg*="participant_refs_mut")]
    pub participant_refs: Vec<String>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("ChoreographyActivityType",rg*="correlation_keys","ChoreographyActivityTypeMut",s,rmg*="correlation_keys_mut")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(attr = "calledChoreographyRef")]
    #[tia("CallChoreographyType",rg*="called_choreography_ref","CallChoreographyTypeMut",s)]
    pub called_choreography_ref: Option<String>,
    #[xml(child = "bpmn:participantAssociation")]
    #[tia("CallChoreographyType",rg*="participant_associations","CallChoreographyTypeMut",s,rmg*="participant_associations_mut")]
    pub participant_associations: Vec<ParticipantAssociation>,
}
#[cast_to]
impl DocumentElement for CallChoreography {
    fn element(&self) -> Element {
        Element::CallChoreography
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for CallChoreography {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.participant_associations.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {CallChoreography => ChoreographyActivityType,ChoreographyActivityTypeMut}
castable_to! {CallChoreography => FlowNodeType,FlowNodeTypeMut}
castable_to! {CallChoreography => FlowElementType,FlowElementTypeMut}
castable_to! {CallChoreography => BaseElementType,BaseElementTypeMut}
//

/// Access to `callChoreography`
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
/// Mutable access to `callChoreography`
pub trait CallChoreographyTypeMut:
    ChoreographyActivityTypeMut + Downcast + Debug + Send + DynClone + CallChoreographyType
{
    /// Set value of attribute `calledChoreographyRef`
    fn set_called_choreography_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `participantAssociation` child
    fn participant_associations_mut(&mut self) -> &mut Vec<ParticipantAssociation>;
    /// Set value of `participantAssociation` child
    fn set_participant_associations(&mut self, value: Vec<ParticipantAssociation>);
}
dyn_clone::clone_trait_object!(CallChoreographyTypeMut);
impl_downcast!(CallChoreographyTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:callConversation")]
pub struct CallConversation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ConversationNodeType",rg*="name","ConversationNodeTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:participantRef")]
    #[tia("ConversationNodeType",rg*="participant_refs","ConversationNodeTypeMut",s,rmg*="participant_refs_mut")]
    pub participant_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:messageFlowRef")]
    #[tia("ConversationNodeType",rg*="message_flow_refs","ConversationNodeTypeMut",s,rmg*="message_flow_refs_mut")]
    pub message_flow_refs: Vec<String>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("ConversationNodeType",rg*="correlation_keys","ConversationNodeTypeMut",s,rmg*="correlation_keys_mut")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(attr = "calledCollaborationRef")]
    #[tia("CallConversationType",rg*="called_collaboration_ref","CallConversationTypeMut",s)]
    pub called_collaboration_ref: Option<String>,
    #[xml(child = "bpmn:participantAssociation")]
    #[tia("CallConversationType",rg*="participant_associations","CallConversationTypeMut",s,rmg*="participant_associations_mut")]
    pub participant_associations: Vec<ParticipantAssociation>,
}
#[cast_to]
impl DocumentElement for CallConversation {
    fn element(&self) -> Element {
        Element::CallConversation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for CallConversation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.participant_associations.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {CallConversation => ConversationNodeType,ConversationNodeTypeMut}
castable_to! {CallConversation => BaseElementType,BaseElementTypeMut}
//

/// Access to `callConversation`
pub trait CallConversationType: ConversationNodeType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `calledCollaborationRef`
    fn called_collaboration_ref(&self) -> &Option<String>;
    /// Get value of `participantAssociation` child
    fn participant_associations(&self) -> &Vec<ParticipantAssociation>;
}
dyn_clone::clone_trait_object!(CallConversationType);
impl_downcast!(CallConversationType);
/// Mutable access to `callConversation`
pub trait CallConversationTypeMut:
    ConversationNodeTypeMut + Downcast + Debug + Send + DynClone + CallConversationType
{
    /// Set value of attribute `calledCollaborationRef`
    fn set_called_collaboration_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `participantAssociation` child
    fn participant_associations_mut(&mut self) -> &mut Vec<ParticipantAssociation>;
    /// Set value of `participantAssociation` child
    fn set_participant_associations(&mut self, value: Vec<ParticipantAssociation>);
}
dyn_clone::clone_trait_object!(CallConversationTypeMut);
impl_downcast!(CallConversationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:cancelEventDefinition")]
pub struct CancelEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
}
#[cast_to]
impl DocumentElement for CancelEventDefinition {
    fn element(&self) -> Element {
        Element::CancelEventDefinition
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for CancelEventDefinition {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl EventDefinitionType for CancelEventDefinition {}
#[cast_to]
impl EventDefinitionTypeMut for CancelEventDefinition {}
castable_to! {CancelEventDefinition => PartialEq<CancelEventDefinition> }
castable_to! {CancelEventDefinition => EventDefinitionType,EventDefinitionTypeMut}
#[cast_to]
impl RootElementType for CancelEventDefinition {}
#[cast_to]
impl RootElementTypeMut for CancelEventDefinition {}
castable_to! {CancelEventDefinition => PartialEq<CancelEventDefinition> }
castable_to! {CancelEventDefinition => RootElementType,RootElementTypeMut}
castable_to! {CancelEventDefinition => BaseElementType,BaseElementTypeMut}
//

/// Access to `cancelEventDefinition`
pub trait CancelEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
}
dyn_clone::clone_trait_object!(CancelEventDefinitionType);
impl_downcast!(CancelEventDefinitionType);
/// Mutable access to `cancelEventDefinition`
pub trait CancelEventDefinitionTypeMut:
    EventDefinitionTypeMut + Downcast + Debug + Send + DynClone + CancelEventDefinitionType
{
}
dyn_clone::clone_trait_object!(CancelEventDefinitionTypeMut);
impl_downcast!(CancelEventDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:catchEvent")]
#[serde(tag = "type")]
pub enum CatchEvent {}
impl CatchEvent {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {}
    }
}
#[cast_to]
impl DocumentElementContainer for CatchEvent {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            _ => None,
        }
    }

    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
#[cast_to]
impl DocumentElement for CatchEvent {
    fn element(&self) -> Element {
        Element::CatchEvent
    }
}
/// Access to `catchEvent`
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
    fn event_definition_refs(&self) -> &Vec<String>;
}
dyn_clone::clone_trait_object!(CatchEventType);
impl_downcast!(CatchEventType);
/// Mutable access to `catchEvent`
pub trait CatchEventTypeMut:
    EventTypeMut + Downcast + Debug + Send + DynClone + CatchEventType
{
    /// Set value of attribute `parallelMultiple`
    fn set_parallel_multiple(&mut self, value: Option<bool>);
    /// Get a mutable value of `dataOutput` child
    fn data_outputs_mut(&mut self) -> &mut Vec<DataOutput>;
    /// Set value of `dataOutput` child
    fn set_data_outputs(&mut self, value: Vec<DataOutput>);
    /// Get a mutable value of `dataOutputAssociation` child
    fn data_output_associations_mut(&mut self) -> &mut Vec<DataOutputAssociation>;
    /// Set value of `dataOutputAssociation` child
    fn set_data_output_associations(&mut self, value: Vec<DataOutputAssociation>);
    /// Get a mutable value of `outputSet` child
    fn output_set_mut(&mut self) -> &mut Option<OutputSet>;
    /// Set value of `outputSet` child
    fn set_output_set(&mut self, value: Option<OutputSet>);
    /// Get a mutable value of `eventDefinition` child
    fn event_definitions_mut(&mut self) -> &mut Vec<EventDefinition>;
    /// Set value of `eventDefinition` child
    fn set_event_definitions(&mut self, value: Vec<EventDefinition>);
    /// Get a mutable value of `eventDefinitionRef` child
    fn event_definition_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `eventDefinitionRef` child
    fn set_event_definition_refs(&mut self, value: Vec<String>);
}
dyn_clone::clone_trait_object!(CatchEventTypeMut);
impl_downcast!(CatchEventTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:category")]
pub struct Category {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CategoryType",rg*="name","CategoryTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:categoryValue")]
    #[tia("CategoryType",rg*="category_values","CategoryTypeMut",s,rmg*="category_values_mut")]
    pub category_values: Vec<CategoryValue>,
}
#[cast_to]
impl DocumentElement for Category {
    fn element(&self) -> Element {
        Element::Category
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Category {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.category_values.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
#[cast_to]
impl RootElementType for Category {}
#[cast_to]
impl RootElementTypeMut for Category {}
castable_to! {Category => PartialEq<Category> }
castable_to! {Category => RootElementType,RootElementTypeMut}
castable_to! {Category => BaseElementType,BaseElementTypeMut}
//

/// Access to `category`
pub trait CategoryType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `categoryValue` child
    fn category_values(&self) -> &Vec<CategoryValue>;
}
dyn_clone::clone_trait_object!(CategoryType);
impl_downcast!(CategoryType);
/// Mutable access to `category`
pub trait CategoryTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + CategoryType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Get a mutable value of `categoryValue` child
    fn category_values_mut(&mut self) -> &mut Vec<CategoryValue>;
    /// Set value of `categoryValue` child
    fn set_category_values(&mut self, value: Vec<CategoryValue>);
}
dyn_clone::clone_trait_object!(CategoryTypeMut);
impl_downcast!(CategoryTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:categoryValue")]
pub struct CategoryValue {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "value")]
    #[tia("CategoryValueType",rg*="value","CategoryValueTypeMut",s)]
    pub value: Option<String>,
}
#[cast_to]
impl DocumentElement for CategoryValue {
    fn element(&self) -> Element {
        Element::CategoryValue
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for CategoryValue {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {CategoryValue => BaseElementType,BaseElementTypeMut}
//

/// Access to `categoryValue`
pub trait CategoryValueType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `value`
    fn value(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(CategoryValueType);
impl_downcast!(CategoryValueType);
/// Mutable access to `categoryValue`
pub trait CategoryValueTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + CategoryValueType
{
    /// Set value of attribute `value`
    fn set_value(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(CategoryValueTypeMut);
impl_downcast!(CategoryValueTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:choreography")]
pub struct Choreography {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CollaborationType",rg*="name","CollaborationTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "isClosed")]
    #[tia("CollaborationType",rg*="is_closed","CollaborationTypeMut",s)]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    #[tia("CollaborationType",rg*="participants","CollaborationTypeMut",s,rmg*="participants_mut")]
    pub participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    #[tia("CollaborationType",rg*="message_flows","CollaborationTypeMut",s,rmg*="message_flows_mut")]
    pub message_flows: Vec<MessageFlow>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("CollaborationType",rg*="artifacts","CollaborationTypeMut",s,rmg*="artifacts_mut")]
    pub artifacts: Vec<Artifact>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    #[tia("CollaborationType",rg*="conversation_nodes","CollaborationTypeMut",s,rmg*="conversation_nodes_mut")]
    pub conversation_nodes: Vec<ConversationNode>,
    #[xml(child = "bpmn:conversationAssociation")]
    #[tia("CollaborationType",rg*="conversation_associations","CollaborationTypeMut",s,rmg*="conversation_associations_mut")]
    pub conversation_associations: Vec<ConversationAssociation>,
    #[xml(child = "bpmn:participantAssociation")]
    #[tia("CollaborationType",rg*="participant_associations","CollaborationTypeMut",s,rmg*="participant_associations_mut")]
    pub participant_associations: Vec<ParticipantAssociation>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    #[tia("CollaborationType",rg*="message_flow_associations","CollaborationTypeMut",s,rmg*="message_flow_associations_mut")]
    pub message_flow_associations: Vec<MessageFlowAssociation>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("CollaborationType",rg*="correlation_keys","CollaborationTypeMut",s,rmg*="correlation_keys_mut")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(flatten_text = "bpmn:choreographyRef")]
    #[tia("CollaborationType",rg*="choreography_refs","CollaborationTypeMut",s,rmg*="choreography_refs_mut")]
    pub choreography_refs: Vec<String>,
    #[xml(child = "bpmn:conversationLink")]
    #[tia("CollaborationType",rg*="conversation_links","CollaborationTypeMut",s,rmg*="conversation_links_mut")]
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
    #[tia("ChoreographyType",rg*="flow_elements","ChoreographyTypeMut",s,rmg*="flow_elements_mut")]
    pub flow_elements: Vec<FlowElement>,
}
#[cast_to]
impl DocumentElement for Choreography {
    fn element(&self) -> Element {
        Element::Choreography
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Choreography {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.flow_elements.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {Choreography => CollaborationType,CollaborationTypeMut}
#[cast_to]
impl RootElementType for Choreography {}
#[cast_to]
impl RootElementTypeMut for Choreography {}
castable_to! {Choreography => PartialEq<Choreography> }
castable_to! {Choreography => RootElementType,RootElementTypeMut}
castable_to! {Choreography => BaseElementType,BaseElementTypeMut}
//

/// Access to `choreography`
pub trait ChoreographyType: CollaborationType + Downcast + Debug + Send + DynClone {
    /// Get value of `flowElement` child
    fn flow_elements(&self) -> &Vec<FlowElement>;
}
dyn_clone::clone_trait_object!(ChoreographyType);
impl_downcast!(ChoreographyType);
/// Mutable access to `choreography`
pub trait ChoreographyTypeMut:
    CollaborationTypeMut + Downcast + Debug + Send + DynClone + ChoreographyType
{
    /// Get a mutable value of `flowElement` child
    fn flow_elements_mut(&mut self) -> &mut Vec<FlowElement>;
    /// Set value of `flowElement` child
    fn set_flow_elements(&mut self, value: Vec<FlowElement>);
}
dyn_clone::clone_trait_object!(ChoreographyTypeMut);
impl_downcast!(ChoreographyTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:choreographyActivity")]
#[serde(tag = "type")]
pub enum ChoreographyActivity {}
impl ChoreographyActivity {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {}
    }
}
#[cast_to]
impl DocumentElementContainer for ChoreographyActivity {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            _ => None,
        }
    }

    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
#[cast_to]
impl DocumentElement for ChoreographyActivity {
    fn element(&self) -> Element {
        Element::ChoreographyActivity
    }
}
/// Access to `choreographyActivity`
pub trait ChoreographyActivityType: FlowNodeType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `initiatingParticipantRef`
    fn initiating_participant_ref(&self) -> &String;
    /// Get value of attribute `loopType`
    fn loop_type(&self) -> &Option<String>;
    /// Get value of `participantRef` child
    fn participant_refs(&self) -> &Vec<String>;
    /// Get value of `correlationKey` child
    fn correlation_keys(&self) -> &Vec<CorrelationKey>;
}
dyn_clone::clone_trait_object!(ChoreographyActivityType);
impl_downcast!(ChoreographyActivityType);
/// Mutable access to `choreographyActivity`
pub trait ChoreographyActivityTypeMut:
    FlowNodeTypeMut + Downcast + Debug + Send + DynClone + ChoreographyActivityType
{
    /// Set value of attribute `initiatingParticipantRef`
    fn set_initiating_participant_ref(&mut self, value: String);
    /// Set value of attribute `loopType`
    fn set_loop_type(&mut self, value: Option<String>);
    /// Get a mutable value of `participantRef` child
    fn participant_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `participantRef` child
    fn set_participant_refs(&mut self, value: Vec<String>);
    /// Get a mutable value of `correlationKey` child
    fn correlation_keys_mut(&mut self) -> &mut Vec<CorrelationKey>;
    /// Set value of `correlationKey` child
    fn set_correlation_keys(&mut self, value: Vec<CorrelationKey>);
}
dyn_clone::clone_trait_object!(ChoreographyActivityTypeMut);
impl_downcast!(ChoreographyActivityTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:choreographyTask")]
pub struct ChoreographyTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "initiatingParticipantRef")]
    #[tia("ChoreographyActivityType",rg*="initiating_participant_ref","ChoreographyActivityTypeMut",s)]
    pub initiating_participant_ref: String,
    #[xml(attr = "loopType")]
    #[tia("ChoreographyActivityType",rg*="loop_type","ChoreographyActivityTypeMut",s)]
    pub loop_type: Option<String>,
    #[xml(flatten_text = "bpmn:participantRef")]
    #[tia("ChoreographyActivityType",rg*="participant_refs","ChoreographyActivityTypeMut",s,rmg*="participant_refs_mut")]
    pub participant_refs: Vec<String>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("ChoreographyActivityType",rg*="correlation_keys","ChoreographyActivityTypeMut",s,rmg*="correlation_keys_mut")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(flatten_text = "bpmn:messageFlowRef")]
    #[tia("ChoreographyTaskType",rg*="message_flow_ref","ChoreographyTaskTypeMut",s,rmg*="message_flow_ref_mut")]
    pub message_flow_ref: String,
}
#[cast_to]
impl DocumentElement for ChoreographyTask {
    fn element(&self) -> Element {
        Element::ChoreographyTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ChoreographyTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {ChoreographyTask => ChoreographyActivityType,ChoreographyActivityTypeMut}
castable_to! {ChoreographyTask => FlowNodeType,FlowNodeTypeMut}
castable_to! {ChoreographyTask => FlowElementType,FlowElementTypeMut}
castable_to! {ChoreographyTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `choreographyTask`
pub trait ChoreographyTaskType:
    ChoreographyActivityType + Downcast + Debug + Send + DynClone
{
    /// Get value of `messageFlowRef` child
    fn message_flow_ref(&self) -> &String;
}
dyn_clone::clone_trait_object!(ChoreographyTaskType);
impl_downcast!(ChoreographyTaskType);
/// Mutable access to `choreographyTask`
pub trait ChoreographyTaskTypeMut:
    ChoreographyActivityTypeMut + Downcast + Debug + Send + DynClone + ChoreographyTaskType
{
    /// Get a mutable value of `messageFlowRef` child
    fn message_flow_ref_mut(&mut self) -> &mut String;
    /// Set value of `messageFlowRef` child
    fn set_message_flow_ref(&mut self, value: String);
}
dyn_clone::clone_trait_object!(ChoreographyTaskTypeMut);
impl_downcast!(ChoreographyTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:collaboration")]
pub struct Collaboration {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CollaborationType",rg*="name","CollaborationTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "isClosed")]
    #[tia("CollaborationType",rg*="is_closed","CollaborationTypeMut",s)]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    #[tia("CollaborationType",rg*="participants","CollaborationTypeMut",s,rmg*="participants_mut")]
    pub participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    #[tia("CollaborationType",rg*="message_flows","CollaborationTypeMut",s,rmg*="message_flows_mut")]
    pub message_flows: Vec<MessageFlow>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("CollaborationType",rg*="artifacts","CollaborationTypeMut",s,rmg*="artifacts_mut")]
    pub artifacts: Vec<Artifact>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    #[tia("CollaborationType",rg*="conversation_nodes","CollaborationTypeMut",s,rmg*="conversation_nodes_mut")]
    pub conversation_nodes: Vec<ConversationNode>,
    #[xml(child = "bpmn:conversationAssociation")]
    #[tia("CollaborationType",rg*="conversation_associations","CollaborationTypeMut",s,rmg*="conversation_associations_mut")]
    pub conversation_associations: Vec<ConversationAssociation>,
    #[xml(child = "bpmn:participantAssociation")]
    #[tia("CollaborationType",rg*="participant_associations","CollaborationTypeMut",s,rmg*="participant_associations_mut")]
    pub participant_associations: Vec<ParticipantAssociation>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    #[tia("CollaborationType",rg*="message_flow_associations","CollaborationTypeMut",s,rmg*="message_flow_associations_mut")]
    pub message_flow_associations: Vec<MessageFlowAssociation>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("CollaborationType",rg*="correlation_keys","CollaborationTypeMut",s,rmg*="correlation_keys_mut")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(flatten_text = "bpmn:choreographyRef")]
    #[tia("CollaborationType",rg*="choreography_refs","CollaborationTypeMut",s,rmg*="choreography_refs_mut")]
    pub choreography_refs: Vec<String>,
    #[xml(child = "bpmn:conversationLink")]
    #[tia("CollaborationType",rg*="conversation_links","CollaborationTypeMut",s,rmg*="conversation_links_mut")]
    pub conversation_links: Vec<ConversationLink>,
}
#[cast_to]
impl DocumentElement for Collaboration {
    fn element(&self) -> Element {
        Element::Collaboration
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Collaboration {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.participants.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.message_flows.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.artifacts.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.conversation_nodes.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.conversation_associations.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.participant_associations.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.message_flow_associations.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.correlation_keys.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.conversation_links.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
        if let Some(e) = self.conversation_links.find_by_id(id) {
            return Some(e);
        }
        None
    }
}
// Traits
#[cast_to]
impl RootElementType for Collaboration {}
#[cast_to]
impl RootElementTypeMut for Collaboration {}
castable_to! {Collaboration => PartialEq<Collaboration> }
castable_to! {Collaboration => RootElementType,RootElementTypeMut}
castable_to! {Collaboration => BaseElementType,BaseElementTypeMut}
//

/// Access to `collaboration`
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
    fn choreography_refs(&self) -> &Vec<String>;
    /// Get value of `conversationLink` child
    fn conversation_links(&self) -> &Vec<ConversationLink>;
}
dyn_clone::clone_trait_object!(CollaborationType);
impl_downcast!(CollaborationType);
/// Mutable access to `collaboration`
pub trait CollaborationTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + CollaborationType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `isClosed`
    fn set_is_closed(&mut self, value: Option<bool>);
    /// Get a mutable value of `participant` child
    fn participants_mut(&mut self) -> &mut Vec<Participant>;
    /// Set value of `participant` child
    fn set_participants(&mut self, value: Vec<Participant>);
    /// Get a mutable value of `messageFlow` child
    fn message_flows_mut(&mut self) -> &mut Vec<MessageFlow>;
    /// Set value of `messageFlow` child
    fn set_message_flows(&mut self, value: Vec<MessageFlow>);
    /// Get a mutable value of `artifact` child
    fn artifacts_mut(&mut self) -> &mut Vec<Artifact>;
    /// Set value of `artifact` child
    fn set_artifacts(&mut self, value: Vec<Artifact>);
    /// Get a mutable value of `conversationNode` child
    fn conversation_nodes_mut(&mut self) -> &mut Vec<ConversationNode>;
    /// Set value of `conversationNode` child
    fn set_conversation_nodes(&mut self, value: Vec<ConversationNode>);
    /// Get a mutable value of `conversationAssociation` child
    fn conversation_associations_mut(&mut self) -> &mut Vec<ConversationAssociation>;
    /// Set value of `conversationAssociation` child
    fn set_conversation_associations(&mut self, value: Vec<ConversationAssociation>);
    /// Get a mutable value of `participantAssociation` child
    fn participant_associations_mut(&mut self) -> &mut Vec<ParticipantAssociation>;
    /// Set value of `participantAssociation` child
    fn set_participant_associations(&mut self, value: Vec<ParticipantAssociation>);
    /// Get a mutable value of `messageFlowAssociation` child
    fn message_flow_associations_mut(&mut self) -> &mut Vec<MessageFlowAssociation>;
    /// Set value of `messageFlowAssociation` child
    fn set_message_flow_associations(&mut self, value: Vec<MessageFlowAssociation>);
    /// Get a mutable value of `correlationKey` child
    fn correlation_keys_mut(&mut self) -> &mut Vec<CorrelationKey>;
    /// Set value of `correlationKey` child
    fn set_correlation_keys(&mut self, value: Vec<CorrelationKey>);
    /// Get a mutable value of `choreographyRef` child
    fn choreography_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `choreographyRef` child
    fn set_choreography_refs(&mut self, value: Vec<String>);
    /// Get a mutable value of `conversationLink` child
    fn conversation_links_mut(&mut self) -> &mut Vec<ConversationLink>;
    /// Set value of `conversationLink` child
    fn set_conversation_links(&mut self, value: Vec<ConversationLink>);
}
dyn_clone::clone_trait_object!(CollaborationTypeMut);
impl_downcast!(CollaborationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:compensateEventDefinition")]
pub struct CompensateEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "waitForCompletion")]
    #[tia("CompensateEventDefinitionType",rg*="wait_for_completion","CompensateEventDefinitionTypeMut",s)]
    pub wait_for_completion: Option<bool>,
    #[xml(attr = "activityRef")]
    #[tia("CompensateEventDefinitionType",rg*="activity_ref","CompensateEventDefinitionTypeMut",s)]
    pub activity_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for CompensateEventDefinition {
    fn element(&self) -> Element {
        Element::CompensateEventDefinition
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for CompensateEventDefinition {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl EventDefinitionType for CompensateEventDefinition {}
#[cast_to]
impl EventDefinitionTypeMut for CompensateEventDefinition {}
castable_to! {CompensateEventDefinition => PartialEq<CompensateEventDefinition> }
castable_to! {CompensateEventDefinition => EventDefinitionType,EventDefinitionTypeMut}
#[cast_to]
impl RootElementType for CompensateEventDefinition {}
#[cast_to]
impl RootElementTypeMut for CompensateEventDefinition {}
castable_to! {CompensateEventDefinition => PartialEq<CompensateEventDefinition> }
castable_to! {CompensateEventDefinition => RootElementType,RootElementTypeMut}
castable_to! {CompensateEventDefinition => BaseElementType,BaseElementTypeMut}
//

/// Access to `compensateEventDefinition`
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
/// Mutable access to `compensateEventDefinition`
pub trait CompensateEventDefinitionTypeMut:
    EventDefinitionTypeMut + Downcast + Debug + Send + DynClone + CompensateEventDefinitionType
{
    /// Set value of attribute `waitForCompletion`
    fn set_wait_for_completion(&mut self, value: Option<bool>);
    /// Set value of attribute `activityRef`
    fn set_activity_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(CompensateEventDefinitionTypeMut);
impl_downcast!(CompensateEventDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:complexBehaviorDefinition")]
pub struct ComplexBehaviorDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:condition")]
    #[tia("ComplexBehaviorDefinitionType",rg*="condition","ComplexBehaviorDefinitionTypeMut",s,rmg*="condition_mut")]
    pub condition: FormalExpression,
    #[xml(child = "bpmn:event")]
    #[tia("ComplexBehaviorDefinitionType",rg*="event","ComplexBehaviorDefinitionTypeMut",s,rmg*="event_mut")]
    pub event: Option<ImplicitThrowEvent>,
}
#[cast_to]
impl DocumentElement for ComplexBehaviorDefinition {
    fn element(&self) -> Element {
        Element::ComplexBehaviorDefinition
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ComplexBehaviorDefinition {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.condition.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.event.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {ComplexBehaviorDefinition => BaseElementType,BaseElementTypeMut}
//

/// Access to `complexBehaviorDefinition`
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
/// Mutable access to `complexBehaviorDefinition`
pub trait ComplexBehaviorDefinitionTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + ComplexBehaviorDefinitionType
{
    /// Get a mutable value of `condition` child
    fn condition_mut(&mut self) -> &mut FormalExpression;
    /// Set value of `condition` child
    fn set_condition(&mut self, value: FormalExpression);
    /// Get a mutable value of `event` child
    fn event_mut(&mut self) -> &mut Option<ImplicitThrowEvent>;
    /// Set value of `event` child
    fn set_event(&mut self, value: Option<ImplicitThrowEvent>);
}
dyn_clone::clone_trait_object!(ComplexBehaviorDefinitionTypeMut);
impl_downcast!(ComplexBehaviorDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:complexGateway")]
pub struct ComplexGateway {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "gatewayDirection")]
    #[tia("GatewayType",rg*="gateway_direction","GatewayTypeMut",s)]
    pub gateway_direction: Option<String>,
    #[xml(attr = "default")]
    #[tia("ComplexGatewayType",rg*="default","ComplexGatewayTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:activationCondition")]
    #[tia("ComplexGatewayType",rg*="activation_condition","ComplexGatewayTypeMut",s,rmg*="activation_condition_mut")]
    pub activation_condition: Option<Expression>,
}
#[cast_to]
impl DocumentElement for ComplexGateway {
    fn element(&self) -> Element {
        Element::ComplexGateway
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ComplexGateway {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.activation_condition.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {ComplexGateway => GatewayType,GatewayTypeMut}
castable_to! {ComplexGateway => FlowNodeType,FlowNodeTypeMut}
castable_to! {ComplexGateway => FlowElementType,FlowElementTypeMut}
castable_to! {ComplexGateway => BaseElementType,BaseElementTypeMut}
//

/// Access to `complexGateway`
pub trait ComplexGatewayType: GatewayType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `default`
    fn default(&self) -> &Option<String>;
    /// Get value of `activationCondition` child
    fn activation_condition(&self) -> &Option<Expression>;
}
dyn_clone::clone_trait_object!(ComplexGatewayType);
impl_downcast!(ComplexGatewayType);
/// Mutable access to `complexGateway`
pub trait ComplexGatewayTypeMut:
    GatewayTypeMut + Downcast + Debug + Send + DynClone + ComplexGatewayType
{
    /// Set value of attribute `default`
    fn set_default(&mut self, value: Option<String>);
    /// Get a mutable value of `activationCondition` child
    fn activation_condition_mut(&mut self) -> &mut Option<Expression>;
    /// Set value of `activationCondition` child
    fn set_activation_condition(&mut self, value: Option<Expression>);
}
dyn_clone::clone_trait_object!(ComplexGatewayTypeMut);
impl_downcast!(ComplexGatewayTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:conditionalEventDefinition")]
pub struct ConditionalEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:condition")]
    #[tia("ConditionalEventDefinitionType",rg*="condition","ConditionalEventDefinitionTypeMut",s,rmg*="condition_mut")]
    pub condition: Expression,
}
#[cast_to]
impl DocumentElement for ConditionalEventDefinition {
    fn element(&self) -> Element {
        Element::ConditionalEventDefinition
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ConditionalEventDefinition {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.condition.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
#[cast_to]
impl EventDefinitionType for ConditionalEventDefinition {}
#[cast_to]
impl EventDefinitionTypeMut for ConditionalEventDefinition {}
castable_to! {ConditionalEventDefinition => PartialEq<ConditionalEventDefinition> }
castable_to! {ConditionalEventDefinition => EventDefinitionType,EventDefinitionTypeMut}
#[cast_to]
impl RootElementType for ConditionalEventDefinition {}
#[cast_to]
impl RootElementTypeMut for ConditionalEventDefinition {}
castable_to! {ConditionalEventDefinition => PartialEq<ConditionalEventDefinition> }
castable_to! {ConditionalEventDefinition => RootElementType,RootElementTypeMut}
castable_to! {ConditionalEventDefinition => BaseElementType,BaseElementTypeMut}
//

/// Access to `conditionalEventDefinition`
pub trait ConditionalEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of `condition` child
    fn condition(&self) -> &Expression;
}
dyn_clone::clone_trait_object!(ConditionalEventDefinitionType);
impl_downcast!(ConditionalEventDefinitionType);
/// Mutable access to `conditionalEventDefinition`
pub trait ConditionalEventDefinitionTypeMut:
    EventDefinitionTypeMut + Downcast + Debug + Send + DynClone + ConditionalEventDefinitionType
{
    /// Get a mutable value of `condition` child
    fn condition_mut(&mut self) -> &mut Expression;
    /// Set value of `condition` child
    fn set_condition(&mut self, value: Expression);
}
dyn_clone::clone_trait_object!(ConditionalEventDefinitionTypeMut);
impl_downcast!(ConditionalEventDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:conversation")]
pub struct Conversation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ConversationNodeType",rg*="name","ConversationNodeTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:participantRef")]
    #[tia("ConversationNodeType",rg*="participant_refs","ConversationNodeTypeMut",s,rmg*="participant_refs_mut")]
    pub participant_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:messageFlowRef")]
    #[tia("ConversationNodeType",rg*="message_flow_refs","ConversationNodeTypeMut",s,rmg*="message_flow_refs_mut")]
    pub message_flow_refs: Vec<String>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("ConversationNodeType",rg*="correlation_keys","ConversationNodeTypeMut",s,rmg*="correlation_keys_mut")]
    pub correlation_keys: Vec<CorrelationKey>,
}
#[cast_to]
impl DocumentElement for Conversation {
    fn element(&self) -> Element {
        Element::Conversation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Conversation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {Conversation => ConversationNodeType,ConversationNodeTypeMut}
castable_to! {Conversation => BaseElementType,BaseElementTypeMut}
//

/// Access to `conversation`
pub trait ConversationType: ConversationNodeType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ConversationType);
impl_downcast!(ConversationType);
/// Mutable access to `conversation`
pub trait ConversationTypeMut:
    ConversationNodeTypeMut + Downcast + Debug + Send + DynClone + ConversationType
{
}
dyn_clone::clone_trait_object!(ConversationTypeMut);
impl_downcast!(ConversationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:conversationAssociation")]
pub struct ConversationAssociation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "innerConversationNodeRef")]
    #[tia("ConversationAssociationType",rg*="inner_conversation_node_ref","ConversationAssociationTypeMut",s)]
    pub inner_conversation_node_ref: String,
    #[xml(attr = "outerConversationNodeRef")]
    #[tia("ConversationAssociationType",rg*="outer_conversation_node_ref","ConversationAssociationTypeMut",s)]
    pub outer_conversation_node_ref: String,
}
#[cast_to]
impl DocumentElement for ConversationAssociation {
    fn element(&self) -> Element {
        Element::ConversationAssociation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ConversationAssociation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {ConversationAssociation => BaseElementType,BaseElementTypeMut}
//

/// Access to `conversationAssociation`
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
/// Mutable access to `conversationAssociation`
pub trait ConversationAssociationTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + ConversationAssociationType
{
    /// Set value of attribute `innerConversationNodeRef`
    fn set_inner_conversation_node_ref(&mut self, value: String);
    /// Set value of attribute `outerConversationNodeRef`
    fn set_outer_conversation_node_ref(&mut self, value: String);
}
dyn_clone::clone_trait_object!(ConversationAssociationTypeMut);
impl_downcast!(ConversationAssociationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:conversationLink")]
pub struct ConversationLink {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ConversationLinkType",rg*="name","ConversationLinkTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "sourceRef")]
    #[tia("ConversationLinkType",rg*="source_ref","ConversationLinkTypeMut",s)]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    #[tia("ConversationLinkType",rg*="target_ref","ConversationLinkTypeMut",s)]
    pub target_ref: String,
}
#[cast_to]
impl DocumentElement for ConversationLink {
    fn element(&self) -> Element {
        Element::ConversationLink
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ConversationLink {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {ConversationLink => BaseElementType,BaseElementTypeMut}
//

/// Access to `conversationLink`
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
/// Mutable access to `conversationLink`
pub trait ConversationLinkTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + ConversationLinkType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `sourceRef`
    fn set_source_ref(&mut self, value: String);
    /// Set value of attribute `targetRef`
    fn set_target_ref(&mut self, value: String);
}
dyn_clone::clone_trait_object!(ConversationLinkTypeMut);
impl_downcast!(ConversationLinkTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:conversationNode")]
#[serde(tag = "type")]
pub enum ConversationNode {
    #[xml(tag = "bpmn:callConversation")]
    CallConversation(CallConversation),
    #[xml(tag = "bpmn:conversation")]
    Conversation(Conversation),
    #[xml(tag = "bpmn:subConversation")]
    SubConversation(SubConversation),
}
impl From<CallConversation> for ConversationNode {
    fn from(element: CallConversation) -> Self {
        Self::CallConversation(element)
    }
}
impl From<Conversation> for ConversationNode {
    fn from(element: Conversation) -> Self {
        Self::Conversation(element)
    }
}
impl From<SubConversation> for ConversationNode {
    fn from(element: SubConversation) -> Self {
        Self::SubConversation(element)
    }
}
impl ConversationNode {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {
            ConversationNode::CallConversation(e) => Box::new(e) as Box<dyn DocumentElement>,
            ConversationNode::Conversation(e) => Box::new(e) as Box<dyn DocumentElement>,
            ConversationNode::SubConversation(e) => Box::new(e) as Box<dyn DocumentElement>,
        }
    }
}
#[cast_to]
impl DocumentElementContainer for ConversationNode {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            ConversationNode::CallConversation(e) => e.find_by_id_mut(id),
            ConversationNode::Conversation(e) => e.find_by_id_mut(id),
            ConversationNode::SubConversation(e) => e.find_by_id_mut(id),

            _ => None,
        }
    }

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
#[cast_to]
impl DocumentElement for ConversationNode {
    fn element(&self) -> Element {
        Element::ConversationNode
    }
}
/// Access to `conversationNode`
pub trait ConversationNodeType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `participantRef` child
    fn participant_refs(&self) -> &Vec<String>;
    /// Get value of `messageFlowRef` child
    fn message_flow_refs(&self) -> &Vec<String>;
    /// Get value of `correlationKey` child
    fn correlation_keys(&self) -> &Vec<CorrelationKey>;
}
dyn_clone::clone_trait_object!(ConversationNodeType);
impl_downcast!(ConversationNodeType);
/// Mutable access to `conversationNode`
pub trait ConversationNodeTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + ConversationNodeType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Get a mutable value of `participantRef` child
    fn participant_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `participantRef` child
    fn set_participant_refs(&mut self, value: Vec<String>);
    /// Get a mutable value of `messageFlowRef` child
    fn message_flow_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `messageFlowRef` child
    fn set_message_flow_refs(&mut self, value: Vec<String>);
    /// Get a mutable value of `correlationKey` child
    fn correlation_keys_mut(&mut self) -> &mut Vec<CorrelationKey>;
    /// Set value of `correlationKey` child
    fn set_correlation_keys(&mut self, value: Vec<CorrelationKey>);
}
dyn_clone::clone_trait_object!(ConversationNodeTypeMut);
impl_downcast!(ConversationNodeTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:correlationKey")]
pub struct CorrelationKey {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CorrelationKeyType",rg*="name","CorrelationKeyTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:correlationPropertyRef")]
    #[tia("CorrelationKeyType",rg*="correlation_property_refs","CorrelationKeyTypeMut",s,rmg*="correlation_property_refs_mut")]
    pub correlation_property_refs: Vec<String>,
}
#[cast_to]
impl DocumentElement for CorrelationKey {
    fn element(&self) -> Element {
        Element::CorrelationKey
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for CorrelationKey {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {CorrelationKey => BaseElementType,BaseElementTypeMut}
//

/// Access to `correlationKey`
pub trait CorrelationKeyType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `correlationPropertyRef` child
    fn correlation_property_refs(&self) -> &Vec<String>;
}
dyn_clone::clone_trait_object!(CorrelationKeyType);
impl_downcast!(CorrelationKeyType);
/// Mutable access to `correlationKey`
pub trait CorrelationKeyTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + CorrelationKeyType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Get a mutable value of `correlationPropertyRef` child
    fn correlation_property_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `correlationPropertyRef` child
    fn set_correlation_property_refs(&mut self, value: Vec<String>);
}
dyn_clone::clone_trait_object!(CorrelationKeyTypeMut);
impl_downcast!(CorrelationKeyTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:correlationProperty")]
pub struct CorrelationProperty {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CorrelationPropertyType",rg*="name","CorrelationPropertyTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "type")]
    #[tia("CorrelationPropertyType",rg*="typ","CorrelationPropertyTypeMut",s)]
    pub typ: Option<String>,
    #[xml(child = "bpmn:correlationPropertyRetrievalExpression")]
    #[tia("CorrelationPropertyType",rg*="correlation_property_retrieval_expressions","CorrelationPropertyTypeMut",s,rmg*="correlation_property_retrieval_expressions_mut")]
    pub correlation_property_retrieval_expressions: Vec<CorrelationPropertyRetrievalExpression>,
}
#[cast_to]
impl DocumentElement for CorrelationProperty {
    fn element(&self) -> Element {
        Element::CorrelationProperty
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for CorrelationProperty {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self
            .correlation_property_retrieval_expressions
            .find_by_id_mut(id)
        {
            return Some(e);
        }
        None
    }

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
#[cast_to]
impl RootElementType for CorrelationProperty {}
#[cast_to]
impl RootElementTypeMut for CorrelationProperty {}
castable_to! {CorrelationProperty => PartialEq<CorrelationProperty> }
castable_to! {CorrelationProperty => RootElementType,RootElementTypeMut}
castable_to! {CorrelationProperty => BaseElementType,BaseElementTypeMut}
//

/// Access to `correlationProperty`
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
/// Mutable access to `correlationProperty`
pub trait CorrelationPropertyTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + CorrelationPropertyType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `type`
    fn set_typ(&mut self, value: Option<String>);
    /// Get a mutable value of `correlationPropertyRetrievalExpression` child
    fn correlation_property_retrieval_expressions_mut(
        &mut self,
    ) -> &mut Vec<CorrelationPropertyRetrievalExpression>;
    /// Set value of `correlationPropertyRetrievalExpression` child
    fn set_correlation_property_retrieval_expressions(
        &mut self,
        value: Vec<CorrelationPropertyRetrievalExpression>,
    );
}
dyn_clone::clone_trait_object!(CorrelationPropertyTypeMut);
impl_downcast!(CorrelationPropertyTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:correlationPropertyBinding")]
pub struct CorrelationPropertyBinding {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "correlationPropertyRef")]
    #[tia("CorrelationPropertyBindingType",rg*="correlation_property_ref","CorrelationPropertyBindingTypeMut",s)]
    pub correlation_property_ref: String,
    #[xml(child = "bpmn:dataPath")]
    #[tia("CorrelationPropertyBindingType",rg*="data_path","CorrelationPropertyBindingTypeMut",s,rmg*="data_path_mut")]
    pub data_path: FormalExpression,
}
#[cast_to]
impl DocumentElement for CorrelationPropertyBinding {
    fn element(&self) -> Element {
        Element::CorrelationPropertyBinding
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for CorrelationPropertyBinding {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_path.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {CorrelationPropertyBinding => BaseElementType,BaseElementTypeMut}
//

/// Access to `correlationPropertyBinding`
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
/// Mutable access to `correlationPropertyBinding`
pub trait CorrelationPropertyBindingTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + CorrelationPropertyBindingType
{
    /// Set value of attribute `correlationPropertyRef`
    fn set_correlation_property_ref(&mut self, value: String);
    /// Get a mutable value of `dataPath` child
    fn data_path_mut(&mut self) -> &mut FormalExpression;
    /// Set value of `dataPath` child
    fn set_data_path(&mut self, value: FormalExpression);
}
dyn_clone::clone_trait_object!(CorrelationPropertyBindingTypeMut);
impl_downcast!(CorrelationPropertyBindingTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:correlationPropertyRetrievalExpression")]
pub struct CorrelationPropertyRetrievalExpression {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "messageRef")]
    #[tia("CorrelationPropertyRetrievalExpressionType",rg*="message_ref","CorrelationPropertyRetrievalExpressionTypeMut",s)]
    pub message_ref: String,
    #[xml(child = "bpmn:messagePath")]
    #[tia("CorrelationPropertyRetrievalExpressionType",rg*="message_path","CorrelationPropertyRetrievalExpressionTypeMut",s,rmg*="message_path_mut")]
    pub message_path: FormalExpression,
}
#[cast_to]
impl DocumentElement for CorrelationPropertyRetrievalExpression {
    fn element(&self) -> Element {
        Element::CorrelationPropertyRetrievalExpression
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for CorrelationPropertyRetrievalExpression {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.message_path.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {CorrelationPropertyRetrievalExpression => BaseElementType,BaseElementTypeMut}
//

/// Access to `correlationPropertyRetrievalExpression`
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
/// Mutable access to `correlationPropertyRetrievalExpression`
pub trait CorrelationPropertyRetrievalExpressionTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + CorrelationPropertyRetrievalExpressionType
{
    /// Set value of attribute `messageRef`
    fn set_message_ref(&mut self, value: String);
    /// Get a mutable value of `messagePath` child
    fn message_path_mut(&mut self) -> &mut FormalExpression;
    /// Set value of `messagePath` child
    fn set_message_path(&mut self, value: FormalExpression);
}
dyn_clone::clone_trait_object!(CorrelationPropertyRetrievalExpressionTypeMut);
impl_downcast!(CorrelationPropertyRetrievalExpressionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:correlationSubscription")]
pub struct CorrelationSubscription {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "correlationKeyRef")]
    #[tia("CorrelationSubscriptionType",rg*="correlation_key_ref","CorrelationSubscriptionTypeMut",s)]
    pub correlation_key_ref: String,
    #[xml(child = "bpmn:correlationPropertyBinding")]
    #[tia("CorrelationSubscriptionType",rg*="correlation_property_bindings","CorrelationSubscriptionTypeMut",s,rmg*="correlation_property_bindings_mut")]
    pub correlation_property_bindings: Vec<CorrelationPropertyBinding>,
}
#[cast_to]
impl DocumentElement for CorrelationSubscription {
    fn element(&self) -> Element {
        Element::CorrelationSubscription
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for CorrelationSubscription {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.correlation_property_bindings.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {CorrelationSubscription => BaseElementType,BaseElementTypeMut}
//

/// Access to `correlationSubscription`
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
/// Mutable access to `correlationSubscription`
pub trait CorrelationSubscriptionTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + CorrelationSubscriptionType
{
    /// Set value of attribute `correlationKeyRef`
    fn set_correlation_key_ref(&mut self, value: String);
    /// Get a mutable value of `correlationPropertyBinding` child
    fn correlation_property_bindings_mut(&mut self) -> &mut Vec<CorrelationPropertyBinding>;
    /// Set value of `correlationPropertyBinding` child
    fn set_correlation_property_bindings(&mut self, value: Vec<CorrelationPropertyBinding>);
}
dyn_clone::clone_trait_object!(CorrelationSubscriptionTypeMut);
impl_downcast!(CorrelationSubscriptionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:dataAssociation")]
pub struct DataAssociation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(flatten_text = "bpmn:sourceRef")]
    #[tia("DataAssociationType",rg*="source_refs","DataAssociationTypeMut",s,rmg*="source_refs_mut")]
    pub source_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:targetRef")]
    #[tia("DataAssociationType",rg*="target_ref","DataAssociationTypeMut",s,rmg*="target_ref_mut")]
    pub target_ref: String,
    #[xml(child = "bpmn:transformation")]
    #[tia("DataAssociationType",rg*="transformation","DataAssociationTypeMut",s,rmg*="transformation_mut")]
    pub transformation: Option<FormalExpression>,
    #[xml(child = "bpmn:assignment")]
    #[tia("DataAssociationType",rg*="assignments","DataAssociationTypeMut",s,rmg*="assignments_mut")]
    pub assignments: Vec<Assignment>,
}
#[cast_to]
impl DocumentElement for DataAssociation {
    fn element(&self) -> Element {
        Element::DataAssociation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for DataAssociation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.transformation.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.assignments.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
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
castable_to! {DataAssociation => BaseElementType,BaseElementTypeMut}
//

/// Access to `dataAssociation`
pub trait DataAssociationType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of `sourceRef` child
    fn source_refs(&self) -> &Vec<String>;
    /// Get value of `targetRef` child
    fn target_ref(&self) -> &String;
    /// Get value of `transformation` child
    fn transformation(&self) -> &Option<FormalExpression>;
    /// Get value of `assignment` child
    fn assignments(&self) -> &Vec<Assignment>;
}
dyn_clone::clone_trait_object!(DataAssociationType);
impl_downcast!(DataAssociationType);
/// Mutable access to `dataAssociation`
pub trait DataAssociationTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + DataAssociationType
{
    /// Get a mutable value of `sourceRef` child
    fn source_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `sourceRef` child
    fn set_source_refs(&mut self, value: Vec<String>);
    /// Get a mutable value of `targetRef` child
    fn target_ref_mut(&mut self) -> &mut String;
    /// Set value of `targetRef` child
    fn set_target_ref(&mut self, value: String);
    /// Get a mutable value of `transformation` child
    fn transformation_mut(&mut self) -> &mut Option<FormalExpression>;
    /// Set value of `transformation` child
    fn set_transformation(&mut self, value: Option<FormalExpression>);
    /// Get a mutable value of `assignment` child
    fn assignments_mut(&mut self) -> &mut Vec<Assignment>;
    /// Set value of `assignment` child
    fn set_assignments(&mut self, value: Vec<Assignment>);
}
dyn_clone::clone_trait_object!(DataAssociationTypeMut);
impl_downcast!(DataAssociationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:dataInput")]
pub struct DataInput {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("DataInputType",rg*="name","DataInputTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("DataInputType",rg*="item_subject_ref","DataInputTypeMut",s)]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "isCollection")]
    #[tia("DataInputType",rg*="is_collection","DataInputTypeMut",s)]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    #[tia("DataInputType",rg*="data_state","DataInputTypeMut",s,rmg*="data_state_mut")]
    pub data_state: Option<DataState>,
}
#[cast_to]
impl DocumentElement for DataInput {
    fn element(&self) -> Element {
        Element::DataInput
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for DataInput {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {DataInput => BaseElementType,BaseElementTypeMut}
//

/// Access to `dataInput`
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
/// Mutable access to `dataInput`
pub trait DataInputTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + DataInputType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `itemSubjectRef`
    fn set_item_subject_ref(&mut self, value: Option<String>);
    /// Set value of attribute `isCollection`
    fn set_is_collection(&mut self, value: Option<bool>);
    /// Get a mutable value of `dataState` child
    fn data_state_mut(&mut self) -> &mut Option<DataState>;
    /// Set value of `dataState` child
    fn set_data_state(&mut self, value: Option<DataState>);
}
dyn_clone::clone_trait_object!(DataInputTypeMut);
impl_downcast!(DataInputTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:dataInputAssociation")]
pub struct DataInputAssociation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(flatten_text = "bpmn:sourceRef")]
    #[tia("DataAssociationType",rg*="source_refs","DataAssociationTypeMut",s,rmg*="source_refs_mut")]
    pub source_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:targetRef")]
    #[tia("DataAssociationType",rg*="target_ref","DataAssociationTypeMut",s,rmg*="target_ref_mut")]
    pub target_ref: String,
    #[xml(child = "bpmn:transformation")]
    #[tia("DataAssociationType",rg*="transformation","DataAssociationTypeMut",s,rmg*="transformation_mut")]
    pub transformation: Option<FormalExpression>,
    #[xml(child = "bpmn:assignment")]
    #[tia("DataAssociationType",rg*="assignments","DataAssociationTypeMut",s,rmg*="assignments_mut")]
    pub assignments: Vec<Assignment>,
}
#[cast_to]
impl DocumentElement for DataInputAssociation {
    fn element(&self) -> Element {
        Element::DataInputAssociation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for DataInputAssociation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {DataInputAssociation => DataAssociationType,DataAssociationTypeMut}
castable_to! {DataInputAssociation => BaseElementType,BaseElementTypeMut}
//

/// Access to `dataInputAssociation`
pub trait DataInputAssociationType:
    DataAssociationType + Downcast + Debug + Send + DynClone
{
}
dyn_clone::clone_trait_object!(DataInputAssociationType);
impl_downcast!(DataInputAssociationType);
/// Mutable access to `dataInputAssociation`
pub trait DataInputAssociationTypeMut:
    DataAssociationTypeMut + Downcast + Debug + Send + DynClone + DataInputAssociationType
{
}
dyn_clone::clone_trait_object!(DataInputAssociationTypeMut);
impl_downcast!(DataInputAssociationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:dataObject")]
pub struct DataObject {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("DataObjectType",rg*="item_subject_ref","DataObjectTypeMut",s)]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "isCollection")]
    #[tia("DataObjectType",rg*="is_collection","DataObjectTypeMut",s)]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    #[tia("DataObjectType",rg*="data_state","DataObjectTypeMut",s,rmg*="data_state_mut")]
    pub data_state: Option<DataState>,
}
#[cast_to]
impl DocumentElement for DataObject {
    fn element(&self) -> Element {
        Element::DataObject
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for DataObject {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {DataObject => FlowElementType,FlowElementTypeMut}
castable_to! {DataObject => BaseElementType,BaseElementTypeMut}
//

/// Access to `dataObject`
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
/// Mutable access to `dataObject`
pub trait DataObjectTypeMut:
    FlowElementTypeMut + Downcast + Debug + Send + DynClone + DataObjectType
{
    /// Set value of attribute `itemSubjectRef`
    fn set_item_subject_ref(&mut self, value: Option<String>);
    /// Set value of attribute `isCollection`
    fn set_is_collection(&mut self, value: Option<bool>);
    /// Get a mutable value of `dataState` child
    fn data_state_mut(&mut self) -> &mut Option<DataState>;
    /// Set value of `dataState` child
    fn set_data_state(&mut self, value: Option<DataState>);
}
dyn_clone::clone_trait_object!(DataObjectTypeMut);
impl_downcast!(DataObjectTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:dataObjectReference")]
pub struct DataObjectReference {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("DataObjectReferenceType",rg*="item_subject_ref","DataObjectReferenceTypeMut",s)]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "dataObjectRef")]
    #[tia("DataObjectReferenceType",rg*="data_object_ref","DataObjectReferenceTypeMut",s)]
    pub data_object_ref: Option<String>,
    #[xml(child = "bpmn:dataState")]
    #[tia("DataObjectReferenceType",rg*="data_state","DataObjectReferenceTypeMut",s,rmg*="data_state_mut")]
    pub data_state: Option<DataState>,
}
#[cast_to]
impl DocumentElement for DataObjectReference {
    fn element(&self) -> Element {
        Element::DataObjectReference
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for DataObjectReference {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {DataObjectReference => FlowElementType,FlowElementTypeMut}
castable_to! {DataObjectReference => BaseElementType,BaseElementTypeMut}
//

/// Access to `dataObjectReference`
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
/// Mutable access to `dataObjectReference`
pub trait DataObjectReferenceTypeMut:
    FlowElementTypeMut + Downcast + Debug + Send + DynClone + DataObjectReferenceType
{
    /// Set value of attribute `itemSubjectRef`
    fn set_item_subject_ref(&mut self, value: Option<String>);
    /// Set value of attribute `dataObjectRef`
    fn set_data_object_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `dataState` child
    fn data_state_mut(&mut self) -> &mut Option<DataState>;
    /// Set value of `dataState` child
    fn set_data_state(&mut self, value: Option<DataState>);
}
dyn_clone::clone_trait_object!(DataObjectReferenceTypeMut);
impl_downcast!(DataObjectReferenceTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:dataOutput")]
pub struct DataOutput {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("DataOutputType",rg*="name","DataOutputTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("DataOutputType",rg*="item_subject_ref","DataOutputTypeMut",s)]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "isCollection")]
    #[tia("DataOutputType",rg*="is_collection","DataOutputTypeMut",s)]
    pub is_collection: Option<bool>,
    #[xml(child = "bpmn:dataState")]
    #[tia("DataOutputType",rg*="data_state","DataOutputTypeMut",s,rmg*="data_state_mut")]
    pub data_state: Option<DataState>,
}
#[cast_to]
impl DocumentElement for DataOutput {
    fn element(&self) -> Element {
        Element::DataOutput
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for DataOutput {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {DataOutput => BaseElementType,BaseElementTypeMut}
//

/// Access to `dataOutput`
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
/// Mutable access to `dataOutput`
pub trait DataOutputTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + DataOutputType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `itemSubjectRef`
    fn set_item_subject_ref(&mut self, value: Option<String>);
    /// Set value of attribute `isCollection`
    fn set_is_collection(&mut self, value: Option<bool>);
    /// Get a mutable value of `dataState` child
    fn data_state_mut(&mut self) -> &mut Option<DataState>;
    /// Set value of `dataState` child
    fn set_data_state(&mut self, value: Option<DataState>);
}
dyn_clone::clone_trait_object!(DataOutputTypeMut);
impl_downcast!(DataOutputTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:dataOutputAssociation")]
pub struct DataOutputAssociation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(flatten_text = "bpmn:sourceRef")]
    #[tia("DataAssociationType",rg*="source_refs","DataAssociationTypeMut",s,rmg*="source_refs_mut")]
    pub source_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:targetRef")]
    #[tia("DataAssociationType",rg*="target_ref","DataAssociationTypeMut",s,rmg*="target_ref_mut")]
    pub target_ref: String,
    #[xml(child = "bpmn:transformation")]
    #[tia("DataAssociationType",rg*="transformation","DataAssociationTypeMut",s,rmg*="transformation_mut")]
    pub transformation: Option<FormalExpression>,
    #[xml(child = "bpmn:assignment")]
    #[tia("DataAssociationType",rg*="assignments","DataAssociationTypeMut",s,rmg*="assignments_mut")]
    pub assignments: Vec<Assignment>,
}
#[cast_to]
impl DocumentElement for DataOutputAssociation {
    fn element(&self) -> Element {
        Element::DataOutputAssociation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for DataOutputAssociation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {DataOutputAssociation => DataAssociationType,DataAssociationTypeMut}
castable_to! {DataOutputAssociation => BaseElementType,BaseElementTypeMut}
//

/// Access to `dataOutputAssociation`
pub trait DataOutputAssociationType:
    DataAssociationType + Downcast + Debug + Send + DynClone
{
}
dyn_clone::clone_trait_object!(DataOutputAssociationType);
impl_downcast!(DataOutputAssociationType);
/// Mutable access to `dataOutputAssociation`
pub trait DataOutputAssociationTypeMut:
    DataAssociationTypeMut + Downcast + Debug + Send + DynClone + DataOutputAssociationType
{
}
dyn_clone::clone_trait_object!(DataOutputAssociationTypeMut);
impl_downcast!(DataOutputAssociationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:dataState")]
pub struct DataState {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("DataStateType",rg*="name","DataStateTypeMut",s)]
    pub name: Option<String>,
}
#[cast_to]
impl DocumentElement for DataState {
    fn element(&self) -> Element {
        Element::DataState
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for DataState {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {DataState => BaseElementType,BaseElementTypeMut}
//

/// Access to `dataState`
pub trait DataStateType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(DataStateType);
impl_downcast!(DataStateType);
/// Mutable access to `dataState`
pub trait DataStateTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + DataStateType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(DataStateTypeMut);
impl_downcast!(DataStateTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:dataStore")]
pub struct DataStore {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("DataStoreType",rg*="name","DataStoreTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "capacity")]
    #[tia("DataStoreType",rg*="capacity","DataStoreTypeMut",s)]
    pub capacity: Option<Integer>,
    #[xml(attr = "isUnlimited")]
    #[tia("DataStoreType",rg*="is_unlimited","DataStoreTypeMut",s)]
    pub is_unlimited: Option<bool>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("DataStoreType",rg*="item_subject_ref","DataStoreTypeMut",s)]
    pub item_subject_ref: Option<String>,
    #[xml(child = "bpmn:dataState")]
    #[tia("DataStoreType",rg*="data_state","DataStoreTypeMut",s,rmg*="data_state_mut")]
    pub data_state: Option<DataState>,
}
#[cast_to]
impl DocumentElement for DataStore {
    fn element(&self) -> Element {
        Element::DataStore
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for DataStore {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
#[cast_to]
impl RootElementType for DataStore {}
#[cast_to]
impl RootElementTypeMut for DataStore {}
castable_to! {DataStore => PartialEq<DataStore> }
castable_to! {DataStore => RootElementType,RootElementTypeMut}
castable_to! {DataStore => BaseElementType,BaseElementTypeMut}
//

/// Access to `dataStore`
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
/// Mutable access to `dataStore`
pub trait DataStoreTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + DataStoreType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `capacity`
    fn set_capacity(&mut self, value: Option<Integer>);
    /// Set value of attribute `isUnlimited`
    fn set_is_unlimited(&mut self, value: Option<bool>);
    /// Set value of attribute `itemSubjectRef`
    fn set_item_subject_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `dataState` child
    fn data_state_mut(&mut self) -> &mut Option<DataState>;
    /// Set value of `dataState` child
    fn set_data_state(&mut self, value: Option<DataState>);
}
dyn_clone::clone_trait_object!(DataStoreTypeMut);
impl_downcast!(DataStoreTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:dataStoreReference")]
pub struct DataStoreReference {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("DataStoreReferenceType",rg*="item_subject_ref","DataStoreReferenceTypeMut",s)]
    pub item_subject_ref: Option<String>,
    #[xml(attr = "dataStoreRef")]
    #[tia("DataStoreReferenceType",rg*="data_store_ref","DataStoreReferenceTypeMut",s)]
    pub data_store_ref: Option<String>,
    #[xml(child = "bpmn:dataState")]
    #[tia("DataStoreReferenceType",rg*="data_state","DataStoreReferenceTypeMut",s,rmg*="data_state_mut")]
    pub data_state: Option<DataState>,
}
#[cast_to]
impl DocumentElement for DataStoreReference {
    fn element(&self) -> Element {
        Element::DataStoreReference
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for DataStoreReference {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {DataStoreReference => FlowElementType,FlowElementTypeMut}
castable_to! {DataStoreReference => BaseElementType,BaseElementTypeMut}
//

/// Access to `dataStoreReference`
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
/// Mutable access to `dataStoreReference`
pub trait DataStoreReferenceTypeMut:
    FlowElementTypeMut + Downcast + Debug + Send + DynClone + DataStoreReferenceType
{
    /// Set value of attribute `itemSubjectRef`
    fn set_item_subject_ref(&mut self, value: Option<String>);
    /// Set value of attribute `dataStoreRef`
    fn set_data_store_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `dataState` child
    fn data_state_mut(&mut self) -> &mut Option<DataState>;
    /// Set value of `dataState` child
    fn set_data_state(&mut self, value: Option<DataState>);
}
dyn_clone::clone_trait_object!(DataStoreReferenceTypeMut);
impl_downcast!(DataStoreReferenceTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:documentation")]
pub struct Documentation {
    #[xml(attr = "id")]
    #[tia("DocumentationType",rg*="id","DocumentationTypeMut",s)]
    pub id: Option<Id>,
    #[xml(attr = "textFormat")]
    #[tia("DocumentationType",rg*="text_format","DocumentationTypeMut",s)]
    pub text_format: Option<String>,
}
#[cast_to]
impl DocumentElement for Documentation {
    fn element(&self) -> Element {
        Element::Documentation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Documentation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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

/// Access to `documentation`
pub trait DocumentationType: Downcast + Debug + Send + DynClone {
    /// Get value of attribute `id`
    fn id(&self) -> &Option<Id>;
    /// Get value of attribute `textFormat`
    fn text_format(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(DocumentationType);
impl_downcast!(DocumentationType);
/// Mutable access to `documentation`
pub trait DocumentationTypeMut: Downcast + Debug + Send + DynClone + DocumentationType {
    /// Set value of attribute `id`
    fn set_id(&mut self, value: Option<Id>);
    /// Set value of attribute `textFormat`
    fn set_text_format(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(DocumentationTypeMut);
impl_downcast!(DocumentationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:endEvent")]
pub struct EndEvent {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(child = "bpmn:property")]
    #[tia("EventType",rg*="properies","EventTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInput")]
    #[tia("ThrowEventType",rg*="data_inputs","ThrowEventTypeMut",s,rmg*="data_inputs_mut")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ThrowEventType",rg*="data_input_associations","ThrowEventTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:inputSet")]
    #[tia("ThrowEventType",rg*="input_set","ThrowEventTypeMut",s,rmg*="input_set_mut")]
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
    #[tia("ThrowEventType",rg*="event_definitions","ThrowEventTypeMut",s,rmg*="event_definitions_mut")]
    pub event_definitions: Vec<EventDefinition>,
    #[xml(flatten_text = "bpmn:eventDefinitionRef")]
    #[tia("ThrowEventType",rg*="event_definition_refs","ThrowEventTypeMut",s,rmg*="event_definition_refs_mut")]
    pub event_definition_refs: Vec<String>,
}
#[cast_to]
impl DocumentElement for EndEvent {
    fn element(&self) -> Element {
        Element::EndEvent
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for EndEvent {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {EndEvent => ThrowEventType,ThrowEventTypeMut}
castable_to! {EndEvent => EventType,EventTypeMut}
castable_to! {EndEvent => FlowNodeType,FlowNodeTypeMut}
castable_to! {EndEvent => FlowElementType,FlowElementTypeMut}
castable_to! {EndEvent => BaseElementType,BaseElementTypeMut}
//

/// Access to `endEvent`
pub trait EndEventType: ThrowEventType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(EndEventType);
impl_downcast!(EndEventType);
/// Mutable access to `endEvent`
pub trait EndEventTypeMut:
    ThrowEventTypeMut + Downcast + Debug + Send + DynClone + EndEventType
{
}
dyn_clone::clone_trait_object!(EndEventTypeMut);
impl_downcast!(EndEventTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:endPoint")]
pub struct EndPoint {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
}
#[cast_to]
impl DocumentElement for EndPoint {
    fn element(&self) -> Element {
        Element::EndPoint
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for EndPoint {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl RootElementType for EndPoint {}
#[cast_to]
impl RootElementTypeMut for EndPoint {}
castable_to! {EndPoint => PartialEq<EndPoint> }
castable_to! {EndPoint => RootElementType,RootElementTypeMut}
castable_to! {EndPoint => BaseElementType,BaseElementTypeMut}
//

/// Access to `endPoint`
pub trait EndPointType: RootElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(EndPointType);
impl_downcast!(EndPointType);
/// Mutable access to `endPoint`
pub trait EndPointTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + EndPointType
{
}
dyn_clone::clone_trait_object!(EndPointTypeMut);
impl_downcast!(EndPointTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:error")]
pub struct Error {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ErrorType",rg*="name","ErrorTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "errorCode")]
    #[tia("ErrorType",rg*="error_code","ErrorTypeMut",s)]
    pub error_code: Option<String>,
    #[xml(attr = "structureRef")]
    #[tia("ErrorType",rg*="structure_ref","ErrorTypeMut",s)]
    pub structure_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for Error {
    fn element(&self) -> Element {
        Element::Error
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Error {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl RootElementType for Error {}
#[cast_to]
impl RootElementTypeMut for Error {}
castable_to! {Error => PartialEq<Error> }
castable_to! {Error => RootElementType,RootElementTypeMut}
castable_to! {Error => BaseElementType,BaseElementTypeMut}
//

/// Access to `error`
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
/// Mutable access to `error`
pub trait ErrorTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + ErrorType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `errorCode`
    fn set_error_code(&mut self, value: Option<String>);
    /// Set value of attribute `structureRef`
    fn set_structure_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(ErrorTypeMut);
impl_downcast!(ErrorTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:errorEventDefinition")]
pub struct ErrorEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "errorRef")]
    #[tia("ErrorEventDefinitionType",rg*="error_ref","ErrorEventDefinitionTypeMut",s)]
    pub error_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for ErrorEventDefinition {
    fn element(&self) -> Element {
        Element::ErrorEventDefinition
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ErrorEventDefinition {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl EventDefinitionType for ErrorEventDefinition {}
#[cast_to]
impl EventDefinitionTypeMut for ErrorEventDefinition {}
castable_to! {ErrorEventDefinition => PartialEq<ErrorEventDefinition> }
castable_to! {ErrorEventDefinition => EventDefinitionType,EventDefinitionTypeMut}
#[cast_to]
impl RootElementType for ErrorEventDefinition {}
#[cast_to]
impl RootElementTypeMut for ErrorEventDefinition {}
castable_to! {ErrorEventDefinition => PartialEq<ErrorEventDefinition> }
castable_to! {ErrorEventDefinition => RootElementType,RootElementTypeMut}
castable_to! {ErrorEventDefinition => BaseElementType,BaseElementTypeMut}
//

/// Access to `errorEventDefinition`
pub trait ErrorEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `errorRef`
    fn error_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(ErrorEventDefinitionType);
impl_downcast!(ErrorEventDefinitionType);
/// Mutable access to `errorEventDefinition`
pub trait ErrorEventDefinitionTypeMut:
    EventDefinitionTypeMut + Downcast + Debug + Send + DynClone + ErrorEventDefinitionType
{
    /// Set value of attribute `errorRef`
    fn set_error_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(ErrorEventDefinitionTypeMut);
impl_downcast!(ErrorEventDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:escalation")]
pub struct Escalation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("EscalationType",rg*="name","EscalationTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "escalationCode")]
    #[tia("EscalationType",rg*="escalation_code","EscalationTypeMut",s)]
    pub escalation_code: Option<String>,
    #[xml(attr = "structureRef")]
    #[tia("EscalationType",rg*="structure_ref","EscalationTypeMut",s)]
    pub structure_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for Escalation {
    fn element(&self) -> Element {
        Element::Escalation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Escalation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl RootElementType for Escalation {}
#[cast_to]
impl RootElementTypeMut for Escalation {}
castable_to! {Escalation => PartialEq<Escalation> }
castable_to! {Escalation => RootElementType,RootElementTypeMut}
castable_to! {Escalation => BaseElementType,BaseElementTypeMut}
//

/// Access to `escalation`
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
/// Mutable access to `escalation`
pub trait EscalationTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + EscalationType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `escalationCode`
    fn set_escalation_code(&mut self, value: Option<String>);
    /// Set value of attribute `structureRef`
    fn set_structure_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(EscalationTypeMut);
impl_downcast!(EscalationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:escalationEventDefinition")]
pub struct EscalationEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "escalationRef")]
    #[tia("EscalationEventDefinitionType",rg*="escalation_ref","EscalationEventDefinitionTypeMut",s)]
    pub escalation_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for EscalationEventDefinition {
    fn element(&self) -> Element {
        Element::EscalationEventDefinition
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for EscalationEventDefinition {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl EventDefinitionType for EscalationEventDefinition {}
#[cast_to]
impl EventDefinitionTypeMut for EscalationEventDefinition {}
castable_to! {EscalationEventDefinition => PartialEq<EscalationEventDefinition> }
castable_to! {EscalationEventDefinition => EventDefinitionType,EventDefinitionTypeMut}
#[cast_to]
impl RootElementType for EscalationEventDefinition {}
#[cast_to]
impl RootElementTypeMut for EscalationEventDefinition {}
castable_to! {EscalationEventDefinition => PartialEq<EscalationEventDefinition> }
castable_to! {EscalationEventDefinition => RootElementType,RootElementTypeMut}
castable_to! {EscalationEventDefinition => BaseElementType,BaseElementTypeMut}
//

/// Access to `escalationEventDefinition`
pub trait EscalationEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `escalationRef`
    fn escalation_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(EscalationEventDefinitionType);
impl_downcast!(EscalationEventDefinitionType);
/// Mutable access to `escalationEventDefinition`
pub trait EscalationEventDefinitionTypeMut:
    EventDefinitionTypeMut + Downcast + Debug + Send + DynClone + EscalationEventDefinitionType
{
    /// Set value of attribute `escalationRef`
    fn set_escalation_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(EscalationEventDefinitionTypeMut);
impl_downcast!(EscalationEventDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:event")]
#[serde(tag = "type")]
pub enum Event {}
impl Event {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {}
    }
}
#[cast_to]
impl DocumentElementContainer for Event {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            _ => None,
        }
    }

    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
#[cast_to]
impl DocumentElement for Event {
    fn element(&self) -> Element {
        Element::Event
    }
}
/// Access to `event`
pub trait EventType: FlowNodeType + Downcast + Debug + Send + DynClone {
    /// Get value of `property` child
    fn properies(&self) -> &Vec<Property>;
}
dyn_clone::clone_trait_object!(EventType);
impl_downcast!(EventType);
/// Mutable access to `event`
pub trait EventTypeMut: FlowNodeTypeMut + Downcast + Debug + Send + DynClone + EventType {
    /// Get a mutable value of `property` child
    fn properies_mut(&mut self) -> &mut Vec<Property>;
    /// Set value of `property` child
    fn set_properies(&mut self, value: Vec<Property>);
}
dyn_clone::clone_trait_object!(EventTypeMut);
impl_downcast!(EventTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:eventBasedGateway")]
pub struct EventBasedGateway {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "gatewayDirection")]
    #[tia("GatewayType",rg*="gateway_direction","GatewayTypeMut",s)]
    pub gateway_direction: Option<String>,
    #[xml(attr = "instantiate")]
    #[tia("EventBasedGatewayType",rg*="instantiate","EventBasedGatewayTypeMut",s)]
    pub instantiate: Option<bool>,
    #[xml(attr = "eventGatewayType")]
    #[tia("EventBasedGatewayType",rg*="event_gateway_type","EventBasedGatewayTypeMut",s)]
    pub event_gateway_type: Option<String>,
}
#[cast_to]
impl DocumentElement for EventBasedGateway {
    fn element(&self) -> Element {
        Element::EventBasedGateway
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for EventBasedGateway {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {EventBasedGateway => GatewayType,GatewayTypeMut}
castable_to! {EventBasedGateway => FlowNodeType,FlowNodeTypeMut}
castable_to! {EventBasedGateway => FlowElementType,FlowElementTypeMut}
castable_to! {EventBasedGateway => BaseElementType,BaseElementTypeMut}
//

/// Access to `eventBasedGateway`
pub trait EventBasedGatewayType: GatewayType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `instantiate`
    fn instantiate(&self) -> &Option<bool>;
    /// Get value of attribute `eventGatewayType`
    fn event_gateway_type(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(EventBasedGatewayType);
impl_downcast!(EventBasedGatewayType);
/// Mutable access to `eventBasedGateway`
pub trait EventBasedGatewayTypeMut:
    GatewayTypeMut + Downcast + Debug + Send + DynClone + EventBasedGatewayType
{
    /// Set value of attribute `instantiate`
    fn set_instantiate(&mut self, value: Option<bool>);
    /// Set value of attribute `eventGatewayType`
    fn set_event_gateway_type(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(EventBasedGatewayTypeMut);
impl_downcast!(EventBasedGatewayTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:eventDefinition")]
#[serde(tag = "type")]
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
impl From<CancelEventDefinition> for EventDefinition {
    fn from(element: CancelEventDefinition) -> Self {
        Self::CancelEventDefinition(element)
    }
}
impl From<CompensateEventDefinition> for EventDefinition {
    fn from(element: CompensateEventDefinition) -> Self {
        Self::CompensateEventDefinition(element)
    }
}
impl From<ConditionalEventDefinition> for EventDefinition {
    fn from(element: ConditionalEventDefinition) -> Self {
        Self::ConditionalEventDefinition(element)
    }
}
impl From<ErrorEventDefinition> for EventDefinition {
    fn from(element: ErrorEventDefinition) -> Self {
        Self::ErrorEventDefinition(element)
    }
}
impl From<EscalationEventDefinition> for EventDefinition {
    fn from(element: EscalationEventDefinition) -> Self {
        Self::EscalationEventDefinition(element)
    }
}
impl From<LinkEventDefinition> for EventDefinition {
    fn from(element: LinkEventDefinition) -> Self {
        Self::LinkEventDefinition(element)
    }
}
impl From<MessageEventDefinition> for EventDefinition {
    fn from(element: MessageEventDefinition) -> Self {
        Self::MessageEventDefinition(element)
    }
}
impl From<SignalEventDefinition> for EventDefinition {
    fn from(element: SignalEventDefinition) -> Self {
        Self::SignalEventDefinition(element)
    }
}
impl From<TerminateEventDefinition> for EventDefinition {
    fn from(element: TerminateEventDefinition) -> Self {
        Self::TerminateEventDefinition(element)
    }
}
impl From<TimerEventDefinition> for EventDefinition {
    fn from(element: TimerEventDefinition) -> Self {
        Self::TimerEventDefinition(element)
    }
}
impl EventDefinition {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {
            EventDefinition::CancelEventDefinition(e) => Box::new(e) as Box<dyn DocumentElement>,
            EventDefinition::CompensateEventDefinition(e) => {
                Box::new(e) as Box<dyn DocumentElement>
            }
            EventDefinition::ConditionalEventDefinition(e) => {
                Box::new(e) as Box<dyn DocumentElement>
            }
            EventDefinition::ErrorEventDefinition(e) => Box::new(e) as Box<dyn DocumentElement>,
            EventDefinition::EscalationEventDefinition(e) => {
                Box::new(e) as Box<dyn DocumentElement>
            }
            EventDefinition::LinkEventDefinition(e) => Box::new(e) as Box<dyn DocumentElement>,
            EventDefinition::MessageEventDefinition(e) => Box::new(e) as Box<dyn DocumentElement>,
            EventDefinition::SignalEventDefinition(e) => Box::new(e) as Box<dyn DocumentElement>,
            EventDefinition::TerminateEventDefinition(e) => Box::new(e) as Box<dyn DocumentElement>,
            EventDefinition::TimerEventDefinition(e) => Box::new(e) as Box<dyn DocumentElement>,
        }
    }
}
#[cast_to]
impl DocumentElementContainer for EventDefinition {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            EventDefinition::CancelEventDefinition(e) => e.find_by_id_mut(id),
            EventDefinition::CompensateEventDefinition(e) => e.find_by_id_mut(id),
            EventDefinition::ConditionalEventDefinition(e) => e.find_by_id_mut(id),
            EventDefinition::ErrorEventDefinition(e) => e.find_by_id_mut(id),
            EventDefinition::EscalationEventDefinition(e) => e.find_by_id_mut(id),
            EventDefinition::LinkEventDefinition(e) => e.find_by_id_mut(id),
            EventDefinition::MessageEventDefinition(e) => e.find_by_id_mut(id),
            EventDefinition::SignalEventDefinition(e) => e.find_by_id_mut(id),
            EventDefinition::TerminateEventDefinition(e) => e.find_by_id_mut(id),
            EventDefinition::TimerEventDefinition(e) => e.find_by_id_mut(id),

            _ => None,
        }
    }

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
#[cast_to]
impl DocumentElement for EventDefinition {
    fn element(&self) -> Element {
        Element::EventDefinition
    }
}
/// Access to `eventDefinition`
pub trait EventDefinitionType: RootElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(EventDefinitionType);
impl_downcast!(EventDefinitionType);
/// Mutable access to `eventDefinition`
pub trait EventDefinitionTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + EventDefinitionType
{
}
dyn_clone::clone_trait_object!(EventDefinitionTypeMut);
impl_downcast!(EventDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:exclusiveGateway")]
pub struct ExclusiveGateway {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "gatewayDirection")]
    #[tia("GatewayType",rg*="gateway_direction","GatewayTypeMut",s)]
    pub gateway_direction: Option<String>,
    #[xml(attr = "default")]
    #[tia("ExclusiveGatewayType",rg*="default","ExclusiveGatewayTypeMut",s)]
    pub default: Option<String>,
}
#[cast_to]
impl DocumentElement for ExclusiveGateway {
    fn element(&self) -> Element {
        Element::ExclusiveGateway
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ExclusiveGateway {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {ExclusiveGateway => GatewayType,GatewayTypeMut}
castable_to! {ExclusiveGateway => FlowNodeType,FlowNodeTypeMut}
castable_to! {ExclusiveGateway => FlowElementType,FlowElementTypeMut}
castable_to! {ExclusiveGateway => BaseElementType,BaseElementTypeMut}
//

/// Access to `exclusiveGateway`
pub trait ExclusiveGatewayType: GatewayType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `default`
    fn default(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(ExclusiveGatewayType);
impl_downcast!(ExclusiveGatewayType);
/// Mutable access to `exclusiveGateway`
pub trait ExclusiveGatewayTypeMut:
    GatewayTypeMut + Downcast + Debug + Send + DynClone + ExclusiveGatewayType
{
    /// Set value of attribute `default`
    fn set_default(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(ExclusiveGatewayTypeMut);
impl_downcast!(ExclusiveGatewayTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:expression")]
pub struct Expression {
    #[xml(attr = "id")]
    #[tia("BaseElementWithMixedContentType",rg*="id","BaseElementWithMixedContentTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementWithMixedContentType",rg*="documentations","BaseElementWithMixedContentTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementWithMixedContentType",rg*="extension_elements","BaseElementWithMixedContentTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    // FIXME: This is a hack because obviously there's nothing about xsi:type
    // in BPMN schema (and rightfully so)
    #[tia(rg*="xsi_type",s)]
    #[xml(attr = "xsi:type")]
    pub xsi_type: Option<String>,
    #[tia("DocumentElementWithContent",rg*="content",
                    "DocumentElementWithContentMut",s,rmg*="content_mut")]
    #[xml(text)]
    pub content: Option<String>,
}
#[cast_to]
impl DocumentElement for Expression {
    fn element(&self) -> Element {
        Element::Expression
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Expression {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {Expression => BaseElementWithMixedContentType,BaseElementWithMixedContentTypeMut}
//

/// Access to `expression`
pub trait ExpressionType:
    BaseElementWithMixedContentType + Downcast + Debug + Send + DynClone
{
}
dyn_clone::clone_trait_object!(ExpressionType);
impl_downcast!(ExpressionType);
/// Mutable access to `expression`
pub trait ExpressionTypeMut:
    BaseElementWithMixedContentTypeMut + Downcast + Debug + Send + DynClone + ExpressionType
{
}
dyn_clone::clone_trait_object!(ExpressionTypeMut);
impl_downcast!(ExpressionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:extension")]
pub struct Extension {
    #[xml(attr = "definition")]
    #[tia("ExtensionType",rg*="definition","ExtensionTypeMut",s)]
    pub definition: Option<String>,
    #[xml(attr = "mustUnderstand")]
    #[tia("ExtensionType",rg*="must_understand","ExtensionTypeMut",s)]
    pub must_understand: Option<bool>,
    #[xml(child = "bpmn:documentation")]
    #[tia("ExtensionType",rg*="documentations","ExtensionTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
}
#[cast_to]
impl DocumentElement for Extension {
    fn element(&self) -> Element {
        Element::Extension
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Extension {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(e) = self.documentations.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(e) = self.documentations.find_by_id(id) {
            return Some(e);
        }
        None
    }
}
// Traits

//

/// Access to `extension`
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
/// Mutable access to `extension`
pub trait ExtensionTypeMut: Downcast + Debug + Send + DynClone + ExtensionType {
    /// Set value of attribute `definition`
    fn set_definition(&mut self, value: Option<String>);
    /// Set value of attribute `mustUnderstand`
    fn set_must_understand(&mut self, value: Option<bool>);
    /// Get a mutable value of `documentation` child
    fn documentations_mut(&mut self) -> &mut Vec<Documentation>;
    /// Set value of `documentation` child
    fn set_documentations(&mut self, value: Vec<Documentation>);
}
dyn_clone::clone_trait_object!(ExtensionTypeMut);
impl_downcast!(ExtensionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:extensionElements")]
pub struct ExtensionElements {}
#[cast_to]
impl DocumentElement for ExtensionElements {
    fn element(&self) -> Element {
        Element::ExtensionElements
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ExtensionElements {}
// Traits

//

/// Access to `extensionElements`
pub trait ExtensionElementsType: Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ExtensionElementsType);
impl_downcast!(ExtensionElementsType);
/// Mutable access to `extensionElements`
pub trait ExtensionElementsTypeMut:
    Downcast + Debug + Send + DynClone + ExtensionElementsType
{
}
dyn_clone::clone_trait_object!(ExtensionElementsTypeMut);
impl_downcast!(ExtensionElementsTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:flowElement")]
#[serde(tag = "type")]
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
impl From<AdHocSubProcess> for FlowElement {
    fn from(element: AdHocSubProcess) -> Self {
        Self::AdHocSubProcess(element)
    }
}
impl From<BoundaryEvent> for FlowElement {
    fn from(element: BoundaryEvent) -> Self {
        Self::BoundaryEvent(element)
    }
}
impl From<BusinessRuleTask> for FlowElement {
    fn from(element: BusinessRuleTask) -> Self {
        Self::BusinessRuleTask(element)
    }
}
impl From<CallActivity> for FlowElement {
    fn from(element: CallActivity) -> Self {
        Self::CallActivity(element)
    }
}
impl From<CallChoreography> for FlowElement {
    fn from(element: CallChoreography) -> Self {
        Self::CallChoreography(element)
    }
}
impl From<ChoreographyTask> for FlowElement {
    fn from(element: ChoreographyTask) -> Self {
        Self::ChoreographyTask(element)
    }
}
impl From<ComplexGateway> for FlowElement {
    fn from(element: ComplexGateway) -> Self {
        Self::ComplexGateway(element)
    }
}
impl From<DataObject> for FlowElement {
    fn from(element: DataObject) -> Self {
        Self::DataObject(element)
    }
}
impl From<DataObjectReference> for FlowElement {
    fn from(element: DataObjectReference) -> Self {
        Self::DataObjectReference(element)
    }
}
impl From<DataStoreReference> for FlowElement {
    fn from(element: DataStoreReference) -> Self {
        Self::DataStoreReference(element)
    }
}
impl From<EndEvent> for FlowElement {
    fn from(element: EndEvent) -> Self {
        Self::EndEvent(element)
    }
}
impl From<Event> for FlowElement {
    fn from(element: Event) -> Self {
        Self::Event(element)
    }
}
impl From<EventBasedGateway> for FlowElement {
    fn from(element: EventBasedGateway) -> Self {
        Self::EventBasedGateway(element)
    }
}
impl From<ExclusiveGateway> for FlowElement {
    fn from(element: ExclusiveGateway) -> Self {
        Self::ExclusiveGateway(element)
    }
}
impl From<ImplicitThrowEvent> for FlowElement {
    fn from(element: ImplicitThrowEvent) -> Self {
        Self::ImplicitThrowEvent(element)
    }
}
impl From<InclusiveGateway> for FlowElement {
    fn from(element: InclusiveGateway) -> Self {
        Self::InclusiveGateway(element)
    }
}
impl From<IntermediateCatchEvent> for FlowElement {
    fn from(element: IntermediateCatchEvent) -> Self {
        Self::IntermediateCatchEvent(element)
    }
}
impl From<IntermediateThrowEvent> for FlowElement {
    fn from(element: IntermediateThrowEvent) -> Self {
        Self::IntermediateThrowEvent(element)
    }
}
impl From<ManualTask> for FlowElement {
    fn from(element: ManualTask) -> Self {
        Self::ManualTask(element)
    }
}
impl From<ParallelGateway> for FlowElement {
    fn from(element: ParallelGateway) -> Self {
        Self::ParallelGateway(element)
    }
}
impl From<ReceiveTask> for FlowElement {
    fn from(element: ReceiveTask) -> Self {
        Self::ReceiveTask(element)
    }
}
impl From<ScriptTask> for FlowElement {
    fn from(element: ScriptTask) -> Self {
        Self::ScriptTask(element)
    }
}
impl From<SendTask> for FlowElement {
    fn from(element: SendTask) -> Self {
        Self::SendTask(element)
    }
}
impl From<SequenceFlow> for FlowElement {
    fn from(element: SequenceFlow) -> Self {
        Self::SequenceFlow(element)
    }
}
impl From<ServiceTask> for FlowElement {
    fn from(element: ServiceTask) -> Self {
        Self::ServiceTask(element)
    }
}
impl From<StartEvent> for FlowElement {
    fn from(element: StartEvent) -> Self {
        Self::StartEvent(element)
    }
}
impl From<SubChoreography> for FlowElement {
    fn from(element: SubChoreography) -> Self {
        Self::SubChoreography(element)
    }
}
impl From<SubProcess> for FlowElement {
    fn from(element: SubProcess) -> Self {
        Self::SubProcess(element)
    }
}
impl From<Task> for FlowElement {
    fn from(element: Task) -> Self {
        Self::Task(element)
    }
}
impl From<Transaction> for FlowElement {
    fn from(element: Transaction) -> Self {
        Self::Transaction(element)
    }
}
impl From<UserTask> for FlowElement {
    fn from(element: UserTask) -> Self {
        Self::UserTask(element)
    }
}
impl FlowElement {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {
            FlowElement::AdHocSubProcess(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::BoundaryEvent(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::BusinessRuleTask(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::CallActivity(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::CallChoreography(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::ChoreographyTask(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::ComplexGateway(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::DataObject(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::DataObjectReference(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::DataStoreReference(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::EndEvent(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::Event(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::EventBasedGateway(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::ExclusiveGateway(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::ImplicitThrowEvent(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::InclusiveGateway(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::IntermediateCatchEvent(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::IntermediateThrowEvent(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::ManualTask(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::ParallelGateway(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::ReceiveTask(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::ScriptTask(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::SendTask(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::SequenceFlow(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::ServiceTask(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::StartEvent(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::SubChoreography(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::SubProcess(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::Task(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::Transaction(e) => Box::new(e) as Box<dyn DocumentElement>,
            FlowElement::UserTask(e) => Box::new(e) as Box<dyn DocumentElement>,
        }
    }
}
#[cast_to]
impl DocumentElementContainer for FlowElement {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            FlowElement::AdHocSubProcess(e) => e.find_by_id_mut(id),
            FlowElement::BoundaryEvent(e) => e.find_by_id_mut(id),
            FlowElement::BusinessRuleTask(e) => e.find_by_id_mut(id),
            FlowElement::CallActivity(e) => e.find_by_id_mut(id),
            FlowElement::CallChoreography(e) => e.find_by_id_mut(id),
            FlowElement::ChoreographyTask(e) => e.find_by_id_mut(id),
            FlowElement::ComplexGateway(e) => e.find_by_id_mut(id),
            FlowElement::DataObject(e) => e.find_by_id_mut(id),
            FlowElement::DataObjectReference(e) => e.find_by_id_mut(id),
            FlowElement::DataStoreReference(e) => e.find_by_id_mut(id),
            FlowElement::EndEvent(e) => e.find_by_id_mut(id),
            FlowElement::Event(e) => e.find_by_id_mut(id),
            FlowElement::EventBasedGateway(e) => e.find_by_id_mut(id),
            FlowElement::ExclusiveGateway(e) => e.find_by_id_mut(id),
            FlowElement::ImplicitThrowEvent(e) => e.find_by_id_mut(id),
            FlowElement::InclusiveGateway(e) => e.find_by_id_mut(id),
            FlowElement::IntermediateCatchEvent(e) => e.find_by_id_mut(id),
            FlowElement::IntermediateThrowEvent(e) => e.find_by_id_mut(id),
            FlowElement::ManualTask(e) => e.find_by_id_mut(id),
            FlowElement::ParallelGateway(e) => e.find_by_id_mut(id),
            FlowElement::ReceiveTask(e) => e.find_by_id_mut(id),
            FlowElement::ScriptTask(e) => e.find_by_id_mut(id),
            FlowElement::SendTask(e) => e.find_by_id_mut(id),
            FlowElement::SequenceFlow(e) => e.find_by_id_mut(id),
            FlowElement::ServiceTask(e) => e.find_by_id_mut(id),
            FlowElement::StartEvent(e) => e.find_by_id_mut(id),
            FlowElement::SubChoreography(e) => e.find_by_id_mut(id),
            FlowElement::SubProcess(e) => e.find_by_id_mut(id),
            FlowElement::Task(e) => e.find_by_id_mut(id),
            FlowElement::Transaction(e) => e.find_by_id_mut(id),
            FlowElement::UserTask(e) => e.find_by_id_mut(id),

            _ => None,
        }
    }

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
#[cast_to]
impl DocumentElement for FlowElement {
    fn element(&self) -> Element {
        Element::FlowElement
    }
}
/// Access to `flowElement`
pub trait FlowElementType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `auditing` child
    fn auditing(&self) -> &Option<Auditing>;
    /// Get value of `monitoring` child
    fn monitoring(&self) -> &Option<Monitoring>;
    /// Get value of `categoryValueRef` child
    fn category_value_refs(&self) -> &Vec<String>;
}
dyn_clone::clone_trait_object!(FlowElementType);
impl_downcast!(FlowElementType);
/// Mutable access to `flowElement`
pub trait FlowElementTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + FlowElementType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Get a mutable value of `auditing` child
    fn auditing_mut(&mut self) -> &mut Option<Auditing>;
    /// Set value of `auditing` child
    fn set_auditing(&mut self, value: Option<Auditing>);
    /// Get a mutable value of `monitoring` child
    fn monitoring_mut(&mut self) -> &mut Option<Monitoring>;
    /// Set value of `monitoring` child
    fn set_monitoring(&mut self, value: Option<Monitoring>);
    /// Get a mutable value of `categoryValueRef` child
    fn category_value_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `categoryValueRef` child
    fn set_category_value_refs(&mut self, value: Vec<String>);
}
dyn_clone::clone_trait_object!(FlowElementTypeMut);
impl_downcast!(FlowElementTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:flowNode")]
#[serde(tag = "type")]
pub enum FlowNode {}
impl FlowNode {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {}
    }
}
#[cast_to]
impl DocumentElementContainer for FlowNode {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            _ => None,
        }
    }

    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
#[cast_to]
impl DocumentElement for FlowNode {
    fn element(&self) -> Element {
        Element::FlowNode
    }
}
/// Access to `flowNode`
pub trait FlowNodeType: FlowElementType + Downcast + Debug + Send + DynClone {
    /// Get value of `incoming` child
    fn incomings(&self) -> &Vec<String>;
    /// Get value of `outgoing` child
    fn outgoings(&self) -> &Vec<String>;
}
dyn_clone::clone_trait_object!(FlowNodeType);
impl_downcast!(FlowNodeType);
/// Mutable access to `flowNode`
pub trait FlowNodeTypeMut:
    FlowElementTypeMut + Downcast + Debug + Send + DynClone + FlowNodeType
{
    /// Get a mutable value of `incoming` child
    fn incomings_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `incoming` child
    fn set_incomings(&mut self, value: Vec<String>);
    /// Get a mutable value of `outgoing` child
    fn outgoings_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `outgoing` child
    fn set_outgoings(&mut self, value: Vec<String>);
}
dyn_clone::clone_trait_object!(FlowNodeTypeMut);
impl_downcast!(FlowNodeTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:formalExpression")]
pub struct FormalExpression {
    #[xml(attr = "id")]
    #[tia("BaseElementWithMixedContentType",rg*="id","BaseElementWithMixedContentTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementWithMixedContentType",rg*="documentations","BaseElementWithMixedContentTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementWithMixedContentType",rg*="extension_elements","BaseElementWithMixedContentTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "language")]
    #[tia("FormalExpressionType",rg*="language","FormalExpressionTypeMut",s)]
    pub language: Option<URI>,
    #[xml(attr = "evaluatesToTypeRef")]
    #[tia("FormalExpressionType",rg*="evaluates_totype_ref","FormalExpressionTypeMut",s)]
    pub evaluates_totype_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for FormalExpression {
    fn element(&self) -> Element {
        Element::FormalExpression
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for FormalExpression {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl ExpressionType for FormalExpression {}
#[cast_to]
impl ExpressionTypeMut for FormalExpression {}
castable_to! {FormalExpression => PartialEq<FormalExpression> }
castable_to! {FormalExpression => ExpressionType,ExpressionTypeMut}
castable_to! {FormalExpression => BaseElementWithMixedContentType,BaseElementWithMixedContentTypeMut}
//

/// Access to `formalExpression`
pub trait FormalExpressionType: ExpressionType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `language`
    fn language(&self) -> &Option<URI>;
    /// Get value of attribute `evaluatesToTypeRef`
    fn evaluates_totype_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(FormalExpressionType);
impl_downcast!(FormalExpressionType);
/// Mutable access to `formalExpression`
pub trait FormalExpressionTypeMut:
    ExpressionTypeMut + Downcast + Debug + Send + DynClone + FormalExpressionType
{
    /// Set value of attribute `language`
    fn set_language(&mut self, value: Option<URI>);
    /// Set value of attribute `evaluatesToTypeRef`
    fn set_evaluates_totype_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(FormalExpressionTypeMut);
impl_downcast!(FormalExpressionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:gateway")]
pub struct Gateway {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "gatewayDirection")]
    #[tia("GatewayType",rg*="gateway_direction","GatewayTypeMut",s)]
    pub gateway_direction: Option<String>,
}
#[cast_to]
impl DocumentElement for Gateway {
    fn element(&self) -> Element {
        Element::Gateway
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Gateway {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {Gateway => FlowNodeType,FlowNodeTypeMut}
castable_to! {Gateway => FlowElementType,FlowElementTypeMut}
castable_to! {Gateway => BaseElementType,BaseElementTypeMut}
//

/// Access to `gateway`
pub trait GatewayType: FlowNodeType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `gatewayDirection`
    fn gateway_direction(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(GatewayType);
impl_downcast!(GatewayType);
/// Mutable access to `gateway`
pub trait GatewayTypeMut:
    FlowNodeTypeMut + Downcast + Debug + Send + DynClone + GatewayType
{
    /// Set value of attribute `gatewayDirection`
    fn set_gateway_direction(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(GatewayTypeMut);
impl_downcast!(GatewayTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:globalBusinessRuleTask")]
pub struct GlobalBusinessRuleTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name","CallableElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs","CallableElementTypeMut",s,rmg*="supported_interface_refs_mut")]
    pub supported_interface_refs: Vec<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification","CallableElementTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings","CallableElementTypeMut",s,rmg*="io_bindings_mut")]
    pub io_bindings: Vec<InputOutputBinding>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("GlobalTaskType",rg*="resource_roles","GlobalTaskTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(attr = "implementation")]
    #[tia("GlobalBusinessRuleTaskType",rg*="implementation","GlobalBusinessRuleTaskTypeMut",s)]
    pub implementation: Option<String>,
}
#[cast_to]
impl DocumentElement for GlobalBusinessRuleTask {
    fn element(&self) -> Element {
        Element::GlobalBusinessRuleTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for GlobalBusinessRuleTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {GlobalBusinessRuleTask => GlobalTaskType,GlobalTaskTypeMut}
castable_to! {GlobalBusinessRuleTask => CallableElementType,CallableElementTypeMut}
#[cast_to]
impl RootElementType for GlobalBusinessRuleTask {}
#[cast_to]
impl RootElementTypeMut for GlobalBusinessRuleTask {}
castable_to! {GlobalBusinessRuleTask => PartialEq<GlobalBusinessRuleTask> }
castable_to! {GlobalBusinessRuleTask => RootElementType,RootElementTypeMut}
castable_to! {GlobalBusinessRuleTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `globalBusinessRuleTask`
pub trait GlobalBusinessRuleTaskType: GlobalTaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `implementation`
    fn implementation(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(GlobalBusinessRuleTaskType);
impl_downcast!(GlobalBusinessRuleTaskType);
/// Mutable access to `globalBusinessRuleTask`
pub trait GlobalBusinessRuleTaskTypeMut:
    GlobalTaskTypeMut + Downcast + Debug + Send + DynClone + GlobalBusinessRuleTaskType
{
    /// Set value of attribute `implementation`
    fn set_implementation(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(GlobalBusinessRuleTaskTypeMut);
impl_downcast!(GlobalBusinessRuleTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:globalChoreographyTask")]
pub struct GlobalChoreographyTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CollaborationType",rg*="name","CollaborationTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "isClosed")]
    #[tia("CollaborationType",rg*="is_closed","CollaborationTypeMut",s)]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    #[tia("CollaborationType",rg*="participants","CollaborationTypeMut",s,rmg*="participants_mut")]
    pub participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    #[tia("CollaborationType",rg*="message_flows","CollaborationTypeMut",s,rmg*="message_flows_mut")]
    pub message_flows: Vec<MessageFlow>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("CollaborationType",rg*="artifacts","CollaborationTypeMut",s,rmg*="artifacts_mut")]
    pub artifacts: Vec<Artifact>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    #[tia("CollaborationType",rg*="conversation_nodes","CollaborationTypeMut",s,rmg*="conversation_nodes_mut")]
    pub conversation_nodes: Vec<ConversationNode>,
    #[xml(child = "bpmn:conversationAssociation")]
    #[tia("CollaborationType",rg*="conversation_associations","CollaborationTypeMut",s,rmg*="conversation_associations_mut")]
    pub conversation_associations: Vec<ConversationAssociation>,
    #[xml(child = "bpmn:participantAssociation")]
    #[tia("CollaborationType",rg*="participant_associations","CollaborationTypeMut",s,rmg*="participant_associations_mut")]
    pub participant_associations: Vec<ParticipantAssociation>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    #[tia("CollaborationType",rg*="message_flow_associations","CollaborationTypeMut",s,rmg*="message_flow_associations_mut")]
    pub message_flow_associations: Vec<MessageFlowAssociation>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("CollaborationType",rg*="correlation_keys","CollaborationTypeMut",s,rmg*="correlation_keys_mut")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(flatten_text = "bpmn:choreographyRef")]
    #[tia("CollaborationType",rg*="choreography_refs","CollaborationTypeMut",s,rmg*="choreography_refs_mut")]
    pub choreography_refs: Vec<String>,
    #[xml(child = "bpmn:conversationLink")]
    #[tia("CollaborationType",rg*="conversation_links","CollaborationTypeMut",s,rmg*="conversation_links_mut")]
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
    #[tia("ChoreographyType",rg*="flow_elements","ChoreographyTypeMut",s,rmg*="flow_elements_mut")]
    pub flow_elements: Vec<FlowElement>,
    #[xml(attr = "initiatingParticipantRef")]
    #[tia("GlobalChoreographyTaskType",rg*="initiating_participant_ref","GlobalChoreographyTaskTypeMut",s)]
    pub initiating_participant_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for GlobalChoreographyTask {
    fn element(&self) -> Element {
        Element::GlobalChoreographyTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for GlobalChoreographyTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {GlobalChoreographyTask => ChoreographyType,ChoreographyTypeMut}
castable_to! {GlobalChoreographyTask => CollaborationType,CollaborationTypeMut}
#[cast_to]
impl RootElementType for GlobalChoreographyTask {}
#[cast_to]
impl RootElementTypeMut for GlobalChoreographyTask {}
castable_to! {GlobalChoreographyTask => PartialEq<GlobalChoreographyTask> }
castable_to! {GlobalChoreographyTask => RootElementType,RootElementTypeMut}
castable_to! {GlobalChoreographyTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `globalChoreographyTask`
pub trait GlobalChoreographyTaskType:
    ChoreographyType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `initiatingParticipantRef`
    fn initiating_participant_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(GlobalChoreographyTaskType);
impl_downcast!(GlobalChoreographyTaskType);
/// Mutable access to `globalChoreographyTask`
pub trait GlobalChoreographyTaskTypeMut:
    ChoreographyTypeMut + Downcast + Debug + Send + DynClone + GlobalChoreographyTaskType
{
    /// Set value of attribute `initiatingParticipantRef`
    fn set_initiating_participant_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(GlobalChoreographyTaskTypeMut);
impl_downcast!(GlobalChoreographyTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:globalConversation")]
pub struct GlobalConversation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CollaborationType",rg*="name","CollaborationTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "isClosed")]
    #[tia("CollaborationType",rg*="is_closed","CollaborationTypeMut",s)]
    pub is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    #[tia("CollaborationType",rg*="participants","CollaborationTypeMut",s,rmg*="participants_mut")]
    pub participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    #[tia("CollaborationType",rg*="message_flows","CollaborationTypeMut",s,rmg*="message_flows_mut")]
    pub message_flows: Vec<MessageFlow>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("CollaborationType",rg*="artifacts","CollaborationTypeMut",s,rmg*="artifacts_mut")]
    pub artifacts: Vec<Artifact>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    #[tia("CollaborationType",rg*="conversation_nodes","CollaborationTypeMut",s,rmg*="conversation_nodes_mut")]
    pub conversation_nodes: Vec<ConversationNode>,
    #[xml(child = "bpmn:conversationAssociation")]
    #[tia("CollaborationType",rg*="conversation_associations","CollaborationTypeMut",s,rmg*="conversation_associations_mut")]
    pub conversation_associations: Vec<ConversationAssociation>,
    #[xml(child = "bpmn:participantAssociation")]
    #[tia("CollaborationType",rg*="participant_associations","CollaborationTypeMut",s,rmg*="participant_associations_mut")]
    pub participant_associations: Vec<ParticipantAssociation>,
    #[xml(child = "bpmn:messageFlowAssociation")]
    #[tia("CollaborationType",rg*="message_flow_associations","CollaborationTypeMut",s,rmg*="message_flow_associations_mut")]
    pub message_flow_associations: Vec<MessageFlowAssociation>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("CollaborationType",rg*="correlation_keys","CollaborationTypeMut",s,rmg*="correlation_keys_mut")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(flatten_text = "bpmn:choreographyRef")]
    #[tia("CollaborationType",rg*="choreography_refs","CollaborationTypeMut",s,rmg*="choreography_refs_mut")]
    pub choreography_refs: Vec<String>,
    #[xml(child = "bpmn:conversationLink")]
    #[tia("CollaborationType",rg*="conversation_links","CollaborationTypeMut",s,rmg*="conversation_links_mut")]
    pub conversation_links: Vec<ConversationLink>,
}
#[cast_to]
impl DocumentElement for GlobalConversation {
    fn element(&self) -> Element {
        Element::GlobalConversation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for GlobalConversation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {GlobalConversation => CollaborationType,CollaborationTypeMut}
#[cast_to]
impl RootElementType for GlobalConversation {}
#[cast_to]
impl RootElementTypeMut for GlobalConversation {}
castable_to! {GlobalConversation => PartialEq<GlobalConversation> }
castable_to! {GlobalConversation => RootElementType,RootElementTypeMut}
castable_to! {GlobalConversation => BaseElementType,BaseElementTypeMut}
//

/// Access to `globalConversation`
pub trait GlobalConversationType: CollaborationType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(GlobalConversationType);
impl_downcast!(GlobalConversationType);
/// Mutable access to `globalConversation`
pub trait GlobalConversationTypeMut:
    CollaborationTypeMut + Downcast + Debug + Send + DynClone + GlobalConversationType
{
}
dyn_clone::clone_trait_object!(GlobalConversationTypeMut);
impl_downcast!(GlobalConversationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:globalManualTask")]
pub struct GlobalManualTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name","CallableElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs","CallableElementTypeMut",s,rmg*="supported_interface_refs_mut")]
    pub supported_interface_refs: Vec<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification","CallableElementTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings","CallableElementTypeMut",s,rmg*="io_bindings_mut")]
    pub io_bindings: Vec<InputOutputBinding>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("GlobalTaskType",rg*="resource_roles","GlobalTaskTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
}
#[cast_to]
impl DocumentElement for GlobalManualTask {
    fn element(&self) -> Element {
        Element::GlobalManualTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for GlobalManualTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {GlobalManualTask => GlobalTaskType,GlobalTaskTypeMut}
castable_to! {GlobalManualTask => CallableElementType,CallableElementTypeMut}
#[cast_to]
impl RootElementType for GlobalManualTask {}
#[cast_to]
impl RootElementTypeMut for GlobalManualTask {}
castable_to! {GlobalManualTask => PartialEq<GlobalManualTask> }
castable_to! {GlobalManualTask => RootElementType,RootElementTypeMut}
castable_to! {GlobalManualTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `globalManualTask`
pub trait GlobalManualTaskType: GlobalTaskType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(GlobalManualTaskType);
impl_downcast!(GlobalManualTaskType);
/// Mutable access to `globalManualTask`
pub trait GlobalManualTaskTypeMut:
    GlobalTaskTypeMut + Downcast + Debug + Send + DynClone + GlobalManualTaskType
{
}
dyn_clone::clone_trait_object!(GlobalManualTaskTypeMut);
impl_downcast!(GlobalManualTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:globalScriptTask")]
pub struct GlobalScriptTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name","CallableElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs","CallableElementTypeMut",s,rmg*="supported_interface_refs_mut")]
    pub supported_interface_refs: Vec<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification","CallableElementTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings","CallableElementTypeMut",s,rmg*="io_bindings_mut")]
    pub io_bindings: Vec<InputOutputBinding>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("GlobalTaskType",rg*="resource_roles","GlobalTaskTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(attr = "scriptLanguage")]
    #[tia("GlobalScriptTaskType",rg*="script_language","GlobalScriptTaskTypeMut",s)]
    pub script_language: Option<URI>,
    #[xml(child = "bpmn:script")]
    #[tia("GlobalScriptTaskType",rg*="script","GlobalScriptTaskTypeMut",s,rmg*="script_mut")]
    pub script: Option<Script>,
}
#[cast_to]
impl DocumentElement for GlobalScriptTask {
    fn element(&self) -> Element {
        Element::GlobalScriptTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for GlobalScriptTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.script.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {GlobalScriptTask => GlobalTaskType,GlobalTaskTypeMut}
castable_to! {GlobalScriptTask => CallableElementType,CallableElementTypeMut}
#[cast_to]
impl RootElementType for GlobalScriptTask {}
#[cast_to]
impl RootElementTypeMut for GlobalScriptTask {}
castable_to! {GlobalScriptTask => PartialEq<GlobalScriptTask> }
castable_to! {GlobalScriptTask => RootElementType,RootElementTypeMut}
castable_to! {GlobalScriptTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `globalScriptTask`
pub trait GlobalScriptTaskType: GlobalTaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `scriptLanguage`
    fn script_language(&self) -> &Option<URI>;
    /// Get value of `script` child
    fn script(&self) -> &Option<Script>;
}
dyn_clone::clone_trait_object!(GlobalScriptTaskType);
impl_downcast!(GlobalScriptTaskType);
/// Mutable access to `globalScriptTask`
pub trait GlobalScriptTaskTypeMut:
    GlobalTaskTypeMut + Downcast + Debug + Send + DynClone + GlobalScriptTaskType
{
    /// Set value of attribute `scriptLanguage`
    fn set_script_language(&mut self, value: Option<URI>);
    /// Get a mutable value of `script` child
    fn script_mut(&mut self) -> &mut Option<Script>;
    /// Set value of `script` child
    fn set_script(&mut self, value: Option<Script>);
}
dyn_clone::clone_trait_object!(GlobalScriptTaskTypeMut);
impl_downcast!(GlobalScriptTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:globalTask")]
pub struct GlobalTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name","CallableElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs","CallableElementTypeMut",s,rmg*="supported_interface_refs_mut")]
    pub supported_interface_refs: Vec<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification","CallableElementTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings","CallableElementTypeMut",s,rmg*="io_bindings_mut")]
    pub io_bindings: Vec<InputOutputBinding>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("GlobalTaskType",rg*="resource_roles","GlobalTaskTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
}
#[cast_to]
impl DocumentElement for GlobalTask {
    fn element(&self) -> Element {
        Element::GlobalTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for GlobalTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.resource_roles.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {GlobalTask => CallableElementType,CallableElementTypeMut}
#[cast_to]
impl RootElementType for GlobalTask {}
#[cast_to]
impl RootElementTypeMut for GlobalTask {}
castable_to! {GlobalTask => PartialEq<GlobalTask> }
castable_to! {GlobalTask => RootElementType,RootElementTypeMut}
castable_to! {GlobalTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `globalTask`
pub trait GlobalTaskType: CallableElementType + Downcast + Debug + Send + DynClone {
    /// Get value of `resourceRole` child
    fn resource_roles(&self) -> &Vec<ResourceRole>;
}
dyn_clone::clone_trait_object!(GlobalTaskType);
impl_downcast!(GlobalTaskType);
/// Mutable access to `globalTask`
pub trait GlobalTaskTypeMut:
    CallableElementTypeMut + Downcast + Debug + Send + DynClone + GlobalTaskType
{
    /// Get a mutable value of `resourceRole` child
    fn resource_roles_mut(&mut self) -> &mut Vec<ResourceRole>;
    /// Set value of `resourceRole` child
    fn set_resource_roles(&mut self, value: Vec<ResourceRole>);
}
dyn_clone::clone_trait_object!(GlobalTaskTypeMut);
impl_downcast!(GlobalTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:globalUserTask")]
pub struct GlobalUserTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name","CallableElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs","CallableElementTypeMut",s,rmg*="supported_interface_refs_mut")]
    pub supported_interface_refs: Vec<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification","CallableElementTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings","CallableElementTypeMut",s,rmg*="io_bindings_mut")]
    pub io_bindings: Vec<InputOutputBinding>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("GlobalTaskType",rg*="resource_roles","GlobalTaskTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(attr = "implementation")]
    #[tia("GlobalUserTaskType",rg*="implementation","GlobalUserTaskTypeMut",s)]
    pub implementation: Option<String>,
    #[xml(child = "bpmn:rendering")]
    #[tia("GlobalUserTaskType",rg*="renderings","GlobalUserTaskTypeMut",s,rmg*="renderings_mut")]
    pub renderings: Vec<Rendering>,
}
#[cast_to]
impl DocumentElement for GlobalUserTask {
    fn element(&self) -> Element {
        Element::GlobalUserTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for GlobalUserTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.renderings.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {GlobalUserTask => GlobalTaskType,GlobalTaskTypeMut}
castable_to! {GlobalUserTask => CallableElementType,CallableElementTypeMut}
#[cast_to]
impl RootElementType for GlobalUserTask {}
#[cast_to]
impl RootElementTypeMut for GlobalUserTask {}
castable_to! {GlobalUserTask => PartialEq<GlobalUserTask> }
castable_to! {GlobalUserTask => RootElementType,RootElementTypeMut}
castable_to! {GlobalUserTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `globalUserTask`
pub trait GlobalUserTaskType: GlobalTaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `implementation`
    fn implementation(&self) -> &Option<String>;
    /// Get value of `rendering` child
    fn renderings(&self) -> &Vec<Rendering>;
}
dyn_clone::clone_trait_object!(GlobalUserTaskType);
impl_downcast!(GlobalUserTaskType);
/// Mutable access to `globalUserTask`
pub trait GlobalUserTaskTypeMut:
    GlobalTaskTypeMut + Downcast + Debug + Send + DynClone + GlobalUserTaskType
{
    /// Set value of attribute `implementation`
    fn set_implementation(&mut self, value: Option<String>);
    /// Get a mutable value of `rendering` child
    fn renderings_mut(&mut self) -> &mut Vec<Rendering>;
    /// Set value of `rendering` child
    fn set_renderings(&mut self, value: Vec<Rendering>);
}
dyn_clone::clone_trait_object!(GlobalUserTaskTypeMut);
impl_downcast!(GlobalUserTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:group")]
pub struct Group {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "categoryValueRef")]
    #[tia("GroupType",rg*="category_value_ref","GroupTypeMut",s)]
    pub category_value_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for Group {
    fn element(&self) -> Element {
        Element::Group
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Group {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl ArtifactType for Group {}
#[cast_to]
impl ArtifactTypeMut for Group {}
castable_to! {Group => PartialEq<Group> }
castable_to! {Group => ArtifactType,ArtifactTypeMut}
castable_to! {Group => BaseElementType,BaseElementTypeMut}
//

/// Access to `group`
pub trait GroupType: ArtifactType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `categoryValueRef`
    fn category_value_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(GroupType);
impl_downcast!(GroupType);
/// Mutable access to `group`
pub trait GroupTypeMut: ArtifactTypeMut + Downcast + Debug + Send + DynClone + GroupType {
    /// Set value of attribute `categoryValueRef`
    fn set_category_value_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(GroupTypeMut);
impl_downcast!(GroupTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:humanPerformer")]
pub struct HumanPerformer {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ResourceRoleType",rg*="name","ResourceRoleTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:resourceRef")]
    #[tia("ResourceRoleType",rg*="resource_ref","ResourceRoleTypeMut",s,rmg*="resource_ref_mut")]
    pub resource_ref: String,
    #[xml(child = "bpmn:resourceParameterBinding")]
    #[tia("ResourceRoleType",rg*="resource_parameter_bindings","ResourceRoleTypeMut",s,rmg*="resource_parameter_bindings_mut")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    #[tia("ResourceRoleType",rg*="resource_assignment_expression","ResourceRoleTypeMut",s,rmg*="resource_assignment_expression_mut")]
    pub resource_assignment_expression: Option<ResourceAssignmentExpression>,
}
#[cast_to]
impl DocumentElement for HumanPerformer {
    fn element(&self) -> Element {
        Element::HumanPerformer
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for HumanPerformer {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl PerformerType for HumanPerformer {}
#[cast_to]
impl PerformerTypeMut for HumanPerformer {}
castable_to! {HumanPerformer => PartialEq<HumanPerformer> }
castable_to! {HumanPerformer => PerformerType,PerformerTypeMut}
castable_to! {HumanPerformer => ResourceRoleType,ResourceRoleTypeMut}
castable_to! {HumanPerformer => BaseElementType,BaseElementTypeMut}
//

/// Access to `humanPerformer`
pub trait HumanPerformerType: PerformerType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(HumanPerformerType);
impl_downcast!(HumanPerformerType);
/// Mutable access to `humanPerformer`
pub trait HumanPerformerTypeMut:
    PerformerTypeMut + Downcast + Debug + Send + DynClone + HumanPerformerType
{
}
dyn_clone::clone_trait_object!(HumanPerformerTypeMut);
impl_downcast!(HumanPerformerTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:implicitThrowEvent")]
pub struct ImplicitThrowEvent {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(child = "bpmn:property")]
    #[tia("EventType",rg*="properies","EventTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInput")]
    #[tia("ThrowEventType",rg*="data_inputs","ThrowEventTypeMut",s,rmg*="data_inputs_mut")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ThrowEventType",rg*="data_input_associations","ThrowEventTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:inputSet")]
    #[tia("ThrowEventType",rg*="input_set","ThrowEventTypeMut",s,rmg*="input_set_mut")]
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
    #[tia("ThrowEventType",rg*="event_definitions","ThrowEventTypeMut",s,rmg*="event_definitions_mut")]
    pub event_definitions: Vec<EventDefinition>,
    #[xml(flatten_text = "bpmn:eventDefinitionRef")]
    #[tia("ThrowEventType",rg*="event_definition_refs","ThrowEventTypeMut",s,rmg*="event_definition_refs_mut")]
    pub event_definition_refs: Vec<String>,
}
#[cast_to]
impl DocumentElement for ImplicitThrowEvent {
    fn element(&self) -> Element {
        Element::ImplicitThrowEvent
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ImplicitThrowEvent {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {ImplicitThrowEvent => ThrowEventType,ThrowEventTypeMut}
castable_to! {ImplicitThrowEvent => EventType,EventTypeMut}
castable_to! {ImplicitThrowEvent => FlowNodeType,FlowNodeTypeMut}
castable_to! {ImplicitThrowEvent => FlowElementType,FlowElementTypeMut}
castable_to! {ImplicitThrowEvent => BaseElementType,BaseElementTypeMut}
//

/// Access to `implicitThrowEvent`
pub trait ImplicitThrowEventType: ThrowEventType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ImplicitThrowEventType);
impl_downcast!(ImplicitThrowEventType);
/// Mutable access to `implicitThrowEvent`
pub trait ImplicitThrowEventTypeMut:
    ThrowEventTypeMut + Downcast + Debug + Send + DynClone + ImplicitThrowEventType
{
}
dyn_clone::clone_trait_object!(ImplicitThrowEventTypeMut);
impl_downcast!(ImplicitThrowEventTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:inclusiveGateway")]
pub struct InclusiveGateway {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "gatewayDirection")]
    #[tia("GatewayType",rg*="gateway_direction","GatewayTypeMut",s)]
    pub gateway_direction: Option<String>,
    #[xml(attr = "default")]
    #[tia("InclusiveGatewayType",rg*="default","InclusiveGatewayTypeMut",s)]
    pub default: Option<String>,
}
#[cast_to]
impl DocumentElement for InclusiveGateway {
    fn element(&self) -> Element {
        Element::InclusiveGateway
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for InclusiveGateway {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {InclusiveGateway => GatewayType,GatewayTypeMut}
castable_to! {InclusiveGateway => FlowNodeType,FlowNodeTypeMut}
castable_to! {InclusiveGateway => FlowElementType,FlowElementTypeMut}
castable_to! {InclusiveGateway => BaseElementType,BaseElementTypeMut}
//

/// Access to `inclusiveGateway`
pub trait InclusiveGatewayType: GatewayType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `default`
    fn default(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(InclusiveGatewayType);
impl_downcast!(InclusiveGatewayType);
/// Mutable access to `inclusiveGateway`
pub trait InclusiveGatewayTypeMut:
    GatewayTypeMut + Downcast + Debug + Send + DynClone + InclusiveGatewayType
{
    /// Set value of attribute `default`
    fn set_default(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(InclusiveGatewayTypeMut);
impl_downcast!(InclusiveGatewayTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:inputSet")]
pub struct InputSet {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("InputSetType",rg*="name","InputSetTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:dataInputRefs")]
    #[tia("InputSetType",rg*="data_input_refss","InputSetTypeMut",s,rmg*="data_input_refss_mut")]
    pub data_input_refss: Vec<String>,
    #[xml(flatten_text = "bpmn:optionalInputRefs")]
    #[tia("InputSetType",rg*="optional_input_refss","InputSetTypeMut",s,rmg*="optional_input_refss_mut")]
    pub optional_input_refss: Vec<String>,
    #[xml(flatten_text = "bpmn:whileExecutingInputRefs")]
    #[tia("InputSetType",rg*="while_executing_input_refss","InputSetTypeMut",s,rmg*="while_executing_input_refss_mut")]
    pub while_executing_input_refss: Vec<String>,
    #[xml(flatten_text = "bpmn:outputSetRefs")]
    #[tia("InputSetType",rg*="output_set_refss","InputSetTypeMut",s,rmg*="output_set_refss_mut")]
    pub output_set_refss: Vec<String>,
}
#[cast_to]
impl DocumentElement for InputSet {
    fn element(&self) -> Element {
        Element::InputSet
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for InputSet {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {InputSet => BaseElementType,BaseElementTypeMut}
//

/// Access to `inputSet`
pub trait InputSetType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `dataInputRefs` child
    fn data_input_refss(&self) -> &Vec<String>;
    /// Get value of `optionalInputRefs` child
    fn optional_input_refss(&self) -> &Vec<String>;
    /// Get value of `whileExecutingInputRefs` child
    fn while_executing_input_refss(&self) -> &Vec<String>;
    /// Get value of `outputSetRefs` child
    fn output_set_refss(&self) -> &Vec<String>;
}
dyn_clone::clone_trait_object!(InputSetType);
impl_downcast!(InputSetType);
/// Mutable access to `inputSet`
pub trait InputSetTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + InputSetType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Get a mutable value of `dataInputRefs` child
    fn data_input_refss_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `dataInputRefs` child
    fn set_data_input_refss(&mut self, value: Vec<String>);
    /// Get a mutable value of `optionalInputRefs` child
    fn optional_input_refss_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `optionalInputRefs` child
    fn set_optional_input_refss(&mut self, value: Vec<String>);
    /// Get a mutable value of `whileExecutingInputRefs` child
    fn while_executing_input_refss_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `whileExecutingInputRefs` child
    fn set_while_executing_input_refss(&mut self, value: Vec<String>);
    /// Get a mutable value of `outputSetRefs` child
    fn output_set_refss_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `outputSetRefs` child
    fn set_output_set_refss(&mut self, value: Vec<String>);
}
dyn_clone::clone_trait_object!(InputSetTypeMut);
impl_downcast!(InputSetTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:interface")]
pub struct Interface {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("InterfaceType",rg*="name","InterfaceTypeMut",s)]
    pub name: String,
    #[xml(attr = "implementationRef")]
    #[tia("InterfaceType",rg*="implementation_ref","InterfaceTypeMut",s)]
    pub implementation_ref: Option<String>,
    #[xml(child = "bpmn:operation")]
    #[tia("InterfaceType",rg*="operations","InterfaceTypeMut",s,rmg*="operations_mut")]
    pub operations: Vec<Operation>,
}
#[cast_to]
impl DocumentElement for Interface {
    fn element(&self) -> Element {
        Element::Interface
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Interface {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.operations.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
#[cast_to]
impl RootElementType for Interface {}
#[cast_to]
impl RootElementTypeMut for Interface {}
castable_to! {Interface => PartialEq<Interface> }
castable_to! {Interface => RootElementType,RootElementTypeMut}
castable_to! {Interface => BaseElementType,BaseElementTypeMut}
//

/// Access to `interface`
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
/// Mutable access to `interface`
pub trait InterfaceTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + InterfaceType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: String);
    /// Set value of attribute `implementationRef`
    fn set_implementation_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `operation` child
    fn operations_mut(&mut self) -> &mut Vec<Operation>;
    /// Set value of `operation` child
    fn set_operations(&mut self, value: Vec<Operation>);
}
dyn_clone::clone_trait_object!(InterfaceTypeMut);
impl_downcast!(InterfaceTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:intermediateCatchEvent")]
pub struct IntermediateCatchEvent {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(child = "bpmn:property")]
    #[tia("EventType",rg*="properies","EventTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(attr = "parallelMultiple")]
    #[tia("CatchEventType",rg*="parallel_multiple","CatchEventTypeMut",s)]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    #[tia("CatchEventType",rg*="data_outputs","CatchEventTypeMut",s,rmg*="data_outputs_mut")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("CatchEventType",rg*="data_output_associations","CatchEventTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:outputSet")]
    #[tia("CatchEventType",rg*="output_set","CatchEventTypeMut",s,rmg*="output_set_mut")]
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
    #[tia("CatchEventType",rg*="event_definitions","CatchEventTypeMut",s,rmg*="event_definitions_mut")]
    pub event_definitions: Vec<EventDefinition>,
    #[xml(flatten_text = "bpmn:eventDefinitionRef")]
    #[tia("CatchEventType",rg*="event_definition_refs","CatchEventTypeMut",s,rmg*="event_definition_refs_mut")]
    pub event_definition_refs: Vec<String>,
}
#[cast_to]
impl DocumentElement for IntermediateCatchEvent {
    fn element(&self) -> Element {
        Element::IntermediateCatchEvent
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for IntermediateCatchEvent {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {IntermediateCatchEvent => CatchEventType,CatchEventTypeMut}
castable_to! {IntermediateCatchEvent => EventType,EventTypeMut}
castable_to! {IntermediateCatchEvent => FlowNodeType,FlowNodeTypeMut}
castable_to! {IntermediateCatchEvent => FlowElementType,FlowElementTypeMut}
castable_to! {IntermediateCatchEvent => BaseElementType,BaseElementTypeMut}
//

/// Access to `intermediateCatchEvent`
pub trait IntermediateCatchEventType: CatchEventType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(IntermediateCatchEventType);
impl_downcast!(IntermediateCatchEventType);
/// Mutable access to `intermediateCatchEvent`
pub trait IntermediateCatchEventTypeMut:
    CatchEventTypeMut + Downcast + Debug + Send + DynClone + IntermediateCatchEventType
{
}
dyn_clone::clone_trait_object!(IntermediateCatchEventTypeMut);
impl_downcast!(IntermediateCatchEventTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:intermediateThrowEvent")]
pub struct IntermediateThrowEvent {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(child = "bpmn:property")]
    #[tia("EventType",rg*="properies","EventTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInput")]
    #[tia("ThrowEventType",rg*="data_inputs","ThrowEventTypeMut",s,rmg*="data_inputs_mut")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ThrowEventType",rg*="data_input_associations","ThrowEventTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:inputSet")]
    #[tia("ThrowEventType",rg*="input_set","ThrowEventTypeMut",s,rmg*="input_set_mut")]
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
    #[tia("ThrowEventType",rg*="event_definitions","ThrowEventTypeMut",s,rmg*="event_definitions_mut")]
    pub event_definitions: Vec<EventDefinition>,
    #[xml(flatten_text = "bpmn:eventDefinitionRef")]
    #[tia("ThrowEventType",rg*="event_definition_refs","ThrowEventTypeMut",s,rmg*="event_definition_refs_mut")]
    pub event_definition_refs: Vec<String>,
}
#[cast_to]
impl DocumentElement for IntermediateThrowEvent {
    fn element(&self) -> Element {
        Element::IntermediateThrowEvent
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for IntermediateThrowEvent {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {IntermediateThrowEvent => ThrowEventType,ThrowEventTypeMut}
castable_to! {IntermediateThrowEvent => EventType,EventTypeMut}
castable_to! {IntermediateThrowEvent => FlowNodeType,FlowNodeTypeMut}
castable_to! {IntermediateThrowEvent => FlowElementType,FlowElementTypeMut}
castable_to! {IntermediateThrowEvent => BaseElementType,BaseElementTypeMut}
//

/// Access to `intermediateThrowEvent`
pub trait IntermediateThrowEventType: ThrowEventType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(IntermediateThrowEventType);
impl_downcast!(IntermediateThrowEventType);
/// Mutable access to `intermediateThrowEvent`
pub trait IntermediateThrowEventTypeMut:
    ThrowEventTypeMut + Downcast + Debug + Send + DynClone + IntermediateThrowEventType
{
}
dyn_clone::clone_trait_object!(IntermediateThrowEventTypeMut);
impl_downcast!(IntermediateThrowEventTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:ioBinding")]
pub struct InputOutputBinding {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "operationRef")]
    #[tia("InputOutputBindingType",rg*="operation_ref","InputOutputBindingTypeMut",s)]
    pub operation_ref: String,
    #[xml(attr = "inputDataRef")]
    #[tia("InputOutputBindingType",rg*="input_data_ref","InputOutputBindingTypeMut",s)]
    pub input_data_ref: String,
    #[xml(attr = "outputDataRef")]
    #[tia("InputOutputBindingType",rg*="output_data_ref","InputOutputBindingTypeMut",s)]
    pub output_data_ref: String,
}
#[cast_to]
impl DocumentElement for InputOutputBinding {
    fn element(&self) -> Element {
        Element::IoBinding
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for InputOutputBinding {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {InputOutputBinding => BaseElementType,BaseElementTypeMut}
//

/// Access to `ioBinding`
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
/// Mutable access to `ioBinding`
pub trait InputOutputBindingTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + InputOutputBindingType
{
    /// Set value of attribute `operationRef`
    fn set_operation_ref(&mut self, value: String);
    /// Set value of attribute `inputDataRef`
    fn set_input_data_ref(&mut self, value: String);
    /// Set value of attribute `outputDataRef`
    fn set_output_data_ref(&mut self, value: String);
}
dyn_clone::clone_trait_object!(InputOutputBindingTypeMut);
impl_downcast!(InputOutputBindingTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:ioSpecification")]
pub struct InputOutputSpecification {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:dataInput")]
    #[tia("InputOutputSpecificationType",rg*="data_inputs","InputOutputSpecificationTypeMut",s,rmg*="data_inputs_mut")]
    pub data_inputs: Vec<DataInput>,
    #[xml(child = "bpmn:dataOutput")]
    #[tia("InputOutputSpecificationType",rg*="data_outputs","InputOutputSpecificationTypeMut",s,rmg*="data_outputs_mut")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:inputSet")]
    #[tia("InputOutputSpecificationType",rg*="input_sets","InputOutputSpecificationTypeMut",s,rmg*="input_sets_mut")]
    pub input_sets: Vec<InputSet>,
    #[xml(child = "bpmn:outputSet")]
    #[tia("InputOutputSpecificationType",rg*="output_sets","InputOutputSpecificationTypeMut",s,rmg*="output_sets_mut")]
    pub output_sets: Vec<OutputSet>,
}
#[cast_to]
impl DocumentElement for InputOutputSpecification {
    fn element(&self) -> Element {
        Element::IoSpecification
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for InputOutputSpecification {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_inputs.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.data_outputs.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.input_sets.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.output_sets.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {InputOutputSpecification => BaseElementType,BaseElementTypeMut}
//

/// Access to `ioSpecification`
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
/// Mutable access to `ioSpecification`
pub trait InputOutputSpecificationTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + InputOutputSpecificationType
{
    /// Get a mutable value of `dataInput` child
    fn data_inputs_mut(&mut self) -> &mut Vec<DataInput>;
    /// Set value of `dataInput` child
    fn set_data_inputs(&mut self, value: Vec<DataInput>);
    /// Get a mutable value of `dataOutput` child
    fn data_outputs_mut(&mut self) -> &mut Vec<DataOutput>;
    /// Set value of `dataOutput` child
    fn set_data_outputs(&mut self, value: Vec<DataOutput>);
    /// Get a mutable value of `inputSet` child
    fn input_sets_mut(&mut self) -> &mut Vec<InputSet>;
    /// Set value of `inputSet` child
    fn set_input_sets(&mut self, value: Vec<InputSet>);
    /// Get a mutable value of `outputSet` child
    fn output_sets_mut(&mut self) -> &mut Vec<OutputSet>;
    /// Set value of `outputSet` child
    fn set_output_sets(&mut self, value: Vec<OutputSet>);
}
dyn_clone::clone_trait_object!(InputOutputSpecificationTypeMut);
impl_downcast!(InputOutputSpecificationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:itemDefinition")]
pub struct ItemDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "structureRef")]
    #[tia("ItemDefinitionType",rg*="structure_ref","ItemDefinitionTypeMut",s)]
    pub structure_ref: Option<String>,
    #[xml(attr = "isCollection")]
    #[tia("ItemDefinitionType",rg*="is_collection","ItemDefinitionTypeMut",s)]
    pub is_collection: Option<bool>,
    #[xml(attr = "itemKind")]
    #[tia("ItemDefinitionType",rg*="item_kind","ItemDefinitionTypeMut",s)]
    pub item_kind: Option<String>,
}
#[cast_to]
impl DocumentElement for ItemDefinition {
    fn element(&self) -> Element {
        Element::ItemDefinition
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ItemDefinition {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl RootElementType for ItemDefinition {}
#[cast_to]
impl RootElementTypeMut for ItemDefinition {}
castable_to! {ItemDefinition => PartialEq<ItemDefinition> }
castable_to! {ItemDefinition => RootElementType,RootElementTypeMut}
castable_to! {ItemDefinition => BaseElementType,BaseElementTypeMut}
//

/// Access to `itemDefinition`
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
/// Mutable access to `itemDefinition`
pub trait ItemDefinitionTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + ItemDefinitionType
{
    /// Set value of attribute `structureRef`
    fn set_structure_ref(&mut self, value: Option<String>);
    /// Set value of attribute `isCollection`
    fn set_is_collection(&mut self, value: Option<bool>);
    /// Set value of attribute `itemKind`
    fn set_item_kind(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(ItemDefinitionTypeMut);
impl_downcast!(ItemDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:lane")]
pub struct Lane {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("LaneType",rg*="name","LaneTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "partitionElementRef")]
    #[tia("LaneType",rg*="partition_element_ref","LaneTypeMut",s)]
    pub partition_element_ref: Option<String>,
    #[xml(child = "bpmn:partitionElement")]
    #[tia("LaneType",rg*="partition_element","LaneTypeMut",s,rmg*="partition_element_mut")]
    pub partition_element: Option<BaseElement>,
    #[xml(flatten_text = "bpmn:flowNodeRef")]
    #[tia("LaneType",rg*="flow_node_refs","LaneTypeMut",s,rmg*="flow_node_refs_mut")]
    pub flow_node_refs: Vec<String>,
    #[xml(child = "bpmn:childLaneSet")]
    #[tia("LaneType",rg*="child_lane_set","LaneTypeMut",s,rmg*="child_lane_set_mut")]
    pub child_lane_set: Option<LaneSet>,
}
#[cast_to]
impl DocumentElement for Lane {
    fn element(&self) -> Element {
        Element::Lane
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Lane {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.partition_element.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.child_lane_set.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.partition_element.find_by_id(id) {
            return Some(e);
        }
        if let Some(e) = self.child_lane_set.find_by_id(id) {
            return Some(e);
        }
        None
    }
}
// Traits
castable_to! {Lane => BaseElementType,BaseElementTypeMut}
//

/// Access to `lane`
pub trait LaneType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `partitionElementRef`
    fn partition_element_ref(&self) -> &Option<String>;
    /// Get value of `partitionElement` child
    fn partition_element(&self) -> &Option<BaseElement>;
    /// Get value of `flowNodeRef` child
    fn flow_node_refs(&self) -> &Vec<String>;
    /// Get value of `childLaneSet` child
    fn child_lane_set(&self) -> &Option<LaneSet>;
}
dyn_clone::clone_trait_object!(LaneType);
impl_downcast!(LaneType);
/// Mutable access to `lane`
pub trait LaneTypeMut: BaseElementTypeMut + Downcast + Debug + Send + DynClone + LaneType {
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `partitionElementRef`
    fn set_partition_element_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `partitionElement` child
    fn partition_element_mut(&mut self) -> &mut Option<BaseElement>;
    /// Set value of `partitionElement` child
    fn set_partition_element(&mut self, value: Option<BaseElement>);
    /// Get a mutable value of `flowNodeRef` child
    fn flow_node_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `flowNodeRef` child
    fn set_flow_node_refs(&mut self, value: Vec<String>);
    /// Get a mutable value of `childLaneSet` child
    fn child_lane_set_mut(&mut self) -> &mut Option<LaneSet>;
    /// Set value of `childLaneSet` child
    fn set_child_lane_set(&mut self, value: Option<LaneSet>);
}
dyn_clone::clone_trait_object!(LaneTypeMut);
impl_downcast!(LaneTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:laneSet")]
pub struct LaneSet {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("LaneSetType",rg*="name","LaneSetTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:lane")]
    #[tia("LaneSetType",rg*="lanes","LaneSetTypeMut",s,rmg*="lanes_mut")]
    pub lanes: Vec<Lane>,
}
#[cast_to]
impl DocumentElement for LaneSet {
    fn element(&self) -> Element {
        Element::LaneSet
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for LaneSet {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.lanes.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {LaneSet => BaseElementType,BaseElementTypeMut}
//

/// Access to `laneSet`
pub trait LaneSetType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `lane` child
    fn lanes(&self) -> &Vec<Lane>;
}
dyn_clone::clone_trait_object!(LaneSetType);
impl_downcast!(LaneSetType);
/// Mutable access to `laneSet`
pub trait LaneSetTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + LaneSetType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Get a mutable value of `lane` child
    fn lanes_mut(&mut self) -> &mut Vec<Lane>;
    /// Set value of `lane` child
    fn set_lanes(&mut self, value: Vec<Lane>);
}
dyn_clone::clone_trait_object!(LaneSetTypeMut);
impl_downcast!(LaneSetTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:linkEventDefinition")]
pub struct LinkEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("LinkEventDefinitionType",rg*="name","LinkEventDefinitionTypeMut",s)]
    pub name: String,
    #[xml(flatten_text = "bpmn:source")]
    #[tia("LinkEventDefinitionType",rg*="sources","LinkEventDefinitionTypeMut",s,rmg*="sources_mut")]
    pub sources: Vec<String>,
    #[xml(flatten_text = "bpmn:target")]
    #[tia("LinkEventDefinitionType",rg*="target","LinkEventDefinitionTypeMut",s,rmg*="target_mut")]
    pub target: Option<String>,
}
#[cast_to]
impl DocumentElement for LinkEventDefinition {
    fn element(&self) -> Element {
        Element::LinkEventDefinition
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for LinkEventDefinition {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl EventDefinitionType for LinkEventDefinition {}
#[cast_to]
impl EventDefinitionTypeMut for LinkEventDefinition {}
castable_to! {LinkEventDefinition => PartialEq<LinkEventDefinition> }
castable_to! {LinkEventDefinition => EventDefinitionType,EventDefinitionTypeMut}
#[cast_to]
impl RootElementType for LinkEventDefinition {}
#[cast_to]
impl RootElementTypeMut for LinkEventDefinition {}
castable_to! {LinkEventDefinition => PartialEq<LinkEventDefinition> }
castable_to! {LinkEventDefinition => RootElementType,RootElementTypeMut}
castable_to! {LinkEventDefinition => BaseElementType,BaseElementTypeMut}
//

/// Access to `linkEventDefinition`
pub trait LinkEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `name`
    fn name(&self) -> &String;
    /// Get value of `source` child
    fn sources(&self) -> &Vec<String>;
    /// Get value of `target` child
    fn target(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(LinkEventDefinitionType);
impl_downcast!(LinkEventDefinitionType);
/// Mutable access to `linkEventDefinition`
pub trait LinkEventDefinitionTypeMut:
    EventDefinitionTypeMut + Downcast + Debug + Send + DynClone + LinkEventDefinitionType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: String);
    /// Get a mutable value of `source` child
    fn sources_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `source` child
    fn set_sources(&mut self, value: Vec<String>);
    /// Get a mutable value of `target` child
    fn target_mut(&mut self) -> &mut Option<String>;
    /// Set value of `target` child
    fn set_target(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(LinkEventDefinitionTypeMut);
impl_downcast!(LinkEventDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:loopCharacteristics")]
#[serde(tag = "type")]
pub enum LoopCharacteristics {
    #[xml(tag = "bpmn:multiInstanceLoopCharacteristics")]
    MultiInstanceLoopCharacteristics(MultiInstanceLoopCharacteristics),
    #[xml(tag = "bpmn:standardLoopCharacteristics")]
    StandardLoopCharacteristics(StandardLoopCharacteristics),
}
impl From<MultiInstanceLoopCharacteristics> for LoopCharacteristics {
    fn from(element: MultiInstanceLoopCharacteristics) -> Self {
        Self::MultiInstanceLoopCharacteristics(element)
    }
}
impl From<StandardLoopCharacteristics> for LoopCharacteristics {
    fn from(element: StandardLoopCharacteristics) -> Self {
        Self::StandardLoopCharacteristics(element)
    }
}
impl LoopCharacteristics {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {
            LoopCharacteristics::MultiInstanceLoopCharacteristics(e) => {
                Box::new(e) as Box<dyn DocumentElement>
            }
            LoopCharacteristics::StandardLoopCharacteristics(e) => {
                Box::new(e) as Box<dyn DocumentElement>
            }
        }
    }
}
#[cast_to]
impl DocumentElementContainer for LoopCharacteristics {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            LoopCharacteristics::MultiInstanceLoopCharacteristics(e) => e.find_by_id_mut(id),
            LoopCharacteristics::StandardLoopCharacteristics(e) => e.find_by_id_mut(id),

            _ => None,
        }
    }

    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            LoopCharacteristics::MultiInstanceLoopCharacteristics(e) => e.find_by_id(id),
            LoopCharacteristics::StandardLoopCharacteristics(e) => e.find_by_id(id),

            _ => None,
        }
    }
}
#[cast_to]
impl DocumentElement for LoopCharacteristics {
    fn element(&self) -> Element {
        Element::LoopCharacteristics
    }
}
/// Access to `loopCharacteristics`
pub trait LoopCharacteristicsType: BaseElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(LoopCharacteristicsType);
impl_downcast!(LoopCharacteristicsType);
/// Mutable access to `loopCharacteristics`
pub trait LoopCharacteristicsTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + LoopCharacteristicsType
{
}
dyn_clone::clone_trait_object!(LoopCharacteristicsTypeMut);
impl_downcast!(LoopCharacteristicsTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:manualTask")]
pub struct ManualTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation","ActivityTypeMut",s)]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity","ActivityTypeMut",s)]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity","ActivityTypeMut",s)]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default","ActivityTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification","ActivityTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies","ActivityTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations","ActivityTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations","ActivityTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles","ActivityTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics","ActivityTypeMut",s,rmg*="loop_characteristics_mut")]
    pub loop_characteristics: Option<LoopCharacteristics>,
}
#[cast_to]
impl DocumentElement for ManualTask {
    fn element(&self) -> Element {
        Element::ManualTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ManualTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl TaskType for ManualTask {}
#[cast_to]
impl TaskTypeMut for ManualTask {}
castable_to! {ManualTask => PartialEq<ManualTask> }
castable_to! {ManualTask => TaskType,TaskTypeMut}
castable_to! {ManualTask => ActivityType,ActivityTypeMut}
castable_to! {ManualTask => FlowNodeType,FlowNodeTypeMut}
castable_to! {ManualTask => FlowElementType,FlowElementTypeMut}
castable_to! {ManualTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `manualTask`
pub trait ManualTaskType: TaskType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ManualTaskType);
impl_downcast!(ManualTaskType);
/// Mutable access to `manualTask`
pub trait ManualTaskTypeMut:
    TaskTypeMut + Downcast + Debug + Send + DynClone + ManualTaskType
{
}
dyn_clone::clone_trait_object!(ManualTaskTypeMut);
impl_downcast!(ManualTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:message")]
pub struct Message {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("MessageType",rg*="name","MessageTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "itemRef")]
    #[tia("MessageType",rg*="item_ref","MessageTypeMut",s)]
    pub item_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for Message {
    fn element(&self) -> Element {
        Element::Message
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Message {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl RootElementType for Message {}
#[cast_to]
impl RootElementTypeMut for Message {}
castable_to! {Message => PartialEq<Message> }
castable_to! {Message => RootElementType,RootElementTypeMut}
castable_to! {Message => BaseElementType,BaseElementTypeMut}
//

/// Access to `message`
pub trait MessageType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `itemRef`
    fn item_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(MessageType);
impl_downcast!(MessageType);
/// Mutable access to `message`
pub trait MessageTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + MessageType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `itemRef`
    fn set_item_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(MessageTypeMut);
impl_downcast!(MessageTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:messageEventDefinition")]
pub struct MessageEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "messageRef")]
    #[tia("MessageEventDefinitionType",rg*="message_ref","MessageEventDefinitionTypeMut",s)]
    pub message_ref: Option<String>,
    #[xml(flatten_text = "bpmn:operationRef")]
    #[tia("MessageEventDefinitionType",rg*="operation_ref","MessageEventDefinitionTypeMut",s,rmg*="operation_ref_mut")]
    pub operation_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for MessageEventDefinition {
    fn element(&self) -> Element {
        Element::MessageEventDefinition
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for MessageEventDefinition {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl EventDefinitionType for MessageEventDefinition {}
#[cast_to]
impl EventDefinitionTypeMut for MessageEventDefinition {}
castable_to! {MessageEventDefinition => PartialEq<MessageEventDefinition> }
castable_to! {MessageEventDefinition => EventDefinitionType,EventDefinitionTypeMut}
#[cast_to]
impl RootElementType for MessageEventDefinition {}
#[cast_to]
impl RootElementTypeMut for MessageEventDefinition {}
castable_to! {MessageEventDefinition => PartialEq<MessageEventDefinition> }
castable_to! {MessageEventDefinition => RootElementType,RootElementTypeMut}
castable_to! {MessageEventDefinition => BaseElementType,BaseElementTypeMut}
//

/// Access to `messageEventDefinition`
pub trait MessageEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `messageRef`
    fn message_ref(&self) -> &Option<String>;
    /// Get value of `operationRef` child
    fn operation_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(MessageEventDefinitionType);
impl_downcast!(MessageEventDefinitionType);
/// Mutable access to `messageEventDefinition`
pub trait MessageEventDefinitionTypeMut:
    EventDefinitionTypeMut + Downcast + Debug + Send + DynClone + MessageEventDefinitionType
{
    /// Set value of attribute `messageRef`
    fn set_message_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `operationRef` child
    fn operation_ref_mut(&mut self) -> &mut Option<String>;
    /// Set value of `operationRef` child
    fn set_operation_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(MessageEventDefinitionTypeMut);
impl_downcast!(MessageEventDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:messageFlow")]
pub struct MessageFlow {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("MessageFlowType",rg*="name","MessageFlowTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "sourceRef")]
    #[tia("MessageFlowType",rg*="source_ref","MessageFlowTypeMut",s)]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    #[tia("MessageFlowType",rg*="target_ref","MessageFlowTypeMut",s)]
    pub target_ref: String,
    #[xml(attr = "messageRef")]
    #[tia("MessageFlowType",rg*="message_ref","MessageFlowTypeMut",s)]
    pub message_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for MessageFlow {
    fn element(&self) -> Element {
        Element::MessageFlow
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for MessageFlow {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {MessageFlow => BaseElementType,BaseElementTypeMut}
//

/// Access to `messageFlow`
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
/// Mutable access to `messageFlow`
pub trait MessageFlowTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + MessageFlowType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `sourceRef`
    fn set_source_ref(&mut self, value: String);
    /// Set value of attribute `targetRef`
    fn set_target_ref(&mut self, value: String);
    /// Set value of attribute `messageRef`
    fn set_message_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(MessageFlowTypeMut);
impl_downcast!(MessageFlowTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:messageFlowAssociation")]
pub struct MessageFlowAssociation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "innerMessageFlowRef")]
    #[tia("MessageFlowAssociationType",rg*="inner_message_flow_ref","MessageFlowAssociationTypeMut",s)]
    pub inner_message_flow_ref: String,
    #[xml(attr = "outerMessageFlowRef")]
    #[tia("MessageFlowAssociationType",rg*="outer_message_flow_ref","MessageFlowAssociationTypeMut",s)]
    pub outer_message_flow_ref: String,
}
#[cast_to]
impl DocumentElement for MessageFlowAssociation {
    fn element(&self) -> Element {
        Element::MessageFlowAssociation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for MessageFlowAssociation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {MessageFlowAssociation => BaseElementType,BaseElementTypeMut}
//

/// Access to `messageFlowAssociation`
pub trait MessageFlowAssociationType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `innerMessageFlowRef`
    fn inner_message_flow_ref(&self) -> &String;
    /// Get value of attribute `outerMessageFlowRef`
    fn outer_message_flow_ref(&self) -> &String;
}
dyn_clone::clone_trait_object!(MessageFlowAssociationType);
impl_downcast!(MessageFlowAssociationType);
/// Mutable access to `messageFlowAssociation`
pub trait MessageFlowAssociationTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + MessageFlowAssociationType
{
    /// Set value of attribute `innerMessageFlowRef`
    fn set_inner_message_flow_ref(&mut self, value: String);
    /// Set value of attribute `outerMessageFlowRef`
    fn set_outer_message_flow_ref(&mut self, value: String);
}
dyn_clone::clone_trait_object!(MessageFlowAssociationTypeMut);
impl_downcast!(MessageFlowAssociationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:monitoring")]
pub struct Monitoring {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
}
#[cast_to]
impl DocumentElement for Monitoring {
    fn element(&self) -> Element {
        Element::Monitoring
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Monitoring {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {Monitoring => BaseElementType,BaseElementTypeMut}
//

/// Access to `monitoring`
pub trait MonitoringType: BaseElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(MonitoringType);
impl_downcast!(MonitoringType);
/// Mutable access to `monitoring`
pub trait MonitoringTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + MonitoringType
{
}
dyn_clone::clone_trait_object!(MonitoringTypeMut);
impl_downcast!(MonitoringTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:multiInstanceLoopCharacteristics")]
pub struct MultiInstanceLoopCharacteristics {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "isSequential")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="is_sequential","MultiInstanceLoopCharacteristicsTypeMut",s)]
    pub is_sequential: Option<bool>,
    #[xml(attr = "behavior")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="behavior","MultiInstanceLoopCharacteristicsTypeMut",s)]
    pub behavior: Option<String>,
    #[xml(attr = "oneBehaviorEventRef")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="one_behavior_event_ref","MultiInstanceLoopCharacteristicsTypeMut",s)]
    pub one_behavior_event_ref: Option<String>,
    #[xml(attr = "noneBehaviorEventRef")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="none_behavior_event_ref","MultiInstanceLoopCharacteristicsTypeMut",s)]
    pub none_behavior_event_ref: Option<String>,
    #[xml(child = "bpmn:loopCardinality")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="loop_cardinality","MultiInstanceLoopCharacteristicsTypeMut",s,rmg*="loop_cardinality_mut")]
    pub loop_cardinality: Option<Expression>,
    #[xml(flatten_text = "bpmn:loopDataInputRef")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="loop_data_input_ref","MultiInstanceLoopCharacteristicsTypeMut",s,rmg*="loop_data_input_ref_mut")]
    pub loop_data_input_ref: Option<String>,
    #[xml(flatten_text = "bpmn:loopDataOutputRef")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="loop_data_output_ref","MultiInstanceLoopCharacteristicsTypeMut",s,rmg*="loop_data_output_ref_mut")]
    pub loop_data_output_ref: Option<String>,
    #[xml(child = "bpmn:inputDataItem")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="input_data_item","MultiInstanceLoopCharacteristicsTypeMut",s,rmg*="input_data_item_mut")]
    pub input_data_item: Option<DataInput>,
    #[xml(child = "bpmn:outputDataItem")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="output_data_item","MultiInstanceLoopCharacteristicsTypeMut",s,rmg*="output_data_item_mut")]
    pub output_data_item: Option<DataOutput>,
    #[xml(child = "bpmn:complexBehaviorDefinition")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="complex_behavior_definitions","MultiInstanceLoopCharacteristicsTypeMut",s,rmg*="complex_behavior_definitions_mut")]
    pub complex_behavior_definitions: Vec<ComplexBehaviorDefinition>,
    #[xml(child = "bpmn:completionCondition")]
    #[tia("MultiInstanceLoopCharacteristicsType",rg*="completion_condition","MultiInstanceLoopCharacteristicsTypeMut",s,rmg*="completion_condition_mut")]
    pub completion_condition: Option<Expression>,
}
#[cast_to]
impl DocumentElement for MultiInstanceLoopCharacteristics {
    fn element(&self) -> Element {
        Element::MultiInstanceLoopCharacteristics
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for MultiInstanceLoopCharacteristics {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.loop_cardinality.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.input_data_item.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.output_data_item.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.complex_behavior_definitions.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.completion_condition.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.loop_cardinality.find_by_id(id) {
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
#[cast_to]
impl LoopCharacteristicsType for MultiInstanceLoopCharacteristics {}
#[cast_to]
impl LoopCharacteristicsTypeMut for MultiInstanceLoopCharacteristics {}
castable_to! {MultiInstanceLoopCharacteristics => PartialEq<MultiInstanceLoopCharacteristics> }
castable_to! {MultiInstanceLoopCharacteristics => LoopCharacteristicsType,LoopCharacteristicsTypeMut}
castable_to! {MultiInstanceLoopCharacteristics => BaseElementType,BaseElementTypeMut}
//

/// Access to `multiInstanceLoopCharacteristics`
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
    fn loop_data_input_ref(&self) -> &Option<String>;
    /// Get value of `loopDataOutputRef` child
    fn loop_data_output_ref(&self) -> &Option<String>;
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
/// Mutable access to `multiInstanceLoopCharacteristics`
pub trait MultiInstanceLoopCharacteristicsTypeMut:
    LoopCharacteristicsTypeMut
    + Downcast
    + Debug
    + Send
    + DynClone
    + MultiInstanceLoopCharacteristicsType
{
    /// Set value of attribute `isSequential`
    fn set_is_sequential(&mut self, value: Option<bool>);
    /// Set value of attribute `behavior`
    fn set_behavior(&mut self, value: Option<String>);
    /// Set value of attribute `oneBehaviorEventRef`
    fn set_one_behavior_event_ref(&mut self, value: Option<String>);
    /// Set value of attribute `noneBehaviorEventRef`
    fn set_none_behavior_event_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `loopCardinality` child
    fn loop_cardinality_mut(&mut self) -> &mut Option<Expression>;
    /// Set value of `loopCardinality` child
    fn set_loop_cardinality(&mut self, value: Option<Expression>);
    /// Get a mutable value of `loopDataInputRef` child
    fn loop_data_input_ref_mut(&mut self) -> &mut Option<String>;
    /// Set value of `loopDataInputRef` child
    fn set_loop_data_input_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `loopDataOutputRef` child
    fn loop_data_output_ref_mut(&mut self) -> &mut Option<String>;
    /// Set value of `loopDataOutputRef` child
    fn set_loop_data_output_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `inputDataItem` child
    fn input_data_item_mut(&mut self) -> &mut Option<DataInput>;
    /// Set value of `inputDataItem` child
    fn set_input_data_item(&mut self, value: Option<DataInput>);
    /// Get a mutable value of `outputDataItem` child
    fn output_data_item_mut(&mut self) -> &mut Option<DataOutput>;
    /// Set value of `outputDataItem` child
    fn set_output_data_item(&mut self, value: Option<DataOutput>);
    /// Get a mutable value of `complexBehaviorDefinition` child
    fn complex_behavior_definitions_mut(&mut self) -> &mut Vec<ComplexBehaviorDefinition>;
    /// Set value of `complexBehaviorDefinition` child
    fn set_complex_behavior_definitions(&mut self, value: Vec<ComplexBehaviorDefinition>);
    /// Get a mutable value of `completionCondition` child
    fn completion_condition_mut(&mut self) -> &mut Option<Expression>;
    /// Set value of `completionCondition` child
    fn set_completion_condition(&mut self, value: Option<Expression>);
}
dyn_clone::clone_trait_object!(MultiInstanceLoopCharacteristicsTypeMut);
impl_downcast!(MultiInstanceLoopCharacteristicsTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:operation")]
pub struct Operation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("OperationType",rg*="name","OperationTypeMut",s)]
    pub name: String,
    #[xml(attr = "implementationRef")]
    #[tia("OperationType",rg*="implementation_ref","OperationTypeMut",s)]
    pub implementation_ref: Option<String>,
    #[xml(flatten_text = "bpmn:inMessageRef")]
    #[tia("OperationType",rg*="in_message_ref","OperationTypeMut",s,rmg*="in_message_ref_mut")]
    pub in_message_ref: String,
    #[xml(flatten_text = "bpmn:outMessageRef")]
    #[tia("OperationType",rg*="out_message_ref","OperationTypeMut",s,rmg*="out_message_ref_mut")]
    pub out_message_ref: Option<String>,
    #[xml(flatten_text = "bpmn:errorRef")]
    #[tia("OperationType",rg*="error_refs","OperationTypeMut",s,rmg*="error_refs_mut")]
    pub error_refs: Vec<String>,
}
#[cast_to]
impl DocumentElement for Operation {
    fn element(&self) -> Element {
        Element::Operation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Operation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {Operation => BaseElementType,BaseElementTypeMut}
//

/// Access to `operation`
pub trait OperationType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &String;
    /// Get value of attribute `implementationRef`
    fn implementation_ref(&self) -> &Option<String>;
    /// Get value of `inMessageRef` child
    fn in_message_ref(&self) -> &String;
    /// Get value of `outMessageRef` child
    fn out_message_ref(&self) -> &Option<String>;
    /// Get value of `errorRef` child
    fn error_refs(&self) -> &Vec<String>;
}
dyn_clone::clone_trait_object!(OperationType);
impl_downcast!(OperationType);
/// Mutable access to `operation`
pub trait OperationTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + OperationType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: String);
    /// Set value of attribute `implementationRef`
    fn set_implementation_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `inMessageRef` child
    fn in_message_ref_mut(&mut self) -> &mut String;
    /// Set value of `inMessageRef` child
    fn set_in_message_ref(&mut self, value: String);
    /// Get a mutable value of `outMessageRef` child
    fn out_message_ref_mut(&mut self) -> &mut Option<String>;
    /// Set value of `outMessageRef` child
    fn set_out_message_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `errorRef` child
    fn error_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `errorRef` child
    fn set_error_refs(&mut self, value: Vec<String>);
}
dyn_clone::clone_trait_object!(OperationTypeMut);
impl_downcast!(OperationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:outputSet")]
pub struct OutputSet {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("OutputSetType",rg*="name","OutputSetTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:dataOutputRefs")]
    #[tia("OutputSetType",rg*="data_output_refss","OutputSetTypeMut",s,rmg*="data_output_refss_mut")]
    pub data_output_refss: Vec<String>,
    #[xml(flatten_text = "bpmn:optionalOutputRefs")]
    #[tia("OutputSetType",rg*="optional_output_refss","OutputSetTypeMut",s,rmg*="optional_output_refss_mut")]
    pub optional_output_refss: Vec<String>,
    #[xml(flatten_text = "bpmn:whileExecutingOutputRefs")]
    #[tia("OutputSetType",rg*="while_executing_output_refss","OutputSetTypeMut",s,rmg*="while_executing_output_refss_mut")]
    pub while_executing_output_refss: Vec<String>,
    #[xml(flatten_text = "bpmn:inputSetRefs")]
    #[tia("OutputSetType",rg*="input_set_refss","OutputSetTypeMut",s,rmg*="input_set_refss_mut")]
    pub input_set_refss: Vec<String>,
}
#[cast_to]
impl DocumentElement for OutputSet {
    fn element(&self) -> Element {
        Element::OutputSet
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for OutputSet {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {OutputSet => BaseElementType,BaseElementTypeMut}
//

/// Access to `outputSet`
pub trait OutputSetType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `dataOutputRefs` child
    fn data_output_refss(&self) -> &Vec<String>;
    /// Get value of `optionalOutputRefs` child
    fn optional_output_refss(&self) -> &Vec<String>;
    /// Get value of `whileExecutingOutputRefs` child
    fn while_executing_output_refss(&self) -> &Vec<String>;
    /// Get value of `inputSetRefs` child
    fn input_set_refss(&self) -> &Vec<String>;
}
dyn_clone::clone_trait_object!(OutputSetType);
impl_downcast!(OutputSetType);
/// Mutable access to `outputSet`
pub trait OutputSetTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + OutputSetType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Get a mutable value of `dataOutputRefs` child
    fn data_output_refss_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `dataOutputRefs` child
    fn set_data_output_refss(&mut self, value: Vec<String>);
    /// Get a mutable value of `optionalOutputRefs` child
    fn optional_output_refss_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `optionalOutputRefs` child
    fn set_optional_output_refss(&mut self, value: Vec<String>);
    /// Get a mutable value of `whileExecutingOutputRefs` child
    fn while_executing_output_refss_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `whileExecutingOutputRefs` child
    fn set_while_executing_output_refss(&mut self, value: Vec<String>);
    /// Get a mutable value of `inputSetRefs` child
    fn input_set_refss_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `inputSetRefs` child
    fn set_input_set_refss(&mut self, value: Vec<String>);
}
dyn_clone::clone_trait_object!(OutputSetTypeMut);
impl_downcast!(OutputSetTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:parallelGateway")]
pub struct ParallelGateway {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "gatewayDirection")]
    #[tia("GatewayType",rg*="gateway_direction","GatewayTypeMut",s)]
    pub gateway_direction: Option<String>,
}
#[cast_to]
impl DocumentElement for ParallelGateway {
    fn element(&self) -> Element {
        Element::ParallelGateway
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ParallelGateway {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {ParallelGateway => GatewayType,GatewayTypeMut}
castable_to! {ParallelGateway => FlowNodeType,FlowNodeTypeMut}
castable_to! {ParallelGateway => FlowElementType,FlowElementTypeMut}
castable_to! {ParallelGateway => BaseElementType,BaseElementTypeMut}
//

/// Access to `parallelGateway`
pub trait ParallelGatewayType: GatewayType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ParallelGatewayType);
impl_downcast!(ParallelGatewayType);
/// Mutable access to `parallelGateway`
pub trait ParallelGatewayTypeMut:
    GatewayTypeMut + Downcast + Debug + Send + DynClone + ParallelGatewayType
{
}
dyn_clone::clone_trait_object!(ParallelGatewayTypeMut);
impl_downcast!(ParallelGatewayTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:participant")]
pub struct Participant {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ParticipantType",rg*="name","ParticipantTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "processRef")]
    #[tia("ParticipantType",rg*="process_ref","ParticipantTypeMut",s)]
    pub process_ref: Option<String>,
    #[xml(flatten_text = "bpmn:interfaceRef")]
    #[tia("ParticipantType",rg*="interface_refs","ParticipantTypeMut",s,rmg*="interface_refs_mut")]
    pub interface_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:endPointRef")]
    #[tia("ParticipantType",rg*="end_point_refs","ParticipantTypeMut",s,rmg*="end_point_refs_mut")]
    pub end_point_refs: Vec<String>,
    #[xml(child = "bpmn:participantMultiplicity")]
    #[tia("ParticipantType",rg*="participant_multiplicity","ParticipantTypeMut",s,rmg*="participant_multiplicity_mut")]
    pub participant_multiplicity: Option<ParticipantMultiplicity>,
}
#[cast_to]
impl DocumentElement for Participant {
    fn element(&self) -> Element {
        Element::Participant
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Participant {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.participant_multiplicity.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.participant_multiplicity.find_by_id(id) {
            return Some(e);
        }
        None
    }
}
// Traits
castable_to! {Participant => BaseElementType,BaseElementTypeMut}
//

/// Access to `participant`
pub trait ParticipantType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `processRef`
    fn process_ref(&self) -> &Option<String>;
    /// Get value of `interfaceRef` child
    fn interface_refs(&self) -> &Vec<String>;
    /// Get value of `endPointRef` child
    fn end_point_refs(&self) -> &Vec<String>;
    /// Get value of `participantMultiplicity` child
    fn participant_multiplicity(&self) -> &Option<ParticipantMultiplicity>;
}
dyn_clone::clone_trait_object!(ParticipantType);
impl_downcast!(ParticipantType);
/// Mutable access to `participant`
pub trait ParticipantTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + ParticipantType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `processRef`
    fn set_process_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `interfaceRef` child
    fn interface_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `interfaceRef` child
    fn set_interface_refs(&mut self, value: Vec<String>);
    /// Get a mutable value of `endPointRef` child
    fn end_point_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `endPointRef` child
    fn set_end_point_refs(&mut self, value: Vec<String>);
    /// Get a mutable value of `participantMultiplicity` child
    fn participant_multiplicity_mut(&mut self) -> &mut Option<ParticipantMultiplicity>;
    /// Set value of `participantMultiplicity` child
    fn set_participant_multiplicity(&mut self, value: Option<ParticipantMultiplicity>);
}
dyn_clone::clone_trait_object!(ParticipantTypeMut);
impl_downcast!(ParticipantTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:participantAssociation")]
pub struct ParticipantAssociation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(flatten_text = "bpmn:innerParticipantRef")]
    #[tia("ParticipantAssociationType",rg*="inner_participant_ref","ParticipantAssociationTypeMut",s,rmg*="inner_participant_ref_mut")]
    pub inner_participant_ref: String,
    #[xml(flatten_text = "bpmn:outerParticipantRef")]
    #[tia("ParticipantAssociationType",rg*="outer_participant_ref","ParticipantAssociationTypeMut",s,rmg*="outer_participant_ref_mut")]
    pub outer_participant_ref: String,
}
#[cast_to]
impl DocumentElement for ParticipantAssociation {
    fn element(&self) -> Element {
        Element::ParticipantAssociation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ParticipantAssociation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {ParticipantAssociation => BaseElementType,BaseElementTypeMut}
//

/// Access to `participantAssociation`
pub trait ParticipantAssociationType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of `innerParticipantRef` child
    fn inner_participant_ref(&self) -> &String;
    /// Get value of `outerParticipantRef` child
    fn outer_participant_ref(&self) -> &String;
}
dyn_clone::clone_trait_object!(ParticipantAssociationType);
impl_downcast!(ParticipantAssociationType);
/// Mutable access to `participantAssociation`
pub trait ParticipantAssociationTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + ParticipantAssociationType
{
    /// Get a mutable value of `innerParticipantRef` child
    fn inner_participant_ref_mut(&mut self) -> &mut String;
    /// Set value of `innerParticipantRef` child
    fn set_inner_participant_ref(&mut self, value: String);
    /// Get a mutable value of `outerParticipantRef` child
    fn outer_participant_ref_mut(&mut self) -> &mut String;
    /// Set value of `outerParticipantRef` child
    fn set_outer_participant_ref(&mut self, value: String);
}
dyn_clone::clone_trait_object!(ParticipantAssociationTypeMut);
impl_downcast!(ParticipantAssociationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:participantMultiplicity")]
pub struct ParticipantMultiplicity {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "minimum")]
    #[tia("ParticipantMultiplicityType",rg*="minimum","ParticipantMultiplicityTypeMut",s)]
    pub minimum: Option<Int>,
    #[xml(attr = "maximum")]
    #[tia("ParticipantMultiplicityType",rg*="maximum","ParticipantMultiplicityTypeMut",s)]
    pub maximum: Option<Int>,
}
#[cast_to]
impl DocumentElement for ParticipantMultiplicity {
    fn element(&self) -> Element {
        Element::ParticipantMultiplicity
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ParticipantMultiplicity {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {ParticipantMultiplicity => BaseElementType,BaseElementTypeMut}
//

/// Access to `participantMultiplicity`
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
/// Mutable access to `participantMultiplicity`
pub trait ParticipantMultiplicityTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + ParticipantMultiplicityType
{
    /// Set value of attribute `minimum`
    fn set_minimum(&mut self, value: Option<Int>);
    /// Set value of attribute `maximum`
    fn set_maximum(&mut self, value: Option<Int>);
}
dyn_clone::clone_trait_object!(ParticipantMultiplicityTypeMut);
impl_downcast!(ParticipantMultiplicityTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:partnerEntity")]
pub struct PartnerEntity {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("PartnerEntityType",rg*="name","PartnerEntityTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:participantRef")]
    #[tia("PartnerEntityType",rg*="participant_refs","PartnerEntityTypeMut",s,rmg*="participant_refs_mut")]
    pub participant_refs: Vec<String>,
}
#[cast_to]
impl DocumentElement for PartnerEntity {
    fn element(&self) -> Element {
        Element::PartnerEntity
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for PartnerEntity {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl RootElementType for PartnerEntity {}
#[cast_to]
impl RootElementTypeMut for PartnerEntity {}
castable_to! {PartnerEntity => PartialEq<PartnerEntity> }
castable_to! {PartnerEntity => RootElementType,RootElementTypeMut}
castable_to! {PartnerEntity => BaseElementType,BaseElementTypeMut}
//

/// Access to `partnerEntity`
pub trait PartnerEntityType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `participantRef` child
    fn participant_refs(&self) -> &Vec<String>;
}
dyn_clone::clone_trait_object!(PartnerEntityType);
impl_downcast!(PartnerEntityType);
/// Mutable access to `partnerEntity`
pub trait PartnerEntityTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + PartnerEntityType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Get a mutable value of `participantRef` child
    fn participant_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `participantRef` child
    fn set_participant_refs(&mut self, value: Vec<String>);
}
dyn_clone::clone_trait_object!(PartnerEntityTypeMut);
impl_downcast!(PartnerEntityTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:partnerRole")]
pub struct PartnerRole {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("PartnerRoleType",rg*="name","PartnerRoleTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:participantRef")]
    #[tia("PartnerRoleType",rg*="participant_refs","PartnerRoleTypeMut",s,rmg*="participant_refs_mut")]
    pub participant_refs: Vec<String>,
}
#[cast_to]
impl DocumentElement for PartnerRole {
    fn element(&self) -> Element {
        Element::PartnerRole
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for PartnerRole {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl RootElementType for PartnerRole {}
#[cast_to]
impl RootElementTypeMut for PartnerRole {}
castable_to! {PartnerRole => PartialEq<PartnerRole> }
castable_to! {PartnerRole => RootElementType,RootElementTypeMut}
castable_to! {PartnerRole => BaseElementType,BaseElementTypeMut}
//

/// Access to `partnerRole`
pub trait PartnerRoleType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `participantRef` child
    fn participant_refs(&self) -> &Vec<String>;
}
dyn_clone::clone_trait_object!(PartnerRoleType);
impl_downcast!(PartnerRoleType);
/// Mutable access to `partnerRole`
pub trait PartnerRoleTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + PartnerRoleType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Get a mutable value of `participantRef` child
    fn participant_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `participantRef` child
    fn set_participant_refs(&mut self, value: Vec<String>);
}
dyn_clone::clone_trait_object!(PartnerRoleTypeMut);
impl_downcast!(PartnerRoleTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:performer")]
pub struct Performer {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ResourceRoleType",rg*="name","ResourceRoleTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:resourceRef")]
    #[tia("ResourceRoleType",rg*="resource_ref","ResourceRoleTypeMut",s,rmg*="resource_ref_mut")]
    pub resource_ref: String,
    #[xml(child = "bpmn:resourceParameterBinding")]
    #[tia("ResourceRoleType",rg*="resource_parameter_bindings","ResourceRoleTypeMut",s,rmg*="resource_parameter_bindings_mut")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    #[tia("ResourceRoleType",rg*="resource_assignment_expression","ResourceRoleTypeMut",s,rmg*="resource_assignment_expression_mut")]
    pub resource_assignment_expression: Option<ResourceAssignmentExpression>,
}
#[cast_to]
impl DocumentElement for Performer {
    fn element(&self) -> Element {
        Element::Performer
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Performer {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {Performer => ResourceRoleType,ResourceRoleTypeMut}
castable_to! {Performer => BaseElementType,BaseElementTypeMut}
//

/// Access to `performer`
pub trait PerformerType: ResourceRoleType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(PerformerType);
impl_downcast!(PerformerType);
/// Mutable access to `performer`
pub trait PerformerTypeMut:
    ResourceRoleTypeMut + Downcast + Debug + Send + DynClone + PerformerType
{
}
dyn_clone::clone_trait_object!(PerformerTypeMut);
impl_downcast!(PerformerTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:potentialOwner")]
pub struct PotentialOwner {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ResourceRoleType",rg*="name","ResourceRoleTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:resourceRef")]
    #[tia("ResourceRoleType",rg*="resource_ref","ResourceRoleTypeMut",s,rmg*="resource_ref_mut")]
    pub resource_ref: String,
    #[xml(child = "bpmn:resourceParameterBinding")]
    #[tia("ResourceRoleType",rg*="resource_parameter_bindings","ResourceRoleTypeMut",s,rmg*="resource_parameter_bindings_mut")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    #[tia("ResourceRoleType",rg*="resource_assignment_expression","ResourceRoleTypeMut",s,rmg*="resource_assignment_expression_mut")]
    pub resource_assignment_expression: Option<ResourceAssignmentExpression>,
}
#[cast_to]
impl DocumentElement for PotentialOwner {
    fn element(&self) -> Element {
        Element::PotentialOwner
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for PotentialOwner {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl HumanPerformerType for PotentialOwner {}
#[cast_to]
impl HumanPerformerTypeMut for PotentialOwner {}
castable_to! {PotentialOwner => PartialEq<PotentialOwner> }
castable_to! {PotentialOwner => HumanPerformerType,HumanPerformerTypeMut}
#[cast_to]
impl PerformerType for PotentialOwner {}
#[cast_to]
impl PerformerTypeMut for PotentialOwner {}
castable_to! {PotentialOwner => PartialEq<PotentialOwner> }
castable_to! {PotentialOwner => PerformerType,PerformerTypeMut}
castable_to! {PotentialOwner => ResourceRoleType,ResourceRoleTypeMut}
castable_to! {PotentialOwner => BaseElementType,BaseElementTypeMut}
//

/// Access to `potentialOwner`
pub trait PotentialOwnerType: HumanPerformerType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(PotentialOwnerType);
impl_downcast!(PotentialOwnerType);
/// Mutable access to `potentialOwner`
pub trait PotentialOwnerTypeMut:
    HumanPerformerTypeMut + Downcast + Debug + Send + DynClone + PotentialOwnerType
{
}
dyn_clone::clone_trait_object!(PotentialOwnerTypeMut);
impl_downcast!(PotentialOwnerTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:process")]
pub struct Process {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("CallableElementType",rg*="name","CallableElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:supportedInterfaceRef")]
    #[tia("CallableElementType",rg*="supported_interface_refs","CallableElementTypeMut",s,rmg*="supported_interface_refs_mut")]
    pub supported_interface_refs: Vec<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("CallableElementType",rg*="io_specification","CallableElementTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:ioBinding")]
    #[tia("CallableElementType",rg*="io_bindings","CallableElementTypeMut",s,rmg*="io_bindings_mut")]
    pub io_bindings: Vec<InputOutputBinding>,
    #[xml(attr = "processType")]
    #[tia("ProcessType",rg*="process_type","ProcessTypeMut",s)]
    pub process_type: Option<String>,
    #[xml(attr = "isClosed")]
    #[tia("ProcessType",rg*="is_closed","ProcessTypeMut",s)]
    pub is_closed: Option<bool>,
    #[xml(attr = "isExecutable")]
    #[tia("ProcessType",rg*="is_executable","ProcessTypeMut",s)]
    pub is_executable: Option<bool>,
    #[xml(attr = "definitionalCollaborationRef")]
    #[tia("ProcessType",rg*="definitional_collaboration_ref","ProcessTypeMut",s)]
    pub definitional_collaboration_ref: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("ProcessType",rg*="auditing","ProcessTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("ProcessType",rg*="monitoring","ProcessTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(child = "bpmn:property")]
    #[tia("ProcessType",rg*="properies","ProcessTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:laneSet")]
    #[tia("ProcessType",rg*="lane_sets","ProcessTypeMut",s,rmg*="lane_sets_mut")]
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
    #[tia("ProcessType",rg*="flow_elements","ProcessTypeMut",s,rmg*="flow_elements_mut")]
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("ProcessType",rg*="artifacts","ProcessTypeMut",s,rmg*="artifacts_mut")]
    pub artifacts: Vec<Artifact>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ProcessType",rg*="resource_roles","ProcessTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(child = "bpmn:correlationSubscription")]
    #[tia("ProcessType",rg*="correlation_subscriptions","ProcessTypeMut",s,rmg*="correlation_subscriptions_mut")]
    pub correlation_subscriptions: Vec<CorrelationSubscription>,
    #[xml(flatten_text = "bpmn:supports")]
    #[tia("ProcessType",rg*="supportss","ProcessTypeMut",s,rmg*="supportss_mut")]
    pub supportss: Vec<String>,
}
#[cast_to]
impl DocumentElement for Process {
    fn element(&self) -> Element {
        Element::Process
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Process {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.auditing.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.monitoring.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.properies.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.lane_sets.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.flow_elements.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.artifacts.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.resource_roles.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.correlation_subscriptions.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
        None
    }
}
// Traits
castable_to! {Process => CallableElementType,CallableElementTypeMut}
#[cast_to]
impl RootElementType for Process {}
#[cast_to]
impl RootElementTypeMut for Process {}
castable_to! {Process => PartialEq<Process> }
castable_to! {Process => RootElementType,RootElementTypeMut}
castable_to! {Process => BaseElementType,BaseElementTypeMut}
//

/// Access to `process`
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
    fn supportss(&self) -> &Vec<String>;
}
dyn_clone::clone_trait_object!(ProcessType);
impl_downcast!(ProcessType);
/// Mutable access to `process`
pub trait ProcessTypeMut:
    CallableElementTypeMut + Downcast + Debug + Send + DynClone + ProcessType
{
    /// Set value of attribute `processType`
    fn set_process_type(&mut self, value: Option<String>);
    /// Set value of attribute `isClosed`
    fn set_is_closed(&mut self, value: Option<bool>);
    /// Set value of attribute `isExecutable`
    fn set_is_executable(&mut self, value: Option<bool>);
    /// Set value of attribute `definitionalCollaborationRef`
    fn set_definitional_collaboration_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `auditing` child
    fn auditing_mut(&mut self) -> &mut Option<Auditing>;
    /// Set value of `auditing` child
    fn set_auditing(&mut self, value: Option<Auditing>);
    /// Get a mutable value of `monitoring` child
    fn monitoring_mut(&mut self) -> &mut Option<Monitoring>;
    /// Set value of `monitoring` child
    fn set_monitoring(&mut self, value: Option<Monitoring>);
    /// Get a mutable value of `property` child
    fn properies_mut(&mut self) -> &mut Vec<Property>;
    /// Set value of `property` child
    fn set_properies(&mut self, value: Vec<Property>);
    /// Get a mutable value of `laneSet` child
    fn lane_sets_mut(&mut self) -> &mut Vec<LaneSet>;
    /// Set value of `laneSet` child
    fn set_lane_sets(&mut self, value: Vec<LaneSet>);
    /// Get a mutable value of `flowElement` child
    fn flow_elements_mut(&mut self) -> &mut Vec<FlowElement>;
    /// Set value of `flowElement` child
    fn set_flow_elements(&mut self, value: Vec<FlowElement>);
    /// Get a mutable value of `artifact` child
    fn artifacts_mut(&mut self) -> &mut Vec<Artifact>;
    /// Set value of `artifact` child
    fn set_artifacts(&mut self, value: Vec<Artifact>);
    /// Get a mutable value of `resourceRole` child
    fn resource_roles_mut(&mut self) -> &mut Vec<ResourceRole>;
    /// Set value of `resourceRole` child
    fn set_resource_roles(&mut self, value: Vec<ResourceRole>);
    /// Get a mutable value of `correlationSubscription` child
    fn correlation_subscriptions_mut(&mut self) -> &mut Vec<CorrelationSubscription>;
    /// Set value of `correlationSubscription` child
    fn set_correlation_subscriptions(&mut self, value: Vec<CorrelationSubscription>);
    /// Get a mutable value of `supports` child
    fn supportss_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `supports` child
    fn set_supportss(&mut self, value: Vec<String>);
}
dyn_clone::clone_trait_object!(ProcessTypeMut);
impl_downcast!(ProcessTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:property")]
pub struct Property {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("PropertyType",rg*="name","PropertyTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "itemSubjectRef")]
    #[tia("PropertyType",rg*="item_subject_ref","PropertyTypeMut",s)]
    pub item_subject_ref: Option<String>,
    #[xml(child = "bpmn:dataState")]
    #[tia("PropertyType",rg*="data_state","PropertyTypeMut",s,rmg*="data_state_mut")]
    pub data_state: Option<DataState>,
}
#[cast_to]
impl DocumentElement for Property {
    fn element(&self) -> Element {
        Element::Property
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Property {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.data_state.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {Property => BaseElementType,BaseElementTypeMut}
//

/// Access to `property`
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
/// Mutable access to `property`
pub trait PropertyTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + PropertyType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `itemSubjectRef`
    fn set_item_subject_ref(&mut self, value: Option<String>);
    /// Get a mutable value of `dataState` child
    fn data_state_mut(&mut self) -> &mut Option<DataState>;
    /// Set value of `dataState` child
    fn set_data_state(&mut self, value: Option<DataState>);
}
dyn_clone::clone_trait_object!(PropertyTypeMut);
impl_downcast!(PropertyTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:receiveTask")]
pub struct ReceiveTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation","ActivityTypeMut",s)]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity","ActivityTypeMut",s)]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity","ActivityTypeMut",s)]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default","ActivityTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification","ActivityTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies","ActivityTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations","ActivityTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations","ActivityTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles","ActivityTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics","ActivityTypeMut",s,rmg*="loop_characteristics_mut")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    #[tia("ReceiveTaskType",rg*="implementation","ReceiveTaskTypeMut",s)]
    pub implementation: Option<String>,
    #[xml(attr = "instantiate")]
    #[tia("ReceiveTaskType",rg*="instantiate","ReceiveTaskTypeMut",s)]
    pub instantiate: Option<bool>,
    #[xml(attr = "messageRef")]
    #[tia("ReceiveTaskType",rg*="message_ref","ReceiveTaskTypeMut",s)]
    pub message_ref: Option<String>,
    #[xml(attr = "operationRef")]
    #[tia("ReceiveTaskType",rg*="operation_ref","ReceiveTaskTypeMut",s)]
    pub operation_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for ReceiveTask {
    fn element(&self) -> Element {
        Element::ReceiveTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ReceiveTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl TaskType for ReceiveTask {}
#[cast_to]
impl TaskTypeMut for ReceiveTask {}
castable_to! {ReceiveTask => PartialEq<ReceiveTask> }
castable_to! {ReceiveTask => TaskType,TaskTypeMut}
castable_to! {ReceiveTask => ActivityType,ActivityTypeMut}
castable_to! {ReceiveTask => FlowNodeType,FlowNodeTypeMut}
castable_to! {ReceiveTask => FlowElementType,FlowElementTypeMut}
castable_to! {ReceiveTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `receiveTask`
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
/// Mutable access to `receiveTask`
pub trait ReceiveTaskTypeMut:
    TaskTypeMut + Downcast + Debug + Send + DynClone + ReceiveTaskType
{
    /// Set value of attribute `implementation`
    fn set_implementation(&mut self, value: Option<String>);
    /// Set value of attribute `instantiate`
    fn set_instantiate(&mut self, value: Option<bool>);
    /// Set value of attribute `messageRef`
    fn set_message_ref(&mut self, value: Option<String>);
    /// Set value of attribute `operationRef`
    fn set_operation_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(ReceiveTaskTypeMut);
impl_downcast!(ReceiveTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:relationship")]
pub struct Relationship {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "type")]
    #[tia("RelationshipType",rg*="typ","RelationshipTypeMut",s)]
    pub typ: String,
    #[xml(attr = "direction")]
    #[tia("RelationshipType",rg*="direction","RelationshipTypeMut",s)]
    pub direction: Option<String>,
    #[xml(flatten_text = "bpmn:source")]
    #[tia("RelationshipType",rg*="sources","RelationshipTypeMut",s,rmg*="sources_mut")]
    pub sources: Vec<String>,
    #[xml(flatten_text = "bpmn:target")]
    #[tia("RelationshipType",rg*="targets","RelationshipTypeMut",s,rmg*="targets_mut")]
    pub targets: Vec<String>,
}
#[cast_to]
impl DocumentElement for Relationship {
    fn element(&self) -> Element {
        Element::Relationship
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Relationship {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {Relationship => BaseElementType,BaseElementTypeMut}
//

/// Access to `relationship`
pub trait RelationshipType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `type`
    fn typ(&self) -> &String;
    /// Get value of attribute `direction`
    fn direction(&self) -> &Option<String>;
    /// Get value of `source` child
    fn sources(&self) -> &Vec<String>;
    /// Get value of `target` child
    fn targets(&self) -> &Vec<String>;
}
dyn_clone::clone_trait_object!(RelationshipType);
impl_downcast!(RelationshipType);
/// Mutable access to `relationship`
pub trait RelationshipTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + RelationshipType
{
    /// Set value of attribute `type`
    fn set_typ(&mut self, value: String);
    /// Set value of attribute `direction`
    fn set_direction(&mut self, value: Option<String>);
    /// Get a mutable value of `source` child
    fn sources_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `source` child
    fn set_sources(&mut self, value: Vec<String>);
    /// Get a mutable value of `target` child
    fn targets_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `target` child
    fn set_targets(&mut self, value: Vec<String>);
}
dyn_clone::clone_trait_object!(RelationshipTypeMut);
impl_downcast!(RelationshipTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:rendering")]
pub struct Rendering {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
}
#[cast_to]
impl DocumentElement for Rendering {
    fn element(&self) -> Element {
        Element::Rendering
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Rendering {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {Rendering => BaseElementType,BaseElementTypeMut}
//

/// Access to `rendering`
pub trait RenderingType: BaseElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(RenderingType);
impl_downcast!(RenderingType);
/// Mutable access to `rendering`
pub trait RenderingTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + RenderingType
{
}
dyn_clone::clone_trait_object!(RenderingTypeMut);
impl_downcast!(RenderingTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:resource")]
pub struct Resource {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ResourceType",rg*="name","ResourceTypeMut",s)]
    pub name: String,
    #[xml(child = "bpmn:resourceParameter")]
    #[tia("ResourceType",rg*="resource_parameters","ResourceTypeMut",s,rmg*="resource_parameters_mut")]
    pub resource_parameters: Vec<ResourceParameter>,
}
#[cast_to]
impl DocumentElement for Resource {
    fn element(&self) -> Element {
        Element::Resource
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Resource {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.resource_parameters.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
#[cast_to]
impl RootElementType for Resource {}
#[cast_to]
impl RootElementTypeMut for Resource {}
castable_to! {Resource => PartialEq<Resource> }
castable_to! {Resource => RootElementType,RootElementTypeMut}
castable_to! {Resource => BaseElementType,BaseElementTypeMut}
//

/// Access to `resource`
pub trait ResourceType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &String;
    /// Get value of `resourceParameter` child
    fn resource_parameters(&self) -> &Vec<ResourceParameter>;
}
dyn_clone::clone_trait_object!(ResourceType);
impl_downcast!(ResourceType);
/// Mutable access to `resource`
pub trait ResourceTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + ResourceType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: String);
    /// Get a mutable value of `resourceParameter` child
    fn resource_parameters_mut(&mut self) -> &mut Vec<ResourceParameter>;
    /// Set value of `resourceParameter` child
    fn set_resource_parameters(&mut self, value: Vec<ResourceParameter>);
}
dyn_clone::clone_trait_object!(ResourceTypeMut);
impl_downcast!(ResourceTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:resourceAssignmentExpression")]
pub struct ResourceAssignmentExpression {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:expression")]
    #[tia("ResourceAssignmentExpressionType",rg*="expression","ResourceAssignmentExpressionTypeMut",s,rmg*="expression_mut")]
    pub expression: Expression,
}
#[cast_to]
impl DocumentElement for ResourceAssignmentExpression {
    fn element(&self) -> Element {
        Element::ResourceAssignmentExpression
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ResourceAssignmentExpression {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.expression.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {ResourceAssignmentExpression => BaseElementType,BaseElementTypeMut}
//

/// Access to `resourceAssignmentExpression`
pub trait ResourceAssignmentExpressionType:
    BaseElementType + Downcast + Debug + Send + DynClone
{
    /// Get value of `expression` child
    fn expression(&self) -> &Expression;
}
dyn_clone::clone_trait_object!(ResourceAssignmentExpressionType);
impl_downcast!(ResourceAssignmentExpressionType);
/// Mutable access to `resourceAssignmentExpression`
pub trait ResourceAssignmentExpressionTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + ResourceAssignmentExpressionType
{
    /// Get a mutable value of `expression` child
    fn expression_mut(&mut self) -> &mut Expression;
    /// Set value of `expression` child
    fn set_expression(&mut self, value: Expression);
}
dyn_clone::clone_trait_object!(ResourceAssignmentExpressionTypeMut);
impl_downcast!(ResourceAssignmentExpressionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:resourceParameter")]
pub struct ResourceParameter {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ResourceParameterType",rg*="name","ResourceParameterTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "type")]
    #[tia("ResourceParameterType",rg*="typ","ResourceParameterTypeMut",s)]
    pub typ: Option<String>,
    #[xml(attr = "isRequired")]
    #[tia("ResourceParameterType",rg*="is_required","ResourceParameterTypeMut",s)]
    pub is_required: Option<bool>,
}
#[cast_to]
impl DocumentElement for ResourceParameter {
    fn element(&self) -> Element {
        Element::ResourceParameter
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ResourceParameter {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {ResourceParameter => BaseElementType,BaseElementTypeMut}
//

/// Access to `resourceParameter`
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
/// Mutable access to `resourceParameter`
pub trait ResourceParameterTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + ResourceParameterType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `type`
    fn set_typ(&mut self, value: Option<String>);
    /// Set value of attribute `isRequired`
    fn set_is_required(&mut self, value: Option<bool>);
}
dyn_clone::clone_trait_object!(ResourceParameterTypeMut);
impl_downcast!(ResourceParameterTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:resourceParameterBinding")]
pub struct ResourceParameterBinding {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "parameterRef")]
    #[tia("ResourceParameterBindingType",rg*="parameter_ref","ResourceParameterBindingTypeMut",s)]
    pub parameter_ref: String,
    #[xml(child = "bpmn:expression")]
    #[tia("ResourceParameterBindingType",rg*="expression","ResourceParameterBindingTypeMut",s,rmg*="expression_mut")]
    pub expression: Expression,
}
#[cast_to]
impl DocumentElement for ResourceParameterBinding {
    fn element(&self) -> Element {
        Element::ResourceParameterBinding
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ResourceParameterBinding {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.expression.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {ResourceParameterBinding => BaseElementType,BaseElementTypeMut}
//

/// Access to `resourceParameterBinding`
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
/// Mutable access to `resourceParameterBinding`
pub trait ResourceParameterBindingTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + ResourceParameterBindingType
{
    /// Set value of attribute `parameterRef`
    fn set_parameter_ref(&mut self, value: String);
    /// Get a mutable value of `expression` child
    fn expression_mut(&mut self) -> &mut Expression;
    /// Set value of `expression` child
    fn set_expression(&mut self, value: Expression);
}
dyn_clone::clone_trait_object!(ResourceParameterBindingTypeMut);
impl_downcast!(ResourceParameterBindingTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:resourceRole")]
pub struct ResourceRole {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ResourceRoleType",rg*="name","ResourceRoleTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:resourceRef")]
    #[tia("ResourceRoleType",rg*="resource_ref","ResourceRoleTypeMut",s,rmg*="resource_ref_mut")]
    pub resource_ref: String,
    #[xml(child = "bpmn:resourceParameterBinding")]
    #[tia("ResourceRoleType",rg*="resource_parameter_bindings","ResourceRoleTypeMut",s,rmg*="resource_parameter_bindings_mut")]
    pub resource_parameter_bindings: Vec<ResourceParameterBinding>,
    #[xml(child = "bpmn:resourceAssignmentExpression")]
    #[tia("ResourceRoleType",rg*="resource_assignment_expression","ResourceRoleTypeMut",s,rmg*="resource_assignment_expression_mut")]
    pub resource_assignment_expression: Option<ResourceAssignmentExpression>,
}
#[cast_to]
impl DocumentElement for ResourceRole {
    fn element(&self) -> Element {
        Element::ResourceRole
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ResourceRole {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.resource_parameter_bindings.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.resource_assignment_expression.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
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
castable_to! {ResourceRole => BaseElementType,BaseElementTypeMut}
//

/// Access to `resourceRole`
pub trait ResourceRoleType: BaseElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of `resourceRef` child
    fn resource_ref(&self) -> &String;
    /// Get value of `resourceParameterBinding` child
    fn resource_parameter_bindings(&self) -> &Vec<ResourceParameterBinding>;
    /// Get value of `resourceAssignmentExpression` child
    fn resource_assignment_expression(&self) -> &Option<ResourceAssignmentExpression>;
}
dyn_clone::clone_trait_object!(ResourceRoleType);
impl_downcast!(ResourceRoleType);
/// Mutable access to `resourceRole`
pub trait ResourceRoleTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + ResourceRoleType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Get a mutable value of `resourceRef` child
    fn resource_ref_mut(&mut self) -> &mut String;
    /// Set value of `resourceRef` child
    fn set_resource_ref(&mut self, value: String);
    /// Get a mutable value of `resourceParameterBinding` child
    fn resource_parameter_bindings_mut(&mut self) -> &mut Vec<ResourceParameterBinding>;
    /// Set value of `resourceParameterBinding` child
    fn set_resource_parameter_bindings(&mut self, value: Vec<ResourceParameterBinding>);
    /// Get a mutable value of `resourceAssignmentExpression` child
    fn resource_assignment_expression_mut(&mut self) -> &mut Option<ResourceAssignmentExpression>;
    /// Set value of `resourceAssignmentExpression` child
    fn set_resource_assignment_expression(&mut self, value: Option<ResourceAssignmentExpression>);
}
dyn_clone::clone_trait_object!(ResourceRoleTypeMut);
impl_downcast!(ResourceRoleTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:rootElement")]
#[serde(tag = "type")]
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
impl From<Category> for RootElement {
    fn from(element: Category) -> Self {
        Self::Category(element)
    }
}
impl From<Collaboration> for RootElement {
    fn from(element: Collaboration) -> Self {
        Self::Collaboration(element)
    }
}
impl From<CorrelationProperty> for RootElement {
    fn from(element: CorrelationProperty) -> Self {
        Self::CorrelationProperty(element)
    }
}
impl From<DataStore> for RootElement {
    fn from(element: DataStore) -> Self {
        Self::DataStore(element)
    }
}
impl From<EndPoint> for RootElement {
    fn from(element: EndPoint) -> Self {
        Self::EndPoint(element)
    }
}
impl From<Error> for RootElement {
    fn from(element: Error) -> Self {
        Self::Error(element)
    }
}
impl From<Escalation> for RootElement {
    fn from(element: Escalation) -> Self {
        Self::Escalation(element)
    }
}
impl From<EventDefinition> for RootElement {
    fn from(element: EventDefinition) -> Self {
        Self::EventDefinition(element)
    }
}
impl From<GlobalBusinessRuleTask> for RootElement {
    fn from(element: GlobalBusinessRuleTask) -> Self {
        Self::GlobalBusinessRuleTask(element)
    }
}
impl From<GlobalManualTask> for RootElement {
    fn from(element: GlobalManualTask) -> Self {
        Self::GlobalManualTask(element)
    }
}
impl From<GlobalScriptTask> for RootElement {
    fn from(element: GlobalScriptTask) -> Self {
        Self::GlobalScriptTask(element)
    }
}
impl From<GlobalTask> for RootElement {
    fn from(element: GlobalTask) -> Self {
        Self::GlobalTask(element)
    }
}
impl From<GlobalUserTask> for RootElement {
    fn from(element: GlobalUserTask) -> Self {
        Self::GlobalUserTask(element)
    }
}
impl From<Interface> for RootElement {
    fn from(element: Interface) -> Self {
        Self::Interface(element)
    }
}
impl From<ItemDefinition> for RootElement {
    fn from(element: ItemDefinition) -> Self {
        Self::ItemDefinition(element)
    }
}
impl From<Message> for RootElement {
    fn from(element: Message) -> Self {
        Self::Message(element)
    }
}
impl From<PartnerEntity> for RootElement {
    fn from(element: PartnerEntity) -> Self {
        Self::PartnerEntity(element)
    }
}
impl From<PartnerRole> for RootElement {
    fn from(element: PartnerRole) -> Self {
        Self::PartnerRole(element)
    }
}
impl From<Process> for RootElement {
    fn from(element: Process) -> Self {
        Self::Process(element)
    }
}
impl From<Resource> for RootElement {
    fn from(element: Resource) -> Self {
        Self::Resource(element)
    }
}
impl From<Signal> for RootElement {
    fn from(element: Signal) -> Self {
        Self::Signal(element)
    }
}
impl RootElement {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {
            RootElement::Category(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::Collaboration(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::CorrelationProperty(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::DataStore(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::EndPoint(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::Error(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::Escalation(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::EventDefinition(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::GlobalBusinessRuleTask(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::GlobalManualTask(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::GlobalScriptTask(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::GlobalTask(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::GlobalUserTask(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::Interface(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::ItemDefinition(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::Message(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::PartnerEntity(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::PartnerRole(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::Process(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::Resource(e) => Box::new(e) as Box<dyn DocumentElement>,
            RootElement::Signal(e) => Box::new(e) as Box<dyn DocumentElement>,
        }
    }
}
#[cast_to]
impl DocumentElementContainer for RootElement {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            RootElement::Category(e) => e.find_by_id_mut(id),
            RootElement::Collaboration(e) => e.find_by_id_mut(id),
            RootElement::CorrelationProperty(e) => e.find_by_id_mut(id),
            RootElement::DataStore(e) => e.find_by_id_mut(id),
            RootElement::EndPoint(e) => e.find_by_id_mut(id),
            RootElement::Error(e) => e.find_by_id_mut(id),
            RootElement::Escalation(e) => e.find_by_id_mut(id),
            RootElement::EventDefinition(e) => e.find_by_id_mut(id),
            RootElement::GlobalBusinessRuleTask(e) => e.find_by_id_mut(id),
            RootElement::GlobalManualTask(e) => e.find_by_id_mut(id),
            RootElement::GlobalScriptTask(e) => e.find_by_id_mut(id),
            RootElement::GlobalTask(e) => e.find_by_id_mut(id),
            RootElement::GlobalUserTask(e) => e.find_by_id_mut(id),
            RootElement::Interface(e) => e.find_by_id_mut(id),
            RootElement::ItemDefinition(e) => e.find_by_id_mut(id),
            RootElement::Message(e) => e.find_by_id_mut(id),
            RootElement::PartnerEntity(e) => e.find_by_id_mut(id),
            RootElement::PartnerRole(e) => e.find_by_id_mut(id),
            RootElement::Process(e) => e.find_by_id_mut(id),
            RootElement::Resource(e) => e.find_by_id_mut(id),
            RootElement::Signal(e) => e.find_by_id_mut(id),

            _ => None,
        }
    }

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
#[cast_to]
impl DocumentElement for RootElement {
    fn element(&self) -> Element {
        Element::RootElement
    }
}
/// Access to `rootElement`
pub trait RootElementType: BaseElementType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(RootElementType);
impl_downcast!(RootElementType);
/// Mutable access to `rootElement`
pub trait RootElementTypeMut:
    BaseElementTypeMut + Downcast + Debug + Send + DynClone + RootElementType
{
}
dyn_clone::clone_trait_object!(RootElementTypeMut);
impl_downcast!(RootElementTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:scriptTask")]
pub struct ScriptTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation","ActivityTypeMut",s)]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity","ActivityTypeMut",s)]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity","ActivityTypeMut",s)]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default","ActivityTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification","ActivityTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies","ActivityTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations","ActivityTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations","ActivityTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles","ActivityTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics","ActivityTypeMut",s,rmg*="loop_characteristics_mut")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "scriptFormat")]
    #[tia("ScriptTaskType",rg*="script_format","ScriptTaskTypeMut",s)]
    pub script_format: Option<String>,
    #[xml(child = "bpmn:script")]
    #[tia("ScriptTaskType",rg*="script","ScriptTaskTypeMut",s,rmg*="script_mut")]
    pub script: Option<Script>,
}
#[cast_to]
impl DocumentElement for ScriptTask {
    fn element(&self) -> Element {
        Element::ScriptTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ScriptTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.script.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
#[cast_to]
impl TaskType for ScriptTask {}
#[cast_to]
impl TaskTypeMut for ScriptTask {}
castable_to! {ScriptTask => PartialEq<ScriptTask> }
castable_to! {ScriptTask => TaskType,TaskTypeMut}
castable_to! {ScriptTask => ActivityType,ActivityTypeMut}
castable_to! {ScriptTask => FlowNodeType,FlowNodeTypeMut}
castable_to! {ScriptTask => FlowElementType,FlowElementTypeMut}
castable_to! {ScriptTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `scriptTask`
pub trait ScriptTaskType: TaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `scriptFormat`
    fn script_format(&self) -> &Option<String>;
    /// Get value of `script` child
    fn script(&self) -> &Option<Script>;
}
dyn_clone::clone_trait_object!(ScriptTaskType);
impl_downcast!(ScriptTaskType);
/// Mutable access to `scriptTask`
pub trait ScriptTaskTypeMut:
    TaskTypeMut + Downcast + Debug + Send + DynClone + ScriptTaskType
{
    /// Set value of attribute `scriptFormat`
    fn set_script_format(&mut self, value: Option<String>);
    /// Get a mutable value of `script` child
    fn script_mut(&mut self) -> &mut Option<Script>;
    /// Set value of `script` child
    fn set_script(&mut self, value: Option<Script>);
}
dyn_clone::clone_trait_object!(ScriptTaskTypeMut);
impl_downcast!(ScriptTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:script")]
pub struct Script {}
#[cast_to]
impl DocumentElement for Script {
    fn element(&self) -> Element {
        Element::Script
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Script {}
// Traits

//

/// Access to `script`
pub trait ScriptType: Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(ScriptType);
impl_downcast!(ScriptType);
/// Mutable access to `script`
pub trait ScriptTypeMut: Downcast + Debug + Send + DynClone + ScriptType {}
dyn_clone::clone_trait_object!(ScriptTypeMut);
impl_downcast!(ScriptTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:sendTask")]
pub struct SendTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation","ActivityTypeMut",s)]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity","ActivityTypeMut",s)]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity","ActivityTypeMut",s)]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default","ActivityTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification","ActivityTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies","ActivityTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations","ActivityTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations","ActivityTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles","ActivityTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics","ActivityTypeMut",s,rmg*="loop_characteristics_mut")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    #[tia("SendTaskType",rg*="implementation","SendTaskTypeMut",s)]
    pub implementation: Option<String>,
    #[xml(attr = "messageRef")]
    #[tia("SendTaskType",rg*="message_ref","SendTaskTypeMut",s)]
    pub message_ref: Option<String>,
    #[xml(attr = "operationRef")]
    #[tia("SendTaskType",rg*="operation_ref","SendTaskTypeMut",s)]
    pub operation_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for SendTask {
    fn element(&self) -> Element {
        Element::SendTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for SendTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl TaskType for SendTask {}
#[cast_to]
impl TaskTypeMut for SendTask {}
castable_to! {SendTask => PartialEq<SendTask> }
castable_to! {SendTask => TaskType,TaskTypeMut}
castable_to! {SendTask => ActivityType,ActivityTypeMut}
castable_to! {SendTask => FlowNodeType,FlowNodeTypeMut}
castable_to! {SendTask => FlowElementType,FlowElementTypeMut}
castable_to! {SendTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `sendTask`
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
/// Mutable access to `sendTask`
pub trait SendTaskTypeMut: TaskTypeMut + Downcast + Debug + Send + DynClone + SendTaskType {
    /// Set value of attribute `implementation`
    fn set_implementation(&mut self, value: Option<String>);
    /// Set value of attribute `messageRef`
    fn set_message_ref(&mut self, value: Option<String>);
    /// Set value of attribute `operationRef`
    fn set_operation_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(SendTaskTypeMut);
impl_downcast!(SendTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:sequenceFlow")]
pub struct SequenceFlow {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(attr = "sourceRef")]
    #[tia("SequenceFlowType",rg*="source_ref","SequenceFlowTypeMut",s)]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    #[tia("SequenceFlowType",rg*="target_ref","SequenceFlowTypeMut",s)]
    pub target_ref: String,
    #[xml(attr = "isImmediate")]
    #[tia("SequenceFlowType",rg*="is_immediate","SequenceFlowTypeMut",s)]
    pub is_immediate: Option<bool>,
    #[xml(child = "bpmn:conditionExpression")]
    #[tia("SequenceFlowType",rg*="condition_expression","SequenceFlowTypeMut",s,rmg*="condition_expression_mut")]
    pub condition_expression: Option<Expression>,
}
#[cast_to]
impl DocumentElement for SequenceFlow {
    fn element(&self) -> Element {
        Element::SequenceFlow
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for SequenceFlow {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.condition_expression.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {SequenceFlow => FlowElementType,FlowElementTypeMut}
castable_to! {SequenceFlow => BaseElementType,BaseElementTypeMut}
//

/// Access to `sequenceFlow`
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
/// Mutable access to `sequenceFlow`
pub trait SequenceFlowTypeMut:
    FlowElementTypeMut + Downcast + Debug + Send + DynClone + SequenceFlowType
{
    /// Set value of attribute `sourceRef`
    fn set_source_ref(&mut self, value: String);
    /// Set value of attribute `targetRef`
    fn set_target_ref(&mut self, value: String);
    /// Set value of attribute `isImmediate`
    fn set_is_immediate(&mut self, value: Option<bool>);
    /// Get a mutable value of `conditionExpression` child
    fn condition_expression_mut(&mut self) -> &mut Option<Expression>;
    /// Set value of `conditionExpression` child
    fn set_condition_expression(&mut self, value: Option<Expression>);
}
dyn_clone::clone_trait_object!(SequenceFlowTypeMut);
impl_downcast!(SequenceFlowTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:serviceTask")]
pub struct ServiceTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation","ActivityTypeMut",s)]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity","ActivityTypeMut",s)]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity","ActivityTypeMut",s)]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default","ActivityTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification","ActivityTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies","ActivityTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations","ActivityTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations","ActivityTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles","ActivityTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics","ActivityTypeMut",s,rmg*="loop_characteristics_mut")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    #[tia("ServiceTaskType",rg*="implementation","ServiceTaskTypeMut",s)]
    pub implementation: Option<String>,
    #[xml(attr = "operationRef")]
    #[tia("ServiceTaskType",rg*="operation_ref","ServiceTaskTypeMut",s)]
    pub operation_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for ServiceTask {
    fn element(&self) -> Element {
        Element::ServiceTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for ServiceTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl TaskType for ServiceTask {}
#[cast_to]
impl TaskTypeMut for ServiceTask {}
castable_to! {ServiceTask => PartialEq<ServiceTask> }
castable_to! {ServiceTask => TaskType,TaskTypeMut}
castable_to! {ServiceTask => ActivityType,ActivityTypeMut}
castable_to! {ServiceTask => FlowNodeType,FlowNodeTypeMut}
castable_to! {ServiceTask => FlowElementType,FlowElementTypeMut}
castable_to! {ServiceTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `serviceTask`
pub trait ServiceTaskType: TaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `implementation`
    fn implementation(&self) -> &Option<String>;
    /// Get value of attribute `operationRef`
    fn operation_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(ServiceTaskType);
impl_downcast!(ServiceTaskType);
/// Mutable access to `serviceTask`
pub trait ServiceTaskTypeMut:
    TaskTypeMut + Downcast + Debug + Send + DynClone + ServiceTaskType
{
    /// Set value of attribute `implementation`
    fn set_implementation(&mut self, value: Option<String>);
    /// Set value of attribute `operationRef`
    fn set_operation_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(ServiceTaskTypeMut);
impl_downcast!(ServiceTaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:signal")]
pub struct Signal {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("SignalType",rg*="name","SignalTypeMut",s)]
    pub name: Option<String>,
    #[xml(attr = "structureRef")]
    #[tia("SignalType",rg*="structure_ref","SignalTypeMut",s)]
    pub structure_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for Signal {
    fn element(&self) -> Element {
        Element::Signal
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Signal {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl RootElementType for Signal {}
#[cast_to]
impl RootElementTypeMut for Signal {}
castable_to! {Signal => PartialEq<Signal> }
castable_to! {Signal => RootElementType,RootElementTypeMut}
castable_to! {Signal => BaseElementType,BaseElementTypeMut}
//

/// Access to `signal`
pub trait SignalType: RootElementType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `name`
    fn name(&self) -> &Option<String>;
    /// Get value of attribute `structureRef`
    fn structure_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(SignalType);
impl_downcast!(SignalType);
/// Mutable access to `signal`
pub trait SignalTypeMut:
    RootElementTypeMut + Downcast + Debug + Send + DynClone + SignalType
{
    /// Set value of attribute `name`
    fn set_name(&mut self, value: Option<String>);
    /// Set value of attribute `structureRef`
    fn set_structure_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(SignalTypeMut);
impl_downcast!(SignalTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:signalEventDefinition")]
pub struct SignalEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "signalRef")]
    #[tia("SignalEventDefinitionType",rg*="signal_ref","SignalEventDefinitionTypeMut",s)]
    pub signal_ref: Option<String>,
}
#[cast_to]
impl DocumentElement for SignalEventDefinition {
    fn element(&self) -> Element {
        Element::SignalEventDefinition
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for SignalEventDefinition {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl EventDefinitionType for SignalEventDefinition {}
#[cast_to]
impl EventDefinitionTypeMut for SignalEventDefinition {}
castable_to! {SignalEventDefinition => PartialEq<SignalEventDefinition> }
castable_to! {SignalEventDefinition => EventDefinitionType,EventDefinitionTypeMut}
#[cast_to]
impl RootElementType for SignalEventDefinition {}
#[cast_to]
impl RootElementTypeMut for SignalEventDefinition {}
castable_to! {SignalEventDefinition => PartialEq<SignalEventDefinition> }
castable_to! {SignalEventDefinition => RootElementType,RootElementTypeMut}
castable_to! {SignalEventDefinition => BaseElementType,BaseElementTypeMut}
//

/// Access to `signalEventDefinition`
pub trait SignalEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
    /// Get value of attribute `signalRef`
    fn signal_ref(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(SignalEventDefinitionType);
impl_downcast!(SignalEventDefinitionType);
/// Mutable access to `signalEventDefinition`
pub trait SignalEventDefinitionTypeMut:
    EventDefinitionTypeMut + Downcast + Debug + Send + DynClone + SignalEventDefinitionType
{
    /// Set value of attribute `signalRef`
    fn set_signal_ref(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(SignalEventDefinitionTypeMut);
impl_downcast!(SignalEventDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:standardLoopCharacteristics")]
pub struct StandardLoopCharacteristics {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "testBefore")]
    #[tia("StandardLoopCharacteristicsType",rg*="test_before","StandardLoopCharacteristicsTypeMut",s)]
    pub test_before: Option<bool>,
    #[xml(attr = "loopMaximum")]
    #[tia("StandardLoopCharacteristicsType",rg*="loop_maximum","StandardLoopCharacteristicsTypeMut",s)]
    pub loop_maximum: Option<Integer>,
    #[xml(child = "bpmn:loopCondition")]
    #[tia("StandardLoopCharacteristicsType",rg*="loop_condition","StandardLoopCharacteristicsTypeMut",s,rmg*="loop_condition_mut")]
    pub loop_condition: Option<Expression>,
}
#[cast_to]
impl DocumentElement for StandardLoopCharacteristics {
    fn element(&self) -> Element {
        Element::StandardLoopCharacteristics
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for StandardLoopCharacteristics {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.loop_condition.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
#[cast_to]
impl LoopCharacteristicsType for StandardLoopCharacteristics {}
#[cast_to]
impl LoopCharacteristicsTypeMut for StandardLoopCharacteristics {}
castable_to! {StandardLoopCharacteristics => PartialEq<StandardLoopCharacteristics> }
castable_to! {StandardLoopCharacteristics => LoopCharacteristicsType,LoopCharacteristicsTypeMut}
castable_to! {StandardLoopCharacteristics => BaseElementType,BaseElementTypeMut}
//

/// Access to `standardLoopCharacteristics`
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
/// Mutable access to `standardLoopCharacteristics`
pub trait StandardLoopCharacteristicsTypeMut:
    LoopCharacteristicsTypeMut + Downcast + Debug + Send + DynClone + StandardLoopCharacteristicsType
{
    /// Set value of attribute `testBefore`
    fn set_test_before(&mut self, value: Option<bool>);
    /// Set value of attribute `loopMaximum`
    fn set_loop_maximum(&mut self, value: Option<Integer>);
    /// Get a mutable value of `loopCondition` child
    fn loop_condition_mut(&mut self) -> &mut Option<Expression>;
    /// Set value of `loopCondition` child
    fn set_loop_condition(&mut self, value: Option<Expression>);
}
dyn_clone::clone_trait_object!(StandardLoopCharacteristicsTypeMut);
impl_downcast!(StandardLoopCharacteristicsTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:startEvent")]
pub struct StartEvent {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(child = "bpmn:property")]
    #[tia("EventType",rg*="properies","EventTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(attr = "parallelMultiple")]
    #[tia("CatchEventType",rg*="parallel_multiple","CatchEventTypeMut",s)]
    pub parallel_multiple: Option<bool>,
    #[xml(child = "bpmn:dataOutput")]
    #[tia("CatchEventType",rg*="data_outputs","CatchEventTypeMut",s,rmg*="data_outputs_mut")]
    pub data_outputs: Vec<DataOutput>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("CatchEventType",rg*="data_output_associations","CatchEventTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:outputSet")]
    #[tia("CatchEventType",rg*="output_set","CatchEventTypeMut",s,rmg*="output_set_mut")]
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
    #[tia("CatchEventType",rg*="event_definitions","CatchEventTypeMut",s,rmg*="event_definitions_mut")]
    pub event_definitions: Vec<EventDefinition>,
    #[xml(flatten_text = "bpmn:eventDefinitionRef")]
    #[tia("CatchEventType",rg*="event_definition_refs","CatchEventTypeMut",s,rmg*="event_definition_refs_mut")]
    pub event_definition_refs: Vec<String>,
    #[xml(attr = "isInterrupting")]
    #[tia("StartEventType",rg*="is_interrupting","StartEventTypeMut",s)]
    pub is_interrupting: Option<bool>,
}
#[cast_to]
impl DocumentElement for StartEvent {
    fn element(&self) -> Element {
        Element::StartEvent
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for StartEvent {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {StartEvent => CatchEventType,CatchEventTypeMut}
castable_to! {StartEvent => EventType,EventTypeMut}
castable_to! {StartEvent => FlowNodeType,FlowNodeTypeMut}
castable_to! {StartEvent => FlowElementType,FlowElementTypeMut}
castable_to! {StartEvent => BaseElementType,BaseElementTypeMut}
//

/// Access to `startEvent`
pub trait StartEventType: CatchEventType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `isInterrupting`
    fn is_interrupting(&self) -> &Option<bool>;
}
dyn_clone::clone_trait_object!(StartEventType);
impl_downcast!(StartEventType);
/// Mutable access to `startEvent`
pub trait StartEventTypeMut:
    CatchEventTypeMut + Downcast + Debug + Send + DynClone + StartEventType
{
    /// Set value of attribute `isInterrupting`
    fn set_is_interrupting(&mut self, value: Option<bool>);
}
dyn_clone::clone_trait_object!(StartEventTypeMut);
impl_downcast!(StartEventTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:subChoreography")]
pub struct SubChoreography {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "initiatingParticipantRef")]
    #[tia("ChoreographyActivityType",rg*="initiating_participant_ref","ChoreographyActivityTypeMut",s)]
    pub initiating_participant_ref: String,
    #[xml(attr = "loopType")]
    #[tia("ChoreographyActivityType",rg*="loop_type","ChoreographyActivityTypeMut",s)]
    pub loop_type: Option<String>,
    #[xml(flatten_text = "bpmn:participantRef")]
    #[tia("ChoreographyActivityType",rg*="participant_refs","ChoreographyActivityTypeMut",s,rmg*="participant_refs_mut")]
    pub participant_refs: Vec<String>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("ChoreographyActivityType",rg*="correlation_keys","ChoreographyActivityTypeMut",s,rmg*="correlation_keys_mut")]
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
    #[tia("SubChoreographyType",rg*="flow_elements","SubChoreographyTypeMut",s,rmg*="flow_elements_mut")]
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("SubChoreographyType",rg*="artifacts","SubChoreographyTypeMut",s,rmg*="artifacts_mut")]
    pub artifacts: Vec<Artifact>,
}
#[cast_to]
impl DocumentElement for SubChoreography {
    fn element(&self) -> Element {
        Element::SubChoreography
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for SubChoreography {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.flow_elements.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.artifacts.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {SubChoreography => ChoreographyActivityType,ChoreographyActivityTypeMut}
castable_to! {SubChoreography => FlowNodeType,FlowNodeTypeMut}
castable_to! {SubChoreography => FlowElementType,FlowElementTypeMut}
castable_to! {SubChoreography => BaseElementType,BaseElementTypeMut}
//

/// Access to `subChoreography`
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
/// Mutable access to `subChoreography`
pub trait SubChoreographyTypeMut:
    ChoreographyActivityTypeMut + Downcast + Debug + Send + DynClone + SubChoreographyType
{
    /// Get a mutable value of `flowElement` child
    fn flow_elements_mut(&mut self) -> &mut Vec<FlowElement>;
    /// Set value of `flowElement` child
    fn set_flow_elements(&mut self, value: Vec<FlowElement>);
    /// Get a mutable value of `artifact` child
    fn artifacts_mut(&mut self) -> &mut Vec<Artifact>;
    /// Set value of `artifact` child
    fn set_artifacts(&mut self, value: Vec<Artifact>);
}
dyn_clone::clone_trait_object!(SubChoreographyTypeMut);
impl_downcast!(SubChoreographyTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:subConversation")]
pub struct SubConversation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("ConversationNodeType",rg*="name","ConversationNodeTypeMut",s)]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:participantRef")]
    #[tia("ConversationNodeType",rg*="participant_refs","ConversationNodeTypeMut",s,rmg*="participant_refs_mut")]
    pub participant_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:messageFlowRef")]
    #[tia("ConversationNodeType",rg*="message_flow_refs","ConversationNodeTypeMut",s,rmg*="message_flow_refs_mut")]
    pub message_flow_refs: Vec<String>,
    #[xml(child = "bpmn:correlationKey")]
    #[tia("ConversationNodeType",rg*="correlation_keys","ConversationNodeTypeMut",s,rmg*="correlation_keys_mut")]
    pub correlation_keys: Vec<CorrelationKey>,
    #[xml(
        child = "bpmn:callConversation",
        child = "bpmn:conversation",
        child = "bpmn:subConversation"
    )]
    #[tia("SubConversationType",rg*="conversation_nodes","SubConversationTypeMut",s,rmg*="conversation_nodes_mut")]
    pub conversation_nodes: Vec<ConversationNode>,
}
#[cast_to]
impl DocumentElement for SubConversation {
    fn element(&self) -> Element {
        Element::SubConversation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for SubConversation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.conversation_nodes.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {SubConversation => ConversationNodeType,ConversationNodeTypeMut}
castable_to! {SubConversation => BaseElementType,BaseElementTypeMut}
//

/// Access to `subConversation`
pub trait SubConversationType: ConversationNodeType + Downcast + Debug + Send + DynClone {
    /// Get value of `conversationNode` child
    fn conversation_nodes(&self) -> &Vec<ConversationNode>;
}
dyn_clone::clone_trait_object!(SubConversationType);
impl_downcast!(SubConversationType);
/// Mutable access to `subConversation`
pub trait SubConversationTypeMut:
    ConversationNodeTypeMut + Downcast + Debug + Send + DynClone + SubConversationType
{
    /// Get a mutable value of `conversationNode` child
    fn conversation_nodes_mut(&mut self) -> &mut Vec<ConversationNode>;
    /// Set value of `conversationNode` child
    fn set_conversation_nodes(&mut self, value: Vec<ConversationNode>);
}
dyn_clone::clone_trait_object!(SubConversationTypeMut);
impl_downcast!(SubConversationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:subProcess")]
pub struct SubProcess {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation","ActivityTypeMut",s)]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity","ActivityTypeMut",s)]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity","ActivityTypeMut",s)]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default","ActivityTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification","ActivityTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies","ActivityTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations","ActivityTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations","ActivityTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles","ActivityTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics","ActivityTypeMut",s,rmg*="loop_characteristics_mut")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "triggeredByEvent")]
    #[tia("SubProcessType",rg*="triggered_byevent","SubProcessTypeMut",s)]
    pub triggered_byevent: Option<bool>,
    #[xml(child = "bpmn:laneSet")]
    #[tia("SubProcessType",rg*="lane_sets","SubProcessTypeMut",s,rmg*="lane_sets_mut")]
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
    #[tia("SubProcessType",rg*="flow_elements","SubProcessTypeMut",s,rmg*="flow_elements_mut")]
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("SubProcessType",rg*="artifacts","SubProcessTypeMut",s,rmg*="artifacts_mut")]
    pub artifacts: Vec<Artifact>,
}
#[cast_to]
impl DocumentElement for SubProcess {
    fn element(&self) -> Element {
        Element::SubProcess
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for SubProcess {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.lane_sets.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.flow_elements.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.artifacts.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
castable_to! {SubProcess => ActivityType,ActivityTypeMut}
castable_to! {SubProcess => FlowNodeType,FlowNodeTypeMut}
castable_to! {SubProcess => FlowElementType,FlowElementTypeMut}
castable_to! {SubProcess => BaseElementType,BaseElementTypeMut}
//

/// Access to `subProcess`
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
/// Mutable access to `subProcess`
pub trait SubProcessTypeMut:
    ActivityTypeMut + Downcast + Debug + Send + DynClone + SubProcessType
{
    /// Set value of attribute `triggeredByEvent`
    fn set_triggered_byevent(&mut self, value: Option<bool>);
    /// Get a mutable value of `laneSet` child
    fn lane_sets_mut(&mut self) -> &mut Vec<LaneSet>;
    /// Set value of `laneSet` child
    fn set_lane_sets(&mut self, value: Vec<LaneSet>);
    /// Get a mutable value of `flowElement` child
    fn flow_elements_mut(&mut self) -> &mut Vec<FlowElement>;
    /// Set value of `flowElement` child
    fn set_flow_elements(&mut self, value: Vec<FlowElement>);
    /// Get a mutable value of `artifact` child
    fn artifacts_mut(&mut self) -> &mut Vec<Artifact>;
    /// Set value of `artifact` child
    fn set_artifacts(&mut self, value: Vec<Artifact>);
}
dyn_clone::clone_trait_object!(SubProcessTypeMut);
impl_downcast!(SubProcessTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:task")]
pub struct Task {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation","ActivityTypeMut",s)]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity","ActivityTypeMut",s)]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity","ActivityTypeMut",s)]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default","ActivityTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification","ActivityTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies","ActivityTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations","ActivityTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations","ActivityTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles","ActivityTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics","ActivityTypeMut",s,rmg*="loop_characteristics_mut")]
    pub loop_characteristics: Option<LoopCharacteristics>,
}
#[cast_to]
impl DocumentElement for Task {
    fn element(&self) -> Element {
        Element::Task
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Task {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {Task => ActivityType,ActivityTypeMut}
castable_to! {Task => FlowNodeType,FlowNodeTypeMut}
castable_to! {Task => FlowElementType,FlowElementTypeMut}
castable_to! {Task => BaseElementType,BaseElementTypeMut}
//

/// Access to `task`
pub trait TaskType: ActivityType + Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(TaskType);
impl_downcast!(TaskType);
/// Mutable access to `task`
pub trait TaskTypeMut: ActivityTypeMut + Downcast + Debug + Send + DynClone + TaskType {}
dyn_clone::clone_trait_object!(TaskTypeMut);
impl_downcast!(TaskTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:terminateEventDefinition")]
pub struct TerminateEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
}
#[cast_to]
impl DocumentElement for TerminateEventDefinition {
    fn element(&self) -> Element {
        Element::TerminateEventDefinition
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for TerminateEventDefinition {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
#[cast_to]
impl EventDefinitionType for TerminateEventDefinition {}
#[cast_to]
impl EventDefinitionTypeMut for TerminateEventDefinition {}
castable_to! {TerminateEventDefinition => PartialEq<TerminateEventDefinition> }
castable_to! {TerminateEventDefinition => EventDefinitionType,EventDefinitionTypeMut}
#[cast_to]
impl RootElementType for TerminateEventDefinition {}
#[cast_to]
impl RootElementTypeMut for TerminateEventDefinition {}
castable_to! {TerminateEventDefinition => PartialEq<TerminateEventDefinition> }
castable_to! {TerminateEventDefinition => RootElementType,RootElementTypeMut}
castable_to! {TerminateEventDefinition => BaseElementType,BaseElementTypeMut}
//

/// Access to `terminateEventDefinition`
pub trait TerminateEventDefinitionType:
    EventDefinitionType + Downcast + Debug + Send + DynClone
{
}
dyn_clone::clone_trait_object!(TerminateEventDefinitionType);
impl_downcast!(TerminateEventDefinitionType);
/// Mutable access to `terminateEventDefinition`
pub trait TerminateEventDefinitionTypeMut:
    EventDefinitionTypeMut + Downcast + Debug + Send + DynClone + TerminateEventDefinitionType
{
}
dyn_clone::clone_trait_object!(TerminateEventDefinitionTypeMut);
impl_downcast!(TerminateEventDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:textAnnotation")]
pub struct TextAnnotation {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "textFormat")]
    #[tia("TextAnnotationType",rg*="text_format","TextAnnotationTypeMut",s)]
    pub text_format: Option<String>,
    #[xml(child = "bpmn:text")]
    #[tia("TextAnnotationType",rg*="text","TextAnnotationTypeMut",s,rmg*="text_mut")]
    pub text: Option<Text>,
}
#[cast_to]
impl DocumentElement for TextAnnotation {
    fn element(&self) -> Element {
        Element::TextAnnotation
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for TextAnnotation {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.text.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
#[cast_to]
impl ArtifactType for TextAnnotation {}
#[cast_to]
impl ArtifactTypeMut for TextAnnotation {}
castable_to! {TextAnnotation => PartialEq<TextAnnotation> }
castable_to! {TextAnnotation => ArtifactType,ArtifactTypeMut}
castable_to! {TextAnnotation => BaseElementType,BaseElementTypeMut}
//

/// Access to `textAnnotation`
pub trait TextAnnotationType: ArtifactType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `textFormat`
    fn text_format(&self) -> &Option<String>;
    /// Get value of `text` child
    fn text(&self) -> &Option<Text>;
}
dyn_clone::clone_trait_object!(TextAnnotationType);
impl_downcast!(TextAnnotationType);
/// Mutable access to `textAnnotation`
pub trait TextAnnotationTypeMut:
    ArtifactTypeMut + Downcast + Debug + Send + DynClone + TextAnnotationType
{
    /// Set value of attribute `textFormat`
    fn set_text_format(&mut self, value: Option<String>);
    /// Get a mutable value of `text` child
    fn text_mut(&mut self) -> &mut Option<Text>;
    /// Set value of `text` child
    fn set_text(&mut self, value: Option<Text>);
}
dyn_clone::clone_trait_object!(TextAnnotationTypeMut);
impl_downcast!(TextAnnotationTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:text")]
pub struct Text {}
#[cast_to]
impl DocumentElement for Text {
    fn element(&self) -> Element {
        Element::Text
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Text {}
// Traits

//

/// Access to `text`
pub trait TextType: Downcast + Debug + Send + DynClone {}
dyn_clone::clone_trait_object!(TextType);
impl_downcast!(TextType);
/// Mutable access to `text`
pub trait TextTypeMut: Downcast + Debug + Send + DynClone + TextType {}
dyn_clone::clone_trait_object!(TextTypeMut);
impl_downcast!(TextTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Hash, XmlRead, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[xml(tag = "bpmn:throwEvent")]
#[serde(tag = "type")]
pub enum ThrowEvent {}
impl ThrowEvent {
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {}
    }
}
#[cast_to]
impl DocumentElementContainer for ThrowEvent {
    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            _ => None,
        }
    }

    #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            _ => None,
        }
    }
}
#[cast_to]
impl DocumentElement for ThrowEvent {
    fn element(&self) -> Element {
        Element::ThrowEvent
    }
}
/// Access to `throwEvent`
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
    fn event_definition_refs(&self) -> &Vec<String>;
}
dyn_clone::clone_trait_object!(ThrowEventType);
impl_downcast!(ThrowEventType);
/// Mutable access to `throwEvent`
pub trait ThrowEventTypeMut:
    EventTypeMut + Downcast + Debug + Send + DynClone + ThrowEventType
{
    /// Get a mutable value of `dataInput` child
    fn data_inputs_mut(&mut self) -> &mut Vec<DataInput>;
    /// Set value of `dataInput` child
    fn set_data_inputs(&mut self, value: Vec<DataInput>);
    /// Get a mutable value of `dataInputAssociation` child
    fn data_input_associations_mut(&mut self) -> &mut Vec<DataInputAssociation>;
    /// Set value of `dataInputAssociation` child
    fn set_data_input_associations(&mut self, value: Vec<DataInputAssociation>);
    /// Get a mutable value of `inputSet` child
    fn input_set_mut(&mut self) -> &mut Option<InputSet>;
    /// Set value of `inputSet` child
    fn set_input_set(&mut self, value: Option<InputSet>);
    /// Get a mutable value of `eventDefinition` child
    fn event_definitions_mut(&mut self) -> &mut Vec<EventDefinition>;
    /// Set value of `eventDefinition` child
    fn set_event_definitions(&mut self, value: Vec<EventDefinition>);
    /// Get a mutable value of `eventDefinitionRef` child
    fn event_definition_refs_mut(&mut self) -> &mut Vec<String>;
    /// Set value of `eventDefinitionRef` child
    fn set_event_definition_refs(&mut self, value: Vec<String>);
}
dyn_clone::clone_trait_object!(ThrowEventTypeMut);
impl_downcast!(ThrowEventTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:timerEventDefinition")]
pub struct TimerEventDefinition {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(child = "bpmn:timeDate")]
    #[tia("TimerEventDefinitionType",rg*="time_date","TimerEventDefinitionTypeMut",s,rmg*="time_date_mut")]
    pub time_date: Option<Expression>,
    #[xml(child = "bpmn:timeDuration")]
    #[tia("TimerEventDefinitionType",rg*="time_duration","TimerEventDefinitionTypeMut",s,rmg*="time_duration_mut")]
    pub time_duration: Option<Expression>,
    #[xml(child = "bpmn:timeCycle")]
    #[tia("TimerEventDefinitionType",rg*="time_cycle","TimerEventDefinitionTypeMut",s,rmg*="time_cycle_mut")]
    pub time_cycle: Option<Expression>,
}
#[cast_to]
impl DocumentElement for TimerEventDefinition {
    fn element(&self) -> Element {
        Element::TimerEventDefinition
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for TimerEventDefinition {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.time_date.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.time_duration.find_by_id_mut(id) {
            return Some(e);
        }
        if let Some(e) = self.time_cycle.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
#[cast_to]
impl EventDefinitionType for TimerEventDefinition {}
#[cast_to]
impl EventDefinitionTypeMut for TimerEventDefinition {}
castable_to! {TimerEventDefinition => PartialEq<TimerEventDefinition> }
castable_to! {TimerEventDefinition => EventDefinitionType,EventDefinitionTypeMut}
#[cast_to]
impl RootElementType for TimerEventDefinition {}
#[cast_to]
impl RootElementTypeMut for TimerEventDefinition {}
castable_to! {TimerEventDefinition => PartialEq<TimerEventDefinition> }
castable_to! {TimerEventDefinition => RootElementType,RootElementTypeMut}
castable_to! {TimerEventDefinition => BaseElementType,BaseElementTypeMut}
//

/// Access to `timerEventDefinition`
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
/// Mutable access to `timerEventDefinition`
pub trait TimerEventDefinitionTypeMut:
    EventDefinitionTypeMut + Downcast + Debug + Send + DynClone + TimerEventDefinitionType
{
    /// Get a mutable value of `timeDate` child
    fn time_date_mut(&mut self) -> &mut Option<Expression>;
    /// Set value of `timeDate` child
    fn set_time_date(&mut self, value: Option<Expression>);
    /// Get a mutable value of `timeDuration` child
    fn time_duration_mut(&mut self) -> &mut Option<Expression>;
    /// Set value of `timeDuration` child
    fn set_time_duration(&mut self, value: Option<Expression>);
    /// Get a mutable value of `timeCycle` child
    fn time_cycle_mut(&mut self) -> &mut Option<Expression>;
    /// Set value of `timeCycle` child
    fn set_time_cycle(&mut self, value: Option<Expression>);
}
dyn_clone::clone_trait_object!(TimerEventDefinitionTypeMut);
impl_downcast!(TimerEventDefinitionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:transaction")]
pub struct Transaction {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation","ActivityTypeMut",s)]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity","ActivityTypeMut",s)]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity","ActivityTypeMut",s)]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default","ActivityTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification","ActivityTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies","ActivityTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations","ActivityTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations","ActivityTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles","ActivityTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics","ActivityTypeMut",s,rmg*="loop_characteristics_mut")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "triggeredByEvent")]
    #[tia("SubProcessType",rg*="triggered_byevent","SubProcessTypeMut",s)]
    pub triggered_byevent: Option<bool>,
    #[xml(child = "bpmn:laneSet")]
    #[tia("SubProcessType",rg*="lane_sets","SubProcessTypeMut",s,rmg*="lane_sets_mut")]
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
    #[tia("SubProcessType",rg*="flow_elements","SubProcessTypeMut",s,rmg*="flow_elements_mut")]
    pub flow_elements: Vec<FlowElement>,
    #[xml(
        child = "bpmn:association",
        child = "bpmn:group",
        child = "bpmn:textAnnotation"
    )]
    #[tia("SubProcessType",rg*="artifacts","SubProcessTypeMut",s,rmg*="artifacts_mut")]
    pub artifacts: Vec<Artifact>,
    #[xml(attr = "method")]
    #[tia("TransactionType",rg*="method","TransactionTypeMut",s)]
    pub method: Option<String>,
}
#[cast_to]
impl DocumentElement for Transaction {
    fn element(&self) -> Element {
        Element::Transaction
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for Transaction {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }

        None
    }

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
castable_to! {Transaction => SubProcessType,SubProcessTypeMut}
castable_to! {Transaction => ActivityType,ActivityTypeMut}
castable_to! {Transaction => FlowNodeType,FlowNodeTypeMut}
castable_to! {Transaction => FlowElementType,FlowElementTypeMut}
castable_to! {Transaction => BaseElementType,BaseElementTypeMut}
//

/// Access to `transaction`
pub trait TransactionType: SubProcessType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `method`
    fn method(&self) -> &Option<String>;
}
dyn_clone::clone_trait_object!(TransactionType);
impl_downcast!(TransactionType);
/// Mutable access to `transaction`
pub trait TransactionTypeMut:
    SubProcessTypeMut + Downcast + Debug + Send + DynClone + TransactionType
{
    /// Set value of attribute `method`
    fn set_method(&mut self, value: Option<String>);
}
dyn_clone::clone_trait_object!(TransactionTypeMut);
impl_downcast!(TransactionTypeMut);
/// Auto-generated from BPNM schema
///
/// (See codegen-rust.xsl)
#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug, Serialize, Deserialize)]
#[xml(tag = "bpmn:userTask")]
pub struct UserTask {
    #[xml(attr = "id")]
    #[tia("BaseElementType",rg*="id","BaseElementTypeMut",s)]
    pub id: Option<Id>,
    #[xml(child = "bpmn:documentation")]
    #[tia("BaseElementType",rg*="documentations","BaseElementTypeMut",s,rmg*="documentations_mut")]
    pub documentations: Vec<Documentation>,
    #[xml(child = "bpmn:extensionElements")]
    #[tia("BaseElementType",rg*="extension_elements","BaseElementTypeMut",s,rmg*="extension_elements_mut")]
    pub extension_elements: Option<ExtensionElements>,
    #[xml(attr = "name")]
    #[tia("FlowElementType",rg*="name","FlowElementTypeMut",s)]
    pub name: Option<String>,
    #[xml(child = "bpmn:auditing")]
    #[tia("FlowElementType",rg*="auditing","FlowElementTypeMut",s,rmg*="auditing_mut")]
    pub auditing: Option<Auditing>,
    #[xml(child = "bpmn:monitoring")]
    #[tia("FlowElementType",rg*="monitoring","FlowElementTypeMut",s,rmg*="monitoring_mut")]
    pub monitoring: Option<Monitoring>,
    #[xml(flatten_text = "bpmn:categoryValueRef")]
    #[tia("FlowElementType",rg*="category_value_refs","FlowElementTypeMut",s,rmg*="category_value_refs_mut")]
    pub category_value_refs: Vec<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    #[tia("FlowNodeType",rg*="incomings","FlowNodeTypeMut",s,rmg*="incomings_mut")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    #[tia("FlowNodeType",rg*="outgoings","FlowNodeTypeMut",s,rmg*="outgoings_mut")]
    pub outgoings: Vec<String>,
    #[xml(attr = "isForCompensation")]
    #[tia("ActivityType",rg*="is_for_compensation","ActivityTypeMut",s)]
    pub is_for_compensation: Option<bool>,
    #[xml(attr = "startQuantity")]
    #[tia("ActivityType",rg*="start_quantity","ActivityTypeMut",s)]
    pub start_quantity: Option<Integer>,
    #[xml(attr = "completionQuantity")]
    #[tia("ActivityType",rg*="completion_quantity","ActivityTypeMut",s)]
    pub completion_quantity: Option<Integer>,
    #[xml(attr = "default")]
    #[tia("ActivityType",rg*="default","ActivityTypeMut",s)]
    pub default: Option<String>,
    #[xml(child = "bpmn:ioSpecification")]
    #[tia("ActivityType",rg*="io_specification","ActivityTypeMut",s,rmg*="io_specification_mut")]
    pub io_specification: Option<InputOutputSpecification>,
    #[xml(child = "bpmn:property")]
    #[tia("ActivityType",rg*="properies","ActivityTypeMut",s,rmg*="properies_mut")]
    pub properies: Vec<Property>,
    #[xml(child = "bpmn:dataInputAssociation")]
    #[tia("ActivityType",rg*="data_input_associations","ActivityTypeMut",s,rmg*="data_input_associations_mut")]
    pub data_input_associations: Vec<DataInputAssociation>,
    #[xml(child = "bpmn:dataOutputAssociation")]
    #[tia("ActivityType",rg*="data_output_associations","ActivityTypeMut",s,rmg*="data_output_associations_mut")]
    pub data_output_associations: Vec<DataOutputAssociation>,
    #[xml(child = "bpmn:resourceRole")]
    #[tia("ActivityType",rg*="resource_roles","ActivityTypeMut",s,rmg*="resource_roles_mut")]
    pub resource_roles: Vec<ResourceRole>,
    #[xml(
        child = "bpmn:multiInstanceLoopCharacteristics",
        child = "bpmn:standardLoopCharacteristics"
    )]
    #[tia("ActivityType",rg*="loop_characteristics","ActivityTypeMut",s,rmg*="loop_characteristics_mut")]
    pub loop_characteristics: Option<LoopCharacteristics>,
    #[xml(attr = "implementation")]
    #[tia("UserTaskType",rg*="implementation","UserTaskTypeMut",s)]
    pub implementation: Option<String>,
    #[xml(child = "bpmn:rendering")]
    #[tia("UserTaskType",rg*="renderings","UserTaskTypeMut",s,rmg*="renderings_mut")]
    pub renderings: Vec<Rendering>,
}
#[cast_to]
impl DocumentElement for UserTask {
    fn element(&self) -> Element {
        Element::UserTask
    }
}
#[allow(unused_variables)]
#[cast_to]
impl DocumentElementContainer for UserTask {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        if let Some(ref id_) = self.id {
            if id_ == id {
                return Some(self);
            }
        }
        if let Some(e) = self.renderings.find_by_id_mut(id) {
            return Some(e);
        }
        None
    }

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
#[cast_to]
impl TaskType for UserTask {}
#[cast_to]
impl TaskTypeMut for UserTask {}
castable_to! {UserTask => PartialEq<UserTask> }
castable_to! {UserTask => TaskType,TaskTypeMut}
castable_to! {UserTask => ActivityType,ActivityTypeMut}
castable_to! {UserTask => FlowNodeType,FlowNodeTypeMut}
castable_to! {UserTask => FlowElementType,FlowElementTypeMut}
castable_to! {UserTask => BaseElementType,BaseElementTypeMut}
//

/// Access to `userTask`
pub trait UserTaskType: TaskType + Downcast + Debug + Send + DynClone {
    /// Get value of attribute `implementation`
    fn implementation(&self) -> &Option<String>;
    /// Get value of `rendering` child
    fn renderings(&self) -> &Vec<Rendering>;
}
dyn_clone::clone_trait_object!(UserTaskType);
impl_downcast!(UserTaskType);
/// Mutable access to `userTask`
pub trait UserTaskTypeMut: TaskTypeMut + Downcast + Debug + Send + DynClone + UserTaskType {
    /// Set value of attribute `implementation`
    fn set_implementation(&mut self, value: Option<String>);
    /// Get a mutable value of `rendering` child
    fn renderings_mut(&mut self) -> &mut Vec<Rendering>;
    /// Set value of `rendering` child
    fn set_renderings(&mut self, value: Vec<Rendering>);
}
dyn_clone::clone_trait_object!(UserTaskTypeMut);
impl_downcast!(UserTaskTypeMut);
