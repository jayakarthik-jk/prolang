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

if a % 5 is 0 and a % 7 is 0 {
    print("FizzBuzz")
}
else if a % 5 is 0 {
    print("Fizz")
}
else {
    print("Buzz")
}
```

`note:` `is` and `is not` are identity operators similar to == and != respectively

## unconditional loop

```
loop {
    print("got stuck in an infinite loop")
}
```

# conditional loop

```
mutable a = 10

loop until a >= 0 {
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
if a < 0 loop until a is 0 {
    print($"a is negative and it is {a}")
    a += 1
}
else if a is 0 loop until a < 10 {
    print($"a is single digit and it is {a}")
    a += 1
}
else loop until a < 100 {
    print($"a is double digit and it is {a}")
    a += 1
}
```

`note:` you can use $ to interpolate variables in strings

## function declaration

`note: `

### functions are in beta. the api may change in future. ofcourse, the entire language is in beta.

```
function add(a, b) {
    return a + b
}
```

### If the last statement of a function is an expression, it is implicitly returned

inspired by rust

```
function add(a, b) {
    a + b
}
```

## functions are first class citizens

inspired by javascript

```
add = function(a, b) {
    a + b
}

sum = add(1, 2)
```
