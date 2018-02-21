# docker-service

To build and run this:

- Make sure Rust and Docker are installed and working
- Install `dmake`
  - `cargo install --git https://github.com/coder543/dmake`
- Build and run the docker service
  - `dmake -r`

`dmake` defaults to the debug target. To build in release mode, simply specify the target:

    dmake release

the `-r` flag simply indicates that the docker container should be run locally once it is completely built, and this will work on debug, release, or any other build mode.

The `Dmake.ini` file is used by `dmake` to automate much of the workflow.