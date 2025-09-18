Website: https://github.com/cursorweb/europy
* Add forever while
* Fix the do while loop
* Rewrite the environment
* Remove all types and re-implement
* Remove shadowing (for now, it's just so complicated and weird)
* Re-add for loops
* Redo arguments and functions
* Redo assignment... just do some static analysis *maybe*
    ~~Or find a fix for~~: found fix
    ```eo
    var a = [];
    a[1] = 2;
    ```