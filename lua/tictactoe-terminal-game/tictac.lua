-- Input for users to select the matrix
-- Draw the matrix of 3 x 3
-- User O or X turn to play
-- Redraw the matrix based on who are the user
-- check who's the winner on three directions
-- Draw result if no winner

turns = 0
matrix = {}
for y = 1, 3 do
	matrix[y] = {" ", " ", " "}
end


function get_winner()
	local now = whos_turn()

	-- Getting Horizontal strike
    for y = 1, 3 do
		if now == matrix[y][1] and matrix[y][2] and matrix[y][3] then
			return true
		end
	end

	-- getting vertical strike
	for x = 1, 3 do
		if now == matrix [1][x] and matrix[2][x] and matrix[3][x] then
			return true
		end
	end

	-- getting diagonal strike
	if now == matrix[1][1] and matrix[2][2] and matrix[3][3] then
		return true
    elseif now == matrix[1][3] and matrix[2][2] and matrix[3][1] then
		return true
	end

end


function draw_matrix()
	print( "\n\t    1   2   3\n")
	for i = 1, 3 do
		print("\t" .. i .. "   " .. matrix[i][1] .. " | " .. matrix[i][2] .. " | " .. matrix[i][3])
		if i < 3 then
			print("\t   ---+---+---")
		end
	end
end

function whos_turn()
	if turns % 2 == 0 then
		return "O"
	else
		return "X"
	end
end



while true do
	draw_matrix()

	print("\nIts "  .. whos_turn() .. " turn now!")
	print("\nInput two numbers eg: \" 1 3\": ")
	local x, y = io.read("*n", "*n")

	if x and y > 0 and x and y < 4 and matrix[y][x] == " " then
		matrix[y][x] = whos_turn()

		if turns > 3 and get_winner() then
			draw_matrix()
			print("\n" .. whos_turn() .. " is the winner!")
			break
		end

		turns = turns + 1

		if (now == 9) then
			draw_matrix()
			print("\nDraw result! Fight Again will ya?")
			break
		end

	end
end
