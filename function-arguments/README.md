# Function arguments

Sometimes you may need to pass arguments to your Rust function. This example shows how a Rust function can handle arguments from JavaScript.

If you look into the `src/lib.rs` file, you'll see these lines in the `add` function:

```Rust
    let num1 = cx
        .argument::<JsNumber>(0)? // Access the first argument
        .value(&mut cx);
    let num2 = cx
        .argument::<JsNumber>(1)? // Access the second argument
        .value(&mut cx);
```

> Lines 4 - 9

`cx.argument()` allows you to get the arguments that you pass from JavaScript. The function uses `0` as it's first index, so you need to access the first argument with `cx.argument(0)`.

You also need to specify the type of argument you're expecting from the argument or else you'll run into an error. In this example we specify the type with `cx.argument::<JsNumber>(0)`.

## How to run this project

To run this project, there are a few things you must do first:

1. `cd` to this folder in your terminal
2. Install the dependencies:

```bash
npm install
```

These steps will help you with installing the Rust and JavaScript dependencies, as well as compiling the Rust code. If you just want to compile the Rust backend, run this command:

```bash
npm run build
```

When the steps are completed, run the `index.js` file in the terminal:

```bash
node index.js
```
