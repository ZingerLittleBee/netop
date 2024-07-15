Language : [🇺🇸 English](./README.md) | 🇨🇳 简体中文

<h1 align="center">netop</h1>
<div align="center">

[![Docker Pulls](https://img.shields.io/docker/pulls/zingerbee/netop?style=flat-square)](https://hub.docker.com/r/zingerbee/netop)
[![Docker Image Size](https://img.shields.io/docker/image-size/zingerbee/netop?style=flat-square)](https://hub.docker.com/r/zingerbee/netop/tags)
[![Docker Image Version (latest by date)](https://img.shields.io/docker/v/zingerbee/netop?style=flat-square)](https://hub.docker.com/r/zingerbee/netop/tags)

</div>

<div align="center">

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ZingerLittleBee/netop/publish.yml?style=flat-square)](https://github.com/ZingerLittleBee/netop/actions)
[![Last Commit](https://img.shields.io/github/last-commit/ZingerLittleBee/netop?style=flat-square)](https://github.com/ZingerLittleBee/netop/commits/main)
[![LICENSE](https://img.shields.io/crates/l/netop?style=flat-square)](./LICENSE)

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

## 安装

### Docker
```bash
docker run -it --rm --net=host zingerbee/netop
```

### Cargo
需要 `rust` 和 `pcap`, 具体查看 [如何构建](#如何从源码构建)
```bash
# 安装
sudo cargo install netop
# 运行
netop
# 或者指定网卡运行
netop -n eth0
```

### NetBSD
在 `NetBSD` 上有官方软件包, 要安装预编译的二进制文件，只需运行
```bash
pkgin install netop
```

或者，如果你喜欢从源代码构建它
```bash
cd /usr/pkgsrc/net/netop
make install
```

### Arch Linux
[AUR](https://aur.archlinux.org/packages/netop)
> 由 @kemelzaidan 提供

## 如何使用

- 按 `e` **进入编辑模式**, 输入新 [bpf](https://biot.com/capstats/bpf.html) 规则, **回车**确认
- 按 `Esc` **退出编辑模式**
- 使用方向键 `<-` 和 `->` 在不同规则间切换
- 在非编辑模式下, 按 `dd` 删除当前规则
- 在非编辑模式下, 按 `q` 退出程序

### 查看帮助
```bash
netop -h
# docker
docker run -it --rm --net=host zingerbee/netop -h
```
输出如下
```bash
netop 0.1.4

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
netop -n eth0
# docker
docker run -it --rm --net=host zingerbee/netop -n eth0
```

## 如何从源码构建
开发环境
- 最好是 `root` 用户, `pcap` 需要权限
- `rust` >= 1.40.0
- `pcap`
  - Ubuntu、Debian: `apt install libpcap-dev`
  - MacOS: 系统自带
  - Windows: 下载 [WinPcap](https://www.winpcap.org/install/default.htm) 开发者包, 添加 `/Lib` 或 `/Lib/x64` 目录到系统环境变量中

```bash
# clone
git clone https://github.com/ZingerLittleBee/netop.git
# run
sudo cargo run
```

## 发现问题或提出建议
[Create an issue](https://github.com/ZingerLittleBee/netop/issues)

## 发行说明
SEE [CHANGELOG](./CHANGELOG.md)
