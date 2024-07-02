# az-scan

Experimenting with Azure APIs to retrieve resources information



## Usage

List VM's

```sh
az login 

cargo run
```

## Notes

### SSL

To compile in WSL2, I had to disable native SSL (disable default features and enable rustls) with `default-features = false, features = ["enable_reqwest_rustls"]`.

```toml
[dependencies]
azure_core = { version = "0.20", default-features = false, features = ["enable_reqwest_rustls"] }
azure_identity = { version = "0.20", default-features = false, features = ["enable_reqwest_rustls"] }
azure_mgmt_compute = { version = "0.20", default-features = false, features = ["default_tag", "enable_reqwest_rustls"] }
```

### Authentication

#### Using Azure cli

```sh
az login 
```

#### Env vars


## References

- Getting Metrics [Get Azure Virtual Machine usage data using the REST API - Azure Virtual Machines](https://learn.microsoft.com/en-us/azure/virtual-machines/linux/metrics-vm-usage-rest)
- VM usage metrics [Understanding Azure virtual machine usage - Azure Virtual Machines](https://learn.microsoft.com/en-us/azure/virtual-machines/vm-usage)