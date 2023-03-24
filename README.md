# Drinkmaster 9000

> _Name is a work in progress_ (maybe)

## What is this?

This is a school project to create a drink dispenser that can be controlled via a web interface. It is designed to be used with a Raspberry Pi. The dispenser is controlled via servos. The web interface is written in [Astro](https://astro.build/) and the backend is written in [Rust](https://www.rust-lang.org/).

## Hardware
These are the parts that are used in this project:

- Raspberry Pi 4 model B (should work with other models as well)
- 3x Servo motor, 9kg/cm
- 1x buck converter (12V to 5V)
- 1x 12V power supply
- 3d printed parts (see [3d models](#3d-models))

## Software
The software is split into two parts: the web interface and the backend.

### Web interface

The web interface is written in Astro. It is a static site that is served by the backend. It is compiled to HTML, CSS and JS at build time. Its primary purpose is to display the current state of the dispenser, allow the user to control it and update some paramters.

> Note that there are no authorization mechanisms in place. Anyone with access to the web interface can control the dispenser.

### Backend

The backend is written in Rust, with the [Actix](https://actix.rs/) web framework. It is responsible for serving the web interface, as well as providing API routes that in turn control the GPIO outputs and dispenser. It also provides a websocket connection that is used to send updates to the web interface.

#### API routes
The backend provides the following API routes:

- `GET /api/drinks` - Get a list of the current drinks in the dispenser
- `POST /api/drinks` - Set the list of drinks in the dispenser
- `POST /api/drinks/make` - Make a drink by giving a JSON list of ingredients

- `GET /api/dispenser` - Get the current state of the dispenser
- `POST /api/dispenser` - Set the state of the dispenser
- `POST /api/dispenser/pusher/angle/{angle}` - Set the servo angle of all dispenser pushers
- `POST /api/dispenser/pusher/{index}/angle/{angle}` - Set the servo angle of a specific dispenser pusher
- `POST /api/dispenser/cup/angle/{angle}` - Set the servo angle of the cup holder


## 3d models
> Not finalized
