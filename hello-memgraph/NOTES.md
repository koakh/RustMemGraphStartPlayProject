# NOTES

- [Rust client guide](https://memgraph.com/docs/client-libraries/rust)

## Run Memgraph

```shell
# locally
$ docker run -it -p 7687:7687 -p 7444:7444 -p 3000:3000 --name memgraph memgraph/memgraph-platform
```

> or connect to running container on koakhserver at <http://192.168.1.1:3000>

test query `MATCH (n)-[r]->(m) RETURN n,r,m`

## First Run

```shell
$ cargo build
   Compiling rsmgclient v2.0.2
error: could not find native static library `mgclient`, perhaps an -L flag is missing?

error: could not compile `rsmgclient` (lib) due to previous error
```

## Build libmgclient.so in Kuartzo Kubuntu 23.10 VM

error: could not compile `rsmgclient` (lib) due to previous error

- [mgclient is a C library interface for Memgraph database](https://github.com/memgraph/mgclient.git)

```shell
# sudo apt-get install -y git cmake make gcc g++ libssl-dev

$ git clone https://github.com/memgraph/mgclient.git
$ cd mgclient
$ mkdir build && cd build && cmake ..
$ make
[  5%] Building C object src/CMakeFiles/mgclient-static.dir/mgallocator.c.o
[ 10%] Building C object src/CMakeFiles/mgclient-static.dir/mgclient.c.o
[ 15%] Building C object src/CMakeFiles/mgclient-static.dir/mgmessage.c.o
[ 20%] Building C object src/CMakeFiles/mgclient-static.dir/mgsession.c.o
[ 25%] Building C object src/CMakeFiles/mgclient-static.dir/mgsession-decoder.c.o
[ 30%] Building C object src/CMakeFiles/mgclient-static.dir/mgsession-encoder.c.o
[ 35%] Building C object src/CMakeFiles/mgclient-static.dir/mgtransport.c.o
[ 40%] Building C object src/CMakeFiles/mgclient-static.dir/mgvalue.c.o
[ 45%] Building C object src/CMakeFiles/mgclient-static.dir/linux/mgsocket.c.o
[ 50%] Linking C static library libmgclient.a
[ 50%] Built target mgclient-static
[ 55%] Building C object src/CMakeFiles/mgclient-shared.dir/mgallocator.c.o
[ 60%] Building C object src/CMakeFiles/mgclient-shared.dir/mgclient.c.o
[ 65%] Building C object src/CMakeFiles/mgclient-shared.dir/mgmessage.c.o
[ 70%] Building C object src/CMakeFiles/mgclient-shared.dir/mgsession.c.o
[ 75%] Building C object src/CMakeFiles/mgclient-shared.dir/mgsession-decoder.c.o
[ 80%] Building C object src/CMakeFiles/mgclient-shared.dir/mgsession-encoder.c.o
[ 85%] Building C object src/CMakeFiles/mgclient-shared.dir/mgtransport.c.o
[ 90%] Building C object src/CMakeFiles/mgclient-shared.dir/mgvalue.c.o
[ 95%] Building C object src/CMakeFiles/mgclient-shared.dir/linux/mgsocket.c.o
[100%] Linking C shared library libmgclient.so
[100%] Built target mgclient-shared

$ find . -name libmgclient.so
./src/libmgclient.so
$ ls -la ./src/libmgclient.so
lrwxrwxrwx 1 mario users 16 dez  7 22:35 ./src/libmgclient.so -> libmgclient.so.2*

# This will build two mgclient library flavours: a static library (named libmgclient.a) and a shared library (named libmgclient.dylib).
# To install the libraries and corresponding header files run:

$ sudo make install
[ 50%] Built target mgclient-static
[100%] Built target mgclient-shared
Install the project...
-- Install configuration: "Release"
-- Installing: /usr/lib64/libmgclient.a
-- Installing: /usr/lib64/libmgclient.so.2
-- Installing: /usr/lib64/libmgclient.so
-- Up-to-date: /usr/include
-- Installing: /usr/include/mgclient.h
-- Installing: /usr/include/mgclient-export.h
```

## Now we can install rsmgclient

- [GitHub - memgraph/rsmgclient: Memgraph database adapter for Rust programming language.](https://github.com/memgraph/rsmgclient)

```shell
# now build it
$ cargo install rsmgclient
...
Unable to find libclang: "couldn't find any valid shared libraries matching: ['libclang.so', 'libclang-*.so', 'libclang.so.*', 'libclang-*.so.*'], set the `LIBCLANG_PATH` environment variable to a path where one of these files can be found (invalid: [])"

$ sudo apt search libclang
...
librust-clang-sys-dev/mantic,now 1.3.0-1 amd64 [installed]
  Rust bindings for libclang - Rust source code

$ sudo apt install librust-clang-sys-dev

# now build it
$ cargo install rsmgclient
# now it works
    Finished release [optimized] target(s) in 7.71s
  Installing /home/mario/.cargo/bin/rsmgclient
   Installed package `rsmgclient v2.0.2` (executable `rsmgclient`)
```

> This will install to system default installation directory. If you want to change this location, use -DCMAKE_INSTALL_PREFIX option when running CMake.

## Install rsmgclient and build project

```shell
# seems that this don't work with sym link
#  cd libs && ln -s ../../mgclient/build/src/libmgclient.so.2 libmgclient.so && ls -la && cd ..
# RUSTFLAGS="-L$(pwd)/libs" cargo build
```

UPDATE: after install `cargo install rsmgclient`, the project build flawless without `RUSTFLAGS`

## Running the Project

```shell
$ time cargo run
________________________________________________________
Executed in   47.55 millis    fish           external
   usr time   35.66 millis  323.00 micros   35.34 millis
   sys time    7.85 millis    0.00 micros    7.85 millis
```
