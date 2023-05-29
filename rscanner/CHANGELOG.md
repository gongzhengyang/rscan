# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# Unreleased

- 
# 0.4.1 (29. May, 2023)
- **changed:** improve receive packets speed

# 0.4.0 (28. May, 2023)

- **added:** add `arp` scan support
- **changed:** move `parse.rs, sockets_iter.rs` into `setting` directory

# 0.3.0 (10. May, 2023)

- **added:** add common trait named `SocketScanner` for `tcp/udp` scan
- **changed:** change `rscanner/execute/tcp` to `impl SocketScanner`
- **added:** add `udp` scanner 

# 0.2.1（19. May, 2023）

- **added:** `icmp`, `tcp` scan support
- **added:** system default limit change
