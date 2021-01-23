# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- Support for `bpmn:tFormalExpression` type name used by Camunda Modeler

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
[Unreleased]: https://github.com/bpxe/bpxe/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/bpxe/bpxe/compare/v0.1.0...v0.1.1

[i1]: https://github.com/bpxe/bpxe/issues/1
[i2]: https://github.com/bpxe/bpxe/issues/2
[i3]: https://github.com/bpxe/bpxe/issues/3
