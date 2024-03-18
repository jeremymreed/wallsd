wallsd.
======================================

======================================
### Table of Contents
1. [Purpose](https://github.com/jeremymreed/wallsd#purpose)
2. [Building](https://github.com/jeremymreed/wallsd#building)
3. [Installing](https://github.com/jeremymreed/wallsd#installing)
4. [License](https://github.com/jeremymreed/wallsd#license)


# Purpose:
This is the daemon for the wallpaper management system.
This is meant to run in the background, and performs the action of setting the wallpaper.
It will receive commands from the front end via dbus.

Internally, it will have three threads, a main thread, an executor (which executes the wallpaper change) thread, and a thread to listen for dbus messages.

We'd like to have the software support several modes: slideshow, and one-shot.  The user needs to select the desired output.
  - slideshow: Change wallpapers on a given interval.  The slideshow will loop itself.  The user can choose to have the wallpapers shown in order, or randomly chosen.
  - one-shot:  A single image is given for a specific output, and the wallpaper should remain until the user changes it.

# Building:

To build:
```
> Cargo build
```

To run:
```
> Cargo run
```


# License:
This program is licensed under the GPLv2 License.
