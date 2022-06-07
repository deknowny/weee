# Weee!
> Weee! It's time for a new release of your product!

`Weee` is version bumper CLI written in Rust for automation version updating process

## Live demo
<video autoplay muted loop>
    <source src="./assets/live-demo.mov" type="video/mov;">
</video>

## Installation
The simplest way is installing from [crates.io page](https://crates.io/crates/rand) with [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):
```shell
cargo install weee
```

Or from the GitHub source
```shell
git clone https://github.com/deknowny/weee && \
cd weee \
cargo install --path .
```

If you want to add some features and install it in the development mode, please, refer to [CONTRIBUTING.md](./CONTRIBUTING.md)

## Advantages over `bumpversion`
* No regexes are required
* Prevent 99% potential errors with pre-run checks
* Has read-only mode to review how files will be changed
* Allows many profiles i.e. for a project version, dependency version and etc.
* Optional templating syntax for dynamic version string builder
* Has a build-in templates according semver rules
* Allows many different style matches of a version in one file
* ~~It's written on Rust~~
