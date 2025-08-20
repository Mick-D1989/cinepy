 #!/bin/sh
source .venv/bin/activate

echo "Testing cinepy rust-python code on .cine file"
python -m timeit --number 100 --setup 'import cine_py; import random; cine_file = cine_py.CineFile("/mnt/g/Programming/cinepy/files/temp.cine");' 'cine_file.get_frame(random.randint(0, 7400))'
echo

echo "Testing OpenCV on .mp4 file."
python -m timeit --number 100 --setup 'import random; import cv2; cap = cv2.VideoCapture("/mnt/g/Programming/cinepy/files/temp.mp4")' 'cap.set(cv2.CAP_PROP_POS_FRAMES, random.randint(0, 7400)); cap.read()'
echo

echo "Testing pycine python code on .cine file"
python -m timeit --number 100 --setup 'import random; from pycine.raw import read_frames' 'frames,_,_=read_frames("/mnt/g/Programming/cinepy/files/temp.cine", start_frame=random.randint(0, 7400), count=1); next(frames)'
echo