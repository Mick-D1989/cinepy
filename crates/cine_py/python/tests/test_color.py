import pytest
import cine_py
import numpy as np
import cv2

temp="chart1"
fPth = f"./files/{temp}.cine"
save_path=f"./files/{temp}.png"

def test_open():
    cine_file = cine_py.CineFile(fPth)

def test_cine_header():
    cine_file = cine_py.CineFile(fPth)

    assert(cine_file.cine_file_header.version == 1)
    assert(cine_file.cine_file_header.compression == 2)

def test_bitmap_header():
    cine_file = cine_py.CineFile(fPth)

    assert(cine_file.bitmap_info_header.bi_width == 2048)
    assert(cine_file.bitmap_info_header.bi_height == 1080)
    assert(cine_file.bitmap_info_header.bi_compression == 256)
    assert(cine_file.bitmap_info_header.bi_bit_count == 16)

def test_setup():
    cine_file = cine_py.CineFile(fPth)

    assert(cine_file.setup.Serial == 16001)
    assert(cine_file.setup.CFA == 3)
    assert(cine_file.setup.BlackLevel == 64)
    assert(cine_file.setup.WhiteLevel == 1014)
    assert(cine_file.setup.FrameRate == 25)
    assert(cine_file.setup.RealBPP == 10)
    assert(cine_file.setup.RecBPP == 12)
    assert(cine_file.setup.ImWidth == cine_file.bitmap_info_header.bi_width)
    assert(cine_file.setup.ImHeight == cine_file.bitmap_info_header.bi_height)

def test_pix_length():
    cine_file = cine_py.CineFile(fPth)

    width, height = cine_file.bitmap_info_header.bi_width, cine_file.bitmap_info_header.bi_height
    frame_no=0
    frame_bytes = cine_file.get_frame(frame_no)
    frame_bytes_as_np = np.asarray(frame_bytes, dtype=np.uint16)

    assert(len(frame_bytes_as_np) == (width*height))

def test_save_file():
    import os
    cine_file = cine_py.CineFile(fPth)
    frame_no=0
    cine_file.save_single_frame(frame_no, save_path)

    assert(os.path.exists(save_path))

# def test_img_no_bytes():
#     import numpy as np
#     import cv2 

#     temp = "temp"
#     fPth = f"/mnt/g/Programming/rust/cine_file/files/{temp}.cine"
#     cine_file = cine_py.CineFile(fPth)

#     frame_no=10
#     frame_bytes = cine_file.get_frame(frame_no)
#     frame_bytes_as_np = np.frombuffer(frame_bytes, np.uint8)
#     image_opencv = cv2.imdecode(frame_bytes_as_np, cv2.IMREAD_GRAYSCALE)