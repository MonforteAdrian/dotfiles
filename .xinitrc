while true; do
  xsetroot -name "$(date +"%F %R" )"
  sleep 15
done &

xrandr --setprovideroutputsource modesetting NVIDIA-0
xrandr --auto

exec dwm
