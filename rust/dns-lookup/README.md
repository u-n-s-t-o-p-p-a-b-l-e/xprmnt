For dns_lookup.rs, compile and run with rustc.
```
rustc dns_lookup.rs -o dns_lookup
```

```
./dns_lookup example.com
```

For dns_lookup_extern.rs, add the trust-dns-resolver crate to our project. Since we're not using Cargo, download the crate and its dependencies manually or add them directly in the project.

```
rustc dns_lookup_extern.rs -o dns_lookup_extern -L path_to_trust_dns_resolver_dependencies && ./dns_lookup_extern example.com

```
