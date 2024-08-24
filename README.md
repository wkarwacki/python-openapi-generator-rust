<p>
  <b>Python Server OpenAPI Generator, Python Client OpenAPI Generator, Kotlin Server OpenAPI Generator</b>
</p>

---

<p align="center">
  <b>Documentation, source code: <a href="https://github.com/wkarwacki/python-openapi-generator-rust" target="_blank">https://github.com/wkarwacki/python-openapi-generator-rust</a></b>
</p>

---
# Trust Spec

Trust Spec is a web integration specification and a set of code generators aiming to be a substitute for OpenAPI. It provides a unified way to describe data transfer interfaces and generates modern, type-safe code.

## Table of Contents
- [**tldr; I just want to glue my services together**](#i-just-want-to-glue-my-services-together)
- [Specification](#specification)
  - [Overview](#overview)
  - [Common Use Cases](#common-use-cases)
  - [OpenAPI Conversion](#openapi-conversion)
- [Code Generation](#code-generation)
  - [Usage](#usage)
    - [With Docker](#with-docker)
    - [From Source](#from-source)
  - [Supported Generators](#supported-generators)
    - [Experimental Generators](#experimental-generators)

## I just want to glue my services together
Install Trust:
```shell
$ pip install trustspeccli
```
Then, run:
```shell
$ trust

Usage: trust <COMMAND>

Commands:
  from-open-api  Convert an OpenAPI specification to a Trust specification. Integrate this into your build process to utilize Trust code generators
  to-open-api    Convert a Trust specification back to an OpenAPI specification, useful when a Trust code generator is not available for your target language
  generate       Generate code based on a Trust specification
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

- If you already have an OpenAPI spec:
  1. first convert it to Trust spec with `from-open-api` command
  2. and then generate the glue code with `generate` command.

  > :exclamation: Note that you may easily automate the whole process combining the two steps above, however it is recommended to make a migration once and leverage Trust spec's expressiveness in long-term.

- **Without an OpenAPI spec:**
  - Start directly with Trust spec.

- **To continue using OpenAPI for code generation:**
  - Use Trust spec as an intermediate format with the `to-open-api` command.

## Specification
Trust specification enhances current integration standards like [OpenAPI](https://github.com/OAI/OpenAPI-Specification). Key benefits include:
- **Clear Notation** - Single way to model and interpret an API.
- **Generic Types** - Customizable types with parameters for different contexts.
- **Minimalistic** - Simple, efficient language without redundant features.
- **Web System Integration Focused** - Ideal for type-safe system integration.
- **Highly Customizable** - Extensive [handlebars](https://github.com/sunng87/handlebars-rust) helpers for template modification.
- **Protocol-Agnostic** - Designed for HTTP but applicable to any API type.

It addresses inherent OpenAPI issues with:
- **Enclosed Algebraic Data Types** - All subtypes of an ADT are grouped in a single `adt` node.

### Overview
Examples and more usage details can be found in [tests](https://github.com/wkarwacki/python-openapi-generator-rust/tree/master/src/lib/test).

**Data Types:**
- **Simple Types:**
  - `type: bool` - equivalent to OpenAPI `type: boolean`
  - `type: int` - equivalent to OpenAPI `type: integer` with `format: int64`
  - `type: dec` - equivalent to OpenAPI `type: number`
  - `type: str` - equivalent to OpenAPI `type: string`
  - `type: enum` - equivalent to OpenAPI `type: string` with `enum`
- **Complex Types:**
  - `type: obj` - equivalent to OpenAPI `type: object`
  - `type: seq` - equivalent to OpenAPI `type: array`
  - `type: map` - equivalent to OpenAPI `type: object` with `additionalProperties`
- **Special Types:**
  - `type: alias` - equivalent to OpenAPI `$ref`
  - `type: struct` - equivalent to OpenAPI empty schema (`{}`)
  - `type: const` - equivalent to OpenAPI `const`

### Common Use Cases

**Including Variables from Other Types:**
  ```yaml
  Parent:
    type: obj
    vars:
      parentVar:
        type: dec
  AnotherParent:
    type: obj
    vars:
      anotherParentVar:
        type: bool
  WithParentsVars:
    type: obj
    mix:
      - path: "defs.Parent"
      - path: "defs.AnotherParent"
    vars:
      ownVar:
        type: int
  ```
Results in:
  ```json
  {
    "parentVar": 1.0,
    "anotherParentVar": true,
    "ownVar": 1
  }
  ```

**Algebraic Data Types (Union Types):**
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
Interpreted as:
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


**Generic Types:**
  ```yaml
  defs:
    SomeNamedString:
      type: str
    ParameterizedType:
      type: obj
      vars:
        varOfParamAbcType:
          param: ParamAbc # to declare a generic type, you need to simply use a 'param' keyword
        varOfParamXyzType:
          param: ParamXyz # similarly to the above, this time with a different name
        anoterVarOfParamXyzType:
          param: ParamXyz # similarly to the above, this time with a different name
    SubtypeOfParameterizedType:
      type: obj
      ext: # extending a generic type
        path: 'defs.ParameterizedType'
        args: # with below type-arguments
          ParamAbc:
            type: bool
          ParamXyz:
            path: 'defs.SomeType'
  ```
Equivalent to:
  ```java
  interface ParameterizedType<ParamAbc, ParamXyz> { 
    ParamAbc varOfParamAbcType; 
    ParamXyz varOfParamXyzType; 
    ParamXyz anoterVarOfParamXyzType; 
  }
  interface SubtypeOfParameterizedType extends ParameterizedType<Boolean, SomeType> { }
  ```

### OpenAPI Conversion

* Control the layout of the generated Trust spec with the `-l` option. For instance, setting it to `tag` organizes the Trust spec by OpenAPI tags, as shown in this [example](https://github.com/wkarwacki/python-openapi-generator-rust/blob/master/test/integration/specs/openapi_fastapi/api.yml#L9).

## Code Generation

### Usage

#### With Docker
Prerequisites:
- [Docker](https://docs.docker.com/engine/install/)
  ```shell
  $ ./docker/build.sh
  $ docker run trust
  ```

#### From Source
Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)
  ```shell
  $ cargo run trust
  ```

### Supported Generators:
- Python HTTP Server ([fastapi](https://github.com/tiangolo/fastapi))
- Python HTTP Client ([httpx](https://github.com/encode/httpx))

#### Experimental Generators:
> :exclamation: Not fully implemented. Use at your own risk.

- Kotlin HTTP Server ([spring](https://github.com/spring-projects/spring-framework))
- Scala HTTP Server ([cask](https://github.com/com-lihaoyi/cask))