const runtime = process.argv[2];

const number = 46;

function fibonacci_js(n) {
    if (n < 2) {
        return n;
    } else {
        return fibonacci_js(n - 1) + fibonacci_js(n - 2);
    }
}

let result;

switch (runtime) {
    case "node": {
        result = fibonacci_js(number);
        break;
    }
    case "native": {
        const {fibonacci_rs} = require("./index.node");
        result = fibonacci_rs(number);
        break;
    }
    case "wasm": {
        const {fibonacci_wasm} = require("./pkg/rust_node_fibonacci");
        result = fibonacci_wasm(number);
        break;
    }
    default: {
        console.log(`invalid runtime ${runtime}`)
    }
}

console.log(result);