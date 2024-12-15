from fabric.widgets.box import Box
from fabric.widgets.wayland import WaylandWindow as Window
from widgets.double_letter_button import DoubleLetterButton


def create_double_letter_buttons():
    """Create a 26x26 grid of buttons with labels 'aa' to 'zz'."""
    buttons = []

    # calculate the button width and height for rectangular buttons
    horizontal = (1920 // 26) + 1
    vertical = (1080 // 26) - 1

    for i in range(26):
        for j in range(26):
            label = f"{chr(97 + i)}{chr(97 + j)}"

            buttons.append(
                DoubleLetterButton(
                    i * horizontal,
                    j * vertical,
                    label=label,
                    size=[horizontal, vertical],
                    on_clicked=lambda b, label=label, *_: b.set_label(
                        f"Button {label}"
                    ),
                )
            )

    return buttons


class Overlay(Window):
    def __init__(self, **kwargs):
        super().__init__(
            layer="overlay",
            anchor="top",
            exclusivity="none",
            keyboard_mode="exclusive",
            **kwargs,
        )

        button_grid = create_double_letter_buttons()

        self.children = Box(
            orientation="v",
            children=[
                Box(
                    orientation="h",
                    children=[
                        Box(orientation="v", children=button_grid[i : i + 26])
                        for i in range(0, len(button_grid), 26)
                    ],
                ),
            ],
        )
