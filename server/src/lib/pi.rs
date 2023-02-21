use rppal::gpio::Gpio;

fn initialize_output_pin(pin: u16) -> Gpio {
    let gpio = Gpio::new().unwrap();
    let pin = gpio.get(pin).unwrap().into_output();

    return pin;
}
