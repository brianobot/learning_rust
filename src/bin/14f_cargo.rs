// this is rust official package management tool
// it can integrate with crate.io (official rust registry for public crates)
// awareness of unittests
// awareness of benchmarks

// when using Cargo, cargo build can be used from any part of the project, including subdirectories

// NOTE: when using cargo to run test, it may run some test concurrently, so ensure the test do not race
// in cases where a simple compilation bu cargo is not enough to build a project,
// the steps needed to build the projec can be specified in a build attribute of the package section
// of the cargo.Toml file, or it can be specified in a file called build.rs.

// the build script is a rust file that would be compiled invoked prior to the compilation of anything else in the package
// the build script can be used to generate code, or to run a command to generate code, or to run a command to generate code.

