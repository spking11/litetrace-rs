# litetrace

用rust实现的简易版的trace-cmd工具

## Dependencies

```
# Ubuntu
sudo apt-get install llvm-dev libclang-dev clang pkg-config libtracefs-dev libtraceevent-dev -y # need hirsute/impish/jammy[universe]

# Arch
sudo pacman -S clang pkg-config libtracefs libtraceevent
```

注：在仓库安装的这俩个依赖都是动态库，静态库需要下载源码自行编译，后续考虑带源码封装成 *-sys FFI 包。

## Design

使用libtracefs对tracefs进行读写，用 [bindgen](https://crates.io/crates/bindgen) 生成rust的FFI绑定。

## Plan

- 支持打开function跟踪和function过滤，动态开启和关闭跟踪
- ~~支持查看当前配置状态：~~
    - ~~跟踪类型  current_tracer~~
    - ~~开关状态  tracing_on~~
    - ~~function过滤器状态 set_ftrace_filter~
- ~~支持导出跟踪结果~~
    - ~~打印trace中的跟踪内容~~
