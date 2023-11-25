#![allow(non_snake_case, clippy::missing_safety_doc)]

use anyhow::Result;
use sf_rs::{FabricLocalClient, PartitionKeyType, QueryClient, ServiceManagementClient};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;
    }

    let client = FabricLocalClient::new()?;
    let service_client: ServiceManagementClient = client.make_client()?;

    let res = service_client
        .resolve_service_partition(
            "fabric:/ZZZ/BlockStorageSFPkg",
            PartitionKeyType::Int64,
            1,
            1000,
        )
        .await?;

    println!("{:#?}", res);

    let query_client: QueryClient = client.make_client()?;
    let res = query_client
        .get_partition_list("fabric:/ZZZ/BlockStorageSFPkg", 1000)
        .await?;

    println!("{:#?}", res);

    Ok(())
}
