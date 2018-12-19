use crate::gateway;
use crate::gateway_grpc::{self, Gateway};
use crate::ZeebeClient;
use futures::{Future, Sink};
use grpcio::{
    Environment, RpcContext, RpcStatus, RpcStatusCode, Server, ServerBuilder, ServerStreamingSink,
    UnarySink, WriteFlags,
};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct MockGateway {
    request: Arc<Mutex<Option<GrpcRequest>>>,
    response: Arc<Mutex<Option<GrpcResponse>>>,
}

#[derive(Debug, Clone)]
pub enum GrpcRequest {
    Topology(gateway::TopologyRequest),
    DeployWorkflow(gateway::DeployWorkflowRequest),
    PublishMessage(gateway::PublishMessageRequest),
    UpdateJobRetries(gateway::UpdateJobRetriesRequest),
    FailJob(gateway::FailJobRequest),
    CompleteJob(gateway::CompleteJobRequest),
    CreateWorkflowInstance(gateway::CreateWorkflowInstanceRequest),
    CancelWorkflowInstance(gateway::CancelWorkflowInstanceRequest),
    UpdateWorkflowInstancePayload(gateway::UpdateWorkflowInstancePayloadRequest),
    ActivateJobs(gateway::ActivateJobsRequest),
    ListWorkflows(gateway::ListWorkflowsRequest),
    GetWorkflow(gateway::GetWorkflowRequest),
    ResolveIncident(gateway::ResolveIncidentRequest),
}

#[derive(Debug, Clone)]
pub enum GrpcResponse {
    Topology(gateway::TopologyResponse),
    DeployWorkflow(gateway::DeployWorkflowResponse),
    PublishMessage(gateway::PublishMessageResponse),
    UpdateJobRetries(gateway::UpdateJobRetriesResponse),
    FailJob(gateway::FailJobResponse),
    CompleteJob(gateway::CompleteJobResponse),
    CreateWorkflowInstance(gateway::CreateWorkflowInstanceResponse),
    CancelWorkflowInstance(gateway::CancelWorkflowInstanceResponse),
    UpdateWorkflowInstancePayload(gateway::UpdateWorkflowInstancePayloadResponse),
    ActivateJobs(Vec<gateway::ActivateJobsResponse>),
    ListWorkflows(gateway::ListWorkflowsResponse),
    GetWorkflow(gateway::GetWorkflowResponse),
    ResolveIncident(gateway::ResolveIncidentResponse),
}

impl Default for MockGateway {
    fn default() -> Self {
        MockGateway {
            request: Arc::new(Mutex::new(None)),
            response: Arc::new(Mutex::new(None)),
        }
    }
}

impl MockGateway {
    pub fn set_request(&self, request: GrpcRequest) {
        *self.request.lock().unwrap() = Some(request);
    }

    pub fn get_request(&self) -> GrpcRequest {
        if let Some(ref r) = *self.request.lock().unwrap() {
            r.clone()
        } else {
            panic!("No request received");
        }
    }

    pub fn set_response(&self, response: GrpcResponse) {
        *self.response.lock().unwrap() = Some(response);
    }

    pub fn take_response(&mut self) -> Option<GrpcResponse> {
        self.response.lock().unwrap().take()
    }

    pub fn init() -> (Arc<Self>, ZeebeClient, Server) {
        let gateway = Arc::new(Self::default());
        let env = Arc::new(Environment::new(1));
        let server = {
            let gateway: &MockGateway = &gateway.clone();
            let service = gateway_grpc::create_gateway(gateway);
            let mut server = ServerBuilder::new(env)
                .register_service(service)
                .bind("0.0.0.0", 0)
                .build()
                .unwrap();
            server.start();
            server
        };

        let (_, port) = server.bind_addrs()[0];

        (
            gateway,
            ZeebeClient::new(&format!("localhost:{}", port)),
            server,
        )
    }
}

macro_rules! mock {
    ($gateway:ident, $ctx:ident, $sink:ident, $request:ident, $variant:ident) => {
        $gateway.set_request(GrpcRequest::$variant($request));
        match $gateway.take_response() {
            Some(GrpcResponse::$variant(ref response)) => {
                let f = $sink
                    .success(response.clone())
                    .map_err(|e| panic!("Failed to respond: {}", e));
                $ctx.spawn(f);
            }
            Some(_) => {
                let f = $sink
                    .fail(RpcStatus::new(
                        RpcStatusCode::Internal,
                        Some(format!("Different response mocked")),
                    ))
                    .map_err(|e| panic!("Failed to respond: {}", e));
                $ctx.spawn(f);
            }
            None => {
                let f = $sink
                    .fail(RpcStatus::new(
                        RpcStatusCode::Unimplemented,
                        Some(format!("No response mocked")),
                    ))
                    .map_err(|e| panic!("Failed to respond: {}", e));
                $ctx.spawn(f);
            }
        }
    };
}

impl Gateway for MockGateway {
    fn topology(
        &mut self,
        ctx: RpcContext,
        req: gateway::TopologyRequest,
        sink: UnarySink<gateway::TopologyResponse>,
    ) {
        mock!(self, ctx, sink, req, Topology);
    }

    fn deploy_workflow(
        &mut self,
        ctx: RpcContext,
        req: gateway::DeployWorkflowRequest,
        sink: UnarySink<gateway::DeployWorkflowResponse>,
    ) {
        mock!(self, ctx, sink, req, DeployWorkflow);
    }

    fn publish_message(
        &mut self,
        ctx: RpcContext,
        req: gateway::PublishMessageRequest,
        sink: UnarySink<gateway::PublishMessageResponse>,
    ) {
        mock!(self, ctx, sink, req, PublishMessage);
    }

    fn update_job_retries(
        &mut self,
        ctx: RpcContext,
        req: gateway::UpdateJobRetriesRequest,
        sink: UnarySink<gateway::UpdateJobRetriesResponse>,
    ) {
        mock!(self, ctx, sink, req, UpdateJobRetries);
    }

    fn fail_job(
        &mut self,
        ctx: RpcContext,
        req: gateway::FailJobRequest,
        sink: UnarySink<gateway::FailJobResponse>,
    ) {
        mock!(self, ctx, sink, req, FailJob);
    }

    fn complete_job(
        &mut self,
        ctx: RpcContext,
        req: gateway::CompleteJobRequest,
        sink: UnarySink<gateway::CompleteJobResponse>,
    ) {
        mock!(self, ctx, sink, req, CompleteJob);
    }

    fn create_workflow_instance(
        &mut self,
        ctx: RpcContext,
        req: gateway::CreateWorkflowInstanceRequest,
        sink: UnarySink<gateway::CreateWorkflowInstanceResponse>,
    ) {
        mock!(self, ctx, sink, req, CreateWorkflowInstance);
    }

    fn cancel_workflow_instance(
        &mut self,
        ctx: RpcContext,
        req: gateway::CancelWorkflowInstanceRequest,
        sink: UnarySink<gateway::CancelWorkflowInstanceResponse>,
    ) {
        mock!(self, ctx, sink, req, CancelWorkflowInstance);
    }

    fn update_workflow_instance_payload(
        &mut self,
        ctx: RpcContext,
        req: gateway::UpdateWorkflowInstancePayloadRequest,
        sink: UnarySink<gateway::UpdateWorkflowInstancePayloadResponse>,
    ) {
        mock!(self, ctx, sink, req, UpdateWorkflowInstancePayload);
    }

    fn activate_jobs(
        &mut self,
        ctx: RpcContext,
        req: gateway::ActivateJobsRequest,
        sink: ServerStreamingSink<gateway::ActivateJobsResponse>,
    ) {
        self.set_request(GrpcRequest::ActivateJobs(req));
        match self.take_response() {
            Some(GrpcResponse::ActivateJobs(ref responses)) => {
                let responses: Vec<_> = responses
                    .into_iter()
                    .map(|r| (r.clone(), WriteFlags::default()))
                    .collect();

                let f = sink
                    .send_all(futures::stream::iter_ok::<_, grpcio::Error>(responses))
                    .map(|_| {})
                    .map_err(|e| panic!("Failed to respond: {}", e));
                ctx.spawn(f);
            }
            Some(_) => {
                let f = sink
                    .fail(RpcStatus::new(
                        RpcStatusCode::Internal,
                        Some(format!("Different response mocked")),
                    ))
                    .map_err(|e| panic!("Failed to respond: {}", e));
                ctx.spawn(f);
            }
            None => {
                let f = sink
                    .fail(RpcStatus::new(
                        RpcStatusCode::Unimplemented,
                        Some(format!("No response mocked")),
                    ))
                    .map_err(|e| panic!("Failed to respond: {}", e));
                ctx.spawn(f);
            }
        }
    }

    fn list_workflows(
        &mut self,
        ctx: RpcContext,
        req: gateway::ListWorkflowsRequest,
        sink: UnarySink<gateway::ListWorkflowsResponse>,
    ) {
        mock!(self, ctx, sink, req, ListWorkflows);
    }

    fn get_workflow(
        &mut self,
        ctx: RpcContext,
        req: gateway::GetWorkflowRequest,
        sink: UnarySink<gateway::GetWorkflowResponse>,
    ) {
        mock!(self, ctx, sink, req, GetWorkflow);
    }

    fn resolve_incident(
        &mut self,
        ctx: RpcContext,
        req: gateway::ResolveIncidentRequest,
        sink: UnarySink<gateway::ResolveIncidentResponse>,
    ) {
        mock!(self, ctx, sink, req, ResolveIncident);
    }
}
