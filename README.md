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
- [I just want to glue my services together](#i-just-want-to-glue-my-services-together)
- [Requirements](#requirements)
- [Specification](#specification)
  - [Overview](#overview)
  - [Common Use Cases](#common-use-cases)
- [Code Generation](#code-generation)
  - [Generator Options](#generator-options)
  - [Supported Generators](#supported-generators)
    - [Experimental Generators](#experimental-generators)
  - [Generators Documentation](#generators-documentation)
- [Conversion from OpenAPI](#conversion-from-openapi)
- [Conversion to OpenAPI](#conversion-to-openapi)
- [Building Trust CLI](#building-trust-cli)
  - [With Docker](#with-docker)
  - [From Source](#from-source)

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
  from-open-api  Convert an OpenAPI specification to a Trust specification. Integrate this into your build process to utilize Trust's code generators
  to-open-api    Convert a Trust specification back to an OpenAPI specification, useful when a Trust code generator is not available for your target language
  generate       Generate code based on a Trust specification
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

- If you already have an OpenAPI spec:
  1. first convert it to Trust spec with `from-open-api` command
  2. and then generate the glue code with `generate` command.

  > :exclamation: Note that you may easily automate the whole process combining the two steps above, however it is recommended to make a migration once and leverage Trust spec's expressiveness in long-term.

- **Without an OpenAPI spec:**
  - Start directly with Trust spec.

- **To continue using OpenAPI for code generation:**
  - Use Trust spec as an intermediate format with the `to-open-api` command.

## Requirements
- Python 3.10 - currently the only supported version

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

## Code Generation

```shell
$ trust generate -h
Generate code based on a Trust specification

Usage: trust generate [OPTIONS] <LANG> <ROLE> <INPUT> <OUTPUT>

Arguments:
  <LANG>    Select the target programming language for the generated code [possible values: kotlin, python, scala, type-script]
  <ROLE>    Specify whether to generate server or client code [possible values: client, server]
  <INPUT>   Path to the Trust specification file
  <OUTPUT>  Directory where the generated code will be saved

Options:
  -c <CONFIG>              Optional path to a generator configuration file. Refer to the Trust documentation for details
  -t <TEMPLATES_PATH>      Optional path to a custom templates directory. For instance, you can override any template found at https://github.com/wkarwacki/python-openapi-generator-rust/tree/master/src/lib/gen/python/server/templates, however this can be configured for all languages and roles
  -h, --help               Print help
```

### Generator Options
You can customize the generator behavior by passing a relevant `yml` configuration file. The following options are available:
* `typeMapping: dict[str, str]` - Map Trust Spec type to any provided type in a type-safe way. Generated code for both server and client is supposed to pick up on mapping provided by the user so that any errors in such will be caught at compile time.
* `module: str` - Specify the module name for the generated code
* `dtoName: str` - Provide the custom Handlebars template for naming DTO classes, by default it is `{{val}}Dto`
* `autoImplement: bool` This option is a fundamental part of the [Trust Spec integration tests suite](https://github.com/wkarwacki/python-openapi-generator-rust/blob/master/do_test.sh#L13).
  * For server generators - Provides default implementation for all the operations
  * For client generators - Generates tests with all required params that verify server's correctness 

### Supported Generators:

- <b>Python HTTP Server ([fastapi](https://github.com/tiangolo/fastapi))</b>
- <b>Python HTTP Client ([httpx](https://github.com/encode/httpx))</b>

#### Experimental Generators:
> :exclamation: Not fully implemented. Use at your own risk.

- Kotlin HTTP Server ([spring](https://github.com/spring-projects/spring-framework))
- Scala HTTP Server ([cask](https://github.com/com-lihaoyi/cask))

### Generators Documentation
For detailed documentation, features and limitations on the supported code generators, refer to:
- [Python Http Client Generator Documentation](https://github.com/wkarwacki/python-openapi-generator-rust/blob/master/src/lib/gen/python/client/Readme.md)
- [Python Http Server Generator Documentation](https://github.com/wkarwacki/python-openapi-generator-rust/blob/master/src/lib/gen/python/server/Readme.md)


## Conversion from OpenAPI
```shell
$ trust from-open-api -h
Convert an OpenAPI specification to a Trust specification. Integrate this into your build process to utilize Trust's code generators

Usage: trust from-open-api [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Path to the OpenAPI specification file
  <OUTPUT>  Directory where the output Trust specification will be saved

Options:
  -l <LAYOUT>      Specify the structure of the converted Trust specification [default: default] [possible values: default, tag]
  -h, --help       Print help (see more with '--help')

```
* Control the layout of the generated Trust spec with the `-l` option. For instance, setting it to `tag` organizes the Trust spec by OpenAPI tags, as shown in this [example](https://github.com/wkarwacki/python-openapi-generator-rust/blob/master/test/integration/specs/openapi_fastapi/api.yml#L9).

## Conversion to OpenAPI

```shell
$ trust to-open-api -h
Convert a Trust specification back to an OpenAPI specification, useful when a Trust code generator is not available for your target language

Usage: trust to-open-api <INPUT>

Arguments:
  <INPUT>  Path to the Trust specification file

Options:
  -h, --help  Print help
```

## Building Trust Cli

### With Docker
Prerequisites:
- [Docker](https://docs.docker.com/engine/install/)
  ```shell
  $ ./docker/build.sh
  $ docker run trust
  ```

### From Source
Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)
  ```shell
  $ cargo run trust
  ```
