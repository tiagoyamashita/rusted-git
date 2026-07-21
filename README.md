<h1 align="center">
<img width="300px" src="assets/logo.png" />

[![CI][s0]][l0] [![crates][s1]][l1] ![MIT][s2] [![UNSAFE][s3]][l3] [![TWEET][s6]][l6] [![dep_status][s7]][l7] [![discord][s8]][l8]

</h1>

[s0]: https://github.com/tiagoyamashita/rusted-git/workflows/CI/badge.svg
[l0]: https://github.com/tiagoyamashita/rusted-git/actions
[s1]: https://img.shields.io/crates/v/rusted-git.svg
[l1]: https://crates.io/crates/rusted-git
[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[s3]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[l3]: https://github.com/rust-secure-code/safety-dance/
[s6]: https://img.shields.io/twitter/follow/extrawurst?label=follow&style=social
[l6]: https://twitter.com/intent/follow?screen_name=extrawurst
[s7]: https://deps.rs/repo/github/tiagoyamashita/rusted-git/status.svg
[l7]: https://deps.rs/repo/github/tiagoyamashita/rusted-git
[s8]: https://img.shields.io/discord/1176858176897953872
[l8]: https://discord.gg/rQNeEnMhus

<h5 align="center">rusted-git provides you with the comfort of a git GUI but right in your terminal</h1>

![](demo.gif)

## <a name="table-of-contents"></a> Table of Contents

1. [Features](#features)
2. [Motivation](#motivation)
3. [Benchmarks](#bench)
4. [Roadmap](#roadmap)
5. [Limitations](#limitations)
6. [Installation](#installation)
7. [Build](#build)
8. [FAQs](#faqs)
9. [Diagnostics](#diagnostics)
10. [Color Theme](#theme)
11. [Key Bindings](#bindings)
12. [Sponsoring](#sponsoring)
13. [Inspiration](#inspiration)
14. [Contributing](#contributing)
15. [Contributors](#contributors)

## 1. <a name="features"></a> Features <small><sup>[Top ▲](#table-of-contents)</sup></small>

- Fast and intuitive **keyboard only** control
- Context based help (**no need to memorize** tons of hot-keys)
- Inspect, commit, and amend changes (incl. hooks: *pre-commit*,*commit-msg*,*post-commit*,*prepare-commit-msg*)
- Stage, unstage, revert and reset files, hunks and lines
- Stashing (save, pop, apply, drop, and inspect)
- Push / Fetch to / from remote
- Branch List (create, rename, delete, checkout, remotes)
- Browse / **Search** commit log, diff committed changes
- Responsive terminal UI
- Async git API for fluid control
- Submodule support
- gpg commit signing with shortcomings (see [#97](https://github.com/tiagoyamashita/rusted-git/issues/97)))

## 2. <a name="motivation"></a> Motivation <small><sup>[Top ▲](#table-of-contents)</sup></small>

I do most of my git work in a terminal but I frequently found myself using git GUIs for some use-cases like: index, commit, diff, stash, blame and log.

Unfortunately popular git GUIs all fail on giant repositories or become unresponsive and unusable.

rusted-git provides you with the user experience and comfort of a git GUI but right in your terminal while being portable, fast, free and opensource.

## 3. <a name="bench"></a> Benchmarks <small><sup>[Top ▲](#table-of-contents)</sup></small>

For a [RustBerlin meetup presentation](https://youtu.be/rpilJV-eIVw?t=5334) ([slides](https://github.com/extrawurst/gitui-presentation)) I compared `lazygit`,`tig` and `rusted-git` by parsing the entire Linux git repository (which contains over 900k commits):

|           | Time       | Memory (GB) | Binary (MB) | Freezes   | Crashes   |
| --------- | ---------- | ----------- | ----------- | --------- | --------- |
| `rusted-git`   | **24 s** ✅ | **0.17** ✅  | 10         | **No** ✅  | **No** ✅  |
| `lazygit` | 57 s       | 2.6         | 25          | Yes       | Sometimes |
| `tig`     | 4 m 20 s   | 1.3         | **0.6** ✅   | Sometimes | **No** ✅  |

## 4. <a name="roadmap"></a> Road(map) to 1.0 <small><sup>[Top ▲](#table-of-contents)</sup></small>

These are the high level goals before calling out `1.0`:

* visualize branching structure in log tab ([#81](https://github.com/tiagoyamashita/rusted-git/issues/81))
* interactive rebase ([#32](https://github.com/tiagoyamashita/rusted-git/issues/32))
- no git-lfs support (see [#2812](https://github.com/tiagoyamashita/rusted-git/issues/2812))

## 5. <a name="limitations"></a> Known Limitations <small><sup>[Top ▲](#table-of-contents)</sup></small>

- no sparse repo support (see [#1226](https://github.com/tiagoyamashita/rusted-git/issues/1226))
- *credential.helper* for https needs to be **explicitly** configured (see [#800](https://github.com/tiagoyamashita/rusted-git/issues/800))

Currently, this tool does not fully substitute the _git shell_, however both tools work well in tandem.

The priorities for `rusted-git` are on features that are making me mad when done on the _git shell_, like stashing, staging lines or hunks. Eventually, I will be able to work on making `rusted-git` a one stop solution - but for that I need help - this is just a spare time project for now.

All support is welcomed! Sponsors as well! ❤️

## 6. <a name="installation"></a> Installation <small><sup>[Top ▲](#table-of-contents)</sup></small>

rusted-git is in beta and may contain bugs and missing features. However, for personal use it is reasonably stable and is being used while developing itself.

<a href="https://repology.org/project/rusted-git/versions">
    <img src="https://repology.org/badge/vertical-allrepos/rusted-git.svg" alt="Packaging status" align="right">
</a>

### Various Package Managers

<details>
  <summary>Install Instructions</summary>

##### [Arch Linux](https://archlinux.org/packages/extra/x86_64/rusted-git/)

```sh
pacman -S rusted-git
```

##### Fedora

```sh
sudo dnf install rusted-git
```

##### Gentoo
Available in [dm9pZCAq overlay](https://github.com/gentoo-mirror/dm9pZCAq)

```sh
sudo eselect repository enable dm9pZCAq
sudo emerge --sync dm9pZCAq
sudo emerge dev-vcs/rusted-git::dm9pZCAq
```

##### [openSUSE](https://software.opensuse.org/package/rusted-git)

```sh
sudo zypper install rusted-git
```

##### Homebrew (macOS)

```sh
brew install rusted-git
```

##### [MacPorts (macOS)](https://ports.macports.org/port/rusted-git/details/)

```sh
port install rusted-git
```

##### [Winget](https://github.com/microsoft/winget-pkgs/tree/master/manifests/s/StephanDilly/rusted-git) (Windows)

```
winget install rusted-git
```

##### [Scoop](https://github.com/ScoopInstaller/Main/blob/master/bucket/rusted-git.json) (Windows)

```
scoop install rusted-git
```

##### [Chocolatey](https://chocolatey.org/packages/rusted-git) (Windows)

```
choco install rusted-git
```

##### [Mise](https://github.com/jdx/mise)

```shell
mise use -g rusted-git@latest
```

##### [Nix](https://search.nixos.org/packages?channel=unstable&show=rusted-git&from=0&size=50&sort=relevance&query=rusted-git) (Nix/NixOS)

Nixpkg
```
nix-env -iA nixpkgs.rusted-git
```
NixOS
```
nix-env -iA nixos.rusted-git
```

##### [Termux](https://github.com/termux/termux-packages/tree/master/packages/rusted-git) (Android)

```
pkg install rusted-git
```

##### [Anaconda](https://anaconda.org/conda-forge/rusted-git)
```
conda install -c conda-forge rusted-git
```

</details>

### Release Binaries

[Available for download in releases](https://github.com/tiagoyamashita/rusted-git/releases)

Binaries available for:

#### Linux

- rusted-git-linux-x86_64.tar.gz (linux musl statically linked)
- rusted-git-linux-aarch64.tar.gz (linux on 64 bit arm)
- rusted-git-linux-arm.tar.gz
- rusted-git-linux-armv7.tar.gz

All contain a single binary file

#### macOS

- rusted-git-mac.tar.gz (arm64)
- rusted-git-mac-x86.tar.gz (intel x86)

#### Windows

- rusted-git-win.tar.gz (single 64bit binary)
- rusted-git-win.msi (64bit Installer package)

### Nightly Builds

see [NIGHTLIES.md](./NIGHTLIES.md)

## 7. <a name="build"></a> Build <small><sup>[Top ▲](#table-of-contents)</sup></small>

### Requirements

- Minimum supported `rust`/`cargo` version: `1.88`
  - See [Install Rust](https://www.rust-lang.org/tools/install)

- To build openssl dependency (see https://docs.rs/openssl/latest/openssl/)
  - perl >= 5.12 (strawberry perl works for windows https://strawberryperl.com/)
  - a c compiler (msvc, gcc or clang, cargo will find it)

- To run the complete test suite python is required (and it must be invocable as `python`)

### Cargo Install

The simplest way to start playing around with `rusted-git` is to have `cargo` build and install it with `cargo install rusted-git --locked`. If you are not familiar with rust and cargo: [Getting Started with Rust](https://doc.rust-lang.org/book/ch01-00-getting-started.html)

### Cargo Features
#### trace-libgit
enable `libgit2` tracing

works if `libgit2` built with `-DENABLE_TRACE=ON`

this feature enabled by default, to disable: `cargo install --no-default-features`

## 8. <a name="faqs"></a> FAQs <small><sup>[Top ▲](#table-of-contents)</sup></small>

see [FAQs page](./FAQ.md)

## 9. <a name="diagnostics"></a> Diagnostics <small><sup>[Top ▲](#table-of-contents)</sup></small>

To run with logging enabled run `rusted-git -l`.

This will log to:

- macOS: `$HOME/Library/Caches/rusted-git/rusted-git.log`
- Linux using `XDG`: `$XDG_CACHE_HOME/rusted-git/rusted-git.log`
- Linux: `$HOME/.cache/rusted-git/rusted-git.log`
- Windows: `%LOCALAPPDATA%/rusted-git/rusted-git.log`

## 10. <a name="theme"></a> Color Theme <small><sup>[Top ▲](#table-of-contents)</sup></small>

![](assets/light-theme.png)

`rusted-git` should automatically work on both light and dark terminal themes.

However, you can customize everything to your liking: See [Themes](THEMES.md).

## 11. <a name="bindings"></a> Key Bindings <small><sup>[Top ▲](#table-of-contents)</sup></small>

The key bindings can be customized: See [Key Config](KEY_CONFIG.md) on how to set them to `vim`-like bindings.

## 12. <a name="sponsoring"></a> Sponsoring <small><sup>[Top ▲](#table-of-contents)</sup></small>

[![github](https://img.shields.io/badge/-GitHub%20Sponsors-fafbfc?logo=GitHub%20Sponsors)](https://github.com/sponsors/extrawurst)

## 13. <a name="inspiration"></a> Inspiration <small><sup>[Top ▲](#table-of-contents)</sup></small>

- [lazygit](https://github.com/jesseduffield/lazygit)
- [tig](https://github.com/jonas/tig)
- [GitUp](https://github.com/git-up/GitUp)
  - It would be nice to come up with a way to have the map view available in a terminal tool
- [git-brunch](https://github.com/andys8/git-brunch)

## 14. <a name="contributing"></a> Contributing <small><sup>[Top ▲](#table-of-contents)</sup></small>

See [CONTRIBUTING.md](CONTRIBUTING.md).

## 15. <a name="contributors"></a> Contributors <small><sup>[Top ▲](#table-of-contents)</sup></small>

Thanks goes to all the contributors that help make rusted-git amazing! ❤️

Wanna become a co-maintainer? We are looking for [you](https://github.com/tiagoyamashita/rusted-git/issues/2084)!

<a href="https://github.com/tiagoyamashita/rusted-git/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=tiagoyamashita/rusted-git" />
</a>
