local cmd = vim.cmd
local opt = vim.opt
local indent = 2

cmd 'syntax enable'
cmd 'colorscheme dracula'
cmd 'filetype plugin indent on'

opt.clipboard = 'unnamedplus'       -- Allow clipboard from outside of vim
opt.expandtab = true                -- Use spaces instead of tabs
opt.foldmethod = 'marker'           -- Fold between {{{ and }}}
opt.hidden = true                   -- Enable background buffers
opt.ignorecase = true               -- Ignore case
opt.list = true                     -- Show some invisible characters
opt.number = true                   -- Show line numbers
opt.mouse = 'a'                     -- Enable mouse use
opt.relativenumber = true           -- Relative line numbers
opt.scrolloff = 4                   -- Lines of context
opt.shiftround = true               -- Round indent
opt.shiftwidth = indent             -- Size of an indent
opt.sidescrolloff = 8               -- Columns of context
opt.smartcase = true                -- Do not ignore case with capitals
opt.smartindent = true              -- Insert indents automatically
opt.splitbelow = true               -- Put new windows below current
opt.splitright = true               -- Put new windows right of current
opt.tabstop = indent                -- Number of spaces tabs count for
opt.termguicolors = true            -- True color support
opt.wildmenu = true                 -- Command-line completion mode
opt.wrap = false                    -- Disable line wrap

-- Highlight on yank
cmd 'au TextYankPost * lua vim.highlight.on_yank {on_visual = false}'
