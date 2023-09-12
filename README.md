programming language for problem solving

## variable declaration

inspired by python

```
name = "peter parker"
age = 21
```

## Immutability

variables are immutable by default
for mutable variables, use the `mutable` keyword

```
mutable is_gwen_alive = true
```

## conditional statements

```

a = 35
mutable str = ""

if a % 5 is 0 and a % 7 is 0 {
    str = "FizzBuzz"
} else if a % 5 is 0 {
    str = "Fizz"
} else if a % 7 is 0 {
    str = "Buzz"
} else {
    str = a
}

```

`note:` `is` and `is not` are identity operators similar to == and != respectively

## unconditional loop

```
loop {
    print("got stuck in an infinite loop")
}
```

## conditional loop

```
mutable a = 10

loop while a >= 0 {
    print(a)
    a -= 1
}
```

`note:` loops are just blocks
you can substitute loop block with any other
regular blocks

```
if a is 0 loop {
    print("a is zero, is zero a number?")
}
```

### using loop with conditional statements together

```
if a < 0 loop while a is 0 {
    print($"a is negative and it is {a}")
    a += 1
}
else if a is 0 loop while a < 10 {
    print($"a is single digit and it is {a}")
    a += 1
}
else loop while a < 100 {
    print($"a is double digit and it is {a}")
    a += 1
}
```

`note:` you can use $ to interpolate variables in strings

## function declaration

```
add = (a, b) => {
    a + b
}
```

`note: ` by default functions return the last expression

## global functions

<!-- list with description -->

- `print` (takes any number of arguments and prints them to stdout)
- `input` (takes optional string as argument and prints it to stdout and returns the input from stdin)
- `number` (converts string to number)
