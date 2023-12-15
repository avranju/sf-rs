use std::{
    ffi::{c_void, OsString},
    os::windows::ffi::OsStrExt,
};

use tokio::sync::mpsc;
use windows::core::{implement, ComInterface};

use crate::{
    agile::AgileRef, channel_send, error::Error, run_with_retry, types::ServicePartition,
    IFabricAsyncOperationCallback, IFabricAsyncOperationCallback_Impl,
    IFabricAsyncOperationContext, IFabricResolvedServicePartitionResult,
    IFabricServiceManagementClient7, MakeClient, PartitionKeyType,
};

#[derive(Debug, Clone)]
pub struct ServiceManagementClient {
    client: AgileRef<IFabricServiceManagementClient7>,
}

impl MakeClient for ServiceManagementClient {
    type Interface = IFabricServiceManagementClient7;

    fn make(client: Self::Interface) -> Result<Self, Error> {
        Ok(Self {
            client: AgileRef::new(client.cast()?)?,
        })
    }
}

impl ServiceManagementClient {
    pub fn new(client: IFabricServiceManagementClient7) -> Result<Self, Error> {
        Ok(Self {
            client: AgileRef::new(client.cast()?)?,
        })
    }

    pub async fn resolve_service_partition(
        &self,
        service_name: &str,
        partition_key_type: PartitionKeyType,
        partition_key: i64,
        timeout_ms: u32,
    ) -> Result<ServicePartition, Error> {
        run_with_retry("resolve_service_partition", move || {
            let client = self.client.clone();
            async move {
                let mut rx = try_resolve_service_partition(
                    client,
                    service_name,
                    partition_key_type,
                    partition_key,
                    timeout_ms,
                )?;
                rx.recv()
                    .await
                    .ok_or(Error::Abandoned("ResolveServicePartition"))?
            }
        })
        .await
    }
}

fn try_resolve_service_partition(
    client: AgileRef<IFabricServiceManagementClient7>,
    service_name: &str,
    partition_key_type: PartitionKeyType,
    mut partition_key: i64,
    timeout_ms: u32,
) -> Result<mpsc::Receiver<Result<ServicePartition, Error>>, Error> {
    let name: OsString = service_name.into();
    let name = name.encode_wide().collect::<Vec<_>>();
    let (tx, rx) = mpsc::channel(1);
    let callback = ResolveCallback {
        tx,
        client: client.clone(),
    };
    let op_callback: IFabricAsyncOperationCallback = callback.into();
    //let op_callback: AgileRef<IFabricAsyncOperationCallback> = AgileRef::new(op_callback.cast()?)?;
    let previous_result: Option<&IFabricResolvedServicePartitionResult> = None;

    let _context = unsafe {
        client.resolve()?.BeginResolveServicePartition(
            name.as_ptr(),
            partition_key_type.into(),
            &mut partition_key as *mut i64 as *mut c_void,
            previous_result,
            timeout_ms,
            Some(&op_callback),
        )
    }?;

    Ok(rx)
}

#[implement(IFabricAsyncOperationCallback)]
struct ResolveCallback {
    tx: mpsc::Sender<Result<ServicePartition, Error>>,
    client: AgileRef<IFabricServiceManagementClient7>,
}

impl IFabricAsyncOperationCallback_Impl for ResolveCallback {
    fn Invoke(&self, context: Option<&IFabricAsyncOperationContext>) {
        let res = unsafe {
            self.client.resolve().and_then(|client| {
                client
                    .EndResolveServicePartition(context)
                    .map_err(|e| e.into())
            })
        };
        match res {
            Ok(res) => {
                let _ = channel_send(self.tx.clone(), ServicePartition::new(res));
            }
            Err(e) => {
                let _ = channel_send(self.tx.clone(), Err(e));
            }
        }
    }
}
