# sf-rs

This crate contains Rust bindings for calling Service Fabric (SF) using the
native COM API on Windows. The SF APIs are specified through IDL files that we
fetch from the [github.com/Azure/sf-c-util](https://github.com/Azure/sf-c-util)
project.

## Generating sf.winmd

The first step is to build the `sf.winmd` file from the SF IDL files. We do this
using the [win32metadata](https://github.com/microsoft/win32metadata) tooling
that knows how to munch IDL files and produce `.winmd` files. An _msbuild_
project file has been crafted to accomplish this which you will find at
[.metadata/generate.proj](.metadata/generate.proj). I simply followed the
instructions from the blog post "[Generating metadata for use with the windows
crate for
Rust](https://withinrafael.com/2023/01/18/generating-metadata-for-the-windows-crate?)"
to write this file up. Here's what you'll need to do to generate (or
re-generate) `sf.winmd`:

1. If you haven't already, install the Visual Studio component called
   `MSVC v143 - VS 2022 C++ ARM64/ARM64EC build tools (Latest)` using the
   _Visual Studio Installer_. If you're doing this out in the future when VS
   2022 is old ancient software, please supplant 2022 with whatever is the
   current newfangled goodness.

2. Open a Visual Studio Development terminal and navigate to the `.metadata`
   folder and run:

```powershell
git submodule init --update
dotnet build
```

That's it. This will generate the `sf.winmd` file in the `.windows\winmd`
folder.

## Generating Rust bindings

Next we generate Rust bindings from the `sf.winmd` file. This is accomplished
using the [windows-bindgen](https://crates.io/crates/windows-bindgen) and
[windows-metadata](https://crates.io/crates/windows-metadata) crates. The code
for generating this lives in
[crates/tools/api/src/main.rs](crates/tools/api/src/main.rs). From a terminal
that's located in the root of the repo, run the following command to generate
(or re-generate) [src/bindings.rs](src/bindings.rs).

```powershell
cargo run --package api
```

This will gobble up `sf.winmd` and produce `bindings.rs` which at the time of
writing this readme, is a measly 132,000 odd lines long. Easy peasy.