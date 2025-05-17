-- Pull in the wezterm API
local wezterm = require 'wezterm'
local act = wezterm.action

-- This will hold the configuration.
local config = wezterm.config_builder()

-- Font
config.font_size = 9
config.font = wezterm.font 'JetBrainsMono Nerd Font Mono'

-- Theme
config.color_scheme = 'Catppuccin Macchiato'
--config.color_scheme = 'Dracula (Official)'
--config.color_scheme = 'OneDark (base16)'

-- Window
config.enable_scroll_bar = false
config.window_padding = {
  left = 0,
  right = 0,
  -- this make the lines fit perfectly
  top = 0,
  bottom = 0,
}

-- Tab
config.use_fancy_tab_bar = false
--config.hide_tab_bar_if_only_one_tab = true

-- Keys
config.disable_default_key_bindings = true
-- Show which key table is active in the status area
wezterm.on('update-right-status', function(window, pane)
  local name = window:active_key_table()
  if name then
    name = 'TABLE: ' .. name
  end
  window:set_right_status(name or '')
end)

config.leader = { key = 't', mods = 'ALT', timeout_milliseconds = 1000 }
config.keys = {
  {
    key = 'd',
    mods = 'LEADER',
    action = act.ActivateCommandPalette,
  },
  {
    key = 't',
    mods = 'LEADER',
    action = act.SpawnTab 'CurrentPaneDomain',
  },
  {
    key = 'q',
    mods = 'LEADER',
    action = act.CloseCurrentTab { confirm = false },
  },
  {
    key = 'w',
    mods = 'LEADER',
    action = act.CloseCurrentPane { confirm = false },
  },

  { key = '=', mods = 'CTRL', action = act.IncreaseFontSize },
  { key = '-', mods = 'CTRL', action = act.DecreaseFontSize },
  { key = '0', mods = 'CTRL', action = act.ResetFontSize },
  { key = 'C', mods = 'CTRL', action = wezterm.action.CopyTo 'Clipboard' },
  { key = 'V', mods = 'CTRL', action = act.PasteFrom 'Clipboard' },



  -- Alt+t, followed by 'r' will put us in resize-pane
  -- mode until we cancel that mode.
  {
    key = 'r',
    mods = 'LEADER',
    action = act.ActivateKeyTable {
      name = 'resize_pane',
      one_shot = false,
    },
  },

  -- Alt+t, followed by 'p' will put us in activate-pane
  -- mode until we press some other key or until 1 second (1000ms)
  -- of time elapses
  {
    key = 'p',
    mods = 'LEADER',
    action = act.ActivateKeyTable {
      name = 'activate_pane',
      timeout_milliseconds = 1000,
    },
  },
  -- Alt+t, followed by 's' will put us in activate-pane
  -- mode until we press some other key or until 1 second (1000ms)
  -- of time elapses
  {
    key = 's',
    mods = 'LEADER',
    action = act.ActivateKeyTable {
      name = 'split_pane',
      timeout_milliseconds = 1000,
    },
  },
}

config.key_tables = {
  -- Defines the keys that are active in our resize-pane mode.
  -- Since we're likely to want to make multiple adjustments,
  -- we made the activation one_shot=false. We therefore need
  -- to define a key assignment for getting out of this mode.
  -- 'resize_pane' here corresponds to the name="resize_pane" in
  -- the key assignments above.
  resize_pane = {
    { key = 'h',      action = act.AdjustPaneSize { 'Left', 1 } },
    { key = 'l',      action = act.AdjustPaneSize { 'Right', 1 } },
    { key = 'k',      action = act.AdjustPaneSize { 'Up', 1 } },
    { key = 'j',      action = act.AdjustPaneSize { 'Down', 1 } },

    -- Cancel the mode by pressing escape
    { key = 'Escape', action = 'PopKeyTable' },
  },

  -- Defines the keys that are active in our activate-pane mode.
  -- 'activate_pane' here corresponds to the name="activate_pane" in
  -- the key assignments above.
  activate_pane = {
    { key = 'h', action = act.ActivatePaneDirection 'Left' },
    { key = 'l', action = act.ActivatePaneDirection 'Right' },
    { key = 'k', action = act.ActivatePaneDirection 'Up' },
    { key = 'j', action = act.ActivatePaneDirection 'Down' },
  },

  -- Defines the keys that are active in our activate-pane mode.
  -- 'activate_pane' here corresponds to the name="activate_pane" in
  -- the key assignments above.
  split_pane = {
    { key = 'h', action = act.SplitPane { direction = 'Left' } },
    { key = 'l', action = act.SplitPane { direction = 'Right' } },
    { key = 'k', action = act.SplitPane { direction = 'Up' } },
    { key = 'j', action = act.SplitPane { direction = 'Down' } },
  },
}

config.mouse_bindings = {
  {
    event = { Down = { streak = 3, button = 'Left' } },
    action = wezterm.action.SelectTextAtMouseCursor 'SemanticZone',
    mods = 'NONE',
  },
}

for i = 1, 8 do
  -- LEADER + number to activate that tab
  table.insert(config.keys, {
    key = tostring(i),
    mods = 'LEADER',
    action = act.ActivateTab(i - 1),
  })
end

config.window_close_confirmation = 'NeverPrompt'
config.skip_close_confirmation_for_processes_named = {
  'bash',
  'sh',
  'zsh',
  'fish',
}
config.harfbuzz_features = { 'calt=0', 'clig=0', 'liga=0' }

-- Finally, return the configuration to wezterm:
return config
