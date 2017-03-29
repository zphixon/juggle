# juggle

*The juggle esoteric programming language*

## explain me

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

