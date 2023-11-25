use std::slice;

use windows::core::{GUID, PWSTR};

use crate::{
    error::Error, IFabricResolvedServicePartitionResult, FABRIC_HEALTH_STATE,
    FABRIC_HEALTH_STATE_ERROR, FABRIC_HEALTH_STATE_INVALID, FABRIC_HEALTH_STATE_OK,
    FABRIC_HEALTH_STATE_UNKNOWN, FABRIC_HEALTH_STATE_WARNING,
    FABRIC_INT64_RANGE_PARTITION_INFORMATION, FABRIC_NAMED_PARTITION_INFORMATION,
    FABRIC_PARTITION_KEY_TYPE, FABRIC_PARTITION_KEY_TYPE_INT64, FABRIC_PARTITION_KEY_TYPE_INVALID,
    FABRIC_PARTITION_KEY_TYPE_NONE, FABRIC_PARTITION_KEY_TYPE_STRING,
    FABRIC_QUERY_SERVICE_PARTITION_STATUS, FABRIC_QUERY_SERVICE_PARTITION_STATUS_DELETING,
    FABRIC_QUERY_SERVICE_PARTITION_STATUS_INVALID,
    FABRIC_QUERY_SERVICE_PARTITION_STATUS_IN_QUORUM_LOSS,
    FABRIC_QUERY_SERVICE_PARTITION_STATUS_NOT_READY, FABRIC_QUERY_SERVICE_PARTITION_STATUS_READY,
    FABRIC_QUERY_SERVICE_PARTITION_STATUS_RECONFIGURING, FABRIC_RESOLVED_SERVICE_ENDPOINT,
    FABRIC_SERVICE_ENDPOINT_ROLE, FABRIC_SERVICE_KIND, FABRIC_SERVICE_KIND_INVALID,
    FABRIC_SERVICE_KIND_STATEFUL, FABRIC_SERVICE_KIND_STATELESS,
    FABRIC_SERVICE_PARTITION_INFORMATION, FABRIC_SERVICE_PARTITION_KIND,
    FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE, FABRIC_SERVICE_PARTITION_KIND_INVALID,
    FABRIC_SERVICE_PARTITION_KIND_NAMED, FABRIC_SERVICE_PARTITION_KIND_SINGLETON,
    FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM, FABRIC_SERVICE_ROLE_INVALID,
    FABRIC_SERVICE_ROLE_STATEFUL_AUXILIARY, FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY,
    FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY_AUXILIARY, FABRIC_SERVICE_ROLE_STATEFUL_SECONDARY,
    FABRIC_SERVICE_ROLE_STATELESS, FABRIC_SINGLETON_PARTITION_INFORMATION,
    FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM,
    FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM,
};

#[derive(Debug)]
pub struct ServicePartition {
    kind: ServicePartitionKind,
    endpoints: Vec<ServiceEndpoint>,
    name: String,
}

impl ServicePartition {
    pub fn new(
        resolved_service_partition: IFabricResolvedServicePartitionResult,
    ) -> Result<Self, Error> {
        let partition = unsafe { resolved_service_partition.get_Partition() };
        let kind = unsafe { (*partition).Info.Kind.into() };
        let endpoints_count = unsafe { (*partition).EndpointCount };
        let endpoint_refs =
            unsafe { slice::from_raw_parts((*partition).Endpoints, endpoints_count as usize) };
        let endpoints = endpoint_refs
            .iter()
            .map(ServiceEndpoint::try_from)
            .collect::<Result<Vec<_>, Error>>()?;
        let name = unsafe { PWSTR::from_raw((*partition).ServiceName).to_string()? };

        Ok(Self {
            kind,
            endpoints,
            name,
        })
    }

    pub fn kind(&self) -> ServicePartitionKind {
        self.kind
    }

    pub fn endpoints(&self) -> &[ServiceEndpoint] {
        &self.endpoints
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub struct ServiceEndpoint {
    address: String,
    role: EndpointRole,
}

impl ServiceEndpoint {
    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn role(&self) -> EndpointRole {
        self.role
    }
}

impl TryFrom<&FABRIC_RESOLVED_SERVICE_ENDPOINT> for ServiceEndpoint {
    type Error = Error;

    fn try_from(value: &FABRIC_RESOLVED_SERVICE_ENDPOINT) -> Result<Self, Self::Error> {
        let role = value.Role.into();
        let address = unsafe { value.Address.to_string()? };
        Ok(Self { address, role })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum EndpointRole {
    Invalid = FABRIC_SERVICE_ROLE_INVALID.0,
    Stateless = FABRIC_SERVICE_ROLE_STATELESS.0,
    StatefulPrimary = FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY.0,
    StatefulSecondary = FABRIC_SERVICE_ROLE_STATEFUL_SECONDARY.0,
    StatefulPrimaryAuxiliary = FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY_AUXILIARY.0,
    StatefulAuxiliary = FABRIC_SERVICE_ROLE_STATEFUL_AUXILIARY.0,
}

impl From<FABRIC_SERVICE_ENDPOINT_ROLE> for EndpointRole {
    fn from(value: FABRIC_SERVICE_ENDPOINT_ROLE) -> Self {
        match value {
            FABRIC_SERVICE_ROLE_INVALID => EndpointRole::Invalid,
            FABRIC_SERVICE_ROLE_STATELESS => EndpointRole::Stateless,
            FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY => EndpointRole::StatefulPrimary,
            FABRIC_SERVICE_ROLE_STATEFUL_SECONDARY => EndpointRole::StatefulSecondary,
            FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY_AUXILIARY => {
                EndpointRole::StatefulPrimaryAuxiliary
            }
            FABRIC_SERVICE_ROLE_STATEFUL_AUXILIARY => EndpointRole::StatefulAuxiliary,
            _ => EndpointRole::Invalid,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ServicePartitionKind {
    Invalid = FABRIC_SERVICE_PARTITION_KIND_INVALID.0,
    Singleton = FABRIC_SERVICE_PARTITION_KIND_SINGLETON.0,
    Int64Range = FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE.0,
    Named = FABRIC_SERVICE_PARTITION_KIND_NAMED.0,
}

impl From<FABRIC_SERVICE_PARTITION_KIND> for ServicePartitionKind {
    fn from(value: FABRIC_SERVICE_PARTITION_KIND) -> Self {
        match value {
            FABRIC_SERVICE_PARTITION_KIND_INVALID => ServicePartitionKind::Invalid,
            FABRIC_SERVICE_PARTITION_KIND_SINGLETON => ServicePartitionKind::Singleton,
            FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE => ServicePartitionKind::Int64Range,
            FABRIC_SERVICE_PARTITION_KIND_NAMED => ServicePartitionKind::Named,
            _ => ServicePartitionKind::Invalid,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum PartitionKeyType {
    Invalid = FABRIC_PARTITION_KEY_TYPE_INVALID.0,
    None = FABRIC_PARTITION_KEY_TYPE_NONE.0,
    Int64 = FABRIC_PARTITION_KEY_TYPE_INT64.0,
    String = FABRIC_PARTITION_KEY_TYPE_STRING.0,
}

impl From<PartitionKeyType> for FABRIC_PARTITION_KEY_TYPE {
    fn from(value: PartitionKeyType) -> Self {
        match value {
            PartitionKeyType::Invalid => FABRIC_PARTITION_KEY_TYPE_INVALID,
            PartitionKeyType::None => FABRIC_PARTITION_KEY_TYPE_NONE,
            PartitionKeyType::Int64 => FABRIC_PARTITION_KEY_TYPE_INT64,
            PartitionKeyType::String => FABRIC_PARTITION_KEY_TYPE_STRING,
        }
    }
}

#[derive(Debug)]
pub enum PartitionQueryResultItem {
    Stateful(StatefulService),
    Stateless(StatelessService),
}

impl TryFrom<&FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM> for PartitionQueryResultItem {
    type Error = Error;

    fn try_from(value: &FABRIC_SERVICE_PARTITION_QUERY_RESULT_ITEM) -> Result<Self, Self::Error> {
        let kind = ServiceKind::from(value.Kind);
        match kind {
            ServiceKind::Stateful => {
                let service = StatefulService::try_from(unsafe {
                    let p =
                        value.Value as *const FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM;
                    &*p
                })?;
                Ok(Self::Stateful(service))
            }
            ServiceKind::Stateless => {
                let service = StatelessService::try_from(unsafe {
                    &*(value.Value as *const FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM)
                })?;
                Ok(Self::Stateless(service))
            }
            _ => Err(Error::InvalidServiceKind),
        }
    }
}

#[derive(Debug)]
pub struct StatefulService {
    pub partition_information: ServicePartitionInformation,
    pub target_replica_size: u32,
    pub min_replica_size: u32,
    pub health_state: HealthState,
    pub partition_status: QueryServicePartitionStatus,
    pub last_quorum_loss_duration_in_seconds: i64,
}

impl TryFrom<&FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM> for StatefulService {
    type Error = Error;

    fn try_from(
        value: &FABRIC_STATEFUL_SERVICE_PARTITION_QUERY_RESULT_ITEM,
    ) -> Result<Self, Self::Error> {
        let partition_information = ServicePartitionInformation::try_from(unsafe {
            &*(value.PartitionInformation as *const FABRIC_SERVICE_PARTITION_INFORMATION)
        })?;
        let target_replica_size = value.TargetReplicaSetSize;
        let min_replica_size = value.MinReplicaSetSize;
        let health_state = HealthState::from(value.HealthState);
        let partition_status = QueryServicePartitionStatus::from(value.PartitionStatus);
        let last_quorum_loss_duration_in_seconds = value.LastQuorumLossDurationInSeconds;

        Ok(Self {
            partition_information,
            target_replica_size,
            min_replica_size,
            health_state,
            partition_status,
            last_quorum_loss_duration_in_seconds,
        })
    }
}

#[derive(Debug)]
pub struct StatelessService {
    pub partition_information: ServicePartitionInformation,
    pub instance_count: u32,
    pub health_state: HealthState,
    pub partition_status: QueryServicePartitionStatus,
}

impl TryFrom<&FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM> for StatelessService {
    type Error = Error;

    fn try_from(
        value: &FABRIC_STATELESS_SERVICE_PARTITION_QUERY_RESULT_ITEM,
    ) -> Result<Self, Self::Error> {
        let partition_information = ServicePartitionInformation::try_from(unsafe {
            &*(value.PartitionInformation as *const FABRIC_SERVICE_PARTITION_INFORMATION)
        })?;
        let instance_count = value.InstanceCount;
        let health_state = HealthState::from(value.HealthState);
        let partition_status = QueryServicePartitionStatus::from(value.PartitionStatus);

        Ok(Self {
            partition_information,
            instance_count,
            health_state,
            partition_status,
        })
    }
}

#[derive(Debug)]
#[repr(i32)]
pub enum HealthState {
    Invalid = FABRIC_HEALTH_STATE_INVALID.0,
    Ok = FABRIC_HEALTH_STATE_OK.0,
    Warning = FABRIC_HEALTH_STATE_WARNING.0,
    Error = FABRIC_HEALTH_STATE_ERROR.0,
    Unknown = FABRIC_HEALTH_STATE_UNKNOWN.0,
}

impl From<FABRIC_HEALTH_STATE> for HealthState {
    fn from(state: FABRIC_HEALTH_STATE) -> Self {
        match state {
            FABRIC_HEALTH_STATE_INVALID => Self::Invalid,
            FABRIC_HEALTH_STATE_OK => Self::Ok,
            FABRIC_HEALTH_STATE_WARNING => Self::Warning,
            FABRIC_HEALTH_STATE_ERROR => Self::Error,
            FABRIC_HEALTH_STATE_UNKNOWN => Self::Unknown,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug)]
#[repr(i32)]
pub enum ServiceKind {
    Invalid = FABRIC_SERVICE_KIND_INVALID.0,
    Stateless = FABRIC_SERVICE_KIND_STATELESS.0,
    Stateful = FABRIC_SERVICE_KIND_STATEFUL.0,
}

impl From<FABRIC_SERVICE_KIND> for ServiceKind {
    fn from(kind: FABRIC_SERVICE_KIND) -> Self {
        match kind {
            FABRIC_SERVICE_KIND_INVALID => Self::Invalid,
            FABRIC_SERVICE_KIND_STATELESS => Self::Stateless,
            FABRIC_SERVICE_KIND_STATEFUL => Self::Stateful,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug)]
#[repr(i32)]
pub enum QueryServicePartitionStatus {
    Invalid = FABRIC_QUERY_SERVICE_PARTITION_STATUS_INVALID.0,
    Ready = FABRIC_QUERY_SERVICE_PARTITION_STATUS_READY.0,
    NotReady = FABRIC_QUERY_SERVICE_PARTITION_STATUS_NOT_READY.0,
    InQuorumLoss = FABRIC_QUERY_SERVICE_PARTITION_STATUS_IN_QUORUM_LOSS.0,
    Reconfiguring = FABRIC_QUERY_SERVICE_PARTITION_STATUS_RECONFIGURING.0,
    Deleting = FABRIC_QUERY_SERVICE_PARTITION_STATUS_DELETING.0,
}

impl From<FABRIC_QUERY_SERVICE_PARTITION_STATUS> for QueryServicePartitionStatus {
    fn from(status: FABRIC_QUERY_SERVICE_PARTITION_STATUS) -> Self {
        match status {
            FABRIC_QUERY_SERVICE_PARTITION_STATUS_INVALID => Self::Invalid,
            FABRIC_QUERY_SERVICE_PARTITION_STATUS_READY => Self::Ready,
            FABRIC_QUERY_SERVICE_PARTITION_STATUS_NOT_READY => Self::NotReady,
            FABRIC_QUERY_SERVICE_PARTITION_STATUS_IN_QUORUM_LOSS => Self::InQuorumLoss,
            FABRIC_QUERY_SERVICE_PARTITION_STATUS_RECONFIGURING => Self::Reconfiguring,
            FABRIC_QUERY_SERVICE_PARTITION_STATUS_DELETING => Self::Deleting,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug)]
pub enum ServicePartitionInformation {
    Singleton(SingletonPartitionInformation),
    Int64Range(Int64RangePartitionInformation),
    Named(NamedPartitionInformation),
}

impl TryFrom<&FABRIC_SERVICE_PARTITION_INFORMATION> for ServicePartitionInformation {
    type Error = Error;

    fn try_from(value: &FABRIC_SERVICE_PARTITION_INFORMATION) -> Result<Self, Self::Error> {
        let kind = value.Kind.into();
        match kind {
            ServicePartitionKind::Singleton => {
                let info =
                    unsafe { &*(value.Value as *const FABRIC_SINGLETON_PARTITION_INFORMATION) };
                Ok(Self::Singleton(info.into()))
            }
            ServicePartitionKind::Int64Range => {
                let info =
                    unsafe { &*(value.Value as *const FABRIC_INT64_RANGE_PARTITION_INFORMATION) };
                Ok(Self::Int64Range(info.into()))
            }
            ServicePartitionKind::Named => {
                let info = unsafe { &*(value.Value as *const FABRIC_NAMED_PARTITION_INFORMATION) };
                Ok(Self::Named(info.try_into()?))
            }
            _ => Err(Error::InvalidServicePartitionKind),
        }
    }
}

#[derive(Debug)]
pub struct SingletonPartitionInformation {
    pub id: GUID,
}

impl From<&FABRIC_SINGLETON_PARTITION_INFORMATION> for SingletonPartitionInformation {
    fn from(value: &FABRIC_SINGLETON_PARTITION_INFORMATION) -> Self {
        let id = value.Id;
        Self { id }
    }
}

#[derive(Debug)]
pub struct Int64RangePartitionInformation {
    pub id: GUID,
    pub low_key: i64,
    pub high_key: i64,
}

impl From<&FABRIC_INT64_RANGE_PARTITION_INFORMATION> for Int64RangePartitionInformation {
    fn from(value: &FABRIC_INT64_RANGE_PARTITION_INFORMATION) -> Self {
        let id = value.Id;
        let low_key = value.LowKey;
        let high_key = value.HighKey;
        Self {
            id,
            low_key,
            high_key,
        }
    }
}

#[derive(Debug)]
pub struct NamedPartitionInformation {
    pub id: GUID,
    pub name: String,
}

impl TryFrom<&FABRIC_NAMED_PARTITION_INFORMATION> for NamedPartitionInformation {
    type Error = Error;

    fn try_from(value: &FABRIC_NAMED_PARTITION_INFORMATION) -> Result<Self, Self::Error> {
        let id = value.Id;
        let name = unsafe { value.Name.to_string()? };
        Ok(Self { id, name })
    }
}
