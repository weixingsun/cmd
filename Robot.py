import PIL.Image
import pyautogui, time
import pygetwindow as gw
import win32gui,win32con,win32api
import PIL
from pandas import DataFrame
def mouse_click(x,y):
    pyautogui.moveRel(x, y)
    pyautogui.click(button='left')
    #pyautogui.click()
    #pyautogui.doubleClick()
    time.sleep(1)
def mouse_move(x,y):
    pyautogui.moveTo(x, y)
    print(pyautogui.position())
    time.sleep(1)

def keyboard(word):
    if word=="enter":
        pyautogui.press(word)
    else:
        pyautogui.typewrite(word)
    time.sleep(1)

def go_loop():
    mouse_move(200,200)
    mouse_click(100,0)
    mouse_click(0,100)
    mouse_click(-100,0)
    mouse_click(0,-100)
#keyboard("hello")
#keyboard("enter")
def move_window():
    hwnd = win32gui.FindWindow(None, '梦幻西游：时空')
    window_rect = win32gui.GetWindowRect(hwnd)
    x = window_rect[0]
    y = window_rect[1]
    width = window_rect[2] - x
    height = window_rect[3] - y
    print("窗口大小:",width,height)
    win32gui.MoveWindow(hwnd, 0, 0, width, height, True)
    win32gui.SetForegroundWindow(hwnd)

def get_info():
    size=pyautogui.size()
    print("屏幕大小",size)
    window=gw.getWindowsWithTitle("梦幻西游：时空")[0]
    print("窗口大小",window)
    window.resize(widthOffset=-350,heightOffset=0)
    print("resize:",window)

def capture_red():
    im = pyautogui.screenshot(region=(878,33, 942-878,13))
    im.save("d:\\robot\\red.jpg")
def capture_blue():
    im = pyautogui.screenshot(region=(878,47, 942-878,13))
    im.save("d:\\robot\\blue.jpg")

def print_image(filename):
    im=PIL.Image.open(filename,'r')
    print("filename:",filename)
    values=list(im.getdata())
    length=len(values)
    df=DataFrame(values)
    #print(df)
    print("R:",df[0].mean())
    print("G:",df[1].mean())
    print("B:",df[2].mean())

    #filename: d:\robot\red_low.jpg
    #R: 125.8179104477612
    #G: 136.58258706467663
    #B: 148.55721393034827
    #filename: d:\robot\red_full.jpg
    #R: 216.5060096153846
    #G: 78.640625
    #B: 39.984375

def check_red():
    im = pyautogui.screenshot(region=(878,33, 942-878,13))
    values=list(im.getdata())
    #length=len(values)
    df=DataFrame(values)
    avg_r=df[0].mean()
    avg_g=df[1].mean()
    avg_b=df[2].mean()
    if avg_r>150 and avg_g<100 and avg_b<100:
        print("full red")
        return False
    elif avg_r<150 and avg_g>100 and avg_b>100:
        print("low red")
        return True
def check_blue():
    im = pyautogui.screenshot(region=(878,47, 942-878,13))
    values=list(im.getdata())
    #length=len(values)
    df=DataFrame(values)
    avg_r=df[0].mean()
    avg_g=df[1].mean()
    avg_b=df[2].mean()
    #print("R:",avg_r)
    #print("G:",avg_g)
    #print("B:",avg_b)
    if avg_b>150 and avg_g<180 and avg_r<100:
        print("full blue")
        return False
    elif avg_b<150 and avg_g>100 and avg_r>100:
        print("low blue")
        return True

#go_loop()
move_window()
#capture_red()
capture_blue()
check_red()
check_blue()
