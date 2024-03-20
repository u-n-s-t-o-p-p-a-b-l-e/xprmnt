local function Pet(name)
	name = name or "Luis"
	return {
		name = name,
		status = "Hungry",

		feed = function(self)
			print(name .. " is fed")
			self.status = "Full"
		end
	}
end

local cat = Pet("Kitty")
local dog = Pet()

print(cat.status)
cat:feed()
print(cat.status)
print(dog.status)
