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
