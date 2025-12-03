# UART

Our microcontroller (like most) has UART ("Universal Asynchronous Receiver/Transmitter)
peripherals. There are two kinds of UART peripheral on the MB2: the older `UART` and the newer
`UARTE` ("UART with Easy DMA").  We will use a `UARTE` peripheral to talk to our hardware serial
port.

Throughout this chapter, we'll use serial communication to exchange information between the
microcontroller and your computer.
