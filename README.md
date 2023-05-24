# Framework Power Measurment Tool

This tool is made to measure power on linux for the Framework laptop 13, it will probally work on others although I have not tested, as long as your laptop's battery starts with BAT in /sys/class/power_supply/ it will be found.


## Build & Run
```bash
cargo build --release

./target/release/framework-power-measurment (settings) (task) (time)
```

## Credits
- Myself.

## License

Unlicense, do whatever the hell you want. Although if you want a feature open a issue and I'll probally implement it.
