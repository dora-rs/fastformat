def main():
    from fastformat.datatypes.bbox import new_xyxy
    from fastformat.datatypes.bbox import BBox

    bbox = new_xyxy([0, 0, 1, 1], [0.93], ["cat"])

if __name__ == "__main__":
    main()
