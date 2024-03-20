
#!/usr/local/bin/lua

print('test')

io.write("hi world, form ",_VERSION,"!\n")

-- this is a comment
-- variable definition
local a , b


-- initialization
a = 10
b = 30

print("value of a:", a)
print("value of b:", b)

-- swapping of variables
b , a = a, b


print("value of a:", a)
print("value of b:", b)

f = 70.0/3.0
print("value of f", f)

-- getting type
print(type("what is my type"))

print(type(3.8*a))
print(type(b))
print(type(false))
print(type(print))
print(type(nil))
print(type(type(ABC)))


A = 10

print(A^2)

print(-A)

print(a ~= b)

print (a == A)

print(a and b)
print(a or b)
local c = "hi "
local d = "world"

print(c..d)

print(#c)

-- operators precedence

x = 7 + 3 * 2
print(x)

h = 5 - 5 + 20
print(h)

-- loop
-- while (true)
-- 	do
-- 		print("this loop will run forever")
-- 	end

if (b < 20)
	then
		print( "b is less than 20" );
	end

print("value of b is: ", b);


function max(num1, num2)

	if (num1 > num2) then
		result = num1;
	else
		result = num2;
	end

	return result;
end

print("the max of the two numbers is ",max(10,4))



-- calling function in function


myprint = function(param)
	print("this is my print function - ##",param,"##")
end

function add(num1, num2, functionPrint)
	result = num1 + num2
	functionPrint(result)
end

myprint(10)
add(2,5,myprint)


-- function with variable argument

function average(...)
	result = 0
	local arg = {...}
	for i, v in ipairs(arg) do
		result = result + v
	end
	return result/#arg
end

print("the average is", average(10,5,3,4,5,6))


--strings

string1 = "Lua"
string2 = 'tutorial'
string3 = [["Lua Tutorial"]]

print("\"String 1 is\"", string1)
print("String 2 is", string2)
print("string 3 is", string3)


-- escape sequence

print("----------------------------------")
print("this is a line \athis is line 1")
print("----------------------------------")
print("this is a line \bthis is line 2")
print("----------------------------------")
print("this is a line \fthis is line 3")
print("----------------------------------")
print("this is a line \nthis is line 4")
print("----------------------------------")
print("this is a line \rthis is line 5")
print("----------------------------------")
print("this is a line \tthis is line 6")
print("----------------------------------")
print("this is a line \vthis is line 7")
print("----------------------------------")
print("this is a line \\this is line 8")
print("----------------------------------")
print("this is a line \"this is line 9")
print("----------------------------------")
print("this is a line \'this is line 10")
-- print("this is a line \[ this is line 2")
-- print("this is a line \] this is line 2")


y = "change this to uppercase"
print( string.upper(y))

print( string.gsub(y,'to', 'this') )

print (string.find('thais', 'i'))

print(string.reverse("xyz\n"))

print(string.format("this is a string"))
-- basic string formatting
string1 = "Lua"
string2 = "Tutorial"

--date formatting
date = 2; month = 1; year = 2024;
print(string.format("date formatting %02d %02d %02d", date, month, year))
--decimal formatting
print(string.format("%.4f", 1/3))

print(string.format("Basic formatting %s %s", string1, string2))

print(string.byte("that"))
--third character
print(string.byte("that",3))
--first character from last
print(string.byte("Lua", -1))
--internal Numeric ASCII conversion
print(string.char(97))




print(string.len("world"))

print(string.rep("repeat this\n", 5))

-- array
array = { "lua", "tutorial" }

for i = 0, 2 do
	print(array[i])
end

array2 = {}

for i = -2, 2 do
	array2[i] = i * 2
end

for i = -1, 2 do
	print(array2[i])
end

print("---------------------------------")
--multidimensional array

array = {}

for i = 1, 3 do
	array[i] = {}

	for j = 1, 3 do
		array[i][j] = i * j
	end

end
-- accessing the array

for i= 1, 3 do

	for j = 1, 3 do
		print(array[i][j])
	end

end


print("---------------------------------")


-- initializing the array

array = {}

maxRows = 3
maxColumns = 3

for row = 1, maxRows do

	for col = 1, maxColumns do
		array[row * maxColumns + col] = row * col
	end
end

--accessing the array

for row = 1, maxRows do

	for col = 1, maxColumns do
		print(array[row * maxColumns + col])
	end

end


print("---------------------------------")
-- generic for iterators

array = {"lua", 'tutorial'}

for key, value in ipairs(array)
do
	print(key, value)
end

-- TODO: learn stateless iterators and stateful iterators again
-- TODO: learn tables 
