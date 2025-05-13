local parser = require("../lua/file_location_parser")

local tests = {
	{ input = [[foo.lua:11]], path = "foo.lua", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [[foo.lua:11:111]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua:11:111-222]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = 222 },
	{ input = [[foo.lua:11:111-22.222]], path = "foo.lua", line = 11, end_line = 22, col = 111, end_col = 222 },
	{ input = [[foo.lua:11.111]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua 11]], path = "foo.lua", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [[foo.lua 11:111]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua 11.111]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua#11]], path = "foo.lua", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [[foo.lua#11:111]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua#11.111]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua, 11]], path = "foo.lua", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [["foo.lua",11]], path = "foo.lua", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [["foo.lua",11:111]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo.lua",11.111]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo.lua",11.111-222]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = 222 },
	{ input = [["foo.lua",11.111-22.222]], path = "foo.lua", line = 11, end_line = 22, col = 111, end_col = 222 },
	{ input = [["foo.lua", line 11]], path = "foo.lua", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [["foo.lua", line 11, col 111]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{
		input = [["foo.lua", line 11, column 111]],
		path = "foo.lua",
		line = 11,
		end_line = nil,
		col = 111,
		end_col = nil,
	},
	{ input = [["foo.lua":line 11]], path = "foo.lua", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [["foo.lua":line 11, col 111]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{
		input = [["foo.lua":line 11, column 111]],
		path = "foo.lua",
		line = 11,
		end_line = nil,
		col = 111,
		end_col = nil,
	},
	{ input = [["foo.lua": line 11]], path = "foo.lua", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [["foo.lua": line 11, col 111]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{
		input = [["foo.lua": line 11, column 111]],
		path = "foo.lua",
		line = 11,
		end_line = nil,
		col = 111,
		end_col = nil,
	},
	{ input = [["foo.lua" on line 11]], path = "foo.lua", line = 11, end_line = nil, col = nil, end_col = nil },
	{
		input = [["foo.lua" on line 11, col 111]],
		path = "foo.lua",
		line = 11,
		end_line = nil,
		col = 111,
		end_col = nil,
	},
	{
		input = [["foo.lua" on line 11, column 111]],
		path = "foo.lua",
		line = 11,
		end_line = nil,
		col = 111,
		end_col = nil,
	},
	{ input = [["foo.lua" line 11 column 111]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{
		input = [["foo.lua", line 11, character 111]],
		path = "foo.lua",
		line = 11,
		end_line = nil,
		col = 111,
		end_col = nil,
	},
	{
		input = [["foo.lua", line 11, characters 111-222]],
		path = "foo.lua",
		line = 11,
		end_line = nil,
		col = 111,
		end_col = 222,
	},
	{ input = [["foo.lua", lines 11-22]], path = "foo.lua", line = 11, end_line = 22, col = nil, end_col = nil },
	{
		input = [["foo.lua", lines 11-22, characters 111-222]],
		path = "foo.lua",
		line = 11,
		end_line = 22,
		col = 111,
		end_col = 222,
	},
	{ input = [[foo.lua(11)]], path = "foo.lua", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [[foo.lua(11,111)]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua(11, 111)]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua (11)]], path = "foo.lua", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [[foo.lua (11,111)]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua (11, 111)]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua: (11)]], path = "foo.lua", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [[foo.lua: (11,111)]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua: (11, 111)]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua(11:111)]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo.lua (11:111)]], path = "foo.lua", line = 11, end_line = nil, col = 111, end_col = nil },
}

function dump(o)
	if type(o) == "table" then
		local s = "{ "
		for k, v in pairs(o) do
			if type(k) ~= "number" then
				k = '"' .. k .. '"'
			end
			s = s .. "[" .. k .. "] = " .. dump(v) .. ","
		end
		return s .. "} "
	else
		return tostring(o)
	end
end

local function test_assertion(result, test)
	for _, key in ipairs({ "line", "col", "end_line", "end_col" }) do
		assert(
			result[key] == test[key],
			key
				.. ' does not match for "'
				.. test.input
				.. '". Expected: '
				.. (test[key] or "nil")
				.. " Actual: "
				.. (result.line or "nil")
				.. " Full: "
				.. dump(result)
		)
	end

	print("PASSED " .. test.input)
end

for _, test in ipairs(tests) do
	local result = parser.parse_file_location(test.input)
	test_assertion(result, test)

	if test.input:match([["]]) then
		test.input = test.input:gsub([["]], "")
		test_assertion(result, test)
	elseif test.input:match("%(") then
		test.input = test.input:gsub("%(", "["):gsub("%)", "]")
		test_assertion(result, test)
	end
end
