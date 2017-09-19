## snail
a function based object oriented language

#### variables

```
a: num = 10 # explicit
b := 10     # inferred
```

### example

```
fib := {
  |0| 0
  |1| 1
  |n| (fib n - 1) + fib n - 2
}
```

### flow-control

```
when := {
  |true body| body!
  |false _|
}
```

```
while := {
  |condition body| when condition! {
    body!
    while condition, body
  }
}
```
