from fastformat import image as ffi

def main():
    print("Hello, World!")

    image = ffi.new_bgr8(data=[0, 0, 0], width=1, height=1, name="test")
    print (image.name(), image.as_ptr())

    raw_data = ffi.raw_data(image.into_arrow())
    view = ffi.view_from_raw_data(raw_data)
    del raw_data
    print (view.name(), view.as_ptr())

    new_image = view.into_rgb8()
    print (new_image.name(), new_image.as_ptr())

def abc(a):
    a = None

if __name__ == "__main__":
    main()
