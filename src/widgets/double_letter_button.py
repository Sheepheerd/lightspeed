from fabric.widgets.button import Button


class DoubleLetterButton(Button):
    def __init__(self, x, y, **kwargs):
        self.x = x
        self.y = y

        super().__init__(**kwargs)
