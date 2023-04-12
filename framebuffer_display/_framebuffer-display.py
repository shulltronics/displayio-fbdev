# A class to use the linux framebuffer as a DisplayIO "display"
# By Shulltronics, March 2023

import sys
import time
from PIL import Image
import displayio

from dataclasses import astuple

# Our pygame "display" doesn't need any init sequence
_INIT_SEQUENCE = None

class FramebufferDisplay(displayio.Display):

    def __init__(self, _width, _height, **kwargs):
        self.running = True
        self._width = _width
        self._height = _height
        # initialize the super class, displayio.Display.
        #   DisplayBus = None
        super().__init__(None, _INIT_SEQUENCE, width=_width, height=_height, **kwargs)

    def _initialize(self, init_sequence):
        # TODO
        # pygame.init()
        # pygame.display.set_caption("Shulltronics Displayio PyGame Testing")
        # self._pygame_screen = pygame.display.set_mode((self._width, self._height))
    

    def refresh(self, *, target_frames_per_second=60, minimum_frames_per_second=1):
        """
        When auto refresh is off, waits for the target frame rate and then refreshes the
        display, returning True. If the call has taken too long since the last refresh call
        for the given target frame rate, then the refresh returns False immediately without
        updating the screen to hopefully help getting caught up.
        If the time since the last successful refresh is below the minimum frame rate, then
        an exception will be raised. Set minimum_frames_per_second to 0 to disable.
        When auto refresh is on, updates the display immediately. (The display will also
        update without calls to this.)
        """

        if self.running:
            self._subrectangles = []

            # Go through groups and and add each to buffer
            if self._core._current_group is not None:

                buffer = Image.new("RGBA", (self._core._width, self._core._height))
                # Recursively have everything draw to the image
                # pylint: disable=protected-access
                self._core._current_group._fill_area(
                    buffer
                )  # pylint: disable=protected-access
                # save image to buffer (or probably refresh buffer so we can compare)
                self._buffer.paste(buffer)

            self._subrectangles = self._core.get_refresh_areas()

            for area in self._subrectangles:
                self._refresh_display_area(area)


    def _refresh_display_area(self, rectangle):
        """Loop through dirty rectangles and redraw that area."""

        img = self._buffer.convert("RGB").crop(astuple(rectangle))
        img = img.rotate(self._rotation, expand=True)
        display_rectangle = self._apply_rotation(rectangle)
        img = img.crop(astuple(self._clip(display_rectangle)))
        raw_str = img.tobytes("raw", "RGB")
        # TODO --> write raw_str to the framebuffer
        # print("({}, {})".format(img.width, img.height))

    def get_mouse_clicks(self):
        event_return = None
        # TODO --> figure how to get keyboard / touchscreen / gamepad inputs 
        # without a display server
        return event_return

    def quit(self):
        """
        Close the program!
        """
        print("Closing the program... goodbye!")
        self.running = False
