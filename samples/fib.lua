prompt = function(...) local __args = {...}
if 1 == #__args then
local question = __args[1]
return function(...) local __args = {...}
print(question)
return read()
end

end

end

identity = prompt("who are you?")
print(identity())