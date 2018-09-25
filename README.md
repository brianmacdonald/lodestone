# <img src="https://github.com/brianmacdonald/lodestone/raw/builtins/docs/logo.png" alt="Lodestone small dark rock logo" width="25px"/> Lodestone

Interpreted language that solves modern problems.

Example:
```
let car := {};
car.wheels := 4;
car.drive := fun(destination) {
    println("driving to " + destination);
};
```

## Build and run interpreter: 
 - `make run`

## Test:
 - `make test`
 - Note: Tests can also be run continuously: `make watch-tests`

