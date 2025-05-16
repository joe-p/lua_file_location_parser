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

return M
