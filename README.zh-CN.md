Language : [🇺🇸 English](./README.md) | 🇨🇳 简体中文

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

## 简介
`netop` 是可以自定义过滤**网络流量**规则的终端命令行界面 🎯

### 特点
- 使用 `bpf` 规则过滤
- 多规则切换
- 实时速率
- 总流量
- 响应式 UI
- 资源占用小，`Rust` 编写
- `docker` 部署



## 如何使用
```bash
docker run -it --rm --net=host zingerbee/netop
```

- 按 `e` 输入新 [bpf](https://biot.com/capstats/bpf.html) 规则 , 回车确认
- 使用方向键 `<-` 和 `->` 在不同规则间切换

### 查看帮助
```bash
docker run -it --rm --net=host zingerbee/netop -h
```
输出如下
```bash
netop 0.1.0

USAGE:
    netop [OPTIONS]

OPTIONS:
    -h, --help           打印帮助信息
    -n, --name <NAME>    指定网卡运行
    -V, --version        打印版本信息
```

### 指定网卡运行
> 参数为**网卡名称**
>
> *unix: 使用 `ifconfig` 查看所有网卡信息
>
> windows: 使用 `ipconfig` 查看所有网卡信息
```bash
docker run -it --rm --net=host zingerbee/netop -n eth0
```