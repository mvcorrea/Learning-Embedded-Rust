# from https://github.com/blacksphere/blackmagic/wiki/Useful-GDB-commands
target extended-remote /dev/cu.usbmodemE1C59EA1
monitor swdp_scan
attach 1
# allow access to known memory, eg. for access to mem-mapped IO
set mem inaccessible-by-default off
