#include "esp_adc_cal.h" 

#define DAC1 25
#define DAC2 26

#define ADCCAL 32
#define ADC1 34
#define ADC2 35

#define H 1000.0

/*Code for Digital calibration to get Precise Readings*/
uint32_t readADC_Cal(int ADC_Raw)
{
esp_adc_cal_characteristics_t adc_chars;
esp_adc_cal_characterize(ADC_UNIT_1, ADC_ATTEN_DB_11, ADC_WIDTH_BIT_12, 1100, &adc_chars);
return(esp_adc_cal_raw_to_voltage(ADC_Raw, &adc_chars));
}


double vRef;

void setup() {
  Serial.begin(115200);
  vRef=readADC_Cal(analogRead(ADCCAL))/1000.0;
  Serial.println("I'm double integrator");
  
}


double double_integrator(double un,double h){

      double x1n,x2n;
      static double x1n1=0.0,x2n1=0.0;    
    
     
        
    //Euler method
    x1n=x1n1+h*x2n1;
    x2n=x2n1+h*un;
    
    
    x1n1=x1n;
    x2n1=x2n;
    
    return x1n;
}

void loop() { 

  unsigned long start_t,stop_t;
  int adcValue1,adcValue2;
  uint8_t dacValue1,dacValue2;
  double u,y;
  
  start_t=micros();


  adcValue1=analogRead(ADC1);
  adcValue2=analogRead(ADC2);
  
  u=vRef*(double)(adcValue1-adcValue2)/4095.0;

  y=double_integrator(u,H/1e6);

  if(y>vRef)
     y=vRef;
  if(y<-vRef)
     y=-vRef;

  if(y>=0){
    dacValue1=y/vRef*255.0;
    dacValue2=0;

    
  }
  else{
    dacValue2=-y/vRef*255.0;
    dacValue1=0;

  }
  
   
  dacWrite(DAC1, dacValue1);
  dacWrite(DAC2, dacValue2);
  stop_t=micros();


  //Serial.printf("%lf,%lf,%d,%d,%lf,%ld\n",u,y,dacValue1,dacValue2,vRef,(stop_t-start_t));
  

  delayMicroseconds(H-(stop_t-start_t));  
}
