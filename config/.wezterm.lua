--- Pull in the wezterm API

local wezterm = require 'wezterm'
local config = wezterm.config_builder()

config.color_scheme = 'Selenized Dark (Gogh)'

config.font_size = 16
config.font = wezterm.font('JetBrains Mono')

return config
