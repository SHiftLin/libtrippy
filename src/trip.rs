#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(
    clippy::module_name_repetitions,
    clippy::option_if_let_else,
    clippy::missing_const_for_fn,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::redundant_pub_crate,
    clippy::struct_excessive_bools,
    clippy::cognitive_complexity,
    clippy::option_option
)]
#![deny(unsafe_code)]

use anyhow::anyhow;
use clap::Parser;
use trippy::config::{Args, TrippyConfig};
use trippy::platform::Platform;
use trippy::{
    configure_logging, create_geoip_lookup, resolve_targets, run_frontend, start_dns_resolver,
    start_tracers,
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    Platform::acquire_privileges()?;
    let platform = Platform::discover()?;
    let cfg = TrippyConfig::from(args, &platform)?;
    let _guard = configure_logging(&cfg);
    let resolver = start_dns_resolver(&cfg)?;
    let geoip_lookup = create_geoip_lookup(&cfg)?;
    let addrs = resolve_targets(&cfg, &resolver)?;
    if addrs.is_empty() {
        return Err(anyhow!(
            "failed to find any valid IP{} addresses for {}",
            cfg.addr_family,
            cfg.targets.join(", ")
        ));
    }
    let traces = start_tracers(&cfg, &addrs, platform.pid)?;
    Platform::drop_privileges()?;
    run_frontend(&cfg, resolver, geoip_lookup, traces)?;
    Ok(())
}
