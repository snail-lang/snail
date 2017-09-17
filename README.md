## snail
a function based object oriented language

#### variables

```
a: num = 10 # explicit
b := 10     # inferred
```

```
fib := {
  |0| 0
  |1| 1
  |n| (fib n - 1) + fib n - 2
}
```
