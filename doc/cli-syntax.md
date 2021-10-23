`hue pair` to pair with a new bridge.

`hue on` turns all lights on to their last state.
`hue off` turns all lights off.
`hue on|off light-names...` turns the specified lights on|off.

`hue bri(ghtness)? [+-]?<int>` sets all lights' brightness to [+-]?<int>.
`hue bri(ghtness)? [+-]?<int> light-names...` sets the brightness of the specified lights to [+-]?<int>.

`hue col(or)? <color>` sets the color of all lights to <color>.
`hue col(or)? <color> light-names...` sets the color of the specified lights to <color>.

`hue scene|set <scene-name>` sets the <scene-name> scene.

<color> should allow for named values and hex values.
Maybe [pastel](https://github.com/sharkdp/pastel) offers a library.
