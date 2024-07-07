<script setup lang="ts">
// @ts-ignore
import { Editor as MonacoTreeEditor, useMessage, useHotkey, useMonaco, type Files } from '~lib'
import { ComputedRef, onMounted, ref, nextTick,  } from 'vue'
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
// @ts-ignore
import * as server from './mock-server'
// @ts-ignore
import axios from 'axios'
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

server.delay().then(() => {
  monacoStore = useMonaco(monaco)
})

// ================ 推送消息 push message ================
const messageStore = useMessage()
onMounted(() => {
  const id = messageStore.info({
    content: 'testing..',
    loading: true,
  });
  setTimeout(() => {
    messageStore.close(id);
    messageStore.success({
      content: 'Hello Editor',
      closeable: true,
      timeoutMs: 15000,
      textTip: 'testing successed!',
    });
  }, 5000);

  // 配置Python语言
  monaco.languages.register({ id: 'python' });
  monaco.languages.setMonarchTokensProvider('python', {
    keywords: ['import', 'from', 'def', 'class', 'if', 'else', 'elif', 'return', 'for', 'while', 'try', 'except', 'finally', 'with', 'as', 'pass', 'break', 'continue', 'yield', 'lambda', 'global', 'nonlocal', 'assert', 'del', 'print'],
    operators: ['+', '-', '*', '**', '/', '//', '%', '@', '<<', '>>', '&', '|', '^', '~', ':', '=', '+=', '-=', '*=', '/=', '//=', '%=', '@=', '&=', '|=', '^=', '>>=', '<<=', '**='],
    brackets: [
      { open: '(', close: ')', token: 'delimiter.parenthesis' },
      { open: '[', close: ']', token: 'delimiter.square' },
      { open: '{', close: '}', token: 'delimiter.curly' }
    ],
    symbols: /[=><!~?:&|+\-*\/\^%]+/,
    tokenizer: {
      root: [
        [/[a-zA-Z_]\w*/, {
          cases: {
            '@keywords': 'keyword',
            '@default': 'identifier'
          }
        }],
        { include: '@whitespace' },
        [/[{}()\[\]]/, '@brackets'],
        [/[<>](?!@symbols)/, '@brackets'],
        [/!(?=([^=]|$))/, 'delimiter'],
        [/@symbols/, {
          cases: {
            '@operators': 'operator',
            '@default': ''
          }
        }],
        [/@\s*[a-zA-Z_\$][\w\$]*/, 'type.identifier'],
        [/\d*\.\d+([eE][\-+]?\d+)?/, 'number.float'],
        [/0[xX][0-9a-fA-F]+/, 'number.hex'],
        [/0[oO]?[0-7]+/, 'number.octal'],
        [/0[bB][01]+/, 'number.binary'],
        [/\d+/, 'number'],
        [/[;,.]/, 'delimiter'],
        [/"([^"\\]|\\.)*$/, 'string.invalid'],
        [/'([^'\\]|\\.)*$/, 'string.invalid'],
        [/'([^'\\]|\\.)*$/, 'string.invalid'],
        [/"/, 'string', '@string_double'],
        [/'/, 'string', '@string_single']
      ],
      whitespace: [
        [/[ \t\r\n]+/, ''],
        [/(^#.*$)/, 'comment'],
      ],
      string_double: [
        [/[^\\"]+/, 'string'],
        [/\\./, 'string.escape'],
        [/"/, 'string', '@pop']
      ],
      string_single: [
        [/[^\\']+/, 'string'],
        [/\\./, 'string.escape'],
        [/'/, 'string', '@pop']
      ],
    },
  });

  monaco.languages.setLanguageConfiguration('python', {
    comments: {
      lineComment: '#'
    },
    brackets: [
      ['{', '}'],
      ['[', ']'],
      ['(', ')']
    ],
    autoClosingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
      { open: '(', close: ')' },
      { open: '"', close: '"' },
      { open: "'", close: "'" }
    ]
  });

  nextTick(() => {
    const container = document.getElementById('editor-container');
    if (container) {
      const editorInstance = monaco.editor.create(container, {
        value: '',
        language: 'python'
      });

      // 加载文件内容并更新编辑器
      axios.get('/path/to/python/file').then(response => {
        editorInstance.setValue(response.data);
      }).catch(error => {
        console.error('Error fetching file:', error);
      });
    } else {
      console.error('Container element not found. Monaco editor instance not created.');
    }
  });
});
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
let monacoStore
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
    <MonacoTreeEditor :font-size="14" :files="files" :sider-min-width="240" filelist-title="os-9-IDE" language="python"
      @reload="handleReload" @new-file="handleNewFile" @new-folder="handleNewFolder" @save-file="handleSaveFile"
      @delete-file="handleDeleteFile" @delete-folder="handleDeleteFolder" @rename-file="handleRename"
      @rename-folder="handleRename" :file-menu="fileMenu" :folder-menu="folderMenu"
      @contextmenu-select="handleContextMenuSelect" :settings-menu="settingsMenu" @drag-in-editor="handleDragInEditor"
      ref="editorRef"></MonacoTreeEditor>
    <button class="run-button" @click="runCode"></button>
    <div v-if="showTerminal" class="terminal-container">
      <div class="terminal-header">
        <span>输出</span>
        <button class="close-button" @click="closeTerminal">❌</button>
      </div>
      <div class="terminal-body">
        <pre>{{ terminalContent }}</pre>
      </div>
    </div>
  </div>
</template>



<style>
.container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.editor-container {
  display: flex;
  flex-direction: column;
  flex: 1;
  position: relative;
}

.monaco-tree-editor {
  flex: 1;
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
  border-bottom: 20px solid rgb(8, 190, 8);
  transform: rotate(90deg);
  margin-right: 5px;
}

.run-button:hover::before {
  border-bottom-color: darkgreen;
}

.output-container {
  background-color: rgb(37, 37, 37);
  color: #d4d4d4;
  padding: 10px;
  border-top: 1px solid #ccc;
  max-height: 200px;
  overflow-y: auto;
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  box-sizing: border-box;
}

.terminal-container {
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 180px;
  background-color: #1e1e1e;
  color: #d4d4d4;
  border-top: 1px solid #333;
  box-sizing: border-box;
}

.terminal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: -10px 10px;
  background-color: #2d2d2d;
  border-bottom: 1px solid #333;
}

.close-button {
  background: none;
  border: none;
  color: #d4d4d4;
  cursor: pointer;
  font-size: 16px;
}

.close-button:hover {
  color: #ff5555;
}

.terminal-body {
  padding: 10px;
  overflow-y: auto;
}
</style>