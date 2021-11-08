# hue

A command line application with short and intuitive syntax for controlling hue lights.

Works on Linux, BSD, MacOS and other Unix-like operating systems, and probably doesn't work on Windows (it's a feature `:^)`).
If you need it on Windows it should work under WSL.

Licensed under the GPLv3.


## Installation

`hue` isn't packaged yet, so you will need to build it from source.

To build it you will need [Rust](https://www.rust-lang.org/tools/install) and (optionally) `git` for cloning the repository.

```
$ git clone https://github.com/DominikRusso/hue.git
$ cargo install --path hue
$ hue --version
hue 0.1.0
```

See `cargo help install` for more information on installing.


## Usage

Use `hue help` to see the help message and `hue help <subcommand>` to see more detailed help on any one of the subcommands.

First you need to pair with your hue bridge.
You can do this by either setting the `HUE_IP` and `HUE_USER` environment variables to the bridge's IP address and a whitelisted user's ID respectively or using the `init` subcommand to find a bridge on the network, create a new user and store the information needed to authenticate to the bridge.

The `init` subcommand takes one positional argument which is the username of the new user.
The username you supply doesn't matter and you won't need it again, but it is stored inside the bridge's configuration.
I recommend you use your device's name.
```
$ hue init thinkpad
```
Then go press your bridge's link button within 120 seconds.

Note that the environment variables take precedence over the stored bridge IP and user ID. \
If your bridge's IP address keeps changing on your network consider giving it a static address.

After you have successfully paired you're ready to start controlling your lights.

`hue on` turns all lights on. \
`hue on <light-name>...` turns on the specified lights.

`hue off` turns all lights off. \
`hue off <light-name>...` turns off the specified lights.

`hue brightness <value>` sets all the active lights' brightness to `<value>`%. \
`hue brightness <value> <light-name>...` sets the specified lights' brightness to `<value>`%. \
If you didn't specify any lights you can pass `-a` or `--all` to turn all lights on and set their brightness in one command. \
You can abbreviate `brightness` with `bri`. \
You can prefix the `<value>` with either `+` or `-` to make the brightness change relative to the current brightness.

`hue color <color>` sets all the active lights' color to `<color>` \
`hue color <color> <light-name>...` sets the specified lights' color to `<color>` \
If you didn't specify any lights you can pass `-a` or `--all` to turn all lights on and set their color in one command. \
You can abbreviate `color` with `col`. \
The value of `<color>` can be a
* color name (`coral`)
* a hex color code (`ff7f50` or `'#ff7f50'`)
* an rgb triplet (`255,127,80` or `'rgb(255, 127, 80)'`)
* or an hsl triplet (`'hsl(16, 100.0%, 65.7%)'`).

`hue scene <scene-name>` sets the scene with the name `<scene-name>`. \
You can use `set` instead of `scene`.
