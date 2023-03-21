from fastapi import FastAPI
import RPi.GPIO as GPIO
import time
GPIO.setmode(GPIO.BCM)
GPIO.setup(18,GPIO.OUT)

app = FastAPI()

@app.post("/")
async def toggle():
    GPIO.output(32, True)

    return {
    }