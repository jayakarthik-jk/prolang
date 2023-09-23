# Prolang
A Simple human readable functional programming language

### Goals
1. Simple
2. Readable
3. Ease of learning
### Solution
(basic) python clone with brackets

---
### list of contents
1. 
2. 
3. 
---


## variable declaration

```
name = "peter parker"
age = 21
```
---
## Immutability

variables are immutable by default
for mutable variables, use the `let` keyword

```
let is_gwen_alive = true
```
---
## conditional statements

```

a = number(input(""))

if a % 5 is 0 and a % 7 is 0 {
    print("FizzBuzz")
} else if a % 5 is 0 {
    print("Fizz")
} else if a % 7 is 0 {
    print("Buzz")
} else {
    print(a)
}

```
`input(args...)` write the args in the console and read an input from it

`number(arg)` convert the string to number if possible

`note:` `is` and `is not` are identity operators similar to == and != respectively

`note:` curly brackets are optional for a block if it contains only single statement

---
## unconditional loop

```
loop {
    print("got stuck in an infinite loop")
}
```
---
## conditional loop
### while loop
exits when the condition gets false
```
let a = 10

loop while a > 0 {
    print(a)
    a -= 1
}

```
### while loop
exits when the condition gets true
```
let a = 10

loop until a is 0 {
    print(a)
    a -= 1
}
```
`note:` loops are just single statement
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
add = (a, b) => a + b

connectToDB = (url) => {
    if url is "" {
        return false
    }
    # connect to the database
    connection
}

```

`note: ` by default functions return the last expression

## global functions

<!-- list with description -->

- `print` (takes any number of arguments and prints them to stdout)
- `input` (takes optional string as argument and prints it to stdout and returns the input from stdin)
- `number` (converts string to number)
- yet to add more