use std::sync::Arc;
use std::fs;

use futures::Stream;

use grpcio::{
    ChannelBuilder,
    EnvBuilder,
};

use crate::gateway;
use crate::gateway_grpc::GatewayClient;
use crate::error::{Result, ZeebeClientError};
use crate::topology::Topology;
use crate::workflow::{Workflow, WorkflowInstance};
use crate::job::Job;
use futures::future::Future;

pub struct ZeebeClient {
    client: GatewayClient,
}

impl Default for ZeebeClient {
    fn default() -> Self {
        ZeebeClient::new("localhost:26500")
    }
}

impl ZeebeClient {
    pub fn new(address: &str) -> Self {
        let env = Arc::new(EnvBuilder::new().build());
        let channel = ChannelBuilder::new(env).connect(address);
        let client = GatewayClient::new(channel);

        ZeebeClient {
            client,
        }
    }

    pub fn topology(&self) -> Result<Topology> {
        let request = gateway::TopologyRequest::default();
        self.client.topology(&request)
            .map(Topology::from)
            .map_err(ZeebeClientError::from)
    }

    pub fn deploy_file(&self, filename: &str) -> Result<(i64, Vec<Workflow>)> {
        let definition = fs::read(filename)?;

        let mut resource = gateway::WorkflowRequestObject::new();
        resource.set_field_type(gateway::WorkflowRequestObject_ResourceType::FILE);
        resource.set_name(filename.to_string());
        resource.set_definition(definition);

        let mut request = gateway::DeployWorkflowRequest::new();
        request.mut_workflows()
            .push(resource);

        self.client.deploy_workflow(&request)
            .map(|r| (r.key, r.workflows.into_iter().map(Workflow::from).collect()))
            .map_err(ZeebeClientError::from)
    }

    pub fn list_workflows(&self) -> Result<Vec<Workflow>> {
        self.list_workflows_request(&gateway::ListWorkflowsRequest::new())
    }

    pub fn list_workflows_by_bpmn_process_id(&self, bpmn_process_id: &str) -> Result<Vec<Workflow>> {
        let mut request = gateway::ListWorkflowsRequest::new();
        request.set_bpmnProcessId(bpmn_process_id.to_string());

        self.list_workflows_request(&request)
    }

    fn list_workflows_request(&self, request: &gateway::ListWorkflowsRequest) -> Result<Vec<Workflow>> {
        self.client.list_workflows(request)
            .map(|r| r.workflows.into_iter().map(Workflow::from).collect())
            .map_err(ZeebeClientError::from)
    }

    pub fn get_workflow(&self, workflow_key: i64) -> Result<(Workflow, String)> {
        let mut request = gateway::GetWorkflowRequest::new();
        request.set_workflowKey(workflow_key);

        self.get_workflow_request(&request)
    }

    pub fn get_latest_workflow(&self, bpmn_process_id: &str) -> Result<(Workflow, String)> {
        self.get_workflow_by_version(bpmn_process_id, -1)
    }

    pub fn get_workflow_by_version(&self, bpmn_process_id: &str, version: i32) -> Result<(Workflow, String)> {
        let mut request = gateway::GetWorkflowRequest::new();
        request.set_bpmnProcessId(bpmn_process_id.to_string());
        request.set_version(version);

        self.get_workflow_request(&request)
    }

    fn get_workflow_request(&self, request: &gateway::GetWorkflowRequest) -> Result<(Workflow, String)> {
        self.client.get_workflow(request)
            .map(|r| (Workflow::from(&r), r.bpmnXml))
            .map_err(ZeebeClientError::from)
    }

    pub fn create_instance(&self, workflow_key: i64, payload: &str) -> Result<WorkflowInstance> {
        let mut request = gateway::CreateWorkflowInstanceRequest::new();
        request.set_workflowKey(workflow_key);
        request.set_payload(payload.to_string());

        self.create_instance_request(&request)
    }

    pub fn create_instance_from_latest(&self, bpmn_process_id: &str, payload: &str) -> Result<WorkflowInstance> {
        self.create_instance_from_version(bpmn_process_id, -1, payload)
    }

    pub fn create_instance_from_version(&self, bpmn_process_id: &str, version: i32, payload: &str) -> Result<WorkflowInstance> {
        let mut request = gateway::CreateWorkflowInstanceRequest::new();
        request.set_bpmnProcessId(bpmn_process_id.to_string());
        request.set_version(version);
        request.set_payload(payload.to_string());

        self.create_instance_request(&request)
    }

    fn create_instance_request(&self, request: &gateway::CreateWorkflowInstanceRequest) -> Result<WorkflowInstance> {
        self.client.create_workflow_instance(request)
            .map(WorkflowInstance::from)
            .map_err(ZeebeClientError::from)
    }

    pub fn cancel_instance(&self, workflow_instance_key: i64) -> Result<()> {
        let mut request = gateway::CancelWorkflowInstanceRequest::new();
        request.set_workflowInstanceKey(workflow_instance_key);

        self.client.cancel_workflow_instance(&request)?;

        Ok(())
    }

    pub fn activate_jobs(&self, job_type: &str, worker: &str, timeout: i64, amount: i32) -> Result<Vec<Job>> {
        let mut request = gateway::ActivateJobsRequest::new();
        request.set_field_type(job_type.to_string());
        request.set_worker(worker.to_string());
        request.set_timeout(timeout);
        request.set_amount(amount);

        let mut stream: grpcio::ClientSStreamReceiver<gateway::ActivateJobsResponse> = self.client.activate_jobs(&request)?;

        let mut jobs = Vec::new();

        loop {
            let f = stream.into_future();
            match f.wait() {
                Ok((Some(mut response), s)) => {
                    stream = s;
                    jobs.extend(response.take_jobs().into_iter().map(Job::from));
                }
                Ok((None, _)) => break,
                Err((e, _)) => panic!("Activating of jobs failed: {:?}", e),
            }
        }

        Ok(jobs)
    }

    pub fn complete_job(&self, job_key: i64, payload: &str) -> Result<()> {
        let mut request = gateway::CompleteJobRequest::new();
        request.set_jobKey(job_key);
        request.set_payload(payload.to_string());

        self.client.complete_job(&request)?;

        Ok(())
    }

    pub fn fail_job(&self, job_key: i64, retries: i32) -> Result<()> {
        self.fail_job_with_message(job_key, retries, "")
    }

    pub fn fail_job_with_message(&self, job_key: i64, retries: i32, error_message: &str) -> Result<()> {
        let mut request = gateway::FailJobRequest::new();
        request.set_jobKey(job_key);
        request.set_retries(retries);
        request.set_errorMessage(error_message.to_string());

        self.client.fail_job(&request);

        Ok(())
    }

    pub fn publish_message(&self, name: &str, correlation_key: &str, time_to_live: i64, payload: &str) -> Result<()> {
        self.publish_message_with_id(name, correlation_key, time_to_live, "", payload)
    }

    pub fn publish_message_with_id(&self, name: &str, correlation_key: &str, time_to_live: i64, message_id: &str, payload: &str) -> Result<()> {
        let mut request = gateway::PublishMessageRequest::new();
        request.set_name(name.to_string());
        request.set_correlationKey(correlation_key.to_string());
        request.set_timeToLive(time_to_live);
        request.set_messageId(message_id.to_string());
        request.set_payload(payload.to_string());

        self.client.publish_message(&request)?;

        Ok(())

    }
}
