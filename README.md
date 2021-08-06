# Iroha p2p

**It's a test task**

## Description

It is semi-centralized p2p crate. Base protocol is TCP, and uses its implementation from Standard Linrary of Rust Language. Also it uses extern crate CLAP for CLI options and flags.


## Usage

### Downloading

For using this programme you should download firstly:

```
$ git clone https://github.com/MrZloHex/iroha_p2p.git
```

### Compiling 

```
$ cd iroha_p2p
$ cargo build --release
```

### Running

Executable file is on path `iroha_p2p/target/release/iroha_p2p*`

For dry-run try:
```
$ ./iroha_p2p --port=8080
```

Output should be this:
```
$ ./iroha_p2p --port=8080
My address is '127.0.0.1:8080'
```

If output isn't the same, try add issue on GitHub

### Startup arguments

#### Flags

 - **-h**, **--help**:</br>
	Help inforamation
 - **-V**, **--version**:</br>
	Version of this crate

#### Options

 - **-l**, **--list**:</br>
	Set path and name to file with peers of one group
 - **-p**, **--period**:</br>
	Set frequency of speaking in seconds
 - **-P**, **--port**:</br>
	Set port of Peer


## Help

If there are more questions try run:

```
$ ./iroha_p2p --help
```

