use crate::gateway;

#[derive(Debug, PartialEq)]
pub struct Workflow {
    pub bpmn_process_id: String,
    pub version: i32,
    pub workflow_key: i64,
    pub resource_name: String,
}

impl From<gateway::WorkflowMetadata> for Workflow {
    fn from(workflow: gateway::WorkflowMetadata) -> Self {
        Workflow {
            bpmn_process_id: workflow.bpmnProcessId,
            version: workflow.version,
            workflow_key: workflow.workflowKey,
            resource_name: workflow.resourceName,
        }
    }
}

impl From<&gateway::GetWorkflowResponse> for Workflow {
    fn from(response: &gateway::GetWorkflowResponse) -> Self {
        Workflow {
            bpmn_process_id: response.bpmnProcessId.to_owned(),
            version: response.version,
            workflow_key: response.workflowKey,
            resource_name: response.resourceName.to_owned(),
        }
    }

}

#[derive(Debug, PartialEq)]
pub struct WorkflowInstance {
    pub bpmn_process_id: String,
    pub version: i32,
    pub workflow_key: i64,
    pub workflow_instance_key: i64,
}

impl From<gateway::CreateWorkflowInstanceResponse> for WorkflowInstance {
    fn from(response: gateway::CreateWorkflowInstanceResponse) -> Self {
        WorkflowInstance {
            bpmn_process_id: response.bpmnProcessId,
            version: response.version,
            workflow_key: response.workflowKey,
            workflow_instance_key: response.workflowInstanceKey,
        }
    }
}
