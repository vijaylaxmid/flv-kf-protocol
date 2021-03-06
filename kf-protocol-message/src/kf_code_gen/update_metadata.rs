/// WARNING: CODE GENERATED FILE
/// * This file is generated by kfspec2code.
/// * Any changes applied to this file will be lost when a new spec is generated.
use serde::{Deserialize, Serialize};

use kf_protocol_api::ErrorCode;
use kf_protocol_api::Request;

use kf_protocol_derive::Decode;
use kf_protocol_derive::Encode;
use kf_protocol_derive::KfDefault;

// -----------------------------------
// KfUpdateMetadataRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfUpdateMetadataRequest {
    /// The controller id.
    pub controller_id: i32,

    /// The controller epoch.
    pub controller_epoch: i32,

    /// The broker epoch.
    #[fluvio_kf(min_version = 5, ignorable)]
    pub broker_epoch: i64,

    /// Each topic that we would like to update.
    #[fluvio_kf(min_version = 5)]
    pub topic_states: Vec<UpdateMetadataRequestTopicState>,

    /// Each partition that we would like to update.
    #[fluvio_kf(max_version = 4)]
    pub partition_states_v0: Vec<UpdateMetadataRequestPartitionStateV0>,

    pub brokers: Vec<UpdateMetadataRequestBroker>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct UpdateMetadataRequestTopicState {
    /// The topic name.
    pub topic_name: String,

    /// The partition that we would like to update.
    #[fluvio_kf(min_version = 5)]
    pub partition_states: Vec<UpdateMetadataPartitionState>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct UpdateMetadataRequestPartitionStateV0 {
    /// The topic name.
    #[fluvio_kf(max_version = 4)]
    pub topic_name: String,

    /// The partition index.
    #[fluvio_kf(max_version = 4)]
    pub partition_index: i32,

    /// The controller epoch.
    #[fluvio_kf(max_version = 4)]
    pub controller_epoch: i32,

    /// The ID of the broker which is the current partition leader.
    #[fluvio_kf(max_version = 4)]
    pub leader: i32,

    /// The leader epoch of this partition.
    #[fluvio_kf(max_version = 4)]
    pub leader_epoch: i32,

    /// The brokers which are in the ISR for this partition.
    #[fluvio_kf(max_version = 4)]
    pub isr: Vec<i32>,

    /// The Zookeeper version.
    #[fluvio_kf(max_version = 4)]
    pub zk_version: i32,

    /// All the replicas of this partition.
    #[fluvio_kf(max_version = 4)]
    pub replicas: Vec<i32>,

    /// The replicas of this partition which are offline.
    #[fluvio_kf(min_version = 4, max_version = 4)]
    pub offline_replicas: Vec<i32>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct UpdateMetadataRequestBroker {
    pub id: i32,

    /// The broker endpoints.
    #[fluvio_kf(min_version = 1)]
    pub endpoints: Vec<UpdateMetadataRequestEndpoint>,

    /// The rack which this broker belongs to.
    #[fluvio_kf(min_version = 2, ignorable)]
    pub rack: Option<String>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct UpdateMetadataRequestEndpoint {
    /// The port of this endpoint
    #[fluvio_kf(min_version = 1)]
    pub port: i32,

    /// The hostname of this endpoint
    #[fluvio_kf(min_version = 1)]
    pub host: String,

    /// The listener name.
    #[fluvio_kf(min_version = 3)]
    pub listener: String,

    /// The security protocol type.
    #[fluvio_kf(min_version = 1)]
    pub security_protocol: i16,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct UpdateMetadataPartitionState {
    /// The partition index.
    #[fluvio_kf(min_version = 5)]
    pub partition_index: i32,

    /// The controller epoch.
    #[fluvio_kf(min_version = 5)]
    pub controller_epoch: i32,

    /// The ID of the broker which is the current partition leader.
    #[fluvio_kf(min_version = 5)]
    pub leader: i32,

    /// The leader epoch of this partition.
    #[fluvio_kf(min_version = 5)]
    pub leader_epoch: i32,

    /// The brokers which are in the ISR for this partition.
    #[fluvio_kf(min_version = 5)]
    pub isr: Vec<i32>,

    /// The Zookeeper version.
    #[fluvio_kf(min_version = 5)]
    pub zk_version: i32,

    /// All the replicas of this partition.
    #[fluvio_kf(min_version = 5)]
    pub replicas: Vec<i32>,

    /// The replicas of this partition which are offline.
    #[fluvio_kf(min_version = 5)]
    pub offline_replicas: Vec<i32>,
}

// -----------------------------------
// KfUpdateMetadataResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfUpdateMetadataResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: ErrorCode,
}

// -----------------------------------
// Implementation - KfUpdateMetadataRequest
// -----------------------------------

impl Request for KfUpdateMetadataRequest {
    const API_KEY: u16 = 6;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 5;
    const DEFAULT_API_VERSION: i16 = 5;

    type Response = KfUpdateMetadataResponse;
}
