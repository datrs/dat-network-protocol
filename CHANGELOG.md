## 2020-02-19, Version 0.3.0
### Commits
- [[`868fcc3c7b`](https://github.com/datrs/dat-network-protocol/commit/868fcc3c7b2d87fb8fac24dd6b08b1324912f281)] (cargo-release) version 0.3.0 (Bruno Tavares)
- [[`68addf1b83`](https://github.com/datrs/dat-network-protocol/commit/68addf1b83eb4a378496633a514b725fbd859cd3)] Merge pull request #9 from bltavares/update-schema (Bruno Tavares)
- [[`4186bb6c61`](https://github.com/datrs/dat-network-protocol/commit/4186bb6c61bd95185f6427e498787dd5c7f392e8)] Update schema proto file (Bruno Tavares)
- [[`ed9b1dfdda`](https://github.com/datrs/dat-network-protocol/commit/ed9b1dfddad5e23d82f4b54019b64dba6e2661d7)] Update changelog (Bruno Tavares)

### Stats
```diff
 CHANGELOG.md           |   29 +-
 Cargo.toml             |    3 +-
 README.md              |    2 +-
 build.rs               |   28 +-
 protos/cancel.proto    |    5 +-
 protos/data.proto      |   12 +-
 protos/feed.proto      |    4 +-
 protos/handshake.proto |    8 +-
 protos/have.proto      |    5 +-
 protos/info.proto      |    4 +-
 protos/lib.proto       |   87 +-
 protos/request.proto   |    6 +-
 protos/unhave.proto    |    4 +-
 protos/unwant.proto    |    4 +-
 protos/want.proto      |    4 +-
 src/cancel.rs          |  297 +-----
 src/data.rs            |  641 +----------
 src/feed.rs            |  283 +----
 src/handshake.rs       |  362 +------
 src/have.rs            |  310 +-----
 src/info.rs            |  255 +----
 src/lib.rs             | 3140 ++++++++++++++++++++++++++++++++++++++++++++++++-
 src/request.rs         |  337 +-----
 src/unhave.rs          |  258 +----
 src/unwant.rs          |  258 +----
 src/want.rs            |  257 +----
 26 files changed, 3250 insertions(+), 3353 deletions(-)
```


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


