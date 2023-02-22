from fastapi import FastAPI
import RPi.GPIO as GPIO
import time
GPIO.setmode(GPIO.BCM)
GPIO.setup(18,GPIO.OUT)

app = FastAPI()

@app.post("/")
async def toggle():
    state = not GPIO.input(18);
    GPIO.output(18, state)

    return {
        "active": state
    }