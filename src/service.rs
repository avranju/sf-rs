use std::{
    ffi::{c_void, OsString},
    os::windows::ffi::OsStrExt,
};

use tokio::sync::mpsc;
use windows::core::implement;

use crate::{
    error::Error, run_with_retry, types::ServicePartition, IFabricAsyncOperationCallback,
    IFabricAsyncOperationCallback_Impl, IFabricAsyncOperationContext,
    IFabricResolvedServicePartitionResult, IFabricServiceManagementClient7, MakeClient,
    PartitionKeyType,
};

pub struct ServiceManagementClient {
    client: IFabricServiceManagementClient7,
}

impl MakeClient for ServiceManagementClient {
    type Interface = IFabricServiceManagementClient7;

    fn make(client: Self::Interface) -> Result<Self, Error> {
        Ok(Self { client })
    }
}

impl ServiceManagementClient {
    pub fn new(client: IFabricServiceManagementClient7) -> Self {
        Self { client }
    }

    pub async fn resolve_service_partition(
        &self,
        service_name: &str,
        partition_key_type: PartitionKeyType,
        partition_key: i64,
        timeout_ms: u32,
    ) -> Result<ServicePartition, Error> {
        run_with_retry("resolve_service_partition", || async move {
            self.try_resolve_service_partition(
                service_name,
                partition_key_type,
                partition_key,
                timeout_ms,
            )
            .await
        })
        .await
    }

    async fn try_resolve_service_partition(
        &self,
        service_name: &str,
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

        let name: OsString = service_name.into();
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
