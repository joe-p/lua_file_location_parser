# File Location Parser

This is a library for parsing file locations from a string. For example, it will parse `foo.rb:11:22` and return `{line = 11, col = 22}`. 

## Supported Formats

Below are the supported formats for the following example

File: `foo.lua`
Start Line: `11`
End Line: `22`
Start Column: `111`
End Column: `222`

- foo.lua:11
- foo.lua:11:111
- foo.lua:11:111-222
- foo.lua:11:111-22.222
- foo.lua:11.111
- foo.lua 11
- foo.lua 11:111
- foo.lua 11.111
- foo.lua#11
- foo.lua#11:111
- foo.lua#11.111
- foo.lua, 11
- "foo.lua",11
- foo.lua,11
- "foo.lua",11:111
- foo.lua,11:111
- "foo.lua",11.111
- foo.lua,11.111
- "foo.lua",11.111-222
- foo.lua,11.111-222
- "foo.lua",11.111-22.222
- foo.lua,11.111-22.222
- "foo.lua", line 11
- foo.lua, line 11
- "foo.lua", line 11, col 111
- foo.lua, line 11, col 111
- "foo.lua", line 11, column 111
- foo.lua, line 11, column 111
- "foo.lua":line 11
- foo.lua:line 11
- "foo.lua":line 11, col 111
- foo.lua:line 11, col 111
- "foo.lua":line 11, column 111
- foo.lua:line 11, column 111
- "foo.lua": line 11
- foo.lua: line 11
- "foo.lua": line 11, col 111
- foo.lua: line 11, col 111
- "foo.lua": line 11, column 111
- foo.lua: line 11, column 111
- "foo.lua" on line 11
- foo.lua on line 11
- "foo.lua" on line 11, col 111
- foo.lua on line 11, col 111
- "foo.lua" on line 11, column 111
- foo.lua on line 11, column 111
- "foo.lua" line 11 column 111
- foo.lua line 11 column 111
- "foo.lua", line 11, character 111
- foo.lua, line 11, character 111
- "foo.lua", line 11, characters 111-222
- foo.lua, line 11, characters 111-222
- "foo.lua", lines 11-22
- foo.lua, lines 11-22
- "foo.lua", lines 11-22, characters 111-222
- foo.lua, lines 11-22, characters 111-222
- foo.lua(11)
- foo.lua[11]
- foo.lua(11,111)
- foo.lua[11,111]
- foo.lua(11, 111)
- foo.lua[11, 111]
- foo.lua (11)
- foo.lua [11]
- foo.lua (11,111)
- foo.lua [11,111]
- foo.lua (11, 111)
- foo.lua [11, 111]
- foo.lua: (11)
- foo.lua: [11]
- foo.lua: (11,111)
- foo.lua: [11,111]
- foo.lua: (11, 111)
- foo.lua: [11, 111]
- foo.lua(11:111)
- foo.lua[11:111]
- foo.lua (11:111)
- foo.lua [11:111]

## Credits

- VSCode for a [comprehensive list of formats](https://github.com/microsoft/vscode/blob/ce2c2f3c79a32b9917e32c61e058392dc5a1b6aa/src/vs/workbench/contrib/terminalContrib/links/browser/terminalLinkParsing.ts#L75-L126)
