require('packer').startup(function(use)
  -- Packer can manage itself
  use {'wbthomason/packer.nvim'}
  -- Dracula
--  use 'dracula/vim'
  use '~/dracula'
  -- Icons required by lualine, tabline and nvim-tree
  use 'kyazdani42/nvim-web-devicons'
  -- PLenary required by Nvim-Tree and Neorg
  use 'nvim-lua/plenary.nvim'
  -- LSP
  use 'neovim/nvim-lspconfig'
  -- Treesitter
  use { 'nvim-treesitter/nvim-treesitter', run = ':TSUpdate' }
  -- LuaLine
  use 'nvim-lualine/lualine.nvim'
  -- TabLine
  use 'kdheepak/tabline.nvim'
  -- Nvim-tree
  use 'kyazdani42/nvim-tree.lua'
  -- ToggleTerm
  use 'akinsho/toggleterm.nvim'
  -- Telescope
  use {
    'nvim-telescope/telescope.nvim',
    requires = {
      'nvim-lua/popup.nvim',
      { 'nvim-telescope/telescope-fzf-native.nvim', run = 'make' }
    }
  }
  -- Neorg
  use 'nvim-neorg/neorg'
  -- VimWiki
  use 'vimwiki/vimwiki'
  -- VimCssColors
  use 'ap/vim-css-color'
  -- Impatient
  use 'lewis6991/impatient.nvim'
  Config = {
    -- Move to lua dir so impatient.nvim can cache it
    compile_path = vim.fn.stdpath('config')..'/lua/packer_compiled.lua'
  }
end)

