# Trust Spec

Web integration specification and a set of code generators.

## Table of Contents
- [**tldr; I just want to glue my services together**](#i-just-want-to-glue-my-services-together)
- [Specification](#specification)
  - [Overview](#overview)
  - [Common use cases](#common-use-cases)
  - [OpenAPI conversion](#openapi-conversion)
- [Server and Client code generation](#server-and-client-code-generation)
  - [Usage](#usage)
    - [With Docker](#with-docker)
    - [Run from source](#run-from-source)
  - [Currently supported generators](#currently-supported-generators)
    - [Experimental generators](#experimental-generators)

## I just want to glue my services together
Install Trust with
```shell
$ pip install trustspecgen
```
and then
```shell
$ trust

Usage: trust <COMMAND>

Commands:
  from-open-api  
  to-open-api    
  generate       
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

- If you already have an OpenAPI spec:
  1. first convert it to Trust spec with `from-open-api` command
  2. and then generate the glue code with `generate` command. 

  > :exclamation: Note that you may easily automate the whole process combining the two steps above, however it is recommended to make a migration once and benefit from Trust spec expressiveness in the future.

- If you don't have OpenAPI spec yet, you may start with Trust spec directly.

- Alternatively, if you wish to keep generating code from OpenAPI spec, you may use Trust spec as an intermediate format. In this case, the `to-open-api` command will be useful.


## Specification
Trust specification aims to be an improvement of the current integration standards, [OpenAPI](https://github.com/OAI/OpenAPI-Specification) mostly. The main advantages of Trust spec include:
- **Unambiguity of notation** - meaning that there is most likely only one way to model a given API and only way to interpret the spec
- **It supports generic types** - you may give your own types their own parameters and reuse them in different contexts with different arguments
- **It is minimalistic** - not bloated with redundant useless features, keeps the language as simple as possible
- **It is focused on integration of web systems** - if you need to seamlessly integrate two systems in a typesafe way, you will find Trust spec more useful than OpenAPI
- **It is widely customizable** - by providing a broad set of [handlebars](https://github.com/sunng87/handlebars-rust) helpers you may modify the templates upon which the code is generated to suit your needs
- **It is protocol-agnostic** - although it is designed with HTTP in mind, it can be used to describe any kind of API

Moreover it addresses particular issues existing inherently in OpenAPI with:
- **enclosed Algebraic Data Types** - by design all subtypes of an ADT are kept together, in a single `adt` node

### Overview
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

* Including vars from other types:
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
  will produce a schema matching:
    ```json
    {
        "parentVar": 1.0,
        "anotherParentVar": true,
        "ownVar": 1
    }
    ```
* Algebraic data type aka "union type"
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
  is interpreted as schema matching:
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
  but not matching:
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
* Generic types
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
  The above is interpreted as
  ```java
  interface ParameterizedType<ParamAbc, ParamXyz> { 
    ParamAbc varOfParamAbcType; 
    ParamXyz varOfParamXyzType; 
    ParamXyz anoterVarOfParamXyzType; 
  }
  interface SubtypeOfParameterizedType extends ParameterizedType<Boolean, SomeType> { }
  ```
  in Java-like languages.
## Server and Client code generation

### Usage

#### With Docker
Prerequisites:
- [Docker](https://docs.docker.com/engine/install/)
```shell
$ ./docker/build.sh
$ docker run trust
```
#### Run from source
Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)
```shell
$ cargo run trust
```

### Currently supported generators:
- Python Http Server ([fastapi](https://github.com/tiangolo/fastapi))
- Python Http Client ([httpx](https://github.com/encode/httpx))

#### Experimental generators:
> :exclamation:  not fully implemented, use at your own risk

- Kotlin Http Server ([spring](https://github.com/spring-projects/spring-framework))
- Scala Http Server ([cask](https://github.com/com-lihaoyi/cask))