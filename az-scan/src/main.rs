use azure_identity::AzureCliCredential;
use futures::stream::StreamExt;
use std::sync::Arc;

//use serde_json::Result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription().await?;
    let client = azure_mgmt_compute::Client::builder(credential).build()?;

    let mut count = 0;
    let mut vms = client.virtual_machines_client().list_all(subscription_id).into_stream();
    while let Some(vms) = vms.next().await {
        let vms = vms?;
        count += vms.value.len();
        for vm in vms.value {
            //println!("{:?}", &vm.resource.id);
            let j = serde_json::to_string(&vm);
           // println!("{:?}", &vm);
           println!("{}", j.unwrap());
            //println!("{:?}", j.unwrap());
        }
    }
    //println!("# of virtual machines {count}");

    Ok(())
}