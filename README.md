# Europa Lang
This language aims to be simple, minimal, and compact. There will not be any classes whatsoever, and importing other files should be painless.

## Example
```
use Math;
use Io;

Io.println("Hello, World!");
var input = Io.stdin.readline();
Io.println("You said: " + input);

Io.println("Random Number 0..1: " + Math.random());

var struct = {{
    a_sruct = 'strings can look like this too'
}};

fn add_two(a, b) {
    return a + b;
}
```

## Credits
Thanks @justamirror and Dart for design suggestions and name suggestions.