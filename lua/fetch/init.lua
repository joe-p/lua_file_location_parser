local function load_fetch_rs()
	local current_script_path = debug.getinfo(1, "S").source:sub(2)
	local system_info = vim.uv.os_uname()

	local repo_root = vim.fn.fnamemodify(current_script_path, ":h:h:h")
	local bin_path = vim.fs.joinpath(repo_root, "bin", (system_info.sysname .. "-" .. system_info.machine):lower())

	vim.opt.rtp:append(bin_path)

	return require("fetch_rs")
end

local fetch_rs = load_fetch_rs()

local M = {}

M.get_links_from_line = function(line)
	return fetch_rs.get_links_from_line(line)
end

M.get_links_on_current_line = function()
	return M.get_links_from_line(vim.api.nvim_get_current_line())
end

M.get_link_at_position_in_line = function(line, pos)
	return fetch_rs.get_link_at_position_in_line(line, pos)
end

M.get_link_under_cursor = function()
	return M.get_link_at_position_in_line(vim.api.nvim_get_current_line(), vim.api.nvim_win_get_cursor(0)[2] + 1)
end

M.open_link_under_cursor = function()
	local link = M.get_link_under_cursor()
	if link and vim.fn.filereadable(link.path.text) then
		vim.cmd("edit " .. link.path.text)
		vim.api.nvim_win_set_cursor(0, { link.suffix.row, link.suffix.col - 1 })
	end
end

return M
