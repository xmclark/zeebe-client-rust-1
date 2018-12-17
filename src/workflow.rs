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

impl From<gateway::GetWorkflowResponse> for Workflow {
    fn from(response: gateway::GetWorkflowResponse) -> Self {
        Workflow {
            bpmn_process_id: response.bpmnProcessId,
            version: response.version,
            workflow_key: response.workflowKey,
            resource_name: response.resourceName,
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


#[cfg(test)]
mod tests {

    use crate::gateway::{WorkflowMetadata, GetWorkflowResponse, CreateWorkflowInstanceResponse};
    use super::{Workflow, WorkflowInstance};

    #[test]
    fn from_workflow_metadata_to_workflow() {
        let bpmn_process_id = "test-process".to_string();
        let version = 12;
        let workflow_key = 34;
        let resource_name = "test-process.bpmn".to_string();

        let mut response = WorkflowMetadata::new();
        response.set_bpmnProcessId(bpmn_process_id.clone());
        response.set_version(version);
        response.set_workflowKey(workflow_key);
        response.set_resourceName(resource_name.clone());

        assert_eq!(Workflow {
            bpmn_process_id,
            version,
            workflow_key,
            resource_name,
        }, response.into());
    }

    #[test]
    fn from_get_workflow_response_to_workflow() {
        let bpmn_process_id = "test-process".to_string();
        let version = 12;
        let workflow_key = 34;
        let resource_name = "test-process.bpmn".to_string();

        let mut response = GetWorkflowResponse::new();
        response.set_bpmnProcessId(bpmn_process_id.clone());
        response.set_version(version);
        response.set_workflowKey(workflow_key);
        response.set_resourceName(resource_name.clone());

        assert_eq!(Workflow {
            bpmn_process_id,
            version,
            workflow_key,
            resource_name,
        }, response.into());
    }

    #[test]
    fn from_create_workflow_instance_response_to_workflow_instance() {
        let bpmn_process_id = "test-process".to_string();
        let version = 12;
        let workflow_key = 34;
        let workflow_instance_key = 56;

        let mut response = CreateWorkflowInstanceResponse::new();
        response.set_bpmnProcessId(bpmn_process_id.clone());
        response.set_version(version);
        response.set_workflowKey(workflow_key);
        response.set_workflowInstanceKey(workflow_instance_key);

        assert_eq!(WorkflowInstance {
            bpmn_process_id,
            version,
            workflow_key,
            workflow_instance_key,
        }, response.into());
    }
}
