We will look at publishing our package to `crate.io` so that other can use it.

First we need to go to `crates.io` and login.

Generate API token by going to Account Settings/API Tokens

Now in terminal run below command:
```sh
cargo login <api token>
```

Now you should be able to publish your packages to `crates.io`.

To publish run below command:

```sh
cargo publish
```

In order to publish our crates we need to first commit our changes. We will need description and license
information as well in `Cargo.toml` before publishing.

```toml
[package]
name = "auth_service"
version = "0.1.0"
edition = "2021"
description = "Example auth service"
license = "MIT"

[dependencies]
rand = "0.8.4"
```

One more thing is package name should be unique.

If you want to prevent anyone from using your published crate you can run below command:

```sh
cargo yank --vers 0.1.0
```

You can also undo this by passing extra `--undo` flag.

```sh
cargo tank --vers 0.1.0 --undo
```

We can also yank a version from `crates.io` dashboard.

I might confuse you why we are calling it crates everywhere eventhough we are publishing packages.
World package and crates are often used interchangebly. On reason is that it's very common to 
publish a package to `crates.io` which only contains one library crate. In those cases, the distinction
between the package and a crate is not very meaningful.
