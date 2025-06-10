-- tic tac toe
-- badkarma976

function _init()
	sleep=0
	-- enable mouse
	poke(0x5f2d, 1)
	p = {"", "", "", "", "", "", "", "", ""}
	current_player = "x"
	game_over = false
	game_pause = 0
end

function _update()
	if sleep>0 then
		sleep = sleep - (1)
		return
	end
	mousex = stat(32)
	mousey = stat(33)

	if btnp(�) then _init() end -- reset game on button press (x)

	if not game_over and stat(34) == 1 then -- check if mouse is clicked
		local index = flr(mousey / 43) * 3 + flr(mousex / 43)
		if p[index + 1] == "" then
			p[index + 1] = current_player
			check_winner()
			current_player = (current_player == "x") and "o" or "x" -- switch player
		end
	end
end

function draw_grid()
	-- draw vertical lines
	line(43, 0, 43, 127)  -- first vertical line
	line(86, 0, 86, 127)  -- second vertical line
	-- draw horizontal lines
	line(0, 43, 127, 43)  -- first horizontal line
	line(0, 86, 127, 86)  -- second horizontal line
end

function draw_mark(x, y, mark)
	if mark == "x" then
		line(x - 10, y - 10, x + 10, y + 10)
		line(x + 10, y - 10, x - 10, y + 10)
	elseif mark == "o" then
		circ(x, y, 10)
	end
end

function draw_board()
	for i = 0, 8 do
		local x = (i % 3) * 43 + 21
		local y = flr(i / 3) * 43 + 21
		draw_mark(x, y, p[i + 1])
	end
end

function check_winner()
	local win_conditions = {
		{0, 1, 2}, {3, 4, 5}, {6, 7, 8}, -- rows
		{0, 3, 6}, {1, 4, 7}, {2, 5, 8}, -- columns
		{0, 4, 8}, {2, 4, 6} -- diagonals
	}

	for _, condition in pairs(win_conditions) do
		if p[condition[1] + 1] ~= "" and 
		p[condition[1] + 1] == p[condition[2] + 1] and 
		p[condition[1] + 1] == p[condition[3] + 1] then
			game_over = true
			current_winner = current_player
			return
		end
	end

	-- check for draw
	if not game_over and not contains_empty() then
		current_winner = "draw"
		game_over = true
	end
end

function contains_empty()
	for i = 1, #p do
		if p[i] == "" then return true end
	end
	return false
end

function _draw()
	cls()
	draw_grid()
	draw_board()
	
	if game_over and game_pause == 0 then
		sleep = 25
		game_pause = 1
	end
	
	function center_message(msg)
		return 64-#msg*2
	end
	
	if game_over and sleep == 0 then
		cls()
		if current_winner == "draw" then
			textmsg = "tie!"
		else
			textmsg = current_winner .. " won the game!"
		end
		print(textmsg, center_message(textmsg), 63, 7)
		textmsg2 = "press � to reset. "
		print(textmsg2, center_message(textmsg2), 53, 7)
	end

	-- draw crosshair
	line(mousex, mousey - 4, mousex, mousey + 4)
	line(mousex - 4, mousey, mousex + 4, mousey)
end
