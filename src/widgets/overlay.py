from fabric.widgets.wayland import WaylandWindow as Window
from fabric.widgets.fixed import Fixed
from fabric.widgets.label import Label


class Overlay(Window):
    def __init__(self, horizontal: int, verticle: int, **kwargs):
        super().__init__(
            layer="top",
            anchor="bottom",
            exclusivity="normal",
            keyboard_mode="exclusive",
            pass_through=True,
            **kwargs,
        )

        fixed = Fixed(size=[horizontal, verticle], h_align="center", v_align="center")

        for i in range(26):
            for j in range(26):
                letters = f"{chr(97 + i)}{chr(97 + j)}"
                label = Label(
                    markup='<span font_desc="courier 20"><b>' + letters + "</b></span>",
                    line_wape="word",
                    justification="center",
                    size=[horizontal // 26, verticle // 26],
                    # size=[100, 100],
                    ellipsization="end",
                    # h_expand=True,
                    # v_expand=True,
                )
                fixed.put(label, (horizontal // 26) * i, (verticle // 26) * j)

        self.children = fixed
