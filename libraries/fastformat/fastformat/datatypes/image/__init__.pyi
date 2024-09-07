class Image:
    def name(self) -> None | str:
        """get the name of the image"""
        ...

    def width(self) -> int:
        """get the width of the image"""
        ...

    def height(self) -> int:
        """get the height of the image"""
        ...

    def as_ptr(self) -> int:
        """get the pointer to the image"""
        ...

    def into_rgb8(self) -> Image:
        """convert the image to rgb8"""
        ...

    def into_bgr8(self) -> Image:
        """convert the image to bgr8"""
        ...

def new_bgr8(data: list, width: int, height: int, name: None | str) -> Image:
    """create a new bgr8 Image"""
    ...

def new_rgb8(data: list, width: int, height: int, name: None | str) -> Image:
    """create a new rgb8 Image"""
    ...

def new_gray8(data: list, width: int, height: int, name: None | str) -> Image:
    """create a new gray8 Image"""
    ...
