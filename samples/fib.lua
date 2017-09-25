_if = function(...) local __args = {...}
if 2 == #__args then
local body = __args[2]
if true == __args[1] then
return body()
end
end

end

prompt = function(...) local __args = {...}
if 1 == #__args then
local question = __args[1]
return function(...) local __args = {...}
print(question)
return read()
end

end

end

who = prompt("who are you?")
identity = who()
print(identity)_if((identity == "niels\n"),function(...) local __args = {...}
return print(10)
end
)