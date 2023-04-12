print("framebuffer-display test...")

import board
import displayio
from framebuffer_display import FramebufferDisplay

display = FramebufferDisplay()
# display.set_orientation(90)
print("width: {}".format(display.width))
print("height: {}".format(display.height))

# Create a bitmap with two colors
bitmap = displayio.Bitmap(display.width, display.height, 2)

# Create a two color palette
palette = displayio.Palette(5)
palette[0] = 0x000000
palette[1] = 0x0000ff # red
palette[2] = 0x00ff00 # green
palette[3] = 0xff0000 # blue
palette[4] = 0xffffff # white

# Create a TileGrid using the Bitmap and Palette
tile_grid = displayio.TileGrid(bitmap, pixel_shader=palette)

# Create a Group
group = displayio.Group()

# Add the TileGrid to the Group
group.append(tile_grid)

# Add the Group to the Display
display.show(group)

# Draw a pixel
bitmap[0, 0] = 1
bitmap[10, 0] = 2
bitmap[0, 10] = 3
bitmap[10, 10] = 4

# Draw even more pixels
for x in range(150, 170):
    for y in range(100, 110):
        bitmap[x, y] = 1

# Loop forever so you can enjoy your image
while True:
    pass
