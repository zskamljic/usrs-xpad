# usrs-xpad

Userspace Xbox controller driver.

## Motivation

I have purchased 2 Xbox One S clone controllers, that didn't work with the default linux kernel driver, nor did they
work with [dkms xpad driver](https://github.com/paroj/xpad), and xboxdrv did not seem to support the
vendor_id/product_id combo, and I was looking for an excuse to do a project in rust.

Note that I had no previous knowledge of how libusb or uinput work, the whole thing was developed in a span of a few
days, after work.

## How it works

In order to access USB devices this program uses libusb. After discovering all devices, it filters the eligible ones,
finding their configs, interfaces and endpoints for reading and writing. After obtaining the relevant data it starts a
scoped thread for each controller found (thanks to The Rust Programming Language Discord). The thread then attempts to
claim the device, then writes the init packet to it. After successful init the thread loops indefinitely (or until an
error occurs), reading from the device on each loop. The read packet is parsed and sent over to uinput device.

In order to make it run fully without sudo access I used the following udev, in `/etc/udev/rules.d/51-xpad.rules`:

```
SUBSYSTEM=="usb", ATTRS{idVendor}=="045e", ATTRS{idProduct}=="02ea", TAG+="uaccess"
```

## Note:

Not all software seems to detect it as an Xbox controller. As I have also managed to since "repair" the dkms-xpad driver
I was able to compare jstest-gtk, and it seems not everything is mapped in the same way. Additionally, if some error
occurs while running thread for the controller no information is printed. This needs to be addressed. Additionally, only
controllers present at program startup are detected, so if the controller is attached after running it, it will not be
detected. Additionally, it appears that two controllers on the same bus are not supported, albeit I seem to have the
same issue with dkms-xpad.

## Supported controllers

At this time only one controller is supported (as I only have the one):

- 045e:02ea

## License

Copyright 2021 zskamljic

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit
persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the
Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.