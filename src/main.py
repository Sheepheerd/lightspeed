# main.py
from fabric import Application
from widgets.overlay import Overlay
from gi.repository import Gdk
import subprocess
import tkinter as tk

first_key = None
second_key = None
first_pass = True
previous_location_x = None
previous_location_y = None

key_associations = {
    "q": (1, 3),
    "w": (3, 3),
    "e": (5, 3),
    "r": (7, 3),
    "a": (1, 7),
    "s": (3, 7),
    "d": (5, 7),
    "f": (7, 7),
    "z": (1, 11),
    "x": (3, 11),
    "c": (5, 11),
    "v": (7, 11),
}


def click():
    subprocess.run(["ydotool", "click", "0xC0"], check=True)


def move_mouse(x, y):
    subprocess.run(["hyprctl", "dispatcher", "movecursor", str(x), str(y)], check=True)


def on_key_press(window, event, *_):
    global first_key, second_key  # Ensure we're using global variables
    global previous_location_x, previous_location_y  # Ensure we're using global variables
    global first_pass

    if event.keyval == Gdk.KEY_Escape:
        print("Quitting")
        exit()

    if first_key is None:
        first_key = event.keyval
        print(f"First key pressed: {chr(first_key)}")

        if first_pass:
            return
        else:
            letter_button_x = int(first_key) - 97
            m_x, m_y = key_associations[chr(first_key)]
            x, y = window.get_size()
            x = (x // 26) // (4) // 2
            y = (y // 26) // (4) // 2

            move_mouse(
                previous_location_x + (x * m_x),
                previous_location_y + (y * m_y),
            )
            exit()

    if second_key is None:
        second_key = event.keyval
        print(f"Second key pressed: {chr(second_key)}")

    if first_pass:
        x, y = window.get_size()
        letter_button_x = int(first_key) - 97
        letter_button_y = int(second_key) - 97

        move_mouse(letter_button_x * (x // 26), letter_button_y * (y // 26))

        window.get_children()[0].get_children()[
            (26 * letter_button_x) + (letter_button_y)
        ].set_markup(
            '<span font_desc="courier 15"><b>q w e r\na s d f\nz x c v\n</b></span>',
        )
        previous_location_x = letter_button_x * (x // 26)
        previous_location_y = letter_button_y * (y // 26)
        first_pass = False
        first_key = None


if __name__ == "__main__":

    horizontal_size = tk.Tk().winfo_screenwidth()
    vertical_size = tk.Tk().winfo_screenheight()

    window = Overlay(horizontal_size, vertical_size)
    window.set_opacity(0.5)
    # window.set_size_request(horizontal_size, vertical_size)

    window.connect("key-press-event", on_key_press)

    app = Application("default", window)
    app.run()
