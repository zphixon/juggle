# juggle

*The juggle esoteric programming language*

## explain me

### table of functions

|  function  |                purpose                |      arguments     |        from       |      to      |
|:----------:|:-------------------------------------:|:------------------:|:-----------------:|:------------:|
|:-----------|:--------------------------------------|:-------------------|:------------------|:-------------|
| `toss`     | push a value into the air             | value/syntax       | hands if no value | air          |
| `catch`    | pop a value from the air to the hands | value              | air               | hands        |
| `curse`    | print value                           | value              | hands             |              |
| `joke`     | print value as char                   | value              | hands             |              |
| `plus`     | add two numbers                       | number, number     | hands, hands      | hands        |
| `minus`    | subtract two numbers                  | number, number     | hands, hands      | hands        |
| `times`    | multiply two numbers                  | number, number     | hands, hansd      | hands        |
| `divided`  | divide a number                       | number, number     | hands, hands      | hands        |
| `modulo`   | take remainder of number              | number, number     | hands, hands      | hands        |
| `equal`    | compare two values                    | value, value       | air, air          | hands        |
| `greater`  | compare two numbers                   | number, number     | air, air          | hands        |
| `lesser`   | compare two numbers                   | number, number     | air, air          | hands        |
| `and`      | logical and two bools                 | bool, bool         | air, air          | hands        |
| `or`       | logical or two bools                  | bool, bool         | air, air          | hands        |
| `not`      | logical not a bool                    | bool               | hands             | hands        |
| `if`       | execute body if true                  | bool               | air               |              |
| `while`    | execute body if true multiple times   | bool               | air               |              |
| `else`     | swap execution status                 |                    |                   |              |
| `end`      | finish if/while                       |                    |                   |              |
| `append`   | append value to array or two values   | array/value, value | hands, hands      | hands        |
| `nth`      | get nth value of array                | array, number      | air, hands        | air          |
| `feedback` | get input                             |                    |                   | air          |
| `rethrow`  | throw value again                     | value              | air               | air, air     |
| `recatch`  | catch value again                     | value              | hands             | hands, hands |
| `drop`     | drop value                            | value              | hands             |              |
| `turn`     | switch direction of air               |                    |                   |              |

Mathematical and boolean operators are converted to infix, i.e. the left
operand is popped first, then the right operand.

### values

A value can have several different types: boolean, number, and array. Strings
are simply arrays of numbers.

### the air and your hands

The air and your hands are two separate stacks. The air is the only place where
values can be compared to one another, and your hands are the only place where
values can be mutated.

### control flow

`if` and `while` are the only options here. They simply execute their bodies if
the value on top of your hands is true, `if` doing so once, `while` doing so
multiple times.

### routines

A juggler can create routines which perform common operations, such as handling
stack juggling or performing computations.

## show me

### hello world

```
toss "Hello, world!" catch
joke
```

### comparison

```
toss 1
toss 2
equal

catch drop
catch drop
catch

if
    toss "yes"
    catch joke
else
    toss "no"
    catch joke
end

drop drop
```

