 #!/bin/bash
# source .venv/bin/activate

echo "Testing cinepy return png rust-python code on .cine file"
python -m timeit --number 100 --setup 'import cine_py; import random;' 'cine_file = cine_py.CinePy("./files/temp.cine"); frame = cine_file.get_frame_as(random.randint(0, 7400), cine_py.PyFrameType.Png)'
echo

echo "Testing cinepy return base64 rust-python code on .cine file"
python -m timeit --number 100 --setup 'import cine_py; import random;' 'cine_file = cine_py.CinePy("./files/temp.cine"); frame = cine_file.get_frame_as(random.randint(0, 7400), cine_py.PyFrameType.Base64)'
echo

echo "Testing OpenCV on .mp4 file."
python -m timeit --number 100 --setup 'import random; import cv2;' 'cap = cv2.VideoCapture("./files/temp.mp4"); cap.set(cv2.CAP_PROP_POS_FRAMES, random.randint(0, 400)); cap.read()'
echo

echo "Testing pycine python code on .cine file"
python -m timeit --number 100 --setup 'import random; from pycine.raw import read_frames' 'frames,_,_=read_frames("./files/temp.cine", start_frame=random.randint(0, 400), count=1); next(frames)'
echo