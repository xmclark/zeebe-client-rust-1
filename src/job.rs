use crate::gateway;
use protobuf::SingularPtrField;

#[derive(Debug, PartialEq)]
pub struct JobHeaders {
    pub workflow_instance_key: i64,
    pub bpmn_process_id: String,
    pub workflow_definition_version: i32,
    pub workflow_key: i64,
    pub element_id: String,
    pub element_instance_key: i64,
}

#[derive(Debug, PartialEq)]
pub struct Job {
    pub key: i64,
    pub job_type: String,
    pub job_headers: JobHeaders,
    pub custom_headers: String,
    pub worker: String,
    pub retries: i32,
    pub deadline: i64,
    pub payload: String,
}

impl From<SingularPtrField<gateway::JobHeaders>> for JobHeaders {
    fn from(headers: SingularPtrField<gateway::JobHeaders>) -> Self {
        let headers = headers.get_ref();
        JobHeaders {
            workflow_instance_key: headers.workflowInstanceKey,
            bpmn_process_id: headers.bpmnProcessId.to_owned(),
            workflow_definition_version: headers.workflowDefinitionVersion,
            workflow_key: headers.workflowKey,
            element_id: headers.elementId.to_owned(),
            element_instance_key: headers.elementInstanceKey,
        }
    }
}

impl From<gateway::ActivatedJob> for Job {
    fn from(job: gateway::ActivatedJob) -> Self {
        Job {
            key: job.key,
            job_type: job.field_type,
            job_headers: job.jobHeaders.into(),
            custom_headers: job.customHeaders,
            worker: job.worker,
            retries: job.retries,
            deadline: job.deadline,
            payload: job.payload,
        }
    }
}