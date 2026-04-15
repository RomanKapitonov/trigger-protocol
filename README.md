# trigger-protocol

`trigger-protocol` defines the fixed-size SPI wire frame exchanged between the
trigger source MCU and the STM32F429 firmware.

It provides the frame constants, the packed trigger-event layout, and
byte-reinterpretation helpers used on both sides of the transport.
