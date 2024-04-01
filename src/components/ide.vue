<script setup lang="ts">
// @ts-ignore
import { Editor as MonacoTreeEditor, useMessage, useHotkey, useMonaco, type Files } from '~lib'
import { ComputedRef, onMounted, onUnmounted,ref } from 'vue'
import * as monaco from 'monaco-editor'
// @ts-ignore
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
// @ts-ignore
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
// @ts-ignore
import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker'
// @ts-ignore
import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker'
// @ts-ignore
import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker'
import * as server from './mock-server'
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import {
  MouseStatus,
  WheelStatus,
  KeyboardStatus,
  MessageType,
  InputEventType,
} from "../common/Constans";
import { Editor } from 'monaco-tree-editor'

let remoteDesktopDpi: Record<string, any>;
  let dc: RTCDataChannel;

const recording = ref(false);
const pause = ref(false);
const pause1 = ref(true);
const startTime = ref<Date | null>(null);
const isRecording = ref(false);
const elapsedTime = ref(0);
const recordstart = ref(true); // 假设初始值为 true
const logs = ref('');
const log = ref('');
const recordStateChangeTime = ref<Date | null>(null);
// ================ 初始化 init monaco-tree-editor ================
window.MonacoEnvironment = {
  getWorker: function (_moduleId, label: string) {
    if (label === 'json') {
      return new jsonWorker()
    } else if (label === 'ts' || label === 'typescript') {
      return new tsWorker()
    } else if (label === 'html' || label === 'handlebars' || label === 'razor') {
      return new htmlWorker()
    } else if (label === 'css' || label === 'scss' || label === 'less') {
      return new cssWorker()
    }
    return new editorWorker()
  },
  globalAPI: true,
}
let monacoStore
server.delay().then(() => {
  monacoStore = useMonaco(monaco)
})

// ================ 推送消息 push message ================
const messageStore = useMessage()
const handleKeyDown = (event:KeyboardEvent) => {
  console.log("ss");
  if (event.key === 'F1') {
    event.preventDefault(); 
    startRecord(); 
  }
};
onMounted(() => {
  console.log("Component mounted, adding event listener.");
  window.addEventListener("keydown", handleKeyDown);
  const id = messageStore.info({
    content: 'testing..',
    loading: true,
  })
  setTimeout(() => {
    messageStore.close(id)
    messageStore.success({
      content: 'Hello Editor',
      closeable: true,
      timeoutMs: 15000,
      textTip: 'testing successed!',
    })
  }, 5000)
  const cleanupListeners = Promise.all([
    // listen('trans', (event) => startRecord()),
    listen('running', (event) => stopRecord()),
    listen('screen', (event) => startScreenshot()),
    listen('opening', (event) => recordWindow()),
    listen('run', (event) => playBack()),
    listen('press-listen-keyboard', (event) => {
      console.log("press-listen-keyboard");
    }),
  ]).then(unlistenFns => () => unlistenFns.forEach(unlisten => unlisten()));
})
onUnmounted(async () => {
  window.removeEventListener("keydown", handleKeyDown);
  });
// ================ 快捷键 hotkey ==================
const hotkeyStore = useHotkey()
hotkeyStore.listen('root', (event: KeyboardEvent) => {})
hotkeyStore.listen('tran', (event: KeyboardEvent) => {
  if (event.key === "F1") {
    event.preventDefault(); 
    startRecord(); 
  }
})
const startRecord = async () => {
  startTime.value = new Date();
  isRecording.value = true;
  elapsedTime.value = 0;
  if (!recording.value) {
    log.value = '';
  }
  recordstart.value = !recordstart.value;
  recording.value = !recording.value;
  if (recording.value) {
    logs.value = '';
    if (!recordStateChangeTime.value) {
      recordStateChangeTime.value = new Date();
    }
    const currentTime = new Date().toLocaleTimeString();
    log.value += `${'录制已开始'} - [${currentTime}]\n`;
    await invoke('start_record', { recordstart: recordstart.value });
    console.log(recording.value + 'ss');
  } else {
    console.log(recording.value + '是');
    // const currentTime = new Date().toLocaleTimeString();
    // const timeElapsed = new Date() - recordStateChangeTime.value;
    // log.value += `本次录制时长 ${timeElapsed} 毫秒\n`;
    // log.value += `${'录制结束,生成脚本已保存到log文件夹下，下次录制时本次日志操作提示被清空！'} - [${currentTime}]\n`;
    recordStateChangeTime.value = null;
  }
};

const stopRecord = async () => {

};
const startScreenshot = async () => {
  // 定义 startScreenshot 函数逻辑...
};

const recordWindow = async () => {
  // 定义 recordWindow 函数逻辑...
};

const playBack = async () => {
  // 定义 playBack 函数逻辑...
};


// ================ 加载文件 load files ================
const files = ref<Files>()
const handleReload = (resolve: () => void, reject: (msg?: string) => void) => {
  server
    .fetchFiles()
    .then((response) => {
      files.value = response
      resolve()
    })
    .catch((e: Error) => {
      reject(e.message)
    })
}



const handleSaveFile = (path: string, content: string, resolve: () => void, reject: (msg?: string) => void) => {
  server
    .createOrSaveFile(path, content)
    .then((_response) => {
      resolve()
    })
    .catch((e: Error) => {
      reject(e.message)
    })
}
const handleDeleteFile = (path: string, resolve: () => void, reject: (msg?: string) => void) => {
  server
    .deleteFile(path)
    .then((_response) => {
      resolve()
    })
    .catch((e: Error) => {
      reject(e.message)
    })
}
const handleDeleteFolder = (path: string, resolve: () => void, reject: (msg?: string) => void) => {
  reject('Operation of delete folder is not supported!')
}
const handleNewFile = (path: string, resolve: Function, reject: Function) => {
  server
    .newFile(path)
    .then((_response) => {
      resolve()
    })
    .catch((e: Error) => {
      reject(e.message)
    })
}
const handleNewFolder = (path: string, resolve: Function, reject: Function) => {
  server
    .newFolder(path)
    .then((_response) => {
      resolve()
    })
    .catch((e: Error) => {
      reject(e.message)
    })
}
const handleRename = (path: string, newPath: string, resolve: () => void, reject: (msg?: string) => void) => {
  server
    .rename(path, newPath)
    .then((_response) => {
      resolve()
    })
    .catch((e: Error) => {
      reject(e.message)
    })
}

// ================ 自定义菜单 custom menu =================
/**
 * fileMenu and folderMenu Will insert into the context menu of sider file list
 */
const fileMenu = ref([
  { label: 'Custom Selection 1', value: 'any type that not null' },
  { label: 'Custom Selection 2', value: 2 },
  { label: '自定义文件选项 3', value: { id: 3, decription: 'value可以是任意非空值' } },
])
const folderMenu = ref([{ label: 'backup', value: 'backupFolder' }])
const settingsMenu = ref([
  {
    label: 'exit',
    handler: () => {
      alert('exit')
    },
  },
])

const handleContextMenuSelect = (path: string, item: { label: string | ComputedRef<string>; value: string }) => {
  console.warn('path: ' + path + '\nitem: ' + item)
}

// ================ 拖拽事件 drag event =================
/**
 * 当把拖动文件树中的数据拖进编辑器时，在当前光标处插入自定义的import语句
 * When drag filelist data into monaco editor, insert custom statement at cursor position
 */
const handleDragInEditor = (srcPath: string, targetPath: string, type: 'file' | 'folder') => {
  if (!targetPath.endsWith('.ts') && !srcPath.endsWith('.js')) {
    return
  }
  const editor = monacoStore.getEditor()
  const lineIndex = editor.getPosition()?.lineNumber!
  let str = 'import "' + _relativePathFrom(srcPath, targetPath) + '"'
  editor.executeEdits('drop', [{ range: new monaco.Range(lineIndex, 0, lineIndex, 0), text: str }])
}

function _longestCommonPrefix(strs: string[]): string {
  if (!strs.length) return ''
  let [a, ...b] = strs
  let result = ''
  for (let i = 0; i < a.length; i++) {
    let flag = b.every((item) => item[i] === a[i])
    if (flag) result += a[i]
    else break
  }
  return result
}

