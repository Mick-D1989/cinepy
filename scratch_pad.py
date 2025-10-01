from cine_py import CinePy, PyFrameType, PySaveType, PyVideoHeader
import numpy as np
import cv2 
import timeit
import random
import time
from pycine.raw import read_frames

# python -m timeit --number 100 --setup 'import cine_py; import random; cine_file = cine_py.CineFile("./files/temp.cine");' 'cine_file.get_frame(random.randint(0, 400))'
# 100 loops, best of 5: 75.3 msec per loop
# python -m timeit --number 100 --setup 'import random; import cv2; cap = cv2.VideoCapture("./files/temp.mp4")' 'cap.set(cv2.CAP_PROP_POS_FRAMES, random.randint(0, 400)); cap.read()'
# 100 loops, best of 5: 14.2 msec per loop
# python -m timeit --number 100 --setup 'import random; from pycine.raw import read_frames' 'read_frames("./files/temp.cine", start_frame=random.randint(0, 400), count=1)'
def utf8len(s):
    return len(s.encode('utf-8'))

temp = "10bit_packed_gray"
fPth = f"./files/{temp}.cine"
cine_file = CinePy(fPth)

bitmap_info_header = cine_file.get_headers(PyVideoHeader.BitmapInfoHeader)
setup = cine_file.get_headers(PyVideoHeader.Setup)
cine_header = cine_file.get_headers(PyVideoHeader.CineFileHeader)

print("end")
