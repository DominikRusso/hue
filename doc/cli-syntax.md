`hue init` to pair with a new bridge.

`hue (on|off) <light-name>*` turns the specified lights, or all of them if none are specified, (on to their last state|off).

`hue bri(ghtness)? [+-]?<int> <light-name>*` sets the brightness of the specified lights, or all of them if none are specified, to [+-]?<int>.

`hue col(or)? <color> <light-name>*` sets the color of the specified lights, or all of them of none are specified, to <color>.

`hue (scene|set) <scene-name>` sets the <scene-name> scene.

`hue create-scene <scene-name> <light-name>*` creates a scene with the current state of the specified lights, or all of them if none are specified, called <scene-name>.

<color> should allow for named values and hex values.
Maybe [pastel](https://github.com/sharkdp/pastel) offers a library.
