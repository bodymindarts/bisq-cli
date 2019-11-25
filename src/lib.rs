#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::too_many_arguments)]

pub mod cli;
mod client;
