
> **Note**
> This project is in it's early stages.

## Build Native (tested on Windows)
You need *cargo* installed. Run ```cargo run```.

## Build Web
You need *wasm-pack* and *npm* installed. Run:
* ```wasm-pack build```
* ```npm install```
* ```npm run start```
Open the local ip address from NPM in a browser.

## potential problems
* thiserror crate has an unstable feature:
  * run ```cargo clean``` and it magically works
