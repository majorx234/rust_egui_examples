# Info
- small examples, create GUI based applications
- working wirh egui or eframe

# Examples
## first app
- simplest gui with a label
## counter
- 2 buttons and a label
## plot
- plot for continous values
- first value as usize gives amount of values
- next values are in float 
- all values seperated by **new line**
- e.g: 

``` bash
6
0.0
0.1
0.2
0.1
0.0
-0.1
```
- testing: `echo "4\n0.1\n0.2\n0.05\n-0.4\n"|target/debug/plot`

## plot_lines
- same as **plot** but with in build values
## painter
- simple paint application
- WIP

## toggle_switch
- switch 
- by clicking on it change state
- uses `egui::ui::painter`

# links
- egui: https://github.com/emilk/egui
- eframe: https://github.com/emilk/egui/tree/master/crates/eframe
