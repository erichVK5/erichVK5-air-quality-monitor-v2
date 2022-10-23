# erichVK5-air-quality-monitor-v2
A leaner version of the original erichVK5 air quality monitor

![assembled board](images/08-assembled-air-quality-monitor-v2.JPG)

The PCB and accompanying software allow the CCS811 sensor to be used to monitor CO2 and total volatile organic compounds, and the AM2320 sensor allows humidity and temperature to be recorded as well.

This is a leaner version of https://github.com/erichVK5/erichVK5-air-quality-monitor, achieving a significant cost reduction by using an Arduino nano and a compact Deek Robot 8122 RTC/SD board on a custom PCB which also mounts the DF Robot CCS811 CO2/VOC sensor board and the AM2320 temperature/humidity sensor. The board is code compatible with the original air quality monitor, except for the real tim clock, which uses a DS1307 instead of a PCF8523.

The revised PCB was designed in pcb-rnd and simplifies the connections between the TM1367 display, CCS811 sensor, AM2320 sensor and Arduino Nano.

![assembled board](images/PCB-top-side.JPG)
![assembled board](images/PCB-bottom-side.JPG)

Low cost data logging air quality monitors remain useful in educational and community settings to provide a qualitative indication of air quality over time. In particular, CO2 levels can be a useful proxy for air turnover in built environments, and by extension, inform measures that seek to reduce the risk of persisting airborne pathogens. The utility of CO2 as a proxy for airborne pathogen loads will be affected by sources of combustion or fermentation (i.e. processes producing CO2 in addition to people) in the indoor space, and by the use of air purifying devices, such as those circulating air through suitable filters.

The PCB has a footprint allowing connection to an inexpensive TM1367 based four digit LED display for the display of readings, but with suitable code modification, other display options are possible.

After assembly of the PCB headers and sub-boards is complete, a battery can be installed on the Deek Robot 8122 datalogging board. The DS1307 clock example code available within the arduino library manager can then be used to set the current time.

In addition to taking and displaying readings every 10 seconds, the accompanying code will write time stamped readings to an SD card every three minutes.

After reset, if all goes well, an initial flash screen "LLS" should appear, after which readings will be displayed sequentially, with new readings being taken every 10 seconds. Every three minutes, cached readings are written to the SD card.

The Deek Robot 8122 datalogging board requires a CR1220 battery for the DS1307 real time clock, and will require a FAT16 or FAT32 formatted SD card of up between 32MB and 32GB in size.

The unit should be powered down before inserting or removing the SD card.

Data logs are written in CSV format, with sequential naming starting with LLSAQM00.CSV

![data from the SD card](images/LLS-data-analysis.jpg)

During operation, updated readings are displayed sequentially on the 4 digit LED display.

- Temperature readings in Centigrade are preceded by "t" on the LED display
- Humidity readings in percent are preceded by "h" on the LED display
- Carbon dioxide readings in ppm (parts per million) are preceded by "CO2" on the LED display
- Total volatile organic compound readings in ppb (parts per billion) are preceded by "tvoc" on the LED display

The code also streams results over USB as a serial device, allowing readings to be viewed as they are collected in a serial monitor in real time.

It is recommended that the CCS811 be given a 48 hour initial burn in period and be allowed 20 minutes for the sensor to warm up to provide reliable readings thereafter.

The v2 PCB layout can be opened in pcb-rnd, and gerbers for the shield have been provided in a zip file to allow ordering from the usual online PCB suppliers.

Here is the schematic for the shield when using the CCS811, AM2320 and TM1637 four digit seven segment display

![Shield schematic using TM1637 display](images/LLS-shield-TM1637-schematic-v1.png)

For data transfers to and from the SD card, the Deek Robot Datalogging Board uses SPI on the following Arduino Uno pins:

- SCK
- MISO
- MOSI
- D10 as CS (chip select)

For real time clock operations, the datalogging shield also shares the i2c bus with with the CCS811 and AM2320 sensors, using the following pins:

- SDA(A4)
- SCL(A5)

This leaves multiple pins free on the Arduino Nano if code customisation is needed, such as for on/off signals for fans or building ventilation systems. Spare pins on the arduino nano have been broken out to pads, to allow wiring to other actuators, sensors or switches.

The PCB also has a footprint to allow the use of a MAX31855 thermocouple, if builders are keen to experiment with temperature measurement, display and control of various hot processes.