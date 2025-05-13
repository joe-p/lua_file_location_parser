local M = {}

local path_loc_seperators = "[:#@,%s(%[]"
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

M.parse_file_location = function(str)
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

return M
