//! # serde utilities
use crate::bpmn::schema::*;
use serde::ser::Error;
use serde::*;

macro_rules! downcast_and_serialize {
    ($value: ident, $serializer: ident, $($type: ty),*) => {{
        $(
            if let Some(object) = $value.downcast_ref::<$type>() {
                return object.serialize($serializer)
            }
        )*
        return Err(S::Error::custom("unsupported type".to_string()));
    }};
}

// The below lint is enabled because if the advice is follwed,
// this will be the error:
//
// ```
// 66 | #[derive(Clone, Debug, Serialize)]
//                        ^^^^^^^^^ the trait `bpxe_bpmn_schema::FlowNodeType`
//                        is not implemented for `Box<(dyn bpxe_bpmn_schema::FlowNodeType + 'static)>`
// ```
#[allow(clippy::borrowed_box)]
pub(crate) fn serialize_flow_node<S>(
    value: &Box<dyn FlowNodeType>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    downcast_and_serialize!(
        value,
        serializer,
        AdHocSubProcess,
        BoundaryEvent,
        BusinessRuleTask,
        CallActivity,
        CallChoreography,
        ChoreographyTask,
        ComplexGateway,
        EndEvent,
        EventBasedGateway,
        ExclusiveGateway,
        Gateway,
        ImplicitThrowEvent,
        InclusiveGateway,
        IntermediateCatchEvent,
        IntermediateThrowEvent,
        ManualTask,
        ParallelGateway,
        ReceiveTask,
        ScriptTask,
        SendTask,
        ServiceTask,
        StartEvent,
        SubChoreography,
        SubProcess,
        Task,
        Transaction,
        UserTask
    );
}
