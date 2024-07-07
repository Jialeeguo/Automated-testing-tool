<script setup lang="ts">
// @ts-ignore
import { Editor as MonacoTreeEditor, useMessage, useHotkey, useMonaco, type Files } from '~lib'
import { ComputedRef} from 'vue'
import { onMounted, ref } from 'vue';


let monacoStore;
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
// @ts-ignore
import axios from 'axios'
import {
  MouseStatus,
  WheelStatus,
  KeyboardStatus,
  MessageType,
  InputEventType,
} from "../common/Constans";

let remoteDesktopDpi: Record<string, any>;
let dc: RTCDataChannel;
onMounted(() => {
  monacoStore = useMonaco(monaco);
});
const insertCode = async (codeType: string) => {
  const editor = monacoStore.getEditor();
  const position = editor.getPosition();
  let codeSnippet = '';
  switch (codeType) {
    case 'screenshot':
      try {
        const response = await axios.post('http://localhost:5000/api/capture-and-crop');
        const imagePath = response.data.path;
        codeSnippet = `screenshot_path = "${imagePath}"\n`; // 插入截图路径
      } catch (error) {
        console.error('截图错误:', error);
      }
      break;
    case 'location':
      codeSnippet = 'location()\n';
      break;
    case 'click':
      codeSnippet = 'click()\n';
      break;
    case 'doubleclick':
      codeSnippet = 'doubleClick()\n';
      break;
    case 'exist':
      codeSnippet = 'exist()\n';
      break;
    case 'input':
      codeSnippet = 'input()\n';
      break;
    case 'type':
      codeSnippet = 'type()\n';
      break;
    case 'sleep':
      codeSnippet = 'sleep()';
      break;
    default:
      break;
  }
  editor.executeEdits('', [{
    range: new monaco.Range(position.lineNumber, position.column, position.lineNumber, position.column),
    text: codeSnippet,
  }]);
};
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
    } else if (label === 'python') { 
      // @ts-ignore
      return new pythonWorker()
    }
    return new editorWorker()
  },
  globalAPI: true,
}

server.delay().then(() => {
  monacoStore = useMonaco(monaco)
})

// ================ 推送消息 push message ================
const messageStore = useMessage()
onMounted(() => {
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
})

// ================ 快捷键 hotkey ==================
const hotkeyStore = useHotkey()
hotkeyStore.listen('root', (event: KeyboardEvent) => { })
hotkeyStore.listen('editor', (event: KeyboardEvent) => {
  if (event.ctrlKey && !event.shiftKey && !event.altKey && event.key === 's') {
    // do something...
  }
})

// ================ 加载文件 load files ================
const files = ref<Files>()
const output = ref('')
const showTerminal = ref(true); 
const terminalContent = ref(''); 
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

const runCode = async () => {
  const editor = monacoStore.getEditor()
  const code = editor.getValue()

  console.log('Sending code to server:', code)

  try {
    const response = await axios.post('http://localhost:5000/run-code', { code });
    console.log('Server response:', response.data);
    showTerminal.value = true; 
    if (response.data.output) {
      terminalContent.value = response.data.output;
    } else if (response.data.error) {
      terminalContent.value = response.data.error;
    } else {
      terminalContent.value = 'No output or error received from server';
    }
  } catch (error) {
    console.error('Error running code:', error.message);
    terminalContent.value = '运行代码时发生错误：' + error.message;
    showTerminal.value = true; 
  }
};
const closeTerminal = () => {
  showTerminal.value = false;
};
</script>

<template>
  <div class="editor-container">
    <MonacoTreeEditor
      :font-size="14"
      :files="files"
      :sider-min-width="240"
      filelist-title="os-9-IDE"
      language="python"
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
    ></MonacoTreeEditor>
    <button class="run-button" @click="runCode"></button>
    <div class="buttons-container">
        <button @click="insertCode('screenshot')">screenshot</button>
        <button @click="insertCode('location')">location</button>
        <button @click="insertCode('click')">click</button>
        <button @click="insertCode('doubleclick')">doubleClick</button>
        <button @click="insertCode('exist')">exist</button>
        <button @click="insertCode('input')">input</button>
        <button @click="insertCode('type')">type</button>
        <button @click="insertCode('sleep')">sleep</button>
    </div>
    <div v-if="showTerminal" class="terminal">
      <div class="terminal-header">
        <span>终端</span>
        <button @click="closeTerminal">关闭</button>
      </div>
      <pre>{{ terminalContent }}</pre>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: #1e1e1e; /* 设置背景颜色为深色 */
}

.editor-container {
  display: flex;
  flex-direction: column;
  flex: 1;
  position: relative;
  background-color: #1e1e1e; /* 确保编辑器容器背景颜色与整个容器一致 */
}

.monaco-tree-editor {
  flex: 1;
  background-color: #1e1e1e; /* 确保Monaco Tree Editor背景颜色与整个容器一致 */
}

.buttons-container {
  display: flex;
  justify-content: space-between;
  align-items: center; /* 垂直居中按钮 */
  padding: 10px;
  background-color: #1e1e1e; /* 设置按钮容器背景色与终端一致 */
  border-top: 1px solid #ddd;
  border-bottom: 1px solid #ddd;
  flex-shrink: 0; /* 禁止按钮容器收缩 */
  width: 100%; /* 让按钮容器占满可用宽度 */
  box-sizing: border-box; /* 确保内边距和边框包含在元素的总宽度和高度内 */
}

.buttons-container button {
  padding: 10px 20px;
  font-size: 14px;
  cursor: pointer;
  background-color: #1e1e1e; /* 设置按钮背景色与终端一致 */
  color: #ffffff; /* 设置按钮文本颜色为白色 */
  flex: 1; /* 让按钮占据可用空间，自由延伸 */
  margin-right: 5px; /* 按钮之间的间距 */
  box-sizing: border-box; /* 确保内边距和边框包含在元素的总宽度和高度内 */
}

.buttons-container button:last-child {
  margin-right: 0; /* 最后一个按钮没有右间距 */
}

.buttons-container button:hover {
  background-color: #333; /* 鼠标悬停时的背景色 */
}

.terminal {
  flex-shrink: 0; /* 禁止终端收缩 */
  background-color: #1e1e1e;
  color: #ffffff;
  padding: 10px;
  overflow-y: auto; /* 如果内容溢出，显示滚动条 */
  height: 130px;
  box-sizing: border-box; /* 确保内边距和边框包含在元素的总宽度和高度内 */
}

.terminal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 10px;
  border-bottom: 1px solid #444;
}

.terminal-header button {
  background: none;
  border: none;
  color: #ffffff;
  cursor: pointer;
}

.terminal pre {
  overflow-y: auto;
  white-space: pre-wrap;
  word-wrap: break-word;
  background-color: #1e1e1e;
  padding: 10px;
  margin: 0;
}

.run-button {
  margin: 10px;
  position: absolute;
  top: -10px;
  left: 180px; 
  background-color: transparent;
  border: none;
  padding: 10px 15px;
  cursor: pointer;
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.run-button::before {
  content: '';
  display: inline-block;
  width: 0;
  height: 0;
  border-left: 10px solid transparent;
  border-right: 10px solid transparent;
  border-bottom: 20px solid rgb(8, 190, 8); /* 这里定义了绿色的三角形 */
  transform: rotate(90deg);
  margin-right: 5px;
}

.run-button:hover::before {
  border-bottom-color: darkgreen; /* 鼠标悬停时改变三角形颜色 */
}
</style>
