/*
 *
 *   Copyright (C) 2022 Erich S. Heinzle
 *   
 *   drawString and drawSprite routines based on examples by Bartosz Bielawski
 *   included in LEDMatrixDriver library
 *
 *   This program is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 2 of the License, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License at <http://www.gnu.org/licenses/> 
 *   for more details.
 *
 */
 
#include "Arduino.h"
#include <LEDMatrixDriver.hpp>
#include "Adafruit_Sensor.h"
#include "Adafruit_AM2320.h"
#include "Adafruit_CCS811.h"
#include "RTClib.h"
//#include "Wire.h"
#include "SD.h"

// Pin 10 CS required by SD card code
#define CHIP_SELECT 10

// Uncomment according to your hardware type
#define HARDWARE_TYPE MD_MAX72XX::FC16_HW
//#define HARDWARE_TYPE MD_MAX72XX::GENERIC_HW

// Defining size, and output pins
#define MAX_DEVICES 4

const uint8_t LEDMATRIX_CS_PIN = 9;

const int LEDMATRIX_SEGMENTS = 4;
const int LEDMATRIX_WIDTH = LEDMATRIX_SEGMENTS * 8;

Adafruit_AM2320 am2320 = Adafruit_AM2320();
RTC_DS1307 RTC;
Adafruit_CCS811 ccs811;

// The LEDMatrixDriver class instance
LEDMatrixDriver lmd(LEDMATRIX_SEGMENTS, LEDMATRIX_CS_PIN);


void setup()
{
  pinMode(CHIP_SELECT, OUTPUT);
  pinMode(A0, INPUT_PULLUP);
  pinMode(A1, INPUT_PULLUP);
  pinMode(A2, INPUT_PULLUP);
  pinMode(A3, INPUT_PULLUP);

  lmd.setEnabled(true);
  lmd.setIntensity(0);

  // start sensors
  am2320.begin();
  ccs811.begin();
  // wait for CCS811 startup
  while(!ccs811.available());

  //Wire.begin();
  // check if RTC is working
  RTC.begin();
  if (!RTC.isrunning()) {
    ;
  }
}

/**
 * This draws a sprite to the given position using the width and height supplied (usually 8x8)
 */
void drawSprite(LEDMatrixDriver *lmd, byte* sprite, int x, int y, int width, int height )
{
  // The mask is used to get the column bit from the sprite row
  byte mask = B10000000;

  for( int iy = 0; iy < height; iy++ )
  {
    for( int ix = 0; ix < width; ix++ )
    {
      lmd->setPixel(x + ix, y + iy, (bool)(sprite[iy] & mask ));

      // shift the mask by one pixel to the right
      mask = mask >> 1;
    }

    // reset column mask
    mask = B10000000;
  }
}

/**
 * This function draws a string of the given length to the given position.
 */
