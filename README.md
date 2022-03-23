# litetrace

用rust实现的简易版的trace-cmd工具

## Dependencies

```
# Ubuntu 测试发现 libtracefs-dev 版本过低，还是手动安装
sudo apt-get install llvm-dev libclang-dev clang pkg-config libtracefs-dev libtraceevent-dev -y # need hirsute/impish/jammy[universe]

# Arch
sudo pacman -S clang pkg-config libtracefs libtraceevent

# manual
git clone https://git.kernel.org/pub/scm/libs/libtrace/libtraceevent.git/
cd libtraceevent
make
sudo make install

git clone https://git.kernel.org/pub/scm/libs/libtrace/libtracefs.git/
cd libtracefs
make
sudo make install
```

注：在仓库安装的这俩个依赖都是动态库，静态库需要下载源码自行编译，后续考虑带源码封装成 *-sys FFI 包。

## Design

尽量与[trace-cmd](https://git.kernel.org/pub/scm/linux/kernel/git/rostedt/trace-cmd.git/about/)保持一致，由于只需要支持部分功能，所以会省略不需要的操作。

- 使用libtracefs对tracefs进行读写，用 [bindgen](https://crates.io/crates/bindgen) 生成rust的FFI绑定，使用[pkg-config](https://crates.io/crates/pkg-config)找到依赖库的链接库与头文件位置；
- 阅读[libtracefs文档](https://www.trace-cmd.org/Documentation/libtracefs/)了解功能，再封装一层[tracefs](./src/tracefs.rs)的safe接口供其它包调用；
- rust程序这边，使用[clap](https://crates.io/crates/clap)做参数解析，[thiserror](https://crates.io/crates/thiserror)来统一可能出现的错误信息，从而使得所有错误都能很方便地一直向上传递进行统一处理；
- 阅读 trace-cmd 与 libtracefs 的源码，仿照主要流程编写代码并调试。

## Plan

- ~~支持打开function跟踪和function过滤，动态开启和关闭跟踪~~
- ~~支持查看当前配置状态：~~
    - ~~跟踪类型  current_tracer~~
    - ~~开关状态  tracing_on~~
    - ~~function过滤器状态 set_ftrace_filter~~
- ~~支持导出跟踪结果~~
    - ~~打印trace中的跟踪内容~~

## 支持的命令说明

以下命令与trace-cmd的行为基本一致

```
USAGE:
    litetrace-rs list [OPTIONS]

OPTIONS:
    -f               -f list available functions to filter on
    -t               -t list available tracers

litetrace-rs list -t # 列出支持的tracer，目前只支持其中的function
```

```
USAGE:
    litetrace-rs start [OPTIONS]

OPTIONS:
    -l <FUNCS>         -l filter function name
    -p <PLUGIN>        -p run command with plugin enabled

litetrace-rs start -p function -l ext4_* # 开始追踪以ext4_开头的函数
```

```
USAGE:
    litetrace-rs show [OPTIONS]

OPTIONS:
        --buffer-size-kb          
        --buffer-total-size-kb    
        --current-tracer          
    -f                            -f display the file path that is being dumped
        --set-ftrace-filter       
        --set-ftrace-notrace      
        --set-ftrace-pid          
        --set-graph-function      
        --set-graph-notrace       
        --tracing-cpumask         
        --tracing-on              

litetrace-rs show # 查看trace的内容
litetrace-rs show --tracing_on # 查看tracing_on的内容
```

```
USAGE:
    litetrace-rs stop

litetrace-rs stop # 停止追踪（关闭tracing_on）
```

```
USAGE:
    litetrace-rs clear

litetrace-rs clear # 清除trace的内容
```
