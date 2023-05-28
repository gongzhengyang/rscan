A modern port scanner and `icmp` scanner. Fast, effective.

Fast scan network by sending `icmp`, `tcp/udp` packets, inspired by `nmap` but doesn't depend on `nmap`.

## Supported Platforms

- `Linux`
- `Android`
- `FreeBSD`

## Supported Scan protocols

- `icmp/ping`
- `tcp`
- `udp`
- `arp`

## Usage

you will need root privileges to execute

### `ping/icmp` scan

you can `icmp` a `cidr`，`ipaddress`, separated by commas, set timeout argument as a global execute time limit, use seconds as unit.

```
$ sudo ./rscan icmp 1.1.1.1/28,1.0.0.0/24 --timeout 10
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
rscan|tcp|1.1.1.10:443|
rscan|tcp|1.1.1.3:80|
rscan|tcp|1.1.1.7:80|
...
```

### `udp` Scan

`udp` scan based on `icmp` reply with Port Unreachable for `udp` packets if `udp` port is not open, please make sure timeout is big enough to receive all `icmp` for all `udp` packets, so the `udp` scan cannot guarantee 100% accuracy.

Each `ip` limit sends `udp` packets at least 0.5 seconds apart.

```
$ sudo ./rscan udp 10.30.6.0/24 --ports 151-165 --timeout=50
rscan|udp|10.30.6.165:161|
rscan|udp|10.30.6.200:162|
...
```

### `arp` Scan

Use the `arp` protocol to scan `intranet` devices

```
$ sudo ./rscan arp 10.30.6.0/16
rscan|arp|10.30.6.10|0c:73:eb:92:0c:ef|
rscan|arp|10.30.6.14|0c:73:eb:94:5e:ed|
rscan|arp|10.30.6.15|20:ce:2a:70:d0:8a|
rscan|arp|10.30.6.16|0c:73:eb:90:d6:73|
rscan|arp|10.30.6.17|e4:f1:4c:57:84:da|
rscan|arp|10.30.6.18|e4:f1:4c:57:82:62|
rscan|arp|10.30.6.26|00:90:0b:99:85:fc|
rscan|arp|10.30.6.57|ac:1f:6b:e6:33:96|

....
```



## License


Licensed under either of

-   Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
-   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
