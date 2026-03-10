# The CLI
`Mond` is both the language and the CLI. It seeks to behave just like `Cargo` does for `Rust`. To get started, simply run:

```shell
mond new hello_world
```

This will create a new directory `hello_world`. You can then run:

```shell
cd hello_world
mond run
```

And you should see "Hello World" printed to stdout.

If you look in `src/main.mond`, you'll see this:

```
(use std)

(let main {}
  (io/println "Hello, world!"))
```

In the next section, we'll go through the language and see what all of this means.
