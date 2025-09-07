import pytest
from cine_py import CinePy, PySaveType, PyFrameType
import numpy as np
import cv2

temp="temp"
fPth = f"./files/{temp}.cine"

def test_open():
    cine_file = CinePy(fPth)

#################################################
#           Save File Type Tests
#################################################

def test_save_img_as_png():
    import os
    save_path=f"./files/test_{temp}.png"
    cine_file = CinePy(fPth)
    cine_file.save_frame_as(0, PySaveType.Png, save_path)
    assert(os.path.exists(save_path))
    # Need to do some more checks to make sure the image is in valid format....not sure how yet

# def test_save_img_as_jpeg():
#     import os
#     cine_file = CinePy(fPth)
#     save_path=f"./files/test_{temp}.jpeg"
#     cine_file.save_frame_as(0, PySaveType.Jpeg, save_path)
#     assert(os.path.exists(save_path))
#     # Need to do some more checks to make sure the image is in valid format....not sure how yet

# def test_save_img_as_mp4():
#     import os
#     cine_file = CinePy(fPth)
#     save_path=f"./files/test_{temp}.mp4"
#     cine_file.save_frame_as(0, PySaveType.Mp4, save_path)
#     assert(os.path.exists(save_path))
#     # Need to do some more checks to make sure the image is in valid format....not sure how yet

#################################################
#           Return Frame Type Tests
#################################################

def test_get_frame_as_png():
    cine_file = CinePy(fPth)
    cine_file.get_frame_as(0, PyFrameType.Png)

def test_get_frame_as_base64():
    cine_file = CinePy(fPth)
    cine_file.get_frame_as(0, PyFrameType.Base64)

# def test_get_frame_as_raw():
#     cine_file = CinePy(fPth)
#     cine_file.get_frame_as(0, PyFrameType.Raw)

# # def test_get_frame_as_bytes():
# #     cine_file = CinePy(fPth)
# #     cine_file.get_frame_as(0, PyFrameType.Bytes)
















# def test_cine_header():
#     cine_file = cine_py.CineFile(fPth)

#     assert(cine_file.cine_file_header.version == 1)
#     assert(cine_file.cine_file_header.compression == 0)

# def test_bitmap_header():
#     cine_file = cine_py.CineFile(fPth)

#     assert(cine_file.bitmap_info_header.bi_width == 768)
#     assert(cine_file.bitmap_info_header.bi_height == 416)
#     assert(cine_file.bitmap_info_header.bi_compression == 256)
#     assert(cine_file.bitmap_info_header.bi_bit_count == 16)

# def test_setup():
#     cine_file = cine_py.CineFile(fPth)

#     assert(cine_file.setup.Serial == 23907)
#     assert(cine_file.setup.CFA == 0)
#     assert(cine_file.setup.BlackLevel == 64)
#     assert(cine_file.setup.WhiteLevel == 1014)
#     assert(cine_file.setup.dFrameRate == 71000.0)
#     assert(cine_file.setup.RealBPP == 10)
#     assert(cine_file.setup.RecBPP == 12)
#     assert(cine_file.setup.ImWidth == cine_file.bitmap_info_header.bi_width)
#     assert(cine_file.setup.ImHeight == cine_file.bitmap_info_header.bi_height)

# def test_pix_length():
#     cine_file = cine_py.CineFile(fPth)

#     width, height = cine_file.bitmap_info_header.bi_width, cine_file.bitmap_info_header.bi_height
#     frame_no=10
#     frame_bytes = cine_file.get_frame(frame_no)
#     frame_bytes_as_np = np.asarray(frame_bytes, dtype=np.uint16)

#     assert(len(frame_bytes_as_np) == (width*height))

# def test_save_file():
#     import os
#     cine_file = cine_py.CineFile(fPth)
#     frame_no=35
#     cine_file.save_single_frame(frame_no, save_path)

#     assert(os.path.exists(save_path))

# def test_base64():
#     cine_file = cine_py.CineFile(fPth)
#     frame_no=35
#     b64 = cine_file.base64_png(frame_no)
#     assert(type(b64) == str)

# # def test_img_no_bytes():
# #     import numpy as np
# #     import cv2 

# #     temp = "temp"
# #     fPth = f"/mnt/g/Programming/rust/cine_file/files/{temp}.cine"
# #     cine_file = cine_py.CineFile(fPth)

# #     frame_no=10
# #     frame_bytes = cine_file.get_frame(frame_no)
# #     frame_bytes_as_np = np.frombuffer(frame_bytes, np.uint8)
# #     image_opencv = cv2.imdecode(frame_bytes_as_np, cv2.IMREAD_GRAYSCALE)