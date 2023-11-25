use std::{ffi::OsString, os::windows::ffi::OsStrExt, ptr, slice};

use tokio::sync::mpsc;
use windows::core::{implement, GUID};

use crate::{
    error::Error, run_with_retry, IFabricAsyncOperationCallback,
    IFabricAsyncOperationCallback_Impl, IFabricAsyncOperationContext, IFabricQueryClient12,
    MakeClient, PartitionQueryResultItem, FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION,
};

pub struct QueryClient {
    client: IFabricQueryClient12,
}

impl MakeClient for QueryClient {
    type Interface = IFabricQueryClient12;

    fn make(client: Self::Interface) -> Result<Self, Error> {
        Ok(Self { client })
    }
}

impl QueryClient {
    pub fn new(client: IFabricQueryClient12) -> Self {
        Self { client }
    }

    pub async fn get_partition_list(
        &self,
        service_name: &str,
        timeout_ms: u32,
    ) -> Result<Vec<PartitionQueryResultItem>, Error> {
        run_with_retry("get_partition_list", || async move {
            self.try_get_partition_list(service_name, timeout_ms).await
        })
        .await
    }

    async fn try_get_partition_list(
        &self,
        service_name: &str,
        timeout_ms: u32,
    ) -> Result<Vec<PartitionQueryResultItem>, Error> {
        #[implement(IFabricAsyncOperationCallback)]
        struct ResolveCallback {
            tx: mpsc::Sender<Result<Vec<PartitionQueryResultItem>, Error>>,
            client: IFabricQueryClient12,
        }

        impl IFabricAsyncOperationCallback_Impl for ResolveCallback {
            fn Invoke(&self, context: Option<&IFabricAsyncOperationContext>) {
                match unsafe { self.client.EndGetPartitionList(context) } {
                    Ok(res) => {
                        let res = unsafe { res.get_PartitionList() };
                        let count = unsafe { (*res).Count };
                        let items = unsafe { slice::from_raw_parts((*res).Items, count as usize) };
                        let items = items
                            .iter()
                            .map(PartitionQueryResultItem::try_from)
                            .collect::<Result<Vec<_>, Error>>();

                        let _ = self.tx.blocking_send(items);
                    }
                    Err(e) => {
                        let _ = self.tx.blocking_send(Err(e.into()));
                    }
                }
            }
        }

        let service_name = OsString::from(service_name);
        let mut service_name = service_name.encode_wide().collect::<Vec<_>>();

        let query_desc = FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION {
            ServiceName: service_name.as_mut_ptr(),
            PartitionIdFilter: GUID::zeroed(),
            Reserved: ptr::null_mut(),
        };
        let (tx, mut rx) = mpsc::channel(1);
        let callback = ResolveCallback {
            tx,
            client: self.client.clone(),
        };
        let op_callback: IFabricAsyncOperationCallback = callback.into();

        let _context = unsafe {
            self.client
                .BeginGetPartitionList(&query_desc, timeout_ms, Some(&op_callback))
        }?;

        rx.recv()
            .await
            .ok_or(Error::Abandoned("ResolveServicePartition"))?
    }
}
