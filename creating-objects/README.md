# Creating objects

With the ability to create objects, you would be able to build much more complex data structures with your Rust function. This, in turn, increases the scope of what you can do with Rust in your project.

The process used to create an object goes like this:

First, create an empty object:

```Rust
let obj = cx.empty_object();
```

Second, create values that we want to store in the object:

```Rust
let name = cx.string("Chigozie");
let age = cx.number(19);
```

Lastly, store those values in the empty object:

```Rust
obj.set(&mut cx, "name", name)?;
obj.set(&mut cx, "age", age)?;
```

## How to run this project

To run this project, there are a few things you must do first:

1. `cd` to this folder in your terminal
2. Install the dependencies:

```bash
npm install
```

These steps will help you with installing the Rust and JavaScript dependencies, as well as compiling the Rust code.

When the steps are completed, run the `index.js` file in the terminal:

```bash
node index.js
```
