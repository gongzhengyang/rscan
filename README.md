A modern port scanner and `icmp` scanner. Fast, effective.

Fast scan network by sending `icmp`, `tcp` packets, inspired by `nmap` but doesn't depend on `nmap`.

## Usage

you will need root privileges to execute

### `ping/icmp` scan

you can `icmp` a `cidr`，`ipaddress`, separated by commas, set timeout argument as a global execute time limit, use seconds as unit.

```
$ sudo ./rscan icmp 1.1.1.1/28,1.0.0.0/24 --timeout 10
rscan|icmp|1.0.0.50|
rscan|icmp|1.0.0.52|
rscan|icmp|1.0.0.121|
rscan|icmp|1.1.1.5|
rscan|icmp|1.1.1.6|
rscan|icmp|1.0.0.60|
rscan|icmp|1.0.0.247|
rscan|icmp|1.1.1.10|
rscan|icmp|1.0.0.55|
rscan|icmp|1.0.0.116|
rscan|icmp|1.1.1.12|
....
send 272 ips, receive packets from 272 ips
```

you can still set a `env` named RUST_LOG to get more log info

```
$ sudo RUST_LOG=debug ./rscan icmp 1.1.1.1/28,1.0.0.0/24 --timeout 10
```

or

```
$ export RUST_LOG=debug
$ sudo ./rscan icmp 1.1.1.1/28,1.0.0.0/24 --timeout 10
```

### `tcp` scan

use `tcp` as argument, add ports options 

```
$ sudo ./rscan tcp 1.1.1.1/28 --ports 80,443 --timeout 10
rscan|tcp|1.1.1.11:80|
rscan|tcp|1.1.1.12:443|
rscan|tcp|1.1.1.11:443|
rscan|tcp|1.1.1.10:80|
rscan|tcp|1.1.1.7:443|
rscan|tcp|1.1.1.3:443|
rscan|tcp|1.1.1.6:443|
rscan|tcp|1.1.1.5:80|
rscan|tcp|1.1.1.10:443|
rscan|tcp|1.1.1.3:80|
rscan|tcp|1.1.1.7:80|
...
```

## License


Licensed under either of

-   Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
-   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
