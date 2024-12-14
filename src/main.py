from fabric import Application
from fabric.widgets.box import Box
from fabric.widgets.label import Label
from fabric.widgets.button import Button
from fabric.widgets.datetime import DateTime
from fabric.widgets.centerbox import CenterBox
from fabric.widgets.wayland import WaylandWindow as Window
from gi.repository import Gdk
import pyautogui


def create_button():  # define a "factory function"
    return Button(
        label="Click Me",
        size=500,
        on_clicked=lambda b, *_: b.set_label("you clicked me"),
    )


class StatusBar(Window):
    def __init__(self, **kwargs):
        super().__init__(
            layer="overlay",
            anchor="center",
            exclusivity="none",
            keyboard_mode="exclusive",
            **kwargs
        )

        self.date_time = DateTime()
        self.children = CenterBox(
            Box(
                orientation="v",
                children=[
                    Label(label="Fabric Buttons Demo (Box 1)"),
                    Box(
                        orientation="h",
                        children=[
                            create_button(),
                            create_button(),
                            create_button(),
                            create_button(),
                        ],
                    ),
                ],
            )
        )


def on_key_press(window, event, *_):  # key press event handler
    if event.keyval == Gdk.KEY_a:  # Check if the "a" key is pressed
        print("The 'a' key was pressed!")
        pyautogui.moveTo(100, 100, duration=0.5)
        # Change labels of all buttons to "You pressed 'a'"
        for widget in window.get_children():
            for button in widget.get_children():
                if isinstance(button, Button):
                    button.set_label("You pressed 'a'")
    elif event.keyval == Gdk.KEY_b:
        pyautogui.moveTo(100, 200, duration=0.5)
    elif event.keyval == Gdk.KEY_Escape:
        print("GoodBye")
        exit()


if __name__ == "__main__":

    # Create a window
    window = StatusBar()

    # Set the window to a specific size of 1920x1080
    window.set_default_size(1920, 1080)

    # Set window opacity
    window.set_opacity(0.5)

    # Make the window float above other windows (keep it above)
    window.set_keep_above(True)

    # Explicitly request focus (make sure this method is available)
    window.set_can_focus(True)
    # Connect the key press event handler to the window
    window.connect("key-press-event", on_key_press)

    # Run the application
    app = Application("default", window)
    app.run()
