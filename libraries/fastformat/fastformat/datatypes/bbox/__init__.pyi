class BBox:
    def into_xyxy(self) -> BBox:
        """convert the bbox to xyxy"""
        ...

    def into_xywh(self) -> BBox:
        """convert the bbox to xywh"""
        ...

def new_xyxy(data: list, confidence: list, label: list) -> BBox:
    """create a new xyxy BBox"""
    ...

def new_xywh(data: list, confidence: list, label: list) -> BBox:
    """create a new xywh BBox"""
    ...
