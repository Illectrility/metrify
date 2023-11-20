# metrify
I don't like using the Imperial system. Whenever someone in a YouTube video mentions fluid ounces, feet and square yards, I pause, open up my browser and search for a conversion site. This is fairly slow, though (and I needed a new project) so I decided to make metrify. Metrify easily allows conversions from Imperial untis to metric units.

# building
To build metrify you can run `rustc main.rs` from `/src` or `cargo build --release` from `/metrify`. You may need to run `chmod +x metrify` to allow execution. You can then copy the metrify binary to `~/.local/bin` to run it from anywhere.

# running
You can run `./metrify` from the `/metrify` directory (or just run `metrify` if you copied it to `~/.local/bin`).

# example
```bash
trility@pop-os:~$ metrify
You have:
8sq ft
Which roughly equals:
0.743m²
You have:
30 mi
Which roughly equals:
48.280km
You have:
8us fl oz
Which roughly equals:
2.366dL
You have:
1 uk pint
Which roughly equals:
5.683dL
You have:
^C
trility@pop-os:~$
```
