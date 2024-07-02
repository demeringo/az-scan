use azure_identity::AzureCliCredential;
use futures::stream::StreamExt;
use std::sync::Arc;

//use serde_json::Result;

/// Print VM details (in json)
///
///
async fn print_vms(
    credential: Arc<AzureCliCredential>,
    subscription_id: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = azure_mgmt_compute::Client::builder(credential).build()?;

    let mut count = 0;
    let mut vms = client
        .virtual_machines_client()
        .list_all(subscription_id)
        .into_stream();
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

/// Print CPU load of VM
///
///
async fn print_cpu_load(
    credential: Arc<AzureCliCredential>,
    subscription_id: &String,
    resourcegroup_name: &String,
    vm_name: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription().await?;

    print_vms(credential.clone(), &subscription_id).await?;
    //print_vms(credential,&subscription_id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    //static RUNNING_INSTANCE_ID: &str = "i-03c8f84a6318a8186";

    #[tokio::test]
    async fn list_vm() {
        let credential = Arc::new(AzureCliCredential::new());
        let subscription_id = AzureCliCredential::get_subscription().await.unwrap();

        print_vms(credential.clone(), &subscription_id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn print_cpu() {
        let credential = Arc::new(AzureCliCredential::new());
        let subscription_id = AzureCliCredential::get_subscription().await.unwrap();

        let resourcegroup_name = "abc".to_owned();
        let vm_name = "def".to_owned();

        print_cpu_load(
            credential.clone(),
            &subscription_id,
            &resourcegroup_name,
            &vm_name,
        )
        .await
        .unwrap();
    }
}
