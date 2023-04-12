print("framebuffer-display simple test...")

import board
import displayio
from framebuffer_display import FramebufferDisplay

display = FramebufferDisplay()
print("width: {}".format(display.width))
print("height: {}".format(display.height))
