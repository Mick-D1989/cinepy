source /.venv/bin/activate

make clean
make python-test

python -m timeit --number 100 --setup 'import cine_py; import random; cine_file = cine_py.CineFile("/mnt/g/Programming/cinepy/files/temp.cine");' 'cine_file.get_frame(random.randint(0, cine_file.cine_file_header.image_count-1))'
python -m timeit --number 100 --setup 'import random; import cv2; cap = cv2.VideoCapture("/mnt/g/Programming/cinepy/files/temp.mp4")' 'cap.set(cv2.CAP_PROP_POS_FRAMES, random.randint(0, 7400)); cap.read()'
python -m timeit --number 100 --setup 'import random; from pycine.raw import read_frames' 'frames,_,_=read_frames("/mnt/g/Programming/cinepy/files/temp.cine", start_frame=random.randint(0, 7400), count=1); next(frames)'