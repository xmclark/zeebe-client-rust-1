use crate::gateway;

#[derive(Debug, PartialEq)]
pub struct Topology {
    pub cluster_size: i32,
    pub partitions_count: i32,
    pub replication_factor: i32,
    pub brokers: Vec<Broker>,
}

#[derive(Debug, PartialEq)]
pub struct Broker {
    pub node_id: i32,
    pub host: String,
    pub port: i32,
    pub partitions: Vec<Partition>
}

#[derive(Debug, PartialEq)]
pub struct Partition {
    pub partition_id: i32,
    pub role: PartitionRole,
}

#[derive(Debug, PartialEq)]
pub enum PartitionRole {
    Leader,
    Follower,
}

impl From<gateway::TopologyResponse> for Topology {
    fn from(response: gateway::TopologyResponse) -> Self {
        Topology {
            cluster_size: response.clusterSize,
            partitions_count: response.partitionsCount,
            replication_factor: response.replicationFactor,
            brokers: response.brokers.into_iter()
                .map(Broker::from)
                .collect(),
        }
    }
}

impl From<gateway::BrokerInfo> for Broker {
    fn from(broker: gateway::BrokerInfo) -> Self {
        Broker {
            node_id: broker.nodeId,
            host: broker.host,
            port: broker.port,
            partitions: broker.partitions.into_iter()
                .map(Partition::from)
                .collect(),
        }
    }
}

impl From<gateway::Partition> for Partition {
    fn from(partition: gateway::Partition) -> Self {
        Partition {
            partition_id: partition.partitionId,
            role: partition.role.into(),
        }
    }
}

impl From<gateway::Partition_PartitionBrokerRole> for PartitionRole {
    fn from(role: gateway::Partition_PartitionBrokerRole) -> Self {
        match role {
            gateway::Partition_PartitionBrokerRole::LEADER => PartitionRole::Leader,
            gateway::Partition_PartitionBrokerRole::FOLLOWER => PartitionRole::Follower,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::gateway::{TopologyResponse, BrokerInfo, Partition as PartitionInfo, Partition_PartitionBrokerRole};
    use super::{Topology, Broker, Partition, PartitionRole};

    #[test]
    fn from_topology_response_to_topology() {
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

        let mut response = TopologyResponse::new();
        response.set_clusterSize(cluster_size);
        response.set_partitionsCount(partitions_count);
        response.set_replicationFactor(replication_factor);

        let mut broker1 = create_broker_info(broker1_node_id, broker1_host, broker1_port);
        broker1.mut_partitions().push(create_partition(partition1_id, Partition_PartitionBrokerRole::LEADER));
        broker1.mut_partitions().push(create_partition(partition2_id, Partition_PartitionBrokerRole::FOLLOWER));
        response.mut_brokers().push(broker1);

        let mut broker2 = create_broker_info(broker2_node_id, broker2_host, broker2_port);
        broker2.mut_partitions().push(create_partition(partition1_id, Partition_PartitionBrokerRole::FOLLOWER));
        broker2.mut_partitions().push(create_partition(partition2_id, Partition_PartitionBrokerRole::LEADER));
        response.mut_brokers().push(broker2);

        assert_eq!(Topology {
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
        }, response.into());
    }

    fn create_broker_info(node_id: i32, host: &str, port: i32) -> BrokerInfo {
        let mut broker = BrokerInfo::new();
        broker.set_nodeId(node_id);
        broker.set_host(host.to_string());
        broker.set_port(port);
        broker
    }

    fn create_partition(partition_id: i32, role: Partition_PartitionBrokerRole) -> PartitionInfo {
        let mut partition = PartitionInfo::new();
        partition.set_partitionId(partition_id);
        partition.set_role(role);
        partition
    }
}
