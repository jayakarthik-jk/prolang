programming language for problem solving

## variable declaration
inspired by python
name = "peter parker"
age = 21


## Immutability
variables are immutable by default
for mutable variables, use the `mutable` keyword
mutable is_gwen_alive = true


```
statements

if condition {

}

if condition {

} else {

}

if condition {

} else if condition {

} else  {

}

```
# unconditional loop

loop {
    #statements
}


# conditional loop

loop until a == 0 {

}

you can also use else to this like below

loop until a == 0 {

}
else {
    # this block is executed only if
    # the above loop is terminated with a
    #break statement
}

you can also use else to this like below

if a == 0 loop {

}

else loop {
    # this block is executed only if
    # the above loop is terminated with a
    # break statement
    # and loops until it breaks
}

loops are just blocks
you can substitute loop block with any other
regular blocks

if a == 0 loop {
    // if the condition is true
    // control will enter into this block
    // and loop indefinitely
}

#you can also use combinations like the example bellow

```
if a < 0 loop until a < 10 {
    print(a)
} else if a = 0 loop until a < 15 {
    print(a)
} else loop until a < 5 {
    print(a)
}
```


## function declaration
inspired by javascript
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
```
add = function(a, b) {
    a + b
}

add(1, 2)
```

