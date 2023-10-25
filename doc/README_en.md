# PVZ Interception Calculator in Rust

[English](./README_en.md)| [简体中文](../README.md) 

A rewrite of the original Interception Calculator in rust with various issues fixed.

## Commands
| Command | Usage |
| -------- | ----------- |
| de/pe/re   |             Set scene |
| wave             |       View current ice times and cob time |
| wave [ice times..] [cob time]  | Set ice times and cob time (ice times can be none)<br>eg. `$ wave 1 400 800` -> use ice at 1, 400; use cob at 800 |
| delay [hit col] (cob tail col)| Calc interceptable interval, earliest eat & iceable (need to provide cob tail col for roof scene)<br>eg. `$ delay 8.8` -> Calc hit col 8.8<br>`$ delay 3.5 4` -> Calc hit col 3.5 for cob tail col 4  |
| delay [hit row] [hit col] (cob tail col)<br>> [garg rows] (garg x range) (u/i) | Calc specific gargs (may specify ice mode)<br>eg. `$ delay 1 8.8 > 2` -> Calc (1,8.8) cob for row 2 garg<br>`$ delay 1 8.8 > 1,2 700,800` -> Calc (1,8.8) cob for row [1,2] gargs with x 700~800<br>`$ delay 1 8.8 > 1,2 700,800 u` -> Same as above, but specify ice mode as uniced  |
| doom [doom row] [doom col]<br>(> [garg rows] (garg x range) (u/i)) | Calc doom for specific gargs (args after ">" are optional; may specify ice mode)<br>eg. `$ doom 3 8` -> Calc 3-8 doom<br>`$ doom 3 8 > 2,5 700,800` -> Calc 3-8 doom for row [2,5] gargs with x 700~800  |
| hit (cob tail col) (delay) |Calc hit col that hits all gargs (may specify delay)<br>eg. `$ hit` -> Calc hit col that hits all gargs<br>`$ wave 300 $ hit 50` -> Calc hit col that hits all gargs at 350cs<br>`$ wave 300 $ hit -50` -> Calc hit col that hits all gargs at 250cs |
| nohit (cob tail col) (delay) |Calc hit col that doesn't hit any garg (may specify delay) |
| max [hit row] [hit col range]<br>> [garg rows] (garg x range) (u/i) | Find hit col that harmlessly intercepts with max delay (may specify ice mode)<br>eg. `$ max 1 7,7.5 > 1,2` -> For hit row 1 and hit col 7~7.5, find hit col that harmlessly intercepts gargs with max delay |
| imp [imp x]          |    Calc x range of garg who can throw imp of this x |
| ?/help              |     Show this help |
|  about              |     About Interception Calculator |