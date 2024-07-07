import os
import random
import pyautogui
from PIL import Image
import tkinter as tk
from multiprocessing import Process, Queue
from flask import Flask, request, jsonify
from flask_cors import CORS
import subprocess
import tempfile

app = Flask(__name__)
CORS(app)  # 启用 CORS

@app.route('/run-code', methods=['POST'])
def run_code():
    data = request.json
    code = data.get('code', '')

    print('Received code:', code)

    # 在当前目录创建临时文件
    tempdir = os.getcwd()  # 获取当前工作目录
    with tempfile.NamedTemporaryFile(suffix='.py', delete=False, dir=tempdir) as temp_script:
        temp_script.write(code.encode('utf-8'))
        temp_script.flush()
        try:
            result = subprocess.run(['python3', temp_script.name], capture_output=True, text=True, check=True)
            output = result.stdout
            error = result.stderr
            print('Execution output:', output)
            print('Execution error:', error)
        except subprocess.CalledProcessError as e:
            output = e.stdout
            error = e.stderr
            print('Execution error:', error)

    return jsonify({'output': output, 'error': error})
def capture_and_save_screenshot(file_name):
    try:
        current_directory = os.getcwd()
        save_path = os.path.join(current_directory, file_name)
        screenshot = pyautogui.screenshot()
        screenshot.save(save_path)
        return save_path
    except Exception as e:
        return str(e)

def run_crop_gui(image_path, queue):
    def on_mouse_press(event):
        nonlocal start_x, start_y
        start_x, start_y = event.x, event.y

    def on_mouse_drag(event):
        canvas.delete("crop_rectangle")
        current_x, current_y = event.x, event.y
        canvas.create_rectangle(start_x, start_y, current_x, current_y, outline="red", tags="crop_rectangle")

    def on_mouse_release(event):
        end_x, end_y = event.x, event.y
        crop_coordinates = (min(start_x, end_x), min(start_y, end_y), max(start_x, end_y), max(start_y, end_y))
        cropped_image = image.crop(crop_coordinates)

        # 生成两位数的随机数
        random_number = random.randint(10, 99)
        base_name, ext = os.path.splitext("cropped_image.png")
        cropped_image_name = f"{base_name}_{random_number}{ext}"

        cropped_image.save(cropped_image_name)
        queue.put(os.path.join(os.getcwd(), cropped_image_name))
        root.quit()

    def open_crop_window():
        nonlocal image, canvas, root, start_x, start_y
        root = tk.Tk()
        image = Image.open(image_path)
        canvas_width, canvas_height = image.width, image.height
        canvas = tk.Canvas(root, width=canvas_width, height=canvas_height)
        canvas.pack()
        start_x, start_y = None, None
        canvas.bind("<ButtonPress-1>", on_mouse_press)
        canvas.bind("<B1-Motion>", on_mouse_drag)
        canvas.bind("<ButtonRelease-1>", on_mouse_release)
        photo = tk.PhotoImage(file=image_path)
        canvas.create_image(0, 0, anchor=tk.NW, image=photo)
        root.mainloop()

    image = None
    canvas = None
    root = None
    start_x, start_y = None, None
    
    open_crop_window()

@app.route('/api/capture-and-crop', methods=['POST'])
def capture_and_crop():
    try:
        file_name = 'screenshot.png'
        save_path = capture_and_save_screenshot(file_name)
        if not os.path.exists(save_path):
            raise FileNotFoundError("Screenshot not saved successfully")
        
        queue = Queue()
        cropper_process = Process(target=run_crop_gui, args=(save_path, queue))
        cropper_process.start()
        cropper_process.join()  # 等待裁剪进程完成

        cropped_image_path = queue.get()  # 获取裁剪后的图片路径

        return jsonify({"message": "Screenshot captured and cropping completed.", "path": cropped_image_path})
    except Exception as e:
        return jsonify({"error": str(e)}), 500
if __name__ == '__main__':
    app.run(debug=True)
