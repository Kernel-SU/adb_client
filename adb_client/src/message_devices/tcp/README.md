# Examples

## Run a shell command on device

```rust no_run
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use adb_client::{tcp::ADBTcpDevice, ADBDeviceExt};

let mut device = ADBTcpDevice::new((IpAddr::from([192, 168, 0, 10]), 43210)).expect("cannot find device");
device.shell_command(&"ls", Some(&mut std::io::stdout()), None);
```
