use zeebe_client::*;
use std::thread;
use std::time::{self, SystemTime, UNIX_EPOCH};


#[test]
fn request_topology() {
    let client = ZeebeClient::default();

    let topology = client.topology().expect("Failed to send topology request");
    assert_eq!(1, topology.cluster_size);
    assert_eq!(1, topology.partitions_count);
    assert_eq!(1, topology.replication_factor);
    assert_eq!(1, topology.brokers.len());

    let broker = &topology.brokers[0];
    assert_eq!(0,  broker.node_id);
    assert!(broker.host.len() > 0);
    assert_eq!(26501,  broker.port);
    assert_eq!(1, broker.partitions.len());

    let partition = &broker.partitions[0];
    assert_eq!(0, partition.partition_id);
    assert_eq!(PartitionRole::Leader, partition.role);
}

#[test]
fn deploy_file() {
    let client = ZeebeClient::default();
    let (key, workflows) = client.deploy_file("tests/order-process.bpmn").expect("Failed to deploy workflow");

    assert!(key > 0);
    assert_eq!(1, workflows.len());

    let workflow = &workflows[0];
    assert_eq!("order-process", workflow.bpmn_process_id);
    assert!(workflow.version > 0);
    assert!(workflow.workflow_key > 0);
    assert_eq!("tests/order-process.bpmn", workflow.resource_name);
}

#[test]
fn list_workflows() {
    let client = ZeebeClient::default();
    client.deploy_file("tests/order-process.bpmn").unwrap();
    client.deploy_file("tests/order-process.bpmn").unwrap();

    let workflows = client.list_workflows().expect("Failed to list workflows");
    assert!(workflows.len() > 1);

    let workflows = client.list_workflows_by_bpmn_process_id("order-process").expect("Failed to list workflows");
    assert!(workflows.len() > 1);
    assert!(workflows.iter().all(|w| w.bpmn_process_id == "order-process"));
}

#[test]
fn get_workflow() {
    let client = ZeebeClient::default();
    let (_, workflows) = client.deploy_file("tests/order-process.bpmn").unwrap();
    let expected = &workflows[0];

    let (workflow, xml)  = client.get_workflow(expected.workflow_key).expect("Failed to get workflow");
    assert_eq!(expected, &workflow);
    assert!(xml.contains("order-process"));

    let (workflow, xml)  = client.get_latest_workflow("order-process").expect("Failed to get workflow");
    assert_eq!("order-process", workflow.bpmn_process_id);
    assert!(workflow.version >= expected.version);
    assert!(xml.contains("order-process"));

    let (workflow, xml)  = client.get_workflow_by_version("order-process", expected.version).expect("Failed to get workflow");
    assert_eq!(expected, &workflow);
    assert!(xml.contains("order-process"));
}

#[test]
fn create_instance() {
    let client = ZeebeClient::default();
    let (_, workflows) = client.deploy_file("tests/order-process.bpmn").unwrap();
    let workflow = &workflows[0];

    let instance = client.create_instance(workflow.workflow_key, "").expect("Failed to create instance");
    assert_eq!("order-process", instance.bpmn_process_id);
    assert_eq!(workflow.version, instance.version);
    assert_eq!(workflow.workflow_key, instance.workflow_key);
    assert!(instance.workflow_instance_key > 0);

    let instance = client.create_instance_from_latest("order-process", "").expect("Failed to create instance");
    assert_eq!("order-process", instance.bpmn_process_id);
    assert!(workflow.version <= instance.version);
    assert!(workflow.workflow_key <= instance.workflow_key);
    assert!(instance.workflow_instance_key > 0);

    let instance = client.create_instance_from_version("order-process", workflow.version, "").expect("Failed to create instance");
    assert_eq!("order-process", instance.bpmn_process_id);
    assert_eq!(workflow.version, instance.version);
    assert_eq!(workflow.workflow_key, instance.workflow_key);
    assert!(instance.workflow_instance_key > 0);
}

#[test]
fn cancel_instance() {
    let client = ZeebeClient::default();
    client.deploy_file("tests/order-process.bpmn").unwrap();
    let instance = client.create_instance_from_latest("order-process", "").expect("Failed to create instance");

    client.cancel_instance(instance.workflow_instance_key).expect("Failed to cancel instance");
}

#[test]
fn activate_jobs() {
    let client = ZeebeClient::default();
    client.deploy_file("tests/order-process.bpmn").unwrap();
    client.create_instance_from_latest("order-process", r#"{"orderId": "foobar"}"#).expect("Failed to create instance");

    thread::sleep(time::Duration::from_millis(200));

    let jobs = client.activate_jobs("payment-service", "test worker", 1_000, 1).expect("Failed to activate jobs");
    assert_eq!(1, jobs.len());

    client.fail_job(jobs[0].key, 2).expect("Failed to fail to job");;

    let jobs = client.activate_jobs("payment-service", "test worker", 1_000, 1).expect("Failed to activate jobs");
    assert_eq!(1, jobs.len());

    client.fail_job_with_message(jobs[0].key, 2, "failed job").expect("Failed to fail to job");

    let jobs = client.activate_jobs("payment-service", "test worker", 1_000, 1).expect("Failed to activate jobs");
    assert_eq!(1, jobs.len());

    client.complete_job(jobs[0].key, "").expect("Failed to complete job");
}

#[test]
fn publish_message() {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let client = ZeebeClient::default();
    client.deploy_file("tests/message.bpmn").unwrap();
    client.create_instance_from_latest("message-process", r#"{"id": "foo"}"#).expect("Failed to create instance");
    client.create_instance_from_latest("message-process", r#"{"id": "bar"}"#).expect("Failed to create instance");

    client.publish_message("message", "foo", 10_000, "").expect("Failed to publish message");
    client.publish_message_with_id("message", "bar", 10_000, &format!("{:?}", timestamp), "").expect("Failed to publish message");
}