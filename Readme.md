# Trust Api

Web integration specification and a set of code generators.

## Specification
Trust specification aims to be an improvement of the current integration standards, [OpenAPI](https://github.com/OAI/OpenAPI-Specification) mostly. The main advantages of Trust spec include:
- **Unambiguity of notation** - meaning that there is most likely only one way to model a given API and to only way to interpret the spec
- **It supports generic types** - you may give your own types their own parameters and reuse them in different contexts with different arguments
- **It is minimalistic** - not bloated with redundant useless features, keeps the language as simple as possible
- **It is focused on integration of web systems** - if you need to seamlessly integrate two systems in a typesafe way, you will find Trust spec more useful than OpenAPI
- **It is widely customizable** - by providing a broad set of [handlebars](https://github.com/sunng87/handlebars-rust) helpers you may modify the templates upon which the code is generated to suit your needs
- **It is protocol-agnostic** - although it is designed with HTTP in mind, it can be used to describe any kind of API

Moreover it addresses particular issues existing inherently in OpenAPI with:
- **enclosed Algebraic Data Types** - by design all subtypes of an ADT are kept together, in a single `adt` node

## Overview
(Usage of all below examples and more may be found in [tests](https://github.com/ramencloud/trust/tree/master/src/lib/test))

When it comes to describing API schemas, Trust spec offers the following data types:
* simple types:
  * `type: bool`, equivalent of `type: boolean` in OpenAPI
  * `type: int`, equivalent of `type: integer` with `format: int64` in OpenAPI
  * `type: dec`, equivalent of `type: number` in OpenAPI
  * `type: str`, equivalent of `type: string` in OpenAPI
  * `type: enum`, equivalent of `type: string` with `enum` in OpenAPI
* complex types:
  * `type: obj`, equivalent of `type: object` in OpenAPI
  * `type: seq`, equivalent of `type: array` in OpenAPI
  * `type: map`, equivalent of `type: object` with `additionalProperties` in OpenAPI
* special types:
  * `type: alias`, equivalent of `$ref` in OpenAPI
  * `type: struct`, equivalent of OpenAPI empty schema i.e. `{}`
  * `type: const`, equivalent of OpenAPI `const` feature

### Common use cases

* Algebraic data type aka "Union type"
```yaml
  AdtType:
    type: obj
    vars: 
      discriminatorVar:
        type: str
      someOtherVar:
        type: dec
    adt:
      var: discriminatorVar
      map:
        firstSubtype:
          vars:
            firstSubtypeVar:
              type: int
        secondSubtype:
          vars:
            secondSubtypeVar:
              type: bool
```
is interpreted as schema such that all of the following JSONs are compliant with it:
```json
{
  "discriminatorVar": "firstSubtype",
  "someOtherVar": 1.0,
  "firstSubtypeVar": 1
}
```
```json
{
  "discriminatorVar": "secondSubtype",
  "someOtherVar": 1.0,
  "secondSubtypeVar": true
}
```
while the following are not:
```json
{
  "discriminatorVar": "firstSubtype",
  "someOtherVar": 1.0,
}
```
```json
{
  "discriminatorVar": "nonExistentSubtype",
  "someOtherVar": 1.0,
  "secondSubtypeVar": true
}
```
```json
{
  "discriminatorVar": "secondSubtype",
  "secondSubtypeVar": true
}
```

## Server and Client code generation

### Currently supported generators:
- Python Http Server ([fastapi](https://github.com/tiangolo/fastapi))
- Python Http Client ([httpx](https://github.com/encode/httpx))

### Experimental (not fully implemented, use at your own risk) generators:

- Kotlin Http Server ([spring](https://github.com/spring-projects/spring-framework))
- Scala Http Server ([cask](https://github.com/com-lihaoyi/cask))

### Getting started

#### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) development environment
* [Docker](https://docs.docker.com/engine/install/) engine

### Main use cases

Examples of usage may be found in [tests](https://github.com/ramencloud/trust/tree/master/src/lib/test).

### Usage

* With cargo
```shell
$ cargo run trust
```
* With docker
```shell
$ ./docker/build.sh
$ docker run trust
```