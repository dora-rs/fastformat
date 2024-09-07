def main():
    from fastformat.datatypes.image import new_bgr8
    from fastformat.datatypes.image import Image

    image = new_bgr8([0, 0, 0], 1, 1, "TestImage")

    print(image.name())
    print(image.width())
    print(image.height())
    print(image.as_ptr())

if __name__ == "__main__":
    main()
