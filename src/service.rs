use std::{
    ffi::{c_void, OsString},
    os::windows::ffi::OsStrExt,
    slice,
};

use tokio::sync::mpsc;
use windows::core::{implement, PWSTR};

use crate::{
    error::Error, IFabricAsyncOperationCallback, IFabricAsyncOperationCallback_Impl,
    IFabricAsyncOperationContext, IFabricResolvedServicePartitionResult,
    IFabricServiceManagementClient7, FABRIC_PARTITION_KEY_TYPE, FABRIC_PARTITION_KEY_TYPE_INT64,
    FABRIC_PARTITION_KEY_TYPE_INVALID, FABRIC_PARTITION_KEY_TYPE_NONE,
    FABRIC_PARTITION_KEY_TYPE_STRING, FABRIC_RESOLVED_SERVICE_ENDPOINT,
    FABRIC_SERVICE_ENDPOINT_ROLE, FABRIC_SERVICE_PARTITION_KIND,
    FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE, FABRIC_SERVICE_PARTITION_KIND_INVALID,
    FABRIC_SERVICE_PARTITION_KIND_NAMED, FABRIC_SERVICE_PARTITION_KIND_SINGLETON,
    FABRIC_SERVICE_ROLE_INVALID, FABRIC_SERVICE_ROLE_STATEFUL_AUXILIARY,
    FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY, FABRIC_SERVICE_ROLE_STATEFUL_PRIMARY_AUXILIARY,
    FABRIC_SERVICE_ROLE_STATEFUL_SECONDARY, FABRIC_SERVICE_ROLE_STATELESS,
};

pub struct ServiceManagementClient {
    client: IFabricServiceManagementClient7,
}

impl ServiceManagementClient {
    pub fn new(client: IFabricServiceManagementClient7) -> Self {
        Self { client }
    }

    pub async fn resolve_service_partition(
        &self,
        name: &str,
        partition_key_type: PartitionKeyType,
        mut partition_key: i64,
        timeout_ms: u32,
    ) -> Result<ServicePartition, Error> {
        #[implement(IFabricAsyncOperationCallback)]
        struct ResolveCallback {
            tx: mpsc::Sender<Result<ServicePartition, Error>>,
            client: IFabricServiceManagementClient7,
        }

        impl IFabricAsyncOperationCallback_Impl for ResolveCallback {
            fn Invoke(&self, context: Option<&IFabricAsyncOperationContext>) {
                match unsafe { self.client.EndResolveServicePartition(context) } {
                    Ok(res) => {
                        let _ = self.tx.blocking_send(ServicePartition::new(res));
                    }
                    Err(e) => {
                        let _ = self.tx.blocking_send(Err(e.into()));
                    }
                }
            }
        }

        let name: OsString = name.into();
        let name = name.encode_wide().collect::<Vec<_>>();
        let (tx, mut rx) = mpsc::channel(1);
        let callback = ResolveCallback {
            tx,
            client: self.client.clone(),
        };
        let op_callback: IFabricAsyncOperationCallback = callback.into();

        let previous_result: Option<&IFabricResolvedServicePartitionResult> = None;

        let _context = unsafe {
            self.client.BeginResolveServicePartition(
                name.as_ptr(),
                partition_key_type.into(),
                &mut partition_key as *mut i64 as *mut c_void,
                previous_result,
                timeout_ms,
                Some(&op_callback),
            )
        }?;

        rx.recv()
            .await
            .ok_or(Error::Abandoned("ResolveServicePartition"))?
    }
}

#[derive(Debug)]
pub struct ServicePartition {
    kind: PartitionKind,
    endpoints: Vec<ServiceEndpoint>,
    name: String,
}

impl ServicePartition {
    pub fn new(
        resolved_service_partition: IFabricResolvedServicePartitionResult,
    ) -> Result<Self, Error> {
        let partition = unsafe { resolved_service_partition.get_Partition() };
        let kind = unsafe { (*partition).Info.Kind.into() };
        let mut endpoints = vec![];
        let endpoints_count = unsafe { (*partition).EndpointCount };
        let endpoint_refs =
            unsafe { slice::from_raw_parts((*partition).Endpoints, endpoints_count as usize) };
        for endpoint in endpoint_refs {
            endpoints.push(endpoint.try_into()?);
        }
        let name = unsafe { PWSTR::from_raw((*partition).ServiceName).to_string()? };

        Ok(Self {
            kind,
            endpoints,
            name,
        })
    }

    pub fn kind(&self) -> PartitionKind {
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
pub enum PartitionKind {
    Invalid = FABRIC_SERVICE_PARTITION_KIND_INVALID.0,
    Singleton = FABRIC_SERVICE_PARTITION_KIND_SINGLETON.0,
    Int64Range = FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE.0,
    Named = FABRIC_SERVICE_PARTITION_KIND_NAMED.0,
}

impl From<FABRIC_SERVICE_PARTITION_KIND> for PartitionKind {
    fn from(value: FABRIC_SERVICE_PARTITION_KIND) -> Self {
        match value {
            FABRIC_SERVICE_PARTITION_KIND_INVALID => PartitionKind::Invalid,
            FABRIC_SERVICE_PARTITION_KIND_SINGLETON => PartitionKind::Singleton,
            FABRIC_SERVICE_PARTITION_KIND_INT64_RANGE => PartitionKind::Int64Range,
            FABRIC_SERVICE_PARTITION_KIND_NAMED => PartitionKind::Named,
            _ => PartitionKind::Invalid,
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
