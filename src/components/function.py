import os
import pyautogui
import time
from PIL import Image
import cv2 as cv
import numpy as np
from collections import Counter
import random

def capture_and_save_screenshot(file_name):
    # 获取当前工作目录
    current_directory = os.getcwd()
    
    # 生成两位数的随机数
    random_number = random.randint(10, 99)
    
    # 拼接文件路径，并在文件名后加上随机数
    base_name, ext = os.path.splitext(file_name)
    new_file_name = f"{base_name}_{random_number}{ext}"
    save_path = os.path.join(current_directory, new_file_name)

    # 截取全屏并保存到指定目录
    screenshot = pyautogui.screenshot()
    screenshot.save(save_path)
    
    return save_path

def get_coordinate(x1, y1, x2, y2, part_num):
    width = (x2 - x1) / 3
    height = (y2 - y1) / 3
    
    coordinates = {
        1: (x1, y1),
        2: (x1 + width, y1),
        3: (x1 + 2 * width, y1),
        4: (x1, y1 + height),
        5: (x1 + width, y1 + height),
        6: (x1 + 2 * width, y1 + height),
        7: (x1, y1 + 2 * height),
        8: (x1 + width, y1 + 2 * height),
        9: (x1 + 2 * width, y1 + 2 * height)
    }
    
    return coordinates.get(part_num, "Invalid part number")

def print_coordinate(cropped_image_path, part_num):
    # 保存截图到当前目录，并将保存的路径返回
    file_name = 'pipei.png'
    save_path = capture_and_save_screenshot(file_name)
    print("截图已保存到：", save_path)

    # 等待2秒以确保截图文件保存完成
    time.sleep(2)

    img = cv.imread(save_path, 0)
    template = cv.imread(cropped_image_path, 0)
    h, w = template.shape[:2]  # rows->h, cols->w

    # 6种匹配方法
    methods = ['cv.TM_CCOEFF', 'cv.TM_CCOEFF_NORMED', 'cv.TM_CCORR',
               'cv.TM_CCORR_NORMED', 'cv.TM_SQDIFF', 'cv.TM_SQDIFF_NORMED']
    
    # 存储坐标部分的列表
    part_coordinates_list = []

    for meth in methods:
        method = eval(meth)
        res = cv.matchTemplate(img, template, method)
        min_val, max_val, min_loc, max_loc = cv.minMaxLoc(res)

        # 如果是平方差匹配TM_SQDIFF或归一化平方差匹配TM_SQDIFF_NORMED，取最小值
        if method in [cv.TM_SQDIFF, cv.TM_SQDIFF_NORMED]:
            top_left = min_loc
        else:
            top_left = max_loc
        bottom_right = (top_left[0] + w, top_left[1] + h)

        x1, y1 = top_left
        x2, y2 = bottom_right

        # 获取对应部分的坐标
        part_coordinate = get_coordinate(x1, y1, x2, y2, part_num)
        part_coordinates_list.append(part_coordinate)

    # 统计出现次数最多的部分坐标
    if part_coordinates_list:
        most_common_part_coordinate = Counter(part_coordinates_list).most_common(1)
        coordinate = most_common_part_coordinate[0][0]
        print(f"Part {part_num} coordinate: {coordinate}")
        return coordinate
    else:
        print("未找到匹配图像")
        return None
def click(x, y):
    """
    单击鼠标左键，在指定位置单击。

    参数:
    x (int): 单击的X坐标。
    y (int): 单击的Y坐标。
    """
    pyautogui.click(x=x, y=y, button='left')
    print(f"单击：位置=({x}, {y})")
def doubleclick(x, y):
    """
    双击鼠标左键，在指定位置双击。

    参数:
    x (int): 双击的X坐标。
    y (int): 双击的Y坐标。
    """
    pyautogui.doubleClick(x=x, y=y)
    print(f"双击：位置=({x}, {y})")
def wait(seconds):
    """
    等待指定秒数。
    参数:
    seconds (float): 等待的秒数。
    """
    time.sleep(seconds)
    print(f"等待了 {seconds} 秒")
def exist(cropped_image_path):
    # 保存截图到当前目录，并将保存的路径返回
    file_name = 'screenshot.png'
    save_path = capture_and_save_screenshot(file_name)
    print("截图已保存到：", save_path)

    # 等待2秒以确保截图文件保存完成
    time.sleep(2)

    # 读取截图和裁剪的图像
    img = cv.imread(save_path, 0)
    template = cv.imread(cropped_image_path, 0)

    if img is None or template is None:
        print("无法读取图片")
        return False

    h, w = template.shape[:2]  # rows->h, cols->w

    # 使用 cv.TM_CCOEFF_NORMED 方法进行模板匹配
    method = cv.TM_CCOEFF_NORMED
    res = cv.matchTemplate(img, template, method)
    min_val, max_val, min_loc, max_loc = cv.minMaxLoc(res)

    # 如果最大值大于某个阈值(如0.8)，则认为找到了匹配的图片
    threshold = 0.8
    if max_val >= threshold:
        print("找到匹配图像")
        return True
    else:
        print("未找到匹配图像")
        return False

def input(text):
    """
    输入指定的文本。

    参数:
    text (str): 要输入的文本。
    """
    pyautogui.typewrite(text)
    print(f"输入文本: {text}")
def type(*keys):
    """
    执行指定的快捷键。

    参数:
    *keys (str): 要执行的快捷键，传入多个键作为参数。
    """
    if keys:
        pyautogui.hotkey(*keys)
        print(f"执行快捷键: {'+'.join(keys)}")


