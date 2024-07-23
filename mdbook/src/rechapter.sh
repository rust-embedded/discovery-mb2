git mv "09-clocks-and-timers" "06-clocks-and-timers"
git mv "06-serial-communication" "08-serial-communication"
git mv "07-uart" "09-uart"
git mv "08-i2c" "10-i2c"
git mv "09-led-compass" "11-led-compass"
git mv "10-punch-o-meter" "12-punch-o-meter"
git mv "11-snake-game" "13-snake-game"
find . -type f -name "*.md" -print |
while read i; do sed -i -f rechapter.sed "$i"; done
