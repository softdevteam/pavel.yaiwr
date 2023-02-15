# pavel.yaiwr

Yet Another Interpreter Witten In Rust

## CI

[Buildbot](https://ci.soft-dev.org/#/builders/1)

## Tests

```shell
# run unit tests
$ cargo test
# run test in a container
$ run_docker_ci_job # optional (--prune)
```

# TODOs

[x] Go through the calc example in the quick start guide

[x] Implement a testing framework

[ ] Add support for custom error handling, i.e InterpError

[ ] Implement print statement based on grmtools

[ ] Use actions to make an AST on the fly (see e.g. https://github.com/ltratt/pizauth/blob/master/src/config.y)

[ ] Split between "compile an AST to Vec<Opcode> and then have an evaluator which takes Vec<Opcode> and executes the program"

[ ] Add variable declaration and out-of-parsing-time computation

[ ] Benchmarking

[ ] Revise Rust in general :)

# Resources

[Which Parsing Approach?](https://tratt.net/laurie/blog/2020/which_parsing_approach.html)

[Yacc](https://web.archive.org/web/20220830093827/dinosaur.compilertools.net/yacc/index.html)

[Quickstart](https://softdevteam.github.io/grmtools/master/book/quickstart.html)
Yet Another Interpreter Written In Rust
