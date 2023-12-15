#![allow(non_snake_case, clippy::missing_safety_doc)]

use std::env::args;

use anyhow::Result;
use sf_rs::{FabricLocalClient, PartitionKeyType, QueryClient, ServiceManagementClient};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;
    }

    let args = args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: sf-connect <service_name>");
        return Ok(());
    }

    let service_name = args[1].clone();

    let client = FabricLocalClient::new()?;
    let service_client: ServiceManagementClient = client.make_client()?;
    let query_client: QueryClient = client.make_client()?;

    // call locally
    doit(
        service_client.clone(),
        query_client.clone(),
        service_name.clone(),
    )
    .await?;

    // spawn a task to call from another tokio task
    let t = tokio::spawn(doit(service_client, query_client, service_name));
    t.await??;

    Ok(())
}

async fn doit(
    service_client: ServiceManagementClient,
    query_client: QueryClient,
    service_name: String,
) -> Result<()> {
    let res = service_client
        .resolve_service_partition(&service_name, PartitionKeyType::Int64, 1, 1000)
        .await?;

    println!("{:#?}", res);

    let res = query_client.get_partition_list(&service_name, 1000).await?;

    println!("{:#?}", res);

    Ok(())
}
