# Offline server

## Introduction
This is an offline variant of the MEA server that allows you to use a CSV file as input. This server is in its alpha stage, and is therefore picky about the format of the CSV file. When the CSV file is exhausted, it starts over from the beginning.

It turned out the chosen CSV library is very slow. We did not have the time to test with a different library, so we implemented a two pass approach to using the server. This was needed to make the server run properly on a laptop. First you build a cache from the CSV file, and then you run the server on that cache.

To see all available commands, run `server help`. 

## Usage
**To build the cache from a CSV file:**
1. Make sure that the .csv file does not contain anything other than the data. In other words no headers or column description should be present in the file.
2. Run `server build FILENAME`. This will generate a cache of the CSV file and save it to hidden files in the current directory.

**To run the server on the cache in the current directory:**
1. Run `server cached`.

**To build the cache and run the server immediately after the cache is built:**
1. Run `server run FILENAME`.

## Building and installing the server
1. Download the toolchain by running `curl https://sh.rustup.rs -sSf | sh`.
2. Build and install the server by running `cargo install --git https://github.com/cyborg-client/offline-server.git`. You can also use the `--root` flag to specify where to install to.

## Uninstalling the server
1. Run `cargo uninstall server`.