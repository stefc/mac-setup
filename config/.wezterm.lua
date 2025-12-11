--- Pull in the wezterm API

local wezterm = require 'wezterm'
local config = wezterm.config_builder()

config.color_scheme = 'Selenized Dark (Gogh)'

config.font_size = 16
config.font = wezterm.font('JetBrains Mono')
config.keys = {
  {
    key = 'w',
    mods = 'SUPER',
    action = wezterm.action.CloseCurrentPane { confirm = true },
  },
}

return config