void drawString(LEDMatrixDriver *lmd, char* text, int len, int x, int y )
{
  byte font[95][8] = {
                     {0,0,0,0,0,0,0,0}, // SPACE
                     {0x10,0x18,0x18,0x18,0x18,0x00,0x18,0x18}, // EXCL
                     {0x28,0x28,0x08,0x00,0x00,0x00,0x00,0x00}, // QUOT
                     {0x00,0x0a,0x7f,0x14,0x28,0xfe,0x50,0x00}, // #
                     {0x10,0x38,0x54,0x70,0x1c,0x54,0x38,0x10}, // $
                     {0x00,0x60,0x66,0x08,0x10,0x66,0x06,0x00}, // %
                     {0,0,0,0,0,0,0,0}, // &
                     {0x00,0x10,0x18,0x18,0x08,0x00,0x00,0x00}, // '
                     {0x02,0x04,0x08,0x08,0x08,0x08,0x08,0x04}, // (
                     {0x40,0x20,0x10,0x10,0x10,0x10,0x10,0x20}, // )
                     {0x00,0x10,0x54,0x38,0x10,0x38,0x54,0x10}, // *
                     {0x00,0x08,0x08,0x08,0x7f,0x08,0x08,0x08}, // +
                     {0x00,0x00,0x00,0x00,0x00,0x18,0x18,0x08}, // COMMA
                     {0x00,0x00,0x00,0x00,0x7e,0x00,0x00,0x00}, // -
                     {0x00,0x00,0x00,0x00,0x00,0x00,0x06,0x06}, // DOT
                     {0x00,0x04,0x04,0x08,0x10,0x20,0x40,0x40}, // /
                     {0x00,0x38,0x44,0x4c,0x54,0x64,0x44,0x38}, // 0
                     {0x00,0x0c,0x14,0x04,0x04,0x04,0x04,0x04}, // 1
                     {0x00,0x30,0x48,0x04,0x04,0x38,0x40,0x7c}, // 2
                     {0x00,0x38,0x44,0x04,0x18,0x04,0x44,0x38}, // 3
                     {0x00,0x04,0x0c,0x14,0x24,0x7e,0x04,0x04}, // 4
                     {0x00,0x7c,0x40,0x40,0x78,0x04,0x04,0x78}, // 5
                     {0x00,0x38,0x40,0x40,0x78,0x44,0x44,0x38}, // 6
                     {0x00,0x7c,0x04,0x04,0x08,0x08,0x10,0x10}, // 7
                     {0x00,0x3c,0x44,0x44,0x38,0x44,0x44,0x78}, // 8
                     {0x00,0x38,0x44,0x44,0x3c,0x04,0x04,0x78}, // 9
                     {0x00,0x00,0x18,0x18,0x00,0x18,0x18,0x00}, // :
                     {0x00,0x18,0x18,0x00,0x00,0x18,0x18,0x08}, // ;
                     {0x00,0x10,0x20,0x40,0x80,0x40,0x20,0x10}, // <
                     {0x00,0x00,0x7e,0x00,0x00,0xfc,0x00,0x00}, // =
                     {0x00,0x08,0x04,0x02,0x01,0x02,0x04,0x08}, // >
                     {0x00,0x38,0x44,0x04,0x08,0x10,0x00,0x10}, // ?
                     {0x00,0x30,0x48,0xba,0xba,0x84,0x78,0x00}, // @
                     {0x00,0x1c,0x22,0x42,0x42,0x7e,0x42,0x42}, // A
                     {0x00,0x78,0x44,0x44,0x78,0x44,0x44,0x7c}, // B
                     {0x00,0x3c,0x44,0x40,0x40,0x40,0x44,0x3c}, // C
                     {0x00,0x7c,0x42,0x42,0x42,0x42,0x44,0x78}, // D
                     {0x00,0x78,0x40,0x40,0x70,0x40,0x40,0x7c}, // E
                     {0x00,0x7c,0x40,0x40,0x78,0x40,0x40,0x40}, // F
                     {0x00,0x3c,0x40,0x40,0x5c,0x44,0x44,0x78}, // G
                     {0x00,0x44,0x44,0x44,0x7c,0x44,0x44,0x44}, // H
                     {0x00,0x7c,0x10,0x10,0x10,0x10,0x10,0x7e}, // I
                     {0x00,0x7e,0x02,0x02,0x02,0x02,0x04,0x38}, // J
                     {0x00,0x44,0x48,0x50,0x60,0x50,0x48,0x44}, // K
                     {0x00,0x40,0x40,0x40,0x40,0x40,0x40,0x7c}, // L
                     {0x00,0x82,0xc6,0xaa,0x92,0x82,0x82,0x82}, // M
                     {0x00,0x42,0x42,0x62,0x52,0x4a,0x46,0x42}, // N
                     {0x00,0x3c,0x42,0x42,0x42,0x42,0x44,0x38}, // O
                     {0x00,0x78,0x44,0x44,0x48,0x70,0x40,0x40}, // P
                     {0x00,0x3c,0x42,0x42,0x52,0x4a,0x44,0x3a}, // Q
                     {0x00,0x78,0x44,0x44,0x78,0x50,0x48,0x44}, // R
                     {0x00,0x38,0x40,0x40,0x38,0x04,0x04,0x78}, // S
                     {0x00,0x7e,0x10,0x10,0x10,0x10,0x10,0x10}, // T
                     {0x00,0x42,0x42,0x42,0x42,0x42,0x42,0x3e}, // U
                     {0x00,0x42,0x42,0x42,0x42,0x44,0x28,0x10}, // V
                     {0x80,0x82,0x82,0x92,0x92,0x92,0x94,0x78}, // W
                     {0x00,0x42,0x42,0x24,0x18,0x24,0x42,0x42}, // X
                     {0x00,0x44,0x44,0x28,0x10,0x10,0x10,0x10}, // Y
                     {0x00,0x7c,0x04,0x08,0x7c,0x20,0x40,0xfe}, // Z
                      // (the font does not contain any lower case letters. you can add your own.)
  };

  for( int idx = 0; idx < len; idx ++ )
  {
    int c = text[idx] - 32;

    // stop if char is outside visible area
    if( x + idx * 6  > LEDMATRIX_WIDTH )
      return;

    // only draw if char is visible
    if( 8 + x + idx * 6 > 0 )
      drawSprite(lmd, font[c], x + idx * 6, y, 8, 8 );
  }
}

void adjustTime(LEDMatrixDriver *lmd, RTC_DS1307 *rtc, DateTime *dt) {
  int min_orig = dt->minute();
  int hr_orig = dt->hour();
  uint32_t ticks = dt->secondstime();
  int hr = hr_orig;
  int mi = min_orig;
  char strbuf[6]; // hh:mm
  sprintf(strbuf, "Set..");
  int len = strlen(strbuf);
  drawString(lmd, strbuf, len, 0, 0);
  lmd->display();   // Toggle display of the new framebuffer
  delay(1000);
  while (analogRead(A0) < 50) {
    if (analogRead(A3) < 50 && analogRead(A1) < 50) {
      mi = (mi - 1)%60;
    } else if (analogRead(A3) < 50 && analogRead(A1) > 500) {
      mi = (mi + 1)%60;    
    } else if (analogRead(A2) < 50 && analogRead(A1) < 50) {
      hr = (hr - 1)%24;
    } else if (analogRead(A2) < 50 && analogRead(A1) > 500) {
      hr = (hr + 1)%24;    
    }
    delay(200);
    sprintf(strbuf, "%02d%c%02d", hr, (true ? ':' : ' '), mi);
    int len = strlen(strbuf);
    drawString(lmd, strbuf, len, 0, 0);
  }
  rtc->adjust(DateTime(ticks+3600*(hr-hr_orig) + 60*(mi-min_orig)));
  delay(200);
  *dt = rtc->now();
}

void loop()
{
  char strbuf[6]; // hh:mm
  int len;
  int current_minute;
  int previous_minute = 0;
  uint16_t co2;
  DateTime dt = RTC.now();

  // these variable are used for adjusting the error in the RTC
  // you will need to see how many seconds slow or fast it runs over an extended
  // period to calculate the number of seconds per day it runs fast or slow
  // in the prototype, it ran 8 min 26 seconds (= 506 seconds) fast over 88 days
  // i.e. 5.75 seconds per day fast
  uint32_t ticks_orig = dt.secondstime();
  uint32_t ticks_current = 0;
  uint16_t ticks_since_adjustment = 0;
  uint16_t ticks_per_adjustment = 15026; // if running 5.75 seconds fast per day
  int ticks_adjustment = 1;              // i.e 5.75 seconds every 86,400 seconds ( = day)
  
  int use_sensors = 1;
  int adjust_error = 1; // set this to 1 if you plan to adjust RTC drift error
  
  while (1)
  {
    dt = RTC.now();
    current_minute = dt.minute();
    ticks_current = dt.secondstime();
    if (adjust_error) {
      ticks_since_adjustment = (ticks_current - ticks_orig)%ticks_per_adjustment;
      if (ticks_since_adjustment >= (ticks_per_adjustment - 3)) { // allow a 2 second window
        RTC.adjust(DateTime(ticks_current - ticks_adjustment));
        delay(3000); // avoid endless readjustment in a loop
      }
    }
    delay(200);
    if (current_minute != previous_minute)
    {
      if (use_sensors) {
       sprintf(strbuf, "T: %02d", (int)am2320.readTemperature());
       len = strlen(strbuf);
       drawString(&lmd, strbuf, len, 0, 0);
       lmd.display();   // Toggle display of the new framebuffer
       delay(4000);
       sprintf(strbuf, "H: %02d", (int)am2320.readHumidity());
       len = strlen(strbuf);
       drawString(&lmd, strbuf, len, 0, 0);
       lmd.display();   // Toggle display of the new framebuffer
       delay(4000);
       if(ccs811.available()){
         if(!ccs811.readData()){
           co2 = (uint16_t)ccs811.geteCO2();
           if (co2 < 1000) {
             sprintf(strbuf, "C:%03d", co2);
           } else {
             sprintf(strbuf, "C%04d", co2);
           }
           len = strlen(strbuf);
           drawString(&lmd, strbuf, len, 0, 0);
           lmd.display();   // Toggle display of the new framebuffer
           delay(4000);
         }
        }
      }
      sprintf(strbuf, "%02d%c%02d", dt.hour(), (true ? ':' : ' '), current_minute);
      len = strlen(strbuf);
      drawString(&lmd, strbuf, len, 0, 0);
      lmd.display();   // Toggle display of the new framebuffer
      delay(5000);
      previous_minute = current_minute;
    }
    // now to see if the time needs adjusting
    // this only works if buttons are attached to A0...A3
    if (0 && analogRead(A0) < 50) {
      delay(50); // software debouncing 
      if (analogRead(A0) < 50) {
        sprintf(strbuf, "A0..");
        len = strlen(strbuf);
        drawString(&lmd, strbuf, len, 0, 0);
        lmd.display();
        delay(500);
        adjustTime(&lmd, &RTC, &dt);
        current_minute = dt.minute();
        sprintf(strbuf, "%02d%c%02d", dt.hour(), (true ? ':' : ' '), dt.minute());
        len = strlen(strbuf);
        drawString(&lmd, strbuf, len, 0, 0);
        lmd.display();   // Toggle display of the new framebuffer
      }
    }
  }
}
