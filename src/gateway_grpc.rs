// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_GATEWAY_TOPOLOGY: ::grpcio::Method<super::gateway::TopologyRequest, super::gateway::TopologyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gateway_protocol.Gateway/Topology",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATEWAY_DEPLOY_WORKFLOW: ::grpcio::Method<super::gateway::DeployWorkflowRequest, super::gateway::DeployWorkflowResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gateway_protocol.Gateway/DeployWorkflow",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATEWAY_PUBLISH_MESSAGE: ::grpcio::Method<super::gateway::PublishMessageRequest, super::gateway::PublishMessageResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gateway_protocol.Gateway/PublishMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATEWAY_UPDATE_JOB_RETRIES: ::grpcio::Method<super::gateway::UpdateJobRetriesRequest, super::gateway::UpdateJobRetriesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gateway_protocol.Gateway/UpdateJobRetries",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATEWAY_FAIL_JOB: ::grpcio::Method<super::gateway::FailJobRequest, super::gateway::FailJobResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gateway_protocol.Gateway/FailJob",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATEWAY_COMPLETE_JOB: ::grpcio::Method<super::gateway::CompleteJobRequest, super::gateway::CompleteJobResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gateway_protocol.Gateway/CompleteJob",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATEWAY_CREATE_WORKFLOW_INSTANCE: ::grpcio::Method<super::gateway::CreateWorkflowInstanceRequest, super::gateway::CreateWorkflowInstanceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gateway_protocol.Gateway/CreateWorkflowInstance",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATEWAY_CANCEL_WORKFLOW_INSTANCE: ::grpcio::Method<super::gateway::CancelWorkflowInstanceRequest, super::gateway::CancelWorkflowInstanceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gateway_protocol.Gateway/CancelWorkflowInstance",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATEWAY_UPDATE_WORKFLOW_INSTANCE_PAYLOAD: ::grpcio::Method<super::gateway::UpdateWorkflowInstancePayloadRequest, super::gateway::UpdateWorkflowInstancePayloadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gateway_protocol.Gateway/UpdateWorkflowInstancePayload",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATEWAY_ACTIVATE_JOBS: ::grpcio::Method<super::gateway::ActivateJobsRequest, super::gateway::ActivateJobsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/gateway_protocol.Gateway/ActivateJobs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATEWAY_LIST_WORKFLOWS: ::grpcio::Method<super::gateway::ListWorkflowsRequest, super::gateway::ListWorkflowsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gateway_protocol.Gateway/ListWorkflows",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATEWAY_GET_WORKFLOW: ::grpcio::Method<super::gateway::GetWorkflowRequest, super::gateway::GetWorkflowResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gateway_protocol.Gateway/GetWorkflow",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GATEWAY_RESOLVE_INCIDENT: ::grpcio::Method<super::gateway::ResolveIncidentRequest, super::gateway::ResolveIncidentResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gateway_protocol.Gateway/ResolveIncident",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GatewayClient {
    client: ::grpcio::Client,
}

impl GatewayClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GatewayClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn topology_opt(&self, req: &super::gateway::TopologyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gateway::TopologyResponse> {
        self.client.unary_call(&METHOD_GATEWAY_TOPOLOGY, req, opt)
    }

    pub fn topology(&self, req: &super::gateway::TopologyRequest) -> ::grpcio::Result<super::gateway::TopologyResponse> {
        self.topology_opt(req, ::grpcio::CallOption::default())
    }

    pub fn topology_async_opt(&self, req: &super::gateway::TopologyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::TopologyResponse>> {
        self.client.unary_call_async(&METHOD_GATEWAY_TOPOLOGY, req, opt)
    }

    pub fn topology_async(&self, req: &super::gateway::TopologyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::TopologyResponse>> {
        self.topology_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn deploy_workflow_opt(&self, req: &super::gateway::DeployWorkflowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gateway::DeployWorkflowResponse> {
        self.client.unary_call(&METHOD_GATEWAY_DEPLOY_WORKFLOW, req, opt)
    }

    pub fn deploy_workflow(&self, req: &super::gateway::DeployWorkflowRequest) -> ::grpcio::Result<super::gateway::DeployWorkflowResponse> {
        self.deploy_workflow_opt(req, ::grpcio::CallOption::default())
    }

    pub fn deploy_workflow_async_opt(&self, req: &super::gateway::DeployWorkflowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::DeployWorkflowResponse>> {
        self.client.unary_call_async(&METHOD_GATEWAY_DEPLOY_WORKFLOW, req, opt)
    }

    pub fn deploy_workflow_async(&self, req: &super::gateway::DeployWorkflowRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::DeployWorkflowResponse>> {
        self.deploy_workflow_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn publish_message_opt(&self, req: &super::gateway::PublishMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gateway::PublishMessageResponse> {
        self.client.unary_call(&METHOD_GATEWAY_PUBLISH_MESSAGE, req, opt)
    }

    pub fn publish_message(&self, req: &super::gateway::PublishMessageRequest) -> ::grpcio::Result<super::gateway::PublishMessageResponse> {
        self.publish_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn publish_message_async_opt(&self, req: &super::gateway::PublishMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::PublishMessageResponse>> {
        self.client.unary_call_async(&METHOD_GATEWAY_PUBLISH_MESSAGE, req, opt)
    }

    pub fn publish_message_async(&self, req: &super::gateway::PublishMessageRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::PublishMessageResponse>> {
        self.publish_message_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_job_retries_opt(&self, req: &super::gateway::UpdateJobRetriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gateway::UpdateJobRetriesResponse> {
        self.client.unary_call(&METHOD_GATEWAY_UPDATE_JOB_RETRIES, req, opt)
    }

    pub fn update_job_retries(&self, req: &super::gateway::UpdateJobRetriesRequest) -> ::grpcio::Result<super::gateway::UpdateJobRetriesResponse> {
        self.update_job_retries_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_job_retries_async_opt(&self, req: &super::gateway::UpdateJobRetriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::UpdateJobRetriesResponse>> {
        self.client.unary_call_async(&METHOD_GATEWAY_UPDATE_JOB_RETRIES, req, opt)
    }

    pub fn update_job_retries_async(&self, req: &super::gateway::UpdateJobRetriesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::UpdateJobRetriesResponse>> {
        self.update_job_retries_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fail_job_opt(&self, req: &super::gateway::FailJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gateway::FailJobResponse> {
        self.client.unary_call(&METHOD_GATEWAY_FAIL_JOB, req, opt)
    }

    pub fn fail_job(&self, req: &super::gateway::FailJobRequest) -> ::grpcio::Result<super::gateway::FailJobResponse> {
        self.fail_job_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fail_job_async_opt(&self, req: &super::gateway::FailJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::FailJobResponse>> {
        self.client.unary_call_async(&METHOD_GATEWAY_FAIL_JOB, req, opt)
    }

    pub fn fail_job_async(&self, req: &super::gateway::FailJobRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::FailJobResponse>> {
        self.fail_job_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn complete_job_opt(&self, req: &super::gateway::CompleteJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gateway::CompleteJobResponse> {
        self.client.unary_call(&METHOD_GATEWAY_COMPLETE_JOB, req, opt)
    }

    pub fn complete_job(&self, req: &super::gateway::CompleteJobRequest) -> ::grpcio::Result<super::gateway::CompleteJobResponse> {
        self.complete_job_opt(req, ::grpcio::CallOption::default())
    }

    pub fn complete_job_async_opt(&self, req: &super::gateway::CompleteJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::CompleteJobResponse>> {
        self.client.unary_call_async(&METHOD_GATEWAY_COMPLETE_JOB, req, opt)
    }

    pub fn complete_job_async(&self, req: &super::gateway::CompleteJobRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::CompleteJobResponse>> {
        self.complete_job_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_workflow_instance_opt(&self, req: &super::gateway::CreateWorkflowInstanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gateway::CreateWorkflowInstanceResponse> {
        self.client.unary_call(&METHOD_GATEWAY_CREATE_WORKFLOW_INSTANCE, req, opt)
    }

    pub fn create_workflow_instance(&self, req: &super::gateway::CreateWorkflowInstanceRequest) -> ::grpcio::Result<super::gateway::CreateWorkflowInstanceResponse> {
        self.create_workflow_instance_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_workflow_instance_async_opt(&self, req: &super::gateway::CreateWorkflowInstanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::CreateWorkflowInstanceResponse>> {
        self.client.unary_call_async(&METHOD_GATEWAY_CREATE_WORKFLOW_INSTANCE, req, opt)
    }

    pub fn create_workflow_instance_async(&self, req: &super::gateway::CreateWorkflowInstanceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::CreateWorkflowInstanceResponse>> {
        self.create_workflow_instance_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_workflow_instance_opt(&self, req: &super::gateway::CancelWorkflowInstanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gateway::CancelWorkflowInstanceResponse> {
        self.client.unary_call(&METHOD_GATEWAY_CANCEL_WORKFLOW_INSTANCE, req, opt)
    }

    pub fn cancel_workflow_instance(&self, req: &super::gateway::CancelWorkflowInstanceRequest) -> ::grpcio::Result<super::gateway::CancelWorkflowInstanceResponse> {
        self.cancel_workflow_instance_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_workflow_instance_async_opt(&self, req: &super::gateway::CancelWorkflowInstanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::CancelWorkflowInstanceResponse>> {
        self.client.unary_call_async(&METHOD_GATEWAY_CANCEL_WORKFLOW_INSTANCE, req, opt)
    }

    pub fn cancel_workflow_instance_async(&self, req: &super::gateway::CancelWorkflowInstanceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::CancelWorkflowInstanceResponse>> {
        self.cancel_workflow_instance_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_workflow_instance_payload_opt(&self, req: &super::gateway::UpdateWorkflowInstancePayloadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gateway::UpdateWorkflowInstancePayloadResponse> {
        self.client.unary_call(&METHOD_GATEWAY_UPDATE_WORKFLOW_INSTANCE_PAYLOAD, req, opt)
    }

    pub fn update_workflow_instance_payload(&self, req: &super::gateway::UpdateWorkflowInstancePayloadRequest) -> ::grpcio::Result<super::gateway::UpdateWorkflowInstancePayloadResponse> {
        self.update_workflow_instance_payload_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_workflow_instance_payload_async_opt(&self, req: &super::gateway::UpdateWorkflowInstancePayloadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::UpdateWorkflowInstancePayloadResponse>> {
        self.client.unary_call_async(&METHOD_GATEWAY_UPDATE_WORKFLOW_INSTANCE_PAYLOAD, req, opt)
    }

    pub fn update_workflow_instance_payload_async(&self, req: &super::gateway::UpdateWorkflowInstancePayloadRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::UpdateWorkflowInstancePayloadResponse>> {
        self.update_workflow_instance_payload_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn activate_jobs_opt(&self, req: &super::gateway::ActivateJobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::gateway::ActivateJobsResponse>> {
        self.client.server_streaming(&METHOD_GATEWAY_ACTIVATE_JOBS, req, opt)
    }

    pub fn activate_jobs(&self, req: &super::gateway::ActivateJobsRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::gateway::ActivateJobsResponse>> {
        self.activate_jobs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_workflows_opt(&self, req: &super::gateway::ListWorkflowsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gateway::ListWorkflowsResponse> {
        self.client.unary_call(&METHOD_GATEWAY_LIST_WORKFLOWS, req, opt)
    }

    pub fn list_workflows(&self, req: &super::gateway::ListWorkflowsRequest) -> ::grpcio::Result<super::gateway::ListWorkflowsResponse> {
        self.list_workflows_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_workflows_async_opt(&self, req: &super::gateway::ListWorkflowsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::ListWorkflowsResponse>> {
        self.client.unary_call_async(&METHOD_GATEWAY_LIST_WORKFLOWS, req, opt)
    }

    pub fn list_workflows_async(&self, req: &super::gateway::ListWorkflowsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::ListWorkflowsResponse>> {
        self.list_workflows_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_workflow_opt(&self, req: &super::gateway::GetWorkflowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gateway::GetWorkflowResponse> {
        self.client.unary_call(&METHOD_GATEWAY_GET_WORKFLOW, req, opt)
    }

    pub fn get_workflow(&self, req: &super::gateway::GetWorkflowRequest) -> ::grpcio::Result<super::gateway::GetWorkflowResponse> {
        self.get_workflow_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_workflow_async_opt(&self, req: &super::gateway::GetWorkflowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::GetWorkflowResponse>> {
        self.client.unary_call_async(&METHOD_GATEWAY_GET_WORKFLOW, req, opt)
    }

    pub fn get_workflow_async(&self, req: &super::gateway::GetWorkflowRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::GetWorkflowResponse>> {
        self.get_workflow_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn resolve_incident_opt(&self, req: &super::gateway::ResolveIncidentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gateway::ResolveIncidentResponse> {
        self.client.unary_call(&METHOD_GATEWAY_RESOLVE_INCIDENT, req, opt)
    }

    pub fn resolve_incident(&self, req: &super::gateway::ResolveIncidentRequest) -> ::grpcio::Result<super::gateway::ResolveIncidentResponse> {
        self.resolve_incident_opt(req, ::grpcio::CallOption::default())
    }

    pub fn resolve_incident_async_opt(&self, req: &super::gateway::ResolveIncidentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::ResolveIncidentResponse>> {
        self.client.unary_call_async(&METHOD_GATEWAY_RESOLVE_INCIDENT, req, opt)
    }

    pub fn resolve_incident_async(&self, req: &super::gateway::ResolveIncidentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gateway::ResolveIncidentResponse>> {
        self.resolve_incident_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Gateway {
    fn topology(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::TopologyRequest, sink: ::grpcio::UnarySink<super::gateway::TopologyResponse>);
    fn deploy_workflow(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::DeployWorkflowRequest, sink: ::grpcio::UnarySink<super::gateway::DeployWorkflowResponse>);
    fn publish_message(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::PublishMessageRequest, sink: ::grpcio::UnarySink<super::gateway::PublishMessageResponse>);
    fn update_job_retries(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::UpdateJobRetriesRequest, sink: ::grpcio::UnarySink<super::gateway::UpdateJobRetriesResponse>);
    fn fail_job(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::FailJobRequest, sink: ::grpcio::UnarySink<super::gateway::FailJobResponse>);
    fn complete_job(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::CompleteJobRequest, sink: ::grpcio::UnarySink<super::gateway::CompleteJobResponse>);
    fn create_workflow_instance(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::CreateWorkflowInstanceRequest, sink: ::grpcio::UnarySink<super::gateway::CreateWorkflowInstanceResponse>);
    fn cancel_workflow_instance(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::CancelWorkflowInstanceRequest, sink: ::grpcio::UnarySink<super::gateway::CancelWorkflowInstanceResponse>);
    fn update_workflow_instance_payload(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::UpdateWorkflowInstancePayloadRequest, sink: ::grpcio::UnarySink<super::gateway::UpdateWorkflowInstancePayloadResponse>);
    fn activate_jobs(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::ActivateJobsRequest, sink: ::grpcio::ServerStreamingSink<super::gateway::ActivateJobsResponse>);
    fn list_workflows(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::ListWorkflowsRequest, sink: ::grpcio::UnarySink<super::gateway::ListWorkflowsResponse>);
    fn get_workflow(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::GetWorkflowRequest, sink: ::grpcio::UnarySink<super::gateway::GetWorkflowResponse>);
    fn resolve_incident(&mut self, ctx: ::grpcio::RpcContext, req: super::gateway::ResolveIncidentRequest, sink: ::grpcio::UnarySink<super::gateway::ResolveIncidentResponse>);
}

pub fn create_gateway<S: Gateway + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATEWAY_TOPOLOGY, move |ctx, req, resp| {
        instance.topology(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATEWAY_DEPLOY_WORKFLOW, move |ctx, req, resp| {
        instance.deploy_workflow(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATEWAY_PUBLISH_MESSAGE, move |ctx, req, resp| {
        instance.publish_message(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATEWAY_UPDATE_JOB_RETRIES, move |ctx, req, resp| {
        instance.update_job_retries(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATEWAY_FAIL_JOB, move |ctx, req, resp| {
        instance.fail_job(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATEWAY_COMPLETE_JOB, move |ctx, req, resp| {
        instance.complete_job(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATEWAY_CREATE_WORKFLOW_INSTANCE, move |ctx, req, resp| {
        instance.create_workflow_instance(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATEWAY_CANCEL_WORKFLOW_INSTANCE, move |ctx, req, resp| {
        instance.cancel_workflow_instance(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATEWAY_UPDATE_WORKFLOW_INSTANCE_PAYLOAD, move |ctx, req, resp| {
        instance.update_workflow_instance_payload(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_GATEWAY_ACTIVATE_JOBS, move |ctx, req, resp| {
        instance.activate_jobs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATEWAY_LIST_WORKFLOWS, move |ctx, req, resp| {
        instance.list_workflows(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATEWAY_GET_WORKFLOW, move |ctx, req, resp| {
        instance.get_workflow(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GATEWAY_RESOLVE_INCIDENT, move |ctx, req, resp| {
        instance.resolve_incident(ctx, req, resp)
    });
    builder.build()
}
