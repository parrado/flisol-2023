#include "esp_adc_cal.h"

#define DAC1 25
#define DAC2 26

#define ADCCAL 32
#define ADC1 34
#define ADC2 35

#define LED 2

/*Code for Digital calibration to get Precise Readings*/
uint32_t readADC_Cal(int ADC_Raw)
{
  esp_adc_cal_characteristics_t adc_chars;
  esp_adc_cal_characterize(ADC_UNIT_1, ADC_ATTEN_DB_11, ADC_WIDTH_BIT_12, 1100, &adc_chars);
  return (esp_adc_cal_raw_to_voltage(ADC_Raw, &adc_chars));
}


double vRef;

void setup() {
  Serial.begin(115200);
  vRef = readADC_Cal(analogRead(ADCCAL)) / 1000.0;
  pinMode (LED, OUTPUT);

}



void loop() {


  int adcValue1, adcValue2;
  uint8_t dacValue1, dacValue2;
  static double u, y;
  uint8_t command;
  uint8_t n_bytes,read_bytes;

  while (true){
    if(Serial.available()>0)
     break;
  }
  command = Serial.read();
  digitalWrite(LED, !digitalRead(LED));







  switch (command) {
    case 0:

      n_bytes=0;
      while(true){
        if(Serial.available()>0){
          *(((uint8_t*)&u)+n_bytes)=Serial.read();
          n_bytes++;
          if(n_bytes==8)
           break;
        }
        
          
        }
      
    
      if (u > vRef)
        u = vRef;
      if (u < -vRef)
        u = -vRef;

      if (u >= 0) {
        dacValue1 = u / vRef * 255.0;
        dacValue2 = 0;
      }
      else {
        dacValue2 = -u / vRef * 255.0;
        dacValue1 = 0;
      }

      dacWrite(DAC1, dacValue1);
      dacWrite(DAC2, dacValue2);

      break;

    case 1:



      adcValue1 = analogRead(ADC1);
      adcValue2 = analogRead(ADC2);

      y = vRef * (double)(adcValue1 - adcValue2) / 4095.0;

      
      Serial.write((uint8_t*)&y, sizeof(double));


      break;
  }






}
