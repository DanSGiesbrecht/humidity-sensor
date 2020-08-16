#!/bin/sh

udev_file='
# Black Magic Probe\n
# There are two connections, one for GDB and one for uart debugging\n
SUBSYSTEM=="tty", ATTRS{interface}=="Black Magic GDB Server", SYMLINK+="ttyBmpGdb", MODE="0666"\n
SUBSYSTEM=="tty", ATTRS{interface}=="Black Magic UART Port", SYMLINK+="ttyBmpTarg", MODE="0666"'

if ! [ -x "$(command -v curl)" ]
then
    sudo apt install -y curl
fi

if ! [ -x "$(command -v gdb-multiarch)" ]
then
    sudo apt install -y gdb-multiarch
fi

if ! [ -x "$(command -v rustup)" ]
then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

rustup target add thumbv6m-none-eabi

if ! [ -f "/etc/udev/rules.d/99-blackmagic.rules" ]
then
    echo $udev_file | sudo tee /etc/udev/rules.d/99-blackmagic.rules > /dev/null
fi
