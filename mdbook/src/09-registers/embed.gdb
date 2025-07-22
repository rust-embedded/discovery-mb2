target remote :1337
set print asm-demangle on
set print pretty on
set style sources off
break main
break DefaultHandler
break HardFault
monitor reset halt
