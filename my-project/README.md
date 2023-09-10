# my-project

This is the default project when you run `npm init neon my-project`.

The only addition is the `index.js` file, which I added, that interacts with the Rust code.

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