//计算相对路径 getRelativePath
const _relativePathFrom = (returnPath: string, fromPath: string): string => {
  const prefix = _longestCommonPrefix([returnPath, fromPath])
  returnPath = returnPath.replace(prefix, '').replace(/\\/g, '/')
  fromPath = fromPath.replace(prefix, '').replace(/\\/g, '/')
  const fromPathArr = fromPath.split('/')
  let relativePath = ''
  if (fromPathArr.length === 1) {
    relativePath = './'
  } else {
    for (let i = fromPathArr.length - 2; i >= 0; i--) {
      relativePath += '../'
    }
  }
  return (relativePath += returnPath)
}
const desktop = ref<HTMLVideoElement>();

const startScreenSharing = async () => {
  try {
    const stream = await navigator.mediaDevices.getDisplayMedia({
      video: true,
      audio: false,
    });

    // 将获取的媒体流设置到视频元素
    if (desktop.value) {
      desktop.value.srcObject = stream;
    } else {
      console.error('Video element not found');
    }
  } catch (error) {
    console.error('Error starting screen sharing:', error);
  }
};

// 鼠标按下事件处理
const mouseDown = (e: MouseEvent) => {
  sendMouseEvent(e.x, e.y, mouseType(MouseStatus.MOUSE_DOWN, e.button));
};

// 鼠标松开事件处理
const mouseUp = (e: MouseEvent) => {
  sendMouseEvent(e.x, e.y, mouseType(MouseStatus.MOUSE_UP, e.button));
};

// 滚轮事件处理
const wheel = (e: WheelEvent) => {
  let type = e.deltaY > 0 ? WheelStatus.WHEEL_UP : WheelStatus.WHEEL_DOWN;
  sendMouseEvent(e.x, e.y, type);
};

// 鼠标移动事件处理
const mouseMove = (e: MouseEvent) => {
  sendMouseEvent(e.x, e.y, MouseStatus.MOUSE_MOVE);
};

// 鼠标右键单击事件处理
const rightClick = (e: MouseEvent) => {
  sendMouseEvent(e.x, e.y, MouseStatus.RIGHT_CLICK);
};

// 发送鼠标事件
const sendMouseEvent = (x: number, y: number, eventType: string) => {
  if (remoteDesktopDpi) {
    let widthRatio = remoteDesktopDpi.width / desktop.value!.clientWidth;
    let heightRatio = remoteDesktopDpi.height / desktop.value!.clientHeight;

    let data = {
      x: parseInt((x * widthRatio).toFixed(0)),
      y: parseInt((y * heightRatio).toFixed(0)),
      eventType: eventType,
    };
    sendToClient({
      type: InputEventType.MOUSE_EVENT,
      data: data,
    });
  }
};
const sendToClient = (msg: Record<string, any>) => {
  let msgJSON = JSON.stringify(msg);
  dc.readyState == "open" && dc.send(msgJSON);
};
// 获取鼠标事件类型
const mouseType = (mouseStatus: MouseStatus, button: number) => {
  let type = "";
  switch (button) {
    case 0:
      type = "left-" + mouseStatus;
      break;
    case 2:
      type = "right-" + mouseStatus;
      break;
    // TODO 更多的按钮
  }

  return type;
};


</script>

<template>

  <MonacoTreeEditor
    :font-size="16"
    :files="files"
    :sider-min-width="240"
    filelist-title="源代码存储管理库"
    language="en-US"
    @reload="handleReload"
    @new-file="handleNewFile"
    @new-folder="handleNewFolder"
    @save-file="handleSaveFile"
    @delete-file="handleDeleteFile"
    @delete-folder="handleDeleteFolder"
    @rename-file="handleRename"
    @rename-folder="handleRename"
    :file-menu="fileMenu"
    :folder-menu="folderMenu"
    @contextmenu-select="handleContextMenuSelect"
    :settings-menu="settingsMenu"
    @drag-in-editor="handleDragInEditor"
    ref="editorRef"
  >
</MonacoTreeEditor>

<video ref="desktop" width=640 height=480   class="resizable-video" style="background-color: black;" 
@mousedown="mouseDown($event)" @mouseup="mouseUp($event)"
    @mousemove="mouseMove($event)" @wheel="wheel($event)" @contextmenu.prevent="rightClick($event)" autoplay></video>

<button @click="startScreenSharing">共享</button>

</template>

<style>
/* 添加可拉伸样式类 */
.resizable-video {
  resize: both;  /* 允许水平和垂直拉伸 */
  overflow: hidden;  /* 隐藏溢出部分 */
  border: 1px solid #ccc;  /* 添加边框 */
}
</style>