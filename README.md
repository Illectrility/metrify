# about
I don't like using the Imperial system. Whenever someone in a YouTube video mentions fluid ounces, feet and square yards, I pause, open up my browser and search for a conversion site. This is fairly slow, though (and I needed a new project) so I decided to make metrify. Metrify easily allows conversions from Imperial units to metric units.

# why not use units?
You may ask yourself: "Why not just install `units` from apt?" And that's a great question. I don't like the interface of units. It works great (arguably far better than metrify) but I wanted a different interface and – as I mentioned above – I was bored and needed something to do.

# features
- Short and long versions of the units (i.e. `square feet` and `sq ft`)
- Plural support for (most) units (i.e. `foot` and `feet`)

# building
To build metrify you can run `rustc main.rs` from `/src` or `cargo build --release` from `/metrify`. You may need to run `chmod +x metrify` to allow execution. You can then copy the metrify binary to `~/.local/bin` to run it from anywhere.

# running
You can run `./metrify` from the `/metrify` directory (or just run `metrify` if you copied it to `~/.local/bin`).

# example
```bash
trility@pop-os:~$ metrify
You have:
8 square feet
Which roughly equals:
0.743m²
You have:
30 miles
Which roughly equals:
48.280km
You have:
8 us fl oz
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
# currently supported units
## length (base unit meter)
- inch (in)
- foot (ft)
- yard (yd)
- mile (mi)
- fathom (ftm)
- cable length (cbl)
- nautical mile (nmi)
## mass (base unit gram)
- pound (lb)
- ounce (oz)
- grain (gt)
- dram (dr)
- stone (st)
- quarter (qr)
- hundredweight (cwt)
- ton (t)
## pressure (base unit bars)
- pounds per square inch (psi)
- feet of water (ftH2O)
- atmospheres (atm)
- pounds per square foot (psf)
## velocity (base unit meters per second)
- feet per second (ft/s)
- miles per hour (mph)
- knots (kn)
## acceleration (base unit meters per second squared)
- feet per second squared (ft/s²)
- miles per hour per second (mph/s)
- inches per second squared (in/s²)
## mileage (base unit liters per 100 kilometers)
- miles per gallon (mpg)
## volume (base unit liter)
- cubic inch (in³)
- cubic foot (ft³)
- us fluid ounce (us fl oz)
- us gallon (us gal)
- uk fluid ounce (uk fl oz)
- uk gallon (uk gal)
- us pint (us pt)
- us quart (us qt)
- uk pint (uk pt)
- uk quart (uk qt)
- us cup (us cup)
- us fluid drams (us fl dr)
## area (base unit square meters)
- square inch (sq in)
- square foot (sq ft)
- square yard (sq yd)
- acre (ac)
- square mile (sq mi)
