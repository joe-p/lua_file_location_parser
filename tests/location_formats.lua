local parser = require("../lua/file_location_parser")

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
	local result = parser.parse_file_location(test.input)
	test_assertion("line", result, test)
	test_assertion("end_line", result, test)
	test_assertion("col", result, test)
	test_assertion("end_col", result, test)
end
