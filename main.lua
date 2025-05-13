local tests = {
	{ input = [[foo:11]], path = "foo", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [[foo:11:111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo:11:111-222]], path = "foo", line = 11, end_line = nil, col = 111, end_col = 222 },
	{ input = [[foo:11:111-22.222]], path = "foo", line = 11, end_line = 22, col = 111, end_col = 222 },
	{ input = [[foo:11.111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo 11]], path = "foo", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [[foo 11:111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo 11.111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo#11]], path = "foo", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [[foo#11:111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo#11.111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo, 11]], path = "foo", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [["foo",11]], path = "foo", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [["foo",11:111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo",11.111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo",11.111-222]], path = "foo", line = 11, end_line = nil, col = 111, end_col = 222 },
	{ input = [["foo",11.111-22.222]], path = "foo", line = 11, end_line = 22, col = 111, end_col = 222 },
	{ input = [["foo", line 11]], path = "foo", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [["foo", line 11, col 111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo", line 11, column 111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo":line 11]], path = "foo", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [["foo":line 11, col 111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo":line 11, column 111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo": line 11]], path = "foo", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [["foo": line 11, col 111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo": line 11, column 111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo" on line 11]], path = "foo", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [["foo" on line 11, col 111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo" on line 11, column 111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo" line 11 column 111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [["foo", line 11, character 111]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{
		input = [["foo", line 11, characters 111-222]],
		path = "foo",
		line = 11,
		end_line = nil,
		col = 111,
		end_col = 222,
	},
	{ input = [["foo", lines 11-22]], path = "foo", line = 11, end_line = 22, col = nil, end_col = nil },
	{
		input = [["foo", lines 11-22, characters 111-222]],
		path = "foo",
		line = 11,
		end_line = 22,
		col = 111,
		end_col = 222,
	},
	{ input = [[foo(11)]], path = "foo", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [[foo(11,111)]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo(11, 111)]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo (11)]], path = "foo", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [[foo (11,111)]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo (11, 111)]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo: (11)]], path = "foo", line = 11, end_line = nil, col = nil, end_col = nil },
	{ input = [[foo: (11,111)]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo: (11, 111)]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo(11:111)]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
	{ input = [[foo (11:111)]], path = "foo", line = 11, end_line = nil, col = 111, end_col = nil },
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

local function parse_path(str)
	local path_loc_seperators = "[:#@,%s(]"
	local line_col_seperators = "[:.,%s]+"

	local line_patterns = { "(%d+)" }
	local line_col_patterns = {
		"(%d+)" .. line_col_seperators .. "(%d+)",
		"line (%d+), col (%d+)",
		"line (%d+),? column (%d+)",
		"line (%d+),? character (%d+)",
	}
	local line_col_end_col_patterns =
		{ "(%d+)" .. line_col_seperators .. "(%d+)-(%d+)", "line (%d+),? characters (%d+)-(%d+)" }
	local line_col_end_line_end_col_patterns = { "(%d+)%.(%d+)-(%d+)%.(%d+)", "(%d+):(%d+)-(%d+)%.(%d+)" }
	local line_end_line_patterns = { "lines (%d+)-(%d+)" }
	local line_end_line_col_end_col_patterns = { "lines (%d+)-(%d+), characters (%d+)-(%d+)" }

	for _, pat in ipairs(line_col_end_line_end_col_patterns) do
		local line, col, end_line, end_col = string.match(str, path_loc_seperators .. pat)
		if line and col and end_line and end_col then
			return {
				line = tonumber(line),
				col = tonumber(col),
				end_col = tonumber(end_col),
				end_line = tonumber(end_line),
				pattern = pat,
			}
		end
	end

	for _, pat in ipairs(line_end_line_col_end_col_patterns) do
		local line, end_line, col, end_col = string.match(str, path_loc_seperators .. pat)
		if line and col and end_line and end_col then
			return {
				line = tonumber(line),
				col = tonumber(col),
				end_col = tonumber(end_col),
				end_line = tonumber(end_line),
				pattern = pat,
			}
		end
	end

	for _, pat in ipairs(line_col_end_col_patterns) do
		local line, col, end_col = string.match(str, path_loc_seperators .. pat)
		if line and col and end_col then
			return { line = tonumber(line), col = tonumber(col), end_col = tonumber(end_col), pattern = pat }
		end
	end

	for _, pat in ipairs(line_end_line_patterns) do
		local line, end_line = string.match(str, path_loc_seperators .. pat)
		if line and end_line then
			return { line = tonumber(line), end_line = tonumber(end_line), pattern = pat }
		end
	end

	for _, pat in ipairs(line_col_patterns) do
		local line, col = string.match(str, path_loc_seperators .. pat)
		if line and col then
			return { line = tonumber(line), col = tonumber(col), pattern = pat }
		end
	end

	for _, pat in ipairs(line_patterns) do
		local line = string.match(str, path_loc_seperators .. pat)
		if line then
			return { line = tonumber(line), pattern = pat }
		end
	end

	return {}
end

local function test_assertion(key, result, test)
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

	print("PASSED " .. key .. " for " .. "`" .. test.input .. "`")
end

for _, test in ipairs(tests) do
	local result = parse_path(test.input)
	test_assertion("line", result, test)
	test_assertion("end_line", result, test)
	test_assertion("col", result, test)
	test_assertion("end_col", result, test)
end
