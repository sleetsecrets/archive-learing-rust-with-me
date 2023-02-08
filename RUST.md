# Installing Rust

## Rustup: the Rust installer and version management tool
The primary way that folks install Rust is through a tool called Rustup, which is a Rust installer and version management tool.
人们安装 Rust 的主要方式是通过一个名为 Rustup 的工具，它是一个 Rust 安装程序和版本管理工具。

## Is Rust up to date?
Rust updates very frequently. If you have installed Rustup some time ago, chances are your Rust version is out of date. Get the latest version of Rust by running `rustup update`.
Rust 更新非常频繁。 如果您之前安装过 Rustup，那么您的 Rust 版本可能已经过时了。 通过运行 `rustup update` 获取最新版本的 Rust。

## Toolchain management with `rustup`
Rust is installed and managed by the `rustup` tool. Rust has a 6-week rapid release process and supports a great number of platforms, so there are many builds of Rust available at any time. `rustup` manages these builds in a consistent way on every platform that Rust supports, enabling installation of Rust from the beta and nightly release channels as well as support for additional cross-compilation targets.
Rust 由 `rustup` 工具安装和管理。 Rust 有一个 6 周的快速发布过程，并且支持大量平台，因此随时可以使用许多 Rust 版本。 `rustup` 在 Rust 支持的每个平台上以一致的方式管理这些构建，支持从 beta 和夜间发布渠道安装 Rust，并支持额外的交叉编译目标。

If you've installed `rustup` in the past, you can update your installation by running `rustup update`.
如果您过去安装过 `rustup`，您可以通过运行 `rustup update` 来更新您的安装。

## Configuring the `PATH` environment variable
In the Rust development environment, all tools are installed to the `~/.cargo/bin` directory, and this is where you will find the Rust toolchain, including `rustc`, `cargo`, and `rustup`.
在 Rust 开发环境中，所有工具都安装在 `~/.cargo/bin` 目录下，您可以在这里找到 Rust 工具链，包括 `rustc`、`cargo` 和 `rustup`。

Accordingly, it is customary for Rust developers to include this directory in their `PATH` environment variable. During installation `rustup` will attempt to configure the `PATH`. Because of differences between platforms, command shells, and bugs in `rustup`, the modifications to `PATH` may not take effect until the console is restarted, or the user is logged out, or it may not succeed at all.
因此，Rust 开发人员习惯将此目录包含在他们的 PATH 环境变量中。 在安装过程中，`rustup` 将尝试配置 `PATH`。 由于平台、command shell 和`rustup` 中的错误，对`PATH` 的修改可能需要重启控制台或用户注销才能生效，或者根本不会成功。

If, after installation, running `rustc --version` in the console fails, this is the most likely reason.
如果在安装后，在控制台中运行 `rustc --version` 失败，这是最可能的原因。

## Uninstall Rust
If at any point you would like to uninstall Rust, you can run `rustup self uninstall`. We'll miss you though!
如果在任何时候你想卸载 Rust，你可以运行 `rustup self uninstall`。 不过我们会想念你的！

```
sleetsecrets@lolita learn-rust-with-me % rustup -h
rustup 1.25.1 (bb60b1e89 2022-07-12)
The Rust toolchain installer

USAGE:
    rustup [FLAGS] [+toolchain] <SUBCOMMAND>

FLAGS:
    -v, --verbose    Enable verbose output
    -q, --quiet      Disable progress output
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <+toolchain>    release channel (e.g. +stable) or custom toolchain to set override
    <+toolchain>    发布渠道（例如 +stable）          或 自定义工具链以设置覆盖

SUBCOMMANDS:
    show           Show the active and installed toolchains or profiles
                   显示活动和已安装的工具链或配置文件
    update         Update Rust toolchains and rustup
                   更新 Rust 工具链和 rustup
    check          Check for updates to Rust toolchains and rustup
                   检查 Rust 工具链和 rustup 的更新
    default        Set the default toolchain
                   设置默认工具链
    toolchain      Modify or query the installed toolchains
                   修改或查询已安装的工具链
    target         Modify a toolchain's supported targets
                   修改工具链支持的目标
    component      Modify a toolchain's installed components
                   修改工具链的已安装组件
    override       Modify directory toolchain overrides
                   修改目录工具链覆盖
    run            Run a command with an environment configured for a given toolchain
                   使用为给定工具链配置的环境运行命令
    which          Display which binary will be run for a given command
                   显示将为给定命令运行哪个二进制文件
    doc            Open the documentation for the current toolchain
                   打开当前工具链的文档
    man            View the man page for a given command
                   查看给定命令的手册页
    self           Modify the rustup installation
                   修改rustup安装
    set            Alter rustup settings
                   更改 rustup 设置
    completions    Generate tab-completion scripts for your shell
                   为你的 shell 生成 tab 完成脚本
    help           Prints this message or the help of the given subcommand(s)
                   打印此消息或给定子命令的帮助

DISCUSSION:
    Rustup installs The Rust Programming Language from the official
    release channels, enabling you to easily switch between stable,
    beta, and nightly compilers and keep them updated. It makes
    cross-compiling simpler with binary builds of the standard library
    for common platforms.
    Rustup 从官方发布渠道安装 Rust 编程语言，使您能够轻松地在稳定版、测试版和夜间编译器之间切换并保持更新。 它使用通用平台标准库的二进制构建使交叉编译更简单。

    If you are new to Rust consider running `rustup doc --book` to
    learn Rust.
    如果您是 Rust 的新手，请考虑运行 `rustup doc --book` 来
     学习生锈。
```

## Cargo: the Rust build tool and package manager

当您安装 Rustup 时，您还将获得最新稳定版本的 Rust 构建工具和包管理器，也称为 Cargo。 Cargo 做了很多事情：
When you install Rustup you’ll also get the latest stable version of the Rust build tool and package manager, also known as Cargo. Cargo does lots of things:

- build your project with cargo build
- run your project with cargo run
- test your project with cargo test
- build documentation for your project with cargo doc
- publish a library to crates.io with cargo publish

To test that you have Rust and Cargo installed, you can run this in your terminal of choice:
要测试您是否安装了 Rust 和 Cargo，您可以在您选择的终端中运行：

`cargo --version`

```
sleetsecrets@lolita learn-rust-with-me % cargo -h
Rust's package manager

USAGE:
    cargo [+toolchain] [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -V, --version               Print version info and exit
        --list                  List installed commands
        --explain <CODE>        Run `rustc --explain CODE`
    -v, --verbose               Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet                 Do not print cargo log messages
        --color <WHEN>          Coloring: auto, always, never
        --frozen                Require Cargo.lock and cache are up to date
        --locked                Require Cargo.lock is up to date
        --offline               Run without accessing the network
        --config <KEY=VALUE>    Override a configuration value
    -Z <FLAG>                   Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for
                                details
    -h, --help                  Print help information

Some common cargo commands are (see all commands with --list):
    build, b    Compile the current package
    check, c    Analyze the current package and report errors, but don't build object files
    clean       Remove the target directory
    doc, d      Build this package's and its dependencies' documentation
    new         Create a new cargo package
    init        Create a new cargo package in an existing directory
    add         Add dependencies to a manifest file
    run, r      Run a binary or example of the local package
    test, t     Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock
    search      Search registry for crates
    publish     Package and upload this package to the registry
    install     Install a Rust binary. Default location is $HOME/.cargo/bin
    uninstall   Uninstall a Rust binary

See 'cargo help <command>' for more information on a specific command.
```
