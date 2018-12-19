use std::fs;
use std::sync::Arc;

use futures::Stream;

use grpcio::{ChannelBuilder, EnvBuilder};

use crate::error::{Result, ZeebeClientError};
use crate::gateway;
use crate::gateway_grpc::GatewayClient;
use crate::job::Job;
use crate::topology::Topology;
use crate::workflow::{Workflow, WorkflowInstance};
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

        ZeebeClient { client }
    }

    pub fn topology(&self) -> Result<Topology> {
        let request = gateway::TopologyRequest::default();
        self.client
            .topology(&request)
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
        request.mut_workflows().push(resource);

        self.client
            .deploy_workflow(&request)
            .map(|r| (r.key, r.workflows.into_iter().map(Workflow::from).collect()))
            .map_err(ZeebeClientError::from)
    }

    pub fn list_workflows(&self) -> Result<Vec<Workflow>> {
        self.list_workflows_request(&gateway::ListWorkflowsRequest::new())
    }

    pub fn list_workflows_by_bpmn_process_id(
        &self,
        bpmn_process_id: &str,
    ) -> Result<Vec<Workflow>> {
        let mut request = gateway::ListWorkflowsRequest::new();
        request.set_bpmnProcessId(bpmn_process_id.to_string());

        self.list_workflows_request(&request)
    }

    fn list_workflows_request(
        &self,
        request: &gateway::ListWorkflowsRequest,
    ) -> Result<Vec<Workflow>> {
        self.client
            .list_workflows(request)
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

    pub fn get_workflow_by_version(
        &self,
        bpmn_process_id: &str,
        version: i32,
    ) -> Result<(Workflow, String)> {
        let mut request = gateway::GetWorkflowRequest::new();
        request.set_bpmnProcessId(bpmn_process_id.to_string());
        request.set_version(version);

        self.get_workflow_request(&request)
    }

    fn get_workflow_request(
        &self,
        request: &gateway::GetWorkflowRequest,
    ) -> Result<(Workflow, String)> {
        self.client
            .get_workflow(request)
            .map(|r| {
                let xml = r.bpmnXml.clone();
                (Workflow::from(r), xml)
            })
            .map_err(ZeebeClientError::from)
    }

    pub fn create_workflow_instance(
        &self,
        workflow_key: i64,
        payload: &str,
    ) -> Result<WorkflowInstance> {
        let mut request = gateway::CreateWorkflowInstanceRequest::new();
        request.set_workflowKey(workflow_key);
        request.set_payload(payload.to_string());

        self.create_workflow_instance_request(&request)
    }

    pub fn create_workflow_instance_from_latest(
        &self,
        bpmn_process_id: &str,
        payload: &str,
    ) -> Result<WorkflowInstance> {
        self.create_workflow_instance_from_version(bpmn_process_id, -1, payload)
    }

    pub fn create_workflow_instance_from_version(
        &self,
        bpmn_process_id: &str,
        version: i32,
        payload: &str,
    ) -> Result<WorkflowInstance> {
        let mut request = gateway::CreateWorkflowInstanceRequest::new();
        request.set_bpmnProcessId(bpmn_process_id.to_string());
        request.set_version(version);
        request.set_payload(payload.to_string());

        self.create_workflow_instance_request(&request)
    }

    fn create_workflow_instance_request(
        &self,
        request: &gateway::CreateWorkflowInstanceRequest,
    ) -> Result<WorkflowInstance> {
        self.client
            .create_workflow_instance(request)
            .map(WorkflowInstance::from)
            .map_err(ZeebeClientError::from)
    }

    pub fn cancel_instance(&self, workflow_instance_key: i64) -> Result<()> {
        let mut request = gateway::CancelWorkflowInstanceRequest::new();
        request.set_workflowInstanceKey(workflow_instance_key);

        self.client.cancel_workflow_instance(&request)?;

        Ok(())
    }

    pub fn activate_jobs(
        &self,
        job_type: &str,
        worker: &str,
        timeout: i64,
        amount: i32,
    ) -> Result<Vec<Job>> {
        let mut request = gateway::ActivateJobsRequest::new();
        request.set_field_type(job_type.to_string());
        request.set_worker(worker.to_string());
        request.set_timeout(timeout);
        request.set_amount(amount);

        let mut stream: grpcio::ClientSStreamReceiver<gateway::ActivateJobsResponse> =
            self.client.activate_jobs(&request)?;

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

    pub fn fail_job_with_message(
        &self,
        job_key: i64,
        retries: i32,
        error_message: &str,
    ) -> Result<()> {
        let mut request = gateway::FailJobRequest::new();
        request.set_jobKey(job_key);
        request.set_retries(retries);
        request.set_errorMessage(error_message.to_string());

        self.client.fail_job(&request)?;

        Ok(())
    }

    pub fn publish_message(
        &self,
        name: &str,
        correlation_key: &str,
        time_to_live: i64,
        payload: &str,
    ) -> Result<()> {
        self.publish_message_with_id(name, correlation_key, time_to_live, "", payload)
    }

    pub fn publish_message_with_id(
        &self,
        name: &str,
        correlation_key: &str,
        time_to_live: i64,
        message_id: &str,
        payload: &str,
    ) -> Result<()> {
        let mut request = gateway::PublishMessageRequest::new();
        request.set_name(name.to_string());
        request.set_correlationKey(correlation_key.to_string());
        request.set_timeToLive(time_to_live);
        request.set_messageId(message_id.to_string());
        request.set_payload(payload.to_string());

        self.client.publish_message(&request)?;

        Ok(())
    }

    pub fn update_job_retries(&self, job_key: i64, retries: i32) -> Result<()> {
        let mut request = gateway::UpdateJobRetriesRequest::new();
        request.set_jobKey(job_key);
        request.set_retries(retries);

        self.client.update_job_retries(&request)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::gateway;
    use crate::gateway_mock::{GrpcRequest, GrpcResponse, MockGateway};
    use crate::*;

    #[test]
    fn topology() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let cluster_size = 12;
        let partitions_count = 34;
        let replication_factor = 56;

        let broker1_node_id = 0;
        let broker1_host = "first";
        let broker1_port = 123;

        let broker2_node_id = 1;
        let broker2_host = "second";
        let broker2_port = 456;

        let partition1_id = 0;
        let partition2_id = 1;

        let mut response = gateway::TopologyResponse::new();
        response.set_clusterSize(cluster_size);
        response.set_partitionsCount(partitions_count);
        response.set_replicationFactor(replication_factor);

        let mut broker1 = create_broker_info(broker1_node_id, broker1_host, broker1_port);
        broker1.mut_partitions().push(create_partition(
            partition1_id,
            gateway::Partition_PartitionBrokerRole::LEADER,
        ));
        broker1.mut_partitions().push(create_partition(
            partition2_id,
            gateway::Partition_PartitionBrokerRole::FOLLOWER,
        ));
        response.mut_brokers().push(broker1);

        let mut broker2 = create_broker_info(broker2_node_id, broker2_host, broker2_port);
        broker2.mut_partitions().push(create_partition(
            partition1_id,
            gateway::Partition_PartitionBrokerRole::FOLLOWER,
        ));
        broker2.mut_partitions().push(create_partition(
            partition2_id,
            gateway::Partition_PartitionBrokerRole::LEADER,
        ));
        response.mut_brokers().push(broker2);

        gateway.set_response(GrpcResponse::Topology(response));

        // when
        let result = client.topology().unwrap();

        // then
        assert_eq!(
            Topology {
                brokers: vec![
                    Broker {
                        node_id: broker1_node_id,
                        host: broker1_host.to_string(),
                        port: broker1_port,
                        partitions: vec![
                            Partition {
                                partition_id: partition1_id,
                                role: PartitionRole::Leader,
                            },
                            Partition {
                                partition_id: partition2_id,
                                role: PartitionRole::Follower,
                            },
                        ],
                    },
                    Broker {
                        node_id: broker2_node_id,
                        host: broker2_host.to_string(),
                        port: broker2_port,
                        partitions: vec![
                            Partition {
                                partition_id: partition1_id,
                                role: PartitionRole::Follower,
                            },
                            Partition {
                                partition_id: partition2_id,
                                role: PartitionRole::Leader,
                            },
                        ],
                    },
                ],
                cluster_size,
                partitions_count,
                replication_factor,
            },
            result
        );

        if let GrpcRequest::Topology(_) = gateway.get_request() {
            // empty request
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn deploy_file() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let key = 123;
        let bpmn_process_id = "test-process";
        let version = 12;
        let workflow_key = 34;
        let resource_name = "etc/order-process.bpmn";
        let definition: &[u8] = &::std::fs::read(resource_name).unwrap();

        let mut metadata = gateway::WorkflowMetadata::new();
        metadata.set_bpmnProcessId(bpmn_process_id.to_string());
        metadata.set_version(version);
        metadata.set_workflowKey(workflow_key);
        metadata.set_resourceName(resource_name.to_string());

        let mut response = gateway::DeployWorkflowResponse::new();
        response.set_key(key);
        response.mut_workflows().push(metadata);

        gateway.set_response(GrpcResponse::DeployWorkflow(response));

        // when
        let result = client.deploy_file(resource_name).unwrap();

        assert_eq!(
            (
                key,
                vec![Workflow {
                    bpmn_process_id: bpmn_process_id.to_string(),
                    version,
                    workflow_key,
                    resource_name: resource_name.to_string(),
                }]
            ),
            result
        );

        if let GrpcRequest::DeployWorkflow(request) = gateway.get_request() {
            assert_eq!(1, request.get_workflows().len());
            let workflow = &request.get_workflows()[0];
            assert_eq!(resource_name, workflow.get_name());
            assert_eq!(definition, workflow.get_definition());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn list_workflows() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let bpmn_process_id = "test-process";
        let version = 12;
        let workflow_key = 34;
        let resource_name = "test-process.bpmn";

        let mut metadata = gateway::WorkflowMetadata::new();
        metadata.set_bpmnProcessId(bpmn_process_id.to_string());
        metadata.set_version(version);
        metadata.set_workflowKey(workflow_key);
        metadata.set_resourceName(resource_name.to_string());

        let mut response = gateway::ListWorkflowsResponse::new();
        response.mut_workflows().push(metadata);

        gateway.set_response(GrpcResponse::ListWorkflows(response));

        // when
        let result = client.list_workflows().unwrap();

        assert_eq!(
            vec![Workflow {
                bpmn_process_id: bpmn_process_id.to_string(),
                version,
                workflow_key,
                resource_name: resource_name.to_string(),
            }],
            result
        );

        if let GrpcRequest::ListWorkflows(request) = gateway.get_request() {
            assert_eq!("", request.get_bpmnProcessId())
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn list_workflows_by_bpmn_process_id() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let bpmn_process_id = "test-process";

        let response = gateway::ListWorkflowsResponse::new();

        gateway.set_response(GrpcResponse::ListWorkflows(response));

        // when
        client
            .list_workflows_by_bpmn_process_id(bpmn_process_id)
            .unwrap();

        // then
        if let GrpcRequest::ListWorkflows(request) = gateway.get_request() {
            assert_eq!(bpmn_process_id, request.get_bpmnProcessId())
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn get_workflow() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let workflow_key = 34;

        let response = gateway::GetWorkflowResponse::new();

        gateway.set_response(GrpcResponse::GetWorkflow(response));

        // when
        client.get_workflow(workflow_key).unwrap();

        // then
        if let GrpcRequest::GetWorkflow(request) = gateway.get_request() {
            assert_eq!(workflow_key, request.get_workflowKey());
            assert_eq!("", request.get_bpmnProcessId());
            assert_eq!(0, request.get_version());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn get_latest_workflow() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let bpmn_process_id = "test-process";
        let version = 12;
        let workflow_key = 34;
        let resource_name = "test-process.bpmn";
        let bpmn_xml = "<xml>";

        let mut response = gateway::GetWorkflowResponse::new();
        response.set_bpmnProcessId(bpmn_process_id.to_string());
        response.set_version(version);
        response.set_workflowKey(workflow_key);
        response.set_resourceName(resource_name.to_string());
        response.set_bpmnXml(bpmn_xml.to_string());

        gateway.set_response(GrpcResponse::GetWorkflow(response));

        // when
        let result = client.get_latest_workflow("test-process").unwrap();

        // then
        assert_eq!(
            (
                Workflow {
                    bpmn_process_id: bpmn_process_id.to_string(),
                    version,
                    workflow_key,
                    resource_name: resource_name.to_string(),
                },
                bpmn_xml.to_string()
            ),
            result
        );

        if let GrpcRequest::GetWorkflow(request) = gateway.get_request() {
            assert_eq!(bpmn_process_id, request.get_bpmnProcessId());
            assert_eq!(-1, request.get_version());
            assert_eq!(0, request.get_workflowKey());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn get_workflow_by_version() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let bpmn_process_id = "test-process";
        let version = 34;

        let response = gateway::GetWorkflowResponse::new();

        gateway.set_response(GrpcResponse::GetWorkflow(response));

        // when
        client
            .get_workflow_by_version(bpmn_process_id, version)
            .unwrap();

        // then
        if let GrpcRequest::GetWorkflow(request) = gateway.get_request() {
            assert_eq!(bpmn_process_id, request.get_bpmnProcessId());
            assert_eq!(version, request.get_version());
            assert_eq!(0, request.get_workflowKey());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn create_workflow_instance() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let bpmn_process_id = "test-process";
        let version = 12;
        let workflow_key = 34;
        let workflow_instance_key = 56;
        let payload = r#"{"hello": "world"}"#;

        let mut response = gateway::CreateWorkflowInstanceResponse::new();
        response.set_bpmnProcessId(bpmn_process_id.to_string());
        response.set_version(version);
        response.set_workflowKey(workflow_key);
        response.set_workflowInstanceKey(workflow_instance_key);

        gateway.set_response(GrpcResponse::CreateWorkflowInstance(response));

        // when
        let result = client
            .create_workflow_instance(workflow_key, payload)
            .unwrap();

        // then
        assert_eq!(
            WorkflowInstance {
                bpmn_process_id: bpmn_process_id.to_string(),
                version,
                workflow_key,
                workflow_instance_key,
            },
            result
        );

        if let GrpcRequest::CreateWorkflowInstance(request) = gateway.get_request() {
            assert_eq!(workflow_key, request.get_workflowKey());
            assert_eq!(payload, request.get_payload());
            assert_eq!("", request.get_bpmnProcessId());
            assert_eq!(0, request.get_version());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn create_workflow_instance_from_latest() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let bpmn_process_id = "test-process";
        let payload = r#"{"hello": "world"}"#;

        let response = gateway::CreateWorkflowInstanceResponse::new();

        gateway.set_response(GrpcResponse::CreateWorkflowInstance(response));

        // when
        client
            .create_workflow_instance_from_latest(bpmn_process_id, payload)
            .unwrap();

        // then
        if let GrpcRequest::CreateWorkflowInstance(request) = gateway.get_request() {
            assert_eq!(bpmn_process_id, request.get_bpmnProcessId());
            assert_eq!(payload, request.get_payload());
            assert_eq!(-1, request.get_version());
            assert_eq!(0, request.get_workflowKey());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn create_workflow_instance_from_version() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let bpmn_process_id = "test-process";
        let version = 234;
        let payload = r#"{"hello": "world"}"#;

        let response = gateway::CreateWorkflowInstanceResponse::new();

        gateway.set_response(GrpcResponse::CreateWorkflowInstance(response));

        // when
        client
            .create_workflow_instance_from_version(bpmn_process_id, version, payload)
            .unwrap();

        // then
        if let GrpcRequest::CreateWorkflowInstance(request) = gateway.get_request() {
            assert_eq!(bpmn_process_id, request.get_bpmnProcessId());
            assert_eq!(payload, request.get_payload());
            assert_eq!(version, request.get_version());
            assert_eq!(0, request.get_workflowKey());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn cancel_instance() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let workflow_instance_key = 123;

        let response = gateway::CancelWorkflowInstanceResponse::new();

        gateway.set_response(GrpcResponse::CancelWorkflowInstance(response));

        // when
        client.cancel_instance(workflow_instance_key).unwrap();

        // then
        if let GrpcRequest::CancelWorkflowInstance(request) = gateway.get_request() {
            assert_eq!(workflow_instance_key, request.get_workflowInstanceKey());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn complete_job() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let job_key = 123;
        let payload = r#"{"hello": "world"}"#;

        let response = gateway::CompleteJobResponse::new();

        gateway.set_response(GrpcResponse::CompleteJob(response));

        // when
        client.complete_job(job_key, payload).unwrap();

        // then
        if let GrpcRequest::CompleteJob(request) = gateway.get_request() {
            assert_eq!(job_key, request.get_jobKey());
            assert_eq!(payload, request.get_payload());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn fail_job() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let job_key = 123;
        let retries = 32;

        let response = gateway::FailJobResponse::new();

        gateway.set_response(GrpcResponse::FailJob(response));

        // when
        client.fail_job(job_key, retries).unwrap();

        // then
        if let GrpcRequest::FailJob(request) = gateway.get_request() {
            assert_eq!(job_key, request.get_jobKey());
            assert_eq!(retries, request.get_retries());
            assert_eq!("", request.get_errorMessage());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn fail_job_with_message() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let job_key = 123;
        let retries = 32;
        let message = "failed";

        let response = gateway::FailJobResponse::new();

        gateway.set_response(GrpcResponse::FailJob(response));

        // when
        client
            .fail_job_with_message(job_key, retries, message)
            .unwrap();

        // then
        if let GrpcRequest::FailJob(request) = gateway.get_request() {
            assert_eq!(job_key, request.get_jobKey());
            assert_eq!(retries, request.get_retries());
            assert_eq!(message, request.get_errorMessage());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn publish_message() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let name = "test-msg";
        let correlation_key = "order-123";
        let time_to_live = 1_200;
        let payload = r#"{"hello": "world"}"#;

        let response = gateway::PublishMessageResponse::new();

        gateway.set_response(GrpcResponse::PublishMessage(response));

        // when
        client
            .publish_message(name, correlation_key, time_to_live, payload)
            .unwrap();

        // then
        if let GrpcRequest::PublishMessage(request) = gateway.get_request() {
            assert_eq!(name, request.get_name());
            assert_eq!(correlation_key, request.get_correlationKey());
            assert_eq!(time_to_live, request.get_timeToLive());
            assert_eq!("", request.get_messageId());
            assert_eq!(payload, request.get_payload());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn publish_message_with_id() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let name = "test-msg";
        let correlation_key = "order-123";
        let time_to_live = 1_200;
        let message_id = "unique";
        let payload = r#"{"hello": "world"}"#;

        let response = gateway::PublishMessageResponse::new();

        gateway.set_response(GrpcResponse::PublishMessage(response));

        // when
        client
            .publish_message_with_id(name, correlation_key, time_to_live, message_id, payload)
            .unwrap();

        // then
        if let GrpcRequest::PublishMessage(request) = gateway.get_request() {
            assert_eq!(name, request.get_name());
            assert_eq!(correlation_key, request.get_correlationKey());
            assert_eq!(time_to_live, request.get_timeToLive());
            assert_eq!(message_id, request.get_messageId());
            assert_eq!(payload, request.get_payload());
        } else {
            panic!("Wrong request received");
        }
    }

    #[test]
    fn update_job_retries() {
        // given
        let (gateway, client, _server) = MockGateway::init();

        let job_key = 123;
        let retries = 43;

        let response = gateway::UpdateJobRetriesResponse::new();

        gateway.set_response(GrpcResponse::UpdateJobRetries(response));

        // when
        client
            .update_job_retries(job_key, retries)
            .unwrap();

        // then
        if let GrpcRequest::UpdateJobRetries(request) = gateway.get_request() {
            assert_eq!(job_key, request.get_jobKey());
            assert_eq!(retries, request.get_retries());
        } else {
            panic!("Wrong request received");
        }
    }


    fn create_broker_info(node_id: i32, host: &str, port: i32) -> gateway::BrokerInfo {
        let mut broker = gateway::BrokerInfo::new();
        broker.set_nodeId(node_id);
        broker.set_host(host.to_string());
        broker.set_port(port);
        broker
    }

    fn create_partition(
        partition_id: i32,
        role: gateway::Partition_PartitionBrokerRole,
    ) -> gateway::Partition {
        let mut partition = gateway::Partition::new();
        partition.set_partitionId(partition_id);
        partition.set_role(role);
        partition
    }
}
