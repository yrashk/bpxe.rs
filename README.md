# BPXE (Business Process eXecution Engine)

[![](http://meritbadge.herokuapp.com/bpxe)](https://crates.io/crates/bpxe)

BPMN 2.0 based business process execution engine implemented in
[Rust](https://rust-lang.org). BPMN stands for **Business Process Model and
Notation**. BPMN's goal is to help stakeholders to have a shared understanding
of processes.

BPXE focuses on the execution aspect of such notation, effectively allowing the
processes described in BPMN to function as if they were programs. BPXE is not
the only such engine, as there are many commercially or community supported
ones. The motivation behind the creation of BPXE was to create an engine with
a particular focus on type and memory safety, performance and multi-tenancy capabilities
(ensuring that a great deal of processes should be able to operate even on
a single server concurrently) and resistant to failures so that workflows can
be resumed with little to no consideration when a failure happen.

## Usage

Since BPXE is not a server in its own right, but rather a library, a good
starting point would be to add this to your Cargo.toml:

```toml
[dependencies]
bpxe = "0.1.1"
```

You can also check out [latest release API documentation](https://docs.rs/bpxe), as well as
bleeding edge [master documentation](https://docs.bpxe.rs/master/bpxe).
