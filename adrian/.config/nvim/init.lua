---- Helpers
local fn = vim.fn
local g = vim.g
local cmd = vim.cmd
local execute = vim.api.nvim_command

-- Map leader to space
g.mapleader = ' '

local function sys_init()
  require ('impatient')
end

local function packer_init()
  local install_path = fn.stdpath "data" .. "/site/pack/packer/opt/packer.nvim"
  if fn.empty(fn.glob(install_path)) > 0 then
    execute("!git clone https://github.com/wbthomason/packer.nvim " .. install_path)
  end
  cmd [[packadd packer.nvim]]
  cmd "autocmd BufWritePost plugins.lua PackerCompile"
end

-- Start loading
sys_init()
require('packer_compiled')
packer_init()
require('settings')
require('keymappings')
require('plugins')
require('config/treesitter')
require('config/lualine')
require('config/tabline')
require('config/nvimtree')
require('config/telescope')
require('config/neorg')
require('lsp-lua/lua-language-server')
-- End loading
