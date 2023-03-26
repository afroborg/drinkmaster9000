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

## Getting started

### Prerequisites

- [Rust](https://www.rust-lang.org)
- [Node.js](https://nodejs.org)
- [Pnpm](https://pnpm.io)
- [Tailscale](https://tailscale.com/)

Mac users can install all of these using [Homebrew](https://brew.sh/).

```sh
brew install rust node pnpm tailscale
```

### Setup

> This installation guide requires manuall invitation to the Tailscale network, as well as a custom `hosts` file configuration.

1. Clone the repo
   ```sh
   git clone https://github.com/afroborg/drinkmixer9000.git
    ```
2. Web interface setup
    
    In the `client` directory:
    1. If you don't already have `pnpm` installed, install it
        ```sh
        npm install -g pnpm
        ```
    2. Install dependencies
        ```sh
        pnpm install
        ```
    3. Build the web interface (optional)
        ```sh
        pnpm build
        ```

3. Backend setup
    
    In the `pi` directory:
    1. Install aarch64-unknown-linux-gnu rust target
        ```sh
        rustup target add aarch64-unknown-linux-gnu
        ```
    2. Build the backend (optional)
        ```sh
        cargo build --release --target aarch64-unknown-linux-gnu
        ```

4. Set up Tailscale VPN
    1. Install [Tailscale](https://tailscale.com/)
    2. Get an invite to the `pi` node
    3. Activate tailscale on your machine

5. Set up your `hosts` file
    1. Get the IP address of the `pi` node
    2. Add the following line to your `/etc/hosts` file:
        ```
        <IP address> pi
        ```
6. Set up you SSH key on the `pi` node
    1. Get the IP address of the `pi` node
    2. Add your SSH key to the `pi` node
        ```sh
        ssh-copy-id pi@pi
        ```
7. Run the [`deploy`](./deploy.sh) script
    ```sh
    ./deploy.sh
    ```
    Deployment can be customized by providing arguments to the script.
    ```sh
    ./deploy.sh # deploy both frontend and backend
    ./deploy.sh frontend # only deploy client, leaving the backend untouched
    ./deploy.sh pi # only deploy backend, leaving the frontend untouched
    ./deploy.sh pi config # deploy the backend, as well as the current config.ron
    ```