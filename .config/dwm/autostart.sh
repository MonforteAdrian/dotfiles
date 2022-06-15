#! /bin/bash
lxsession &
picom &
solaar --window=hide &

# uncomment this line to restore last saved wallpaper...
# xargs xwallpaper --stretch < ~/.xwallpaper &

# ...or uncomment this line to set a random wallpaper on login
find ~/wallpapers/ -type f | shuf -n 1 | xargs xwallpaper --stretch &

xrandr --output eDP-1-1 --auto --output HDMI-0 --auto --left-of eDP-1-1
