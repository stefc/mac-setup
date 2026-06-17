--- Pull in the wezterm API

local wezterm = require 'wezterm'
local config = wezterm.config_builder()

local function scheme_for_appearance(appearance)
  if appearance:find 'Dark' then
    return 'Warm Burnout Dark'
  end
  return 'Warm Burnout Light'
end

--- config.color_scheme = scheme_for_appearance(wezterm.gui.get_appearance())
config.color_scheme = 'Warm Burnout Dark'
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
