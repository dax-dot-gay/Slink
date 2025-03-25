use const_format::formatcp;

// Application constants
pub const APP_NAME: &'static str = "slink-server";
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Container constants
pub const JAVA_CONTAINER_BASE: &'static str = "amazoncorretto:{version}";
pub const CONTAINER_WORKING_DIRECTORY: &'static str = "/minecraft";
pub const SERVER_BINARY_NAME: &'static str = "server.jar";

// Security constants
pub const HASHING_MEMORY: u32 = 16;
pub const HASHING_ITERATIONS: u32 = 4;

// Networking constants
pub const USER_AGENT: &'static str = formatcp!("{APP_NAME}/{APP_VERSION}");
