Language : đēđ¸ English | [đ¨đŗ įŽäŊä¸­æ](./README.zh-CN.md)

<h1 align="center">netop</h1>
<div align="center">

[![Docker Pulls](https://img.shields.io/docker/pulls/zingerbee/netop?style=flat)](https://hub.docker.com/r/zingerbee/netop)
[![Docker Image Size](https://img.shields.io/docker/image-size/zingerbee/netop)](https://hub.docker.com/r/zingerbee/netop/tags)
[![Docker Image Version (latest by date)](https://img.shields.io/docker/v/zingerbee/netop)](https://hub.docker.com/r/zingerbee/netop/tags)

</div>

<div align="center">

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ZingerLittleBee/netop/Docker%20Images%20CI)](https://github.com/ZingerLittleBee/netop/actions)
[![Last Commit](https://img.shields.io/github/last-commit/ZingerLittleBee/netop)](https://github.com/ZingerLittleBee/netop/commits/main)
[![LICENSE](https://img.shields.io/crates/l/port-selector)](./LICENSE)

</div>

<div align="center">
<img src="./snapshot/dashboard.gif">
</div>

## Overview
`netop` is a terminal command line interface that can customize the filter **network traffic** rule đ¯

### Features
- Use the `bpf` rule filter
- Multi-rule switching
- Real-time rate
- Total traffic
- Response UI
- Resource occupation is small, `rust` Written
- Support `docker` deployment


## How to use
### Docker
```bash
docker run -it --rm --net=host zingerbee/netop
```

- Press `e` to input [bpf rule](https://biot.com/capstats/bpf.html), and then press `enter`
- Use the `<-` or `->` to switch between different rules

### View help
```bash
docker run -it --rm --net=host zingerbee/netop -h
```
Output as follows
```bash
netop 0.1.0

USAGE:
    netop [OPTIONS]

OPTIONS:
    -h, --help           Print help information
    -n, --name <NAME>    Name of Network Interface Card
    -V, --version        Print version information
```

### Designated network interface card running
> The parameter is **name of network interface card**
>
> *unix: Use `ifconfig` to view network interface cards information
>
> windows: Use `ipconfig` to view network interface cards information
```bash
docker run -it --rm --net=host zingerbee/netop -n eth0
```

### Cargo
need `rust` and `pcap`, detail look at [How to build](#how-to-build)
```bash
# install
sudo cargo install netop
# and then run
netop
# or designated network interface card running
netop -n eth0
```

### NetBSD
On NetBSD there's an official package available.
To install a pre-compiled binary, simply run
```bash
pkgin install netop
```

Or, if you prefer to build it from source,
```bash
cd /usr/pkgsrc/net/netop
make install
```

## How to build
Development environment
- It is best to be `root` user, `pcap` requires permissions
- `rust` >= 1.40.0
- `pcap`
    - UbuntuãDebian: `apt install libpcap-dev`
    - MacOS: installed by default
    - Windows: Download the [WinPcap](https://www.winpcap.org/install/default.htm) Developer's Pack. Add the /Lib or /Lib/x64 folder to your LIB environment variable.

```bash
sudo cargo install --path .
# or
sudo cargo run
```
