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
1. `GetOutputsSettings`: This method returns the settings for all outputs.
2. `SetOutputImages`: This method sets the images for the given output.
3. `SetOutputMode`: This method sets the mode for the given output.
4. `SetOutputOncalendar`: This method sets the output's oncalendar string.  This tells the daemon when to change the wallpaper in slideshow mode.  This string is passed to `systemd-analyze calendar`.

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

1. `GetOutputsSettings`: Get the settings for all outputs:
`busctl --user call com.thetechforest.WallsD /com/thetechforest/WallsD com.thetechforest.WallsD GetOutputsSettings`

Response
```
usa(susst) 0 "" 2 "HDMI-A-1" 0 "*-*-* *:0/15" "/home/jeremyr/Pictures/Wallpapers/wallhaven-nejr-fixed-d31073bc8bc01bcef826ad8716e818995acd4e8b0b95d132fa28bfd1e2316401-2560x1440.png" 13 "eDP-1" 1 "*-*-* *:0/15" "/home/jeremyr/Pictures/Wallpapers/DoDovgx-02c976fa55a4cb2351f2a324b97fb9e10f5be122568611b98e118ef40f108190-1920x1080.jpeg" 3364
```

2. `SetOutputMode`: Set the mode for a given output:  This one sets HDMI-A-1 to oneshot mode.
`busctl --user call com.thetechforest.WallsD /com/thetechforest/WallsD com.thetechforest.WallsD SetOutputMode su "HDMI-A-1" 0`

Response:
```
us 0 ""
```

3. `SetOutputImages`: Set the images for a given output:  This one sets images for HDMI-A-1.  Some of these are paths to directories.  Some of these are invalid.
`busctl --user call com.thetechforest.WallsD /com/thetechforest/WallsD com.thetechforest.WallsD SetOutputImages sas "HDMI-A-1" 5 "/home/jeremyr/Pictures/Wallpapers/Test" "/home/jeremyr/Pictures/Wallpapers/95b31050-5290-11ea-bbbb-c9d5dfaa12ea-7b7cc31d35316f417d0476a0eaa6963ceb8b3b647beda2638c749a4e212af833-1920x1080.jpeg" "Foo_Bar.jpeg" "/home/jeremyr/Pictures/Wallpapers/DoesNotExist/" "/home/jeremyr/Pictures/Wallpapers/wallhaven-zypzov-fa48e20427e95ed8c6316759aeacbe897699ebc4ab65a81e6e83926df7e5098d-1920x1080.jpeg"`

Response:
```
uas 0 2 "Couldn\'t process: Foo_Bar.jpeg, Error: No such file or directory (os error 2)" "Couldn\'t process: /home/jeremyr/Pictures/Wallpapers/DoesNotExist/, Error: No such file or directory (os error 2)"
```

4. `SetOutputOncalendar`: Set the oncalendar string for a given output:  This one sets the oncalendar string for HDMI-A-1 to "*-*-* *:*:0/30".  If the mode is set to slideshow, the daemon will change the wallpaper every 30 seconds.  If the mode is set to oneshot, there is no apparent change.  This only changes the oncalendar string, and does not change the mode.
`busctl --user call com.thetechforest.WallsD /com/thetechforest/WallsD com.thetechforest.WallsD SetOutputOncalendar ss "HDMI-A-1" "*-*-* *:*:0/30"`

Response:
```
us 0 ""
```

5. `SetOutputImages`: To set an output to onshot mode, and specify the desired wallpaper:  Note that you have to send a SetOutputMode command to force the daemon to change the wallpaper, even if the output is already in oneshot mode.
First send the image to be used.  By default, the daemon will look at the first image in the images vector.  If the command has additional images, those are loaded into the images vector.  However, the daemon will not use these additional images while it is in oneshot mode.
Second, send an SetOutputMode command to force the daemon to change the wallpaper.
```
busctl --user call com.thetechforest.WallsD /com/thetechforest/WallsD com.thetechforest.WallsD SetOutputImages sas "HDMI-A-1" 1 "/home/jeremyr/Pictures/Wallpapers/wallhaven-3z5999-9bcf8883147fbff4a62f9857c7748f20f0019b381bfb29b593ed3c031f5e15b6-1920x1080.jpeg"

busctl --user call com.thetechforest.WallsD /com/thetechforest/WallsD com.thetechforest.WallsD SetOutputMode su "HDMI-A-1" 0
```

Responses:
```
uas 0 0

us 0 ""
```

# License:
This program is licensed under the Apache 2.0 License.
