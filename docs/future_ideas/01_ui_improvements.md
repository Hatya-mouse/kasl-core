Making able to create custom UI elements using `ui_attribute` syntax:

```
ui_attribute rgb_slider(r: Int, g: Int, b: Int, min: Float = 0.0, max: Float = 1.0) {
    // Slider drawing code here
}

#rgb_slider(50, 0, 256) // Use default value for min and max
input gain = 0.0
```
