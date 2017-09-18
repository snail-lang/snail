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

output

```
local fib = function(...) local __args = {...}
if 1 == #__args then
if 0 == __args[1] then
return 0
end
end

if 1 == #__args then
if 1 == __args[1] then
return 1
end
end

if 1 == #__args then
local n = __args[1]
return fib(n - 1) + fib(n - 2)
end

end
```
