# Serial Port

The closest thing to a universal I/O standard for modern day embedded boards is the "serial port". Pretty much every microcontroller has a way to make a few of its pins act as a serial port, and pretty much every microcontroller board makes these pins easy to get to. The MB2 is no exception.

In this chapter, we will describe what a serial port even is. We will then show you how to set up your computer with a "virtual serial port" using USB and use that virtual port with "terminal software" to talk to a serial port on the MB2.

So what's this [serial port]? It's a place where two devices exchange data one bit at a time (*serially*) using one data line in each direction (*full-duplex*) plus a common ground. The serial port originated as "RS-232": see the history later in this chapter for details. However, the protocol spoken on the transmit and receive lines doesn't have an official name I'm aware of — it's just "serial" or maybe "async serial" or "UART serial".

To be clear: most communication channels in modern computers are serial. USB (the "Universal Serial Bus") is a serial channel; I2C (which we will talk about later) is a serial channel. This chapter and the next are *not* about the general concept of serial communication: these chapters are about a specific thing called a "serial port" that has its own implementation and history.

Serial port communication is *asynchronous* in the sense that none of the shared lines carries a clock signal. Instead, both parties must agree on roughly how fast data will be sent along the wire *before* the communication occurs. A peripheral called a Universal Asynchronous Receiver/Transmitter (UART) sends bits at the specified rate on its output wire, and watches for the start of bits on its input wire.

<p align="center">
<img class="white_bg" height="100" title="Serial Protocol" src="../assets/serial-proto.svg" />
</p>

The serial-port communications protocol works with frames, each carrying a byte of data. Each frame has one *start* bit, 5 to 9 bits of payload data (sent lsb-to-msb; modern applications rarely send a 9-bit byte; 7 or fewer bits in a frame will be left-padded to an 8-bit byte with zeros) and 1 to 2 *stop bits*.  In the diagram above, an ASCII 'E' character is sent using 8 data bits and 1 stop bit.

The speed of the protocol is known as *baud rate* and it's quoted in bits per second (bps). (If you're thinking that this sounds wrong — it is. "Baud" is supposed to be *symbols* per second; a symbol should correspond to a frame; even if a data bit is regarded as the "symbol" they aren't sent at this rate because of the rest of the protocol. It's a convention, and doesn't have to make sense.) Historically common baud rates for UART serial are 9600bps, 19200bps, and 115200bps, but it is not uncommon in our modern world to send data at 921,600bps.

With the "normal" configuration of 1 start bit, 8 bits of data, 1 stop bit and a bit rate of 921.6K bps we can send and receive 92.16K bytes per second — fast enough to transmit single-channel uncompressed CD audio. At the bit rate of 115,200 bps that we'll be using, we can send and receive 11.52K bytes per second. This is fine for most purposes.

We'll be using a serial port (indirectly) to exchange data between the MB2 and your computer. Now you might be asking yourself: why exactly aren't we using RTT for this like we did before? RTT is a protocol that is meant to be used solely for debugging. You will not find devices that use RTT to communicate with other devices. However, serial communication is used quite often. For example, some GPS receivers send the position information they receive via serial. In addition RTT, like many debugging protocols, is slow compared to serial transfer rates.

<p align="center">
<img class="white_bg" height="500" title="Serial" src="../assets/serial.svg" />
</p>

Today's computers don't usually have a serial port, and even if they do the voltage they use (+5V on a modern serial port, ±12V on an ancient RS-232 port) is outside the range that the MB2 hardware will accept and may result in damaging it. *You can't directly connect your computer to the microcontroller.* 

<a href="https://en.wikipedia.org/wiki/File:UART_to_USB_adapter.jpg">
<p align="center">
<img height="240" title="UART To USB Adapter" src="https://upload.wikimedia.org/wikipedia/commons/2/24/UART_to_USB_adapter.jpg" />
</p>
</a>

You *can* buy inexpensive (typically under US$5) USB←→serial converters that will support the +3.3V inputs of most modern microcontroller boards. The board shown above is a common one that I use regularly. We will be talking to the MB2 serial port through the MB2's built-in USB port. However, if you want to connect directly to a hardware serial port, on the MB2 or some other board, a serial converter is the way to go.

A separate USB channel on the MB2's USB port can be used to talk to the MB2's built-in USB←→serial converter. (This is the right-hand path in the figure above.) This USB←→serial conversion is implemented using the "[communications microcontroller]" of the MB2: the communications microcontroller exposes a serial interface to the microcontroller and a virtual USB serial interface to your computer. The computer presents a virtual serial interface via the USB CDC-ACM ("Communications Device Class - Abstract Control Model", ugh) device class. The MB2 microcontroller will see your computer as a device connected to its hardware serial port; your computer will see the MB2 serial port as a virtual serial device.

Now, let's get familiar with the USB serial port interface that your OS offers. Pick a route:

- [Linux/UNIX](nix-tooling.md)
- [Windows](windows-tooling.md)

For MacOS check out the Linux documentation, although your experience may differ somewhat.

[serial port]: https://en.wikipedia.org/wiki/Serial_port
[communications microcontroller]: ../05-meet-your-software/flash-it.md
