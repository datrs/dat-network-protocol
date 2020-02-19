## 2020-02-19, Version 0.2.0
### Commits
- [[`965338ae75`](https://github.com/datrs/dat-network-protocol/commit/965338ae75b36274c6c888ea69565a172cee8e1b)] Merge pull request #8 from bltavares/bump (Bruno Tavares)
- [[`ed44347713`](https://github.com/datrs/dat-network-protocol/commit/ed4434771306b515f9010dc1ed79e9b153e887ac)] Bump to 2018 edition (Bruno Tavares)
- [[`2de6266452`](https://github.com/datrs/dat-network-protocol/commit/2de626645211ac9db23666388df1a07d95f60ec1)] Fix travis build (Bruno Tavares)
- [[`50d68e3f30`](https://github.com/datrs/dat-network-protocol/commit/50d68e3f30422740168bf5c5b2e84b6de952b6bb)] Migrate codegen to pure-rust generator (Bruno Tavares)
- [[`808778d352`](https://github.com/datrs/dat-network-protocol/commit/808778d3525c70ba3798c63f9059eb02a7533b72)] Update protobuf requirement from ~1.5 to ~2.0 (#5) (dependabot[bot])

### Stats
```diff
 .travis.yml       |   6 +-
 Cargo.toml        |   7 +-
 build.rs          |  12 +-
 protos/feed.proto |   2 +-
 src/cancel.rs     | 134 ++++++++++---------------
 src/data.rs       | 295 ++++++++++++++++++++++---------------------------------
 src/feed.rs       | 122 ++++++++++-------------
 src/handshake.rs  | 168 +++++++++++++------------------
 src/have.rs       | 140 +++++++++++---------------
 src/info.rs       | 113 +++++++++------------
 src/lib.rs        |   2 +-
 src/request.rs    | 156 ++++++++++++-----------------
 src/unhave.rs     | 113 +++++++++------------
 src/unwant.rs     | 113 +++++++++------------
 src/want.rs       | 112 +++++++++------------
 15 files changed, 657 insertions(+), 838 deletions(-)
```


