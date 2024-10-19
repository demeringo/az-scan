use azure_identity::DefaultAzureCredential;
use azure_mgmt_compute::operations::{virtual_machines, virtual_machine_instance_view};
use azure_mgmt_compute::models::VirtualMachine;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an authentication credential
    let credential = DefaultAzureCredential::default();

    // Define the subscription ID and resource group
    let subscription_id = "YOUR_SUBSCRIPTION_ID";
    let resource_group = "YOUR_RESOURCE_GROUP";

    // Get the list of VMs
    let vm_list = virtual_machines::list(subscription_id, resource_group, credential.clone()).await?;

    for vm in vm_list.value {
        let vm_name = vm.name.clone().unwrap_or_default();
        println!("VM Name: {}", vm_name);

        // Retrieve the instance view (for state)
        let instance_view = virtual_machine_instance_view::get(subscription_id, resource_group, &vm_name, credential.clone()).await?;
        
        // Display VM status
        if let Some(statuses) = instance_view.statuses {
            for status in statuses {
                println!("Status: {}", status.display_status.clone().unwrap_or_default());
            }
        }

        // Fetch and display attached disks
        if let Some(storage_profile) = vm.storage_profile {
            if let Some(os_disk) = storage_profile.os_disk {
                if let Some(os_disk_managed_disk) = os_disk.managed_disk {
                    println!("OS Disk ID: {}", os_disk_managed_disk.id.clone().unwrap_or_default());
                }
                if let Some(disk_size_gb) = os_disk.disk_size_gb {
                    println!("OS Disk Size (GB): {}", disk_size_gb);
                }
            }

            if let Some(data_disks) = storage_profile.data_disks {
                for (i, data_disk) in data_disks.into_iter().enumerate() {
                    println!("Data Disk {}: ID = {:?}", i + 1, data_disk.managed_disk.clone().and_then(|md| md.id));
                    println!("Data Disk {}: Size (GB) = {:?}", i + 1, data_disk.disk_size_gb);
                }
            }
        }
    }

    Ok(())
}
