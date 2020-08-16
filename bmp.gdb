target extended-remote /dev/ttyBmpGdb
set mem inaccessible-by-default off
monitor tpwr enable
shell sleep 0.25
monitor swdp_scan
attach 1
set print asm-demangle on
set backtrace limit 32
break DefaultHandler
break HardFault
break rust_begin_unwind
break main
load
continue
