fib = function(...) local __args = {...}
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
return (fib((n - 1)) + fib((n - 2)))
end

end
print(fib(12))