import cine_py
import numpy as np
import cv2 
import timeit
import random
import time
from pycine.raw import read_frames

# python -m timeit --number 100 --setup 'import cine_py; import random; cine_file = cine_py.CineFile("/mnt/g/Programming/cinepy/files/temp.cine");' 'cine_file.get_frame(random.randint(0, cine_file.cine_file_header.image_count-1))'
# 100 loops, best of 5: 75.3 msec per loop
# python -m timeit --number 100 --setup 'import random; import cv2; cap = cv2.VideoCapture("/mnt/g/Programming/cinepy/files/temp.mp4")' 'cap.set(cv2.CAP_PROP_POS_FRAMES, random.randint(0, 7400)); cap.read()'
# 100 loops, best of 5: 14.2 msec per loop
# python -m timeit --number 100 --setup 'import random; from pycine.raw import read_frames' 'read_frames("/mnt/g/Programming/cinepy/files/temp.cine", start_frame=random.randint(0, 7400), count=1)'

temp = "chart1"
fPth = f"./files/{temp}.cine"
cine_file = cine_py.CineFile(fPth)

width, height = cine_file.bitmap_info_header.bi_width, cine_file.bitmap_info_header.bi_height

frame_no=0

start_cine = time.perf_counter()
cine_file = cine_py.CineFile(fPth)
frame_bytes = cine_file.get_frame(frame_no)
end_cine = time.perf_counter()

frame_bytes_as_np = np.asarray(frame_bytes, dtype=np.uint16)
frame_bytes_as_np.shape = (height,width)

# image_opencv = cv2.normalize(frame_bytes_as_np, None, 0, 255, cv2.NORM_MINMAX, cv2.CV_8U)
image_opencv = cv2.cvtColor(frame_bytes_as_np, cv2.COLOR_Bayer_gr)
cv2.imshow("Decoded Image", image_opencv)
cv2.waitKey(0)
cv2.destroyAllWindows()


# assert(cine_file.cine_file_header.version)
# assert(cine_file.setup.serial == 23907)