# Framework Power Measurment Tool

This tool is made to measure power on linux for the Framework laptop 13, it will probally work on others although I have not tested, as long as your laptop's battery starts with BAT in /sys/class/power_supply/ it will be found.

It outputs the recorded data to a .csv file and it will (soon again) plot the data to a graph .png.


## Build & Run
```bash
cargo build --release

./target/release/framework-power-measurment help

./compile.sh # To Compile the Data into a single csv

```
After you have your final .csv you can import it into "https://app.rawgraphs.io/" after which you wanna choose Line Graph and then put Time on the X axis, Wattage on the Y axis, and then probally info as the Lines and either Settings or Info as the color depending on what you wanna see. 

## Credits
- Myself.

## Todo

[ ] Implement graph splitting and compilation
[ ] Implement Load Emulation(Video, Text, Document Editing)
[ ] Reimplement Visualization


## License

MIT License
