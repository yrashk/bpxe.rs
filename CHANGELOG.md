# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- Initial support for wasm32 (to enable running BPXE in the browser)

## [0.2.0] - 2021-01-31

### Added

- Inclusive gateway support
- `scriptTask` activity support
- `standardLoopCharacteristics` support for activities
- Limited support for data objects
- Limited support for `multiInstanceLoopCharacteristics`
- Limited support for `dataInputAssociation`, `dataOutputAssociation` and `ioSpecification`

### Fixed

- Rhai-based condition expressions shouldn't be full scripts
- Parsing of `script` element (`bpxe-bpmn-schema` crate)
- Dependency on platform-dependent `linkme` crate (`bpxe-bpmn-schema` crate)
- Improper pluralization for elements like `properties` and `.._refs` (`bxpe-bpmn-schema` crate)
- Parsing of child elements with names different from their type (`bpxe-bpmn-schema` create)

## [0.1.2] - 2021-01-23

### Added

- Support for `bpmn:tFormalExpression` type name used by Camunda Modeler

### Fixed

- Implementation of default path handling in exclusive gateways

## [0.1.1] - 2021-01-22

### Changed

- Single-field BPMN schema structures were changed to the contained types

### Fixed

- Broken XML parsing of expressions fixed (see [#1][i1] and #[#2][i2])

## [0.1.0] - 2021-01-21

### Added
- BPMN data model (`bpmn::schema`)
- XML parser (limited, see [#1][i1] and [#2][i2])
- Serde serialization/deserialization support (limited, see [#3][i3]))
- Process scheduler (`bpmn::process`)
- Parallel Gateway (`bpmn::gateway::parallel`)
- Exclusive Gateway (`bpmn::gateway::exclusive`)
- Event-based Gateway (`bpmn::gateway::event`)
- Start Event (`bpmn::event::start_event`)
- End Event (`bpmn::event::end_event`)
- Intermediate Throw Event (`bpmn::event::throw_event`) (limited)
- Intermediate Catch Event (`bpmn::event::catch_event`) (limited)
- Rudimentary expression language evaluation (`bpmn::language`)

<!-- next-url -->
[Unreleased]: https://github.com/bpxe/bpxe/compare/bpxe-v0.2.0...HEAD
[0.2.0]: https://github.com/bpxe/bpxe/compare/bpxe-v0.1.2...bpxe-v0.2.0
[0.1.2]: https://github.com/bpxe/bpxe/compare/v0.1.1...bpxe-v0.1.2
[0.1.1]: https://github.com/bpxe/bpxe/compare/v0.1.0...v0.1.1

[i1]: https://github.com/bpxe/bpxe/issues/1
[i2]: https://github.com/bpxe/bpxe/issues/2
[i3]: https://github.com/bpxe/bpxe/issues/3
