TODO
====

  0. Finish finalising configuration structure
  1. Call VM (which in turn will configuration & instantiate: transport, VM, consensus, &etc.) in accordance with Layers interoperability scheme: https://github.com/Fantom-foundation/consensus-rough-notes/blob/master/Layers-interoperation-schema.md
  2. Produce two binaries, with different defaults (one for running a fully participating node [full-cli] the other for [light-cli])

Purpose TODO in detail
====
0. Generate keys. There should be a cmd line switch (and config file parameter) specifying the type of keys generated.
NB: currently only one implementation: `libsignature-ed25119-dalek`
Available types of keys are listed in libsignature::SignatureType
Keys are generated using methods of `libsignature` trait.
Generated key should be written into files specified by cmd line switches (or by default names specified in configuration file)

1. Setup network. There should be cmd line switches (and config file parameter) specifying the number of peers in the network,
the list of network addresses for these peers (or filename from where to read this list),
then we need to generate private/public key for each peer,
then we need to generate `peers.json` file according to `libconsensus::ConsensusType` for peer list of corresponding type,
basically we just need to initialise this data structure in memore and the serialize in into file to be read on `full-cli-rs`
invocations with commands to run or benhmark the network.
