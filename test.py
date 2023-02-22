# test script for checking that GPIO pin works

import RPi.GPIO as GPIO 
from time import sleep
GPIO.setmode(GPIO.BCM)
GPIO.setup(18, GPIO.OUT)

PIN = 18;

try:
    while True:
        GPIO.output(PIN, 1)
        sleep(0.5)
        GPIO.output(PIN, 0)
        sleep(0.5)

except:
    GPIO.cleanup()