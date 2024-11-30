# Spec coverage project for `asciidoc-parser`  project

[![CI](https://github.com/scouten/asciidoc-parser-coverage/actions/workflows/ci.yml/badge.svg)](https://github.com/scouten/asciidoc-parser-coverage/actions/workflows/ci.yml)

Spec coverage: [![Codecov report](https://codecov.io/gh/scouten/asciidoc-parser-coverage/graph/badge.svg?token=zzzzz)](https://codecov.io/gh/scouten/asciidoc-parser-coverage)

This repo supports the development of the [`asciidoc-parser`]
(Rust parser for AsciiDoc language) project.

It contains a relatively-current copy of the [AsciiDoc language description]
(whose authors are careful to describe it as not yet a formal specification)
and adapts the [Codecov] tool with some custom instrumentation to measure
`asciidoc-parser`'s compliance with the AsciiDoc language description.

I call this "spec-driven development," in that it aims (eventually) to achieve
100% coverage of the spec (or in this case, description).

[`asciidoc-parser`]: https://github.com/scouten/asciidoc-parser
[Codecov]: https://about.codecov.io
