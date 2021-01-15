#! /usr/bin/env bash

mkdir -p src/bpmn

saxon-he schemas/BPMN20.xsd codegen-rust.xsl | rustfmt > src/bpmn/schema.rs
