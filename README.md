wallsd.
======================================

======================================
### Table of Contents
1. [Purpose](https://github.com/jeremymreed/wallsd#purpose)
2. [Building](https://github.com/jeremymreed/wallsd#building)
3. [Installing](https://github.com/jeremymreed/wallsd#installing)
4. [busctl examples](https://github.com/jeremymreed/wallsd#busctl-examples)
5. [License](https://github.com/jeremymreed/wallsd#license)


# Purpose:
This is the daemon for the wallpaper management system.
This is meant to run in the background, and performs the action of setting the wallpaper.

The daemon has two modes:
1. Oneshot: This mode sets the wallpaper for the given output.  (Internally, the wallpaper is set to output.images[0].  Currently this value is hardcoded, and should be selectable by the user.)  This mode stops the slideshow.
2. Slideshow: This mode tells the daemon that it should shuffle through the images.  The daemon will pick a random image.  The daemon will then wait a specified amount of time, before picking another random image.

The daemon will accept messages via dbus.  Currently, I am using `busctl` to interact with the daemon.  In the near future there will be a flutter based front end application that will interact with the daemon.

Output from `busctl --user introspect com.thetechforest.WallsD /com/thetechforest/WallsD`:
```
NAME                                TYPE      SIGNATURE RESULT/VALUE FLAGS
com.thetechforest.WallsD            interface -         -            -
.GetOutputsSettings                 method    -         (usa(susst)) -
.SetOutputImages                    method    (sas)     (uas)        -
.SetOutputMode                      method    (su)      (us)         -
.SetOutputOncalendar                method    (ss)      (us)         -
... (additional methods removed)
```

Commands:
1. GetOutputsSettings: This method returns the settings for all outputs.
2. SetOutputImages: This method sets the images for the given output.
3. SetOutputMode: This method sets the mode for the given output.
4. SetOutputOncalendar: This method sets the output's oncalendar string.  This tells the daemon when to change the wallpaper in slideshow mode.  This string is passed to `systemd-analyze calendar`.

help message from `wallsd --help`:
```
A daemon to control wallpaper settings

Usage: wallsd

Options:
  -h, --help     Print help
  -V, --version  Print version
```

# Building:

To build:
```
> Cargo build
```

To run:
```
> Cargo run
```

# Installing:
There is no formal installation mechanism yet.  You can copy the binaries from `target/debug/` or `target/release/` to a location in your path.
I will create a PKGBUILD file for this daemon.

# busctl examples:
Some example usages of busctl to interact with this daemon.

# License:
This program is licensed under the GPLv2 License.
