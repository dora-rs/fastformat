from fastformat import image as ffi

def main():
    print("Hello, World!")

    image = ffi.new_bgr8(data=[0, 0, 0], width=1, height=1, name="test")
    ptr = image.as_ptr()

    image2 = image.into_rgb8()
    ptr2 = image2.as_ptr()

    print(ptr)
    print(ptr2)

    arrow = image2.into_arrow()
    image3 = ffi.from_arrow(arrow)
    print(image3.name())


def abc(a):
    a = None

if __name__ == "__main__":
    main()
