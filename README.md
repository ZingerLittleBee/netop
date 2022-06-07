Language : 🇺🇸 English | [🇨🇳 简体中文](./README.zh-CN.md)

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
`netop` is a terminal command line interface that can customize the filter **network traffic** rule 🎯

### Features
- Use the `bpf` rule filter
- Multi-rule switching
- Real-time rate
- Total traffic
- Response UI
- Resource occupation is small, `rust` Written
- Support `docker` deployment


## How to use
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