use std::net::IpAddr;

use crate::dns::resolver::{DnsEntry, ResolvedIpAddrs, Resolver, Result, Unresolved};

pub struct NoResolver;

impl NoResolver {
    pub fn new() -> Self {
        NoResolver {}
    }
}

impl Resolver for NoResolver {
    fn lookup(&self, hostname: impl AsRef<str>) -> Result<ResolvedIpAddrs> {
        Ok(ResolvedIpAddrs(vec![hostname
            .as_ref()
            .parse()
            .expect("NoResolver cannot lookup a non-IP hostname!")]))
    }
    #[must_use]
    fn reverse_lookup(&self, addr: impl Into<IpAddr>) -> DnsEntry {
        DnsEntry::NotFound(Unresolved::Normal(addr.into()))
    }
    #[must_use]
    fn reverse_lookup_with_asinfo(&self, addr: impl Into<IpAddr>) -> DnsEntry {
        DnsEntry::NotFound(Unresolved::Normal(addr.into()))
    }
    #[must_use]
    fn lazy_reverse_lookup(&self, addr: impl Into<IpAddr>) -> DnsEntry {
        DnsEntry::NotFound(Unresolved::Normal(addr.into()))
    }
    #[must_use]
    fn lazy_reverse_lookup_with_asinfo(&self, addr: impl Into<IpAddr>) -> DnsEntry {
        DnsEntry::NotFound(Unresolved::Normal(addr.into()))
    }
}
