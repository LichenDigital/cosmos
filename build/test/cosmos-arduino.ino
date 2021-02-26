int led_pin = 13;

struct timer {
  unsigned long time;
  unsigned long durration;
};


int check_timer (struct timer timer) {
  if (millis() - timer.time >= timer.durration) {
    return 1;
  }
  else {
    return 0;
  }
}

struct timer register_timer (unsigned long time, unsigned long durration) {
  struct timer timer;
  timer.time = time;
  timer.durration = durration;
  return timer;
}

void setup(void) {
  // Set pinmodes
  pinMode(led_pin, OUTPUT); // Built in LED, used for displaying when data has been recieved
  
  Serial.begin(115200);
}

void loop(void) {

  int input = Serial.available();

  if (input > 0) {
    int val  = Serial.parseInt();
    Serial.println(val);
  }
  
}
