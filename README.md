# Viddy

<p align="center">
<img src="images/logo.png" width="200" alt="viddy" title="viddy" />
</p>

Modern `watch` command.

Viddy well, gopher. Viddy well.

## Demo

<p align="center">
<img src="images/demo.gif" width="100%" alt="viddy" title="viddy" />
</p>

## Features

* Basic features of original watch command.
    * Execute command periodically, and display the result.
    * color output.
    * diff highlight.
* Time machine mode. 😎
    * Rewind like video.
    * Go to the past, and back to the future.
* Look back history.
    * Save and load history.
* See output in pager.
* Vim like keymaps.
* Search text.
* Suspend and restart execution.
* Support shell alias
    * See detail https://github.com/sachaos/viddy/issues/2#issuecomment-904002053
* Customize keymappings.
* Customize color.

## Install

### Cargo

```shell
cargo install viddy
```

### [Homebrew](https://brew.sh)

```shell
brew install viddy
```

### Linux

```shell
wget -O viddy.tar.gz https://github.com/sachaos/viddy/releases/download/v1.3.0/viddy-v1.3.0-linux-x86_64.tar.gz && tar xvf viddy.tar.gz && mv viddy /usr/local/bin
```

### Other

Download from [release page](https://github.com/sachaos/viddy/releases).

## Install with Other Package Managers (Community-Maintained)

### [pkg.haus](https://pkg.haus)

APT packages for Debian stable, testing and unstable (amd64 and arm64), built from source at release tags. Set up the archive per the instructions on [pkg.haus](https://pkg.haus), then:

```shell
sudo apt install viddy
```

### [MacPorts](https://www.macports.org)

```shell
sudo port install viddy
```

### [Scoop](https://scoop.sh/)

To install Viddy on Windows, first install the Scoop package manager, and then run the commands below.

**NOTE**: The git package is required in order to add additional Scoop "buckets".

```
scoop install git
scoop bucket add extras
scoop install extras/viddy
```

### ArchLinux ( AUR )

```shell
yay -S viddy
```
Alternatively you can use the [AUR Git repo](https://aur.archlinux.org/packages/viddy/) directly

### Alpine Linux

After [enabling the community repository](https://wiki.alpinelinux.org/wiki/Enable_Community_Repository):

```shell
apk add viddy
```

### [asdf version manager](https://asdf-vm.com)

```shell
asdf plugin add viddy
asdf install viddy latest
asdf global viddy latest
```

## Keymaps

| key       |                                            |
|-----------|--------------------------------------------|
| SPACE     | Toggle time machine mode                   |
| s         | Toggle <ins>s</ins>uspend execution                   |
| b         | Toggle ring terminal <ins>b</ins>ell                  |
| d         | Toggle <ins>d</ins>iff                                |
| t         | Toggle header/<ins>t</ins>itle display                      |
| ?         | Toggle help view                           |
| /         | Search text                                |
| j         | Pager: next line                           |
| k         | Pager: previous line                       |
| h         | Pager: move left                           |
| l         | Pager: move right                          |
| Control-F | Pager: page down                           |
| Control-B | Pager: page up                             |
| g         | Pager: go to top of page                   |
| Shift-G   | Pager: go to bottom of page                |
| Shift-J   | (Time machine mode) Go to the past         |
| Shift-K   | (Time machine mode) Back to the future     |
| Shift-F   | (Time machine mode) Go to more past        |
| Shift-B   | (Time machine mode) Back to more future    |
| Shift-O   | (Time machine mode) Go to oldest position  |
| Shift-N   | (Time machine mode) Go to current position |

## Configuration

Viddy can be used without any configuration.
However, if you want to customize the keybindings or default behavior, you can do so.

Install your config file as `config.toml` (or `.json5`, `.json`, `.yaml`, `.ini`, checked in that
order) in viddy's config directory:

- macOS: `~/Library/Application Support/dev.sachaos.viddy/config.toml`
- Linux: `$XDG_CONFIG_HOME/viddy/config.toml` (falls back to `~/.config/viddy/config.toml`)
- Windows: `{FOLDERID_RoamingAppData}\sachaos\viddy\config\config.toml`

Override the directory with the `VIDDY_CONFIG` environment variable.

```toml
[general]
no_shell = false
shell = "zsh"
shell_options = ""
skip_empty_diffs = false
disable_mouse = true
disable_auto_save = false
no_title = false
no_status = false
min_interval_ms = 500
interval_step_ms = 500

[keybindings.All]
"<Shift-j>" = "GoToPast"
"<Shift-f>" = "GoToMorePast"
"<Shift-k>" = "GoToFuture"
"<Shift-b>" = "GoToMoreFuture"
"<Shift-o>" = "GoToOldest"
"<Shift-n>" = "GoToCurrent"
"<h>" = "ScrollLeft"
"<l>" = "ScrollRight"
"<k>" = "ResultScrollUp"
"<j>" = "ResultScrollDown"
"<Ctrl-u>" = "ResultHalfPageUp"
"<Ctrl-d>" = "ResultHalfPageDown"
"<Ctrl-b>" = "ResultPageUp"
"<Ctrl-f>" = "ResultPageDown"
"<Shift-g>" = "BottomOfPage"
"<g><g>" = "TopOfPage"

[styles.All]
background = "white" # Default value is inherited from terminal color.
```

This replaces the older `viddy.toml` (`[general]`/`[keymap]`/`[color]`) config format. For backward
compatibility, viddy still checks for that file first — at `~/Library/Application Support/viddy.toml`
(macOS) or `$XDG_CONFIG_HOME/viddy.toml` — and only reads the format and location above if it's
absent.

## What is "viddy" ?

"viddy" is Nadsat word meaning to see.
Nadsat is fictional argot of gangs in the violent book and movie "A Clockwork Orange".

## Credits

The gopher's logo of viddy is licensed under the Creative Commons 3.0 Attributions license.

The original Go gopher was designed by [Renee French](https://reneefrench.blogspot.com/).
