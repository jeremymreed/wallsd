#!/usr/bin/env bash
#

# D-Bus spec: https://dbus.freedesktop.org/doc/dbus-specification.html

# --user: Connect to and use the user/session bus.
# call: Send a method call message. (D-Bus also supports signals, error messages, and method replies.)
# <destination> (org.freedesktop.Notifications): The name of the service.
# <object path> (/org/freedesktop/Notifications>: The Object/Interface path.
# <interface> (org.freedesktop.Notifications): The interface name.  (Methods are organized in interfaces,
# 					       here it's the same as the service.)
# <method> (Notify): The name of the method to call.
# <signature> (susssasa{sv}i): That string means the method takes 8 arguments of various types.
# 			       's' for example is string.  'as' is for array of strings.
#                  See https://dbus.freedesktop.org/doc/dbus-specification.html#type-system.
# <args> ("my-app" 0 "dialog-information" "A summary" "Some body" 0 0 5000): The method arguments.

busctl --user call \   # --user connect to and use the user/session bus.
	org.freedesktop.Notifications \
	/org/freedesktop/Notifications \
	org.freedesktop.Notifications \
	Notify \
	susssasa\{sv\}i \
	"my-app" 0 "dialog-information" "A summary" "Some body" 0 0 5000

