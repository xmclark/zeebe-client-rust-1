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

#[cfg(test)]
mod tests {
    use super::{Job, JobHeaders};
    use crate::gateway::ActivatedJob;

    #[test]
    fn from_activated_job_to_job() {
        let key = 12;
        let job_type = "foo";
        let workflow_instance_key = 123;
        let bpmn_process_id = "test-process";
        let workflow_definition_version = 456;
        let workflow_key = 789;
        let element_id = "start_event";
        let element_instance_key = 1234;
        let custom_headers = r#"{"one": "two"}"#;
        let worker = "testWorker";
        let retries = 34;
        let deadline = 56;
        let payload = r#"{"hello": "world"}"#;

        let mut response = ActivatedJob::new();
        response.set_key(key);
        response.set_field_type(job_type.to_string());
        let headers = response.mut_jobHeaders();
        headers.set_workflowInstanceKey(workflow_instance_key);
        headers.set_bpmnProcessId(bpmn_process_id.to_string());
        headers.set_workflowDefinitionVersion(workflow_definition_version);
        headers.set_workflowKey(workflow_key);
        headers.set_elementId(element_id.to_string());
        headers.set_elementInstanceKey(element_instance_key);
        response.set_customHeaders(custom_headers.to_string());
        response.set_worker(worker.to_string());
        response.set_retries(retries);
        response.set_deadline(deadline);
        response.set_payload(payload.to_string());

        assert_eq!(
            Job {
                key,
                job_type: job_type.to_string(),
                job_headers: JobHeaders {
                    workflow_instance_key,
                    bpmn_process_id: bpmn_process_id.to_string(),
                    workflow_definition_version,
                    workflow_key,
                    element_id: element_id.to_string(),
                    element_instance_key,
                },
                custom_headers: custom_headers.to_string(),
                worker: worker.to_string(),
                retries,
                deadline,
                payload: payload.to_string(),
            },
            response.into()
        );
    }
}
