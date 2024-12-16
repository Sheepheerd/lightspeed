# main.py
from fabric import Application
from widgets.double_letter_button import DoubleLetterButton
from widgets.overlay import Overlay
from gi.repository import Gdk
import time
import subprocess
import tkinter as tk

first_key = None
second_key = None


def click():
    time.sleep(0.1)  # Small delay before click
    subprocess.run(["ydotool", "click", "0xC0"], check=True)


def move_mouse(x, y):
    subprocess.run(["hyprctl", "dispatcher", "movecursor", str(x), str(y)], check=True)


def on_key_press(window, event, *_):
    global first_key, second_key  # Ensure we're using global variables

    if event.keyval == Gdk.KEY_Escape:
        print("Quitting")
        exit()

    if first_key is None:
        first_key = event.keyval
        print(f"First key pressed: {chr(first_key)}")
        return  # Exit to avoid handling second key in this function

    if second_key is None:
        second_key = event.keyval
        print(f"Second key pressed: {chr(second_key)}")

    if first_key is not None and second_key is not None:
        label_to_match = chr(first_key) + chr(second_key)
        print(f"Matching label: {label_to_match}")

        widget = window.get_children()[0]  # This is the main container
        box = widget.get_children()[0]  # This is where your grid of buttons is

        for row_index, row in enumerate(box.get_children()):
            # Iterate through each button in the current row
            for col_index, double_letter_button in enumerate(row.get_children()):
                if isinstance(double_letter_button, DoubleLetterButton):
                    if double_letter_button.get_label() == label_to_match:
                        print(
                            f"Button found at row {row_index}, col {col_index} with label {label_to_match}"
                        )
                        move_mouse(double_letter_button.x, double_letter_button.y)
                        print(double_letter_button.x, double_letter_button.y)


if __name__ == "__main__":

    horizontal_size = tk.Tk().winfo_screenwidth()
    vertical_size = tk.Tk().winfo_screenheight()

    window = Overlay(horizontal_size, vertical_size)

    window.set_opacity(0.5)
    window.set_size_request(horizontal_size, vertical_size)

    window.set_keep_above(True)

    window.set_can_focus(True)

    window.connect("key-press-event", on_key_press)

    app = Application("default", window)
    app.run()
