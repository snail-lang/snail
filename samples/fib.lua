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

respond = function(...) local __args = {...}
if 1 == #__args then
if "niels\n" == __args[1] then
return "yes ok hello\n"
end
end

if 1 == #__args then
local n = __args[1]
return ("u r not cool, " .. n)
end

end

game = function(...) local __args = {...}
who = prompt("who are you?")

identity = who()

print(respond(identity))
return game()
end

game()