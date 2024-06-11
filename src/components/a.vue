<script setup lang="ts">
import { ref, reactive, onBeforeMount } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
// import TauriWebsocket from 'tauri-plugin-websocket-api';
// import WebSocket from "tauri-plugin-websocket-api";
import {
  MouseStatus,
  WheelStatus,
  KeyboardStatus,
  MessageType,
  InputEventType,
} from "../common/Constans";
import { handleKeyboardEvent, handleMouseEvent } from "../common/InputEvent";
// 用于存储响应式数据的对象
const data = reactive({
  account: {
    id: "",
    password: "",
  },
  receiverAccount: {
    id: "",
    password: "",
  },
  isShowRemoteDesktop: false,
});

// 对象用于引用视频元素，DOM对象
const desktop = ref<HTMLVideoElement>();

// WebSocket 连接和RTC其他变量
let ws: WebSocket;
let pc: RTCPeerConnection;
let dc: RTCDataChannel;
let webcamStream: MediaStream;
//分辨率
let remoteDesktopDpi: Record<string, any>;

// 在组件挂载之前执行的异步操作
onBeforeMount(async () => {
  data.account = await invoke("generate_account");
  initWebSocket();
});

/********************************* connect *************************************/

// 初始化 WebSocket 连接
const initWebSocket = () => {
  ws = new WebSocket(`ws://172.20.10.5:8081/conn/${data.account.id}`);
 
  ws.onopen = (e: Event) => {
    // 向服务器发送心跳消息
    setInterval(() => {
      sendToServer({
        msg_type: "heartbeat",
        receiver: "",
        sender: "",
        msg: "",
      });
    }, 1000 * 60);
  };

  ws.onmessage = async (e: MessageEvent) => {
    const msg: Record<string, any> = JSON.parse(e.data);
    switch (msg.msg_type) {
      case MessageType.VIDEO_OFFER: // 视频通话邀请
        handleVideoOfferMsg(msg);
        break;
      case MessageType.VIDEO_ANSWER: // 对方已接受邀请
        handleVideoAnswerMsg(msg);
        break;
      case MessageType.NEW_ICE_CANDIDATE: // 收到新的 ICE 候选项
        handleNewICECandidateMsg(msg);
        break;
      case MessageType.REMOTE_DESKTOP:
        handleRemoteDesktopRequest(msg);
        break;
      case MessageType.CLOSE_REMOTE_DESKTOP:
        close();
        break;
    }
  };

  ws.onerror = (e: Event) => {
    console.log("WebSocket 连接错误:", e);
  };
};

// 处理视频邀请消息
const handleVideoOfferMsg = async (msg: Record<string, any>) => {
  data.receiverAccount.id = msg.sender;

  await initRTCPeerConnection();

  const desc = new RTCSessionDescription(JSON.parse(msg.msg));
  await pc.setRemoteDescription(desc);

  await pc.setLocalDescription(await pc.createAnswer());
  sendToServer({
    msg_type: MessageType.VIDEO_ANSWER,
    receiver: data.receiverAccount.id,
    msg: JSON.stringify(pc.localDescription),
    sender: data.account.id,
  });
};

// 处理视频回应消息
const handleVideoAnswerMsg = async (msg: Record<string, any>) => {
  const desc = new RTCSessionDescription(JSON.parse(msg.msg));
  await pc.setRemoteDescription(desc).catch(reportError);
};

// 处理新的 ICE 候选项消息
const handleNewICECandidateMsg = async (msg: Record<string, any>) => {
  const candidate = new RTCIceCandidate(JSON.parse(msg.msg));
  try {
    await pc.addIceCandidate(candidate);
  } catch (err) {
    reportError(err);
  }
};

// 处理远程桌面请求消息
const handleRemoteDesktopRequest = async (msg: Record<string, any>) => {
  if (msg.msg != data.account.password) {
    console.log("密码错误！");
    return;
  }

  data.receiverAccount.id = msg.sender;

  await initRTCPeerConnection();

  initRTCDataChannel();

  // 获取本地桌面流
  webcamStream = await navigator.mediaDevices.getDisplayMedia({
    video: true,
    audio: false,
  });

  webcamStream.getTracks().forEach((track: MediaStreamTrack) =>
    pc.addTrack(track, webcamStream)
  );

  sendOffer();
};

// 初始化 RTCPeerConnection
const initRTCPeerConnection = () => {
  const iceServer: object = {
    iceServers: [
      {
        url: "stun:stun.l.google.com:19302",
      },
      {
        url: "turn:numb.viagenie.ca",
        username: "webrtc@live.com",
        credential: "muazkh",
      },
    ],
  };

  pc = new RTCPeerConnection(iceServer);

  pc.onicecandidate = handleICECandidateEvent;
  pc.oniceconnectionstatechange = handleICEConnectionStateChangeEvent;
  pc.onicegatheringstatechange = handleICEGatheringStateChangeEvent;
  pc.onsignalingstatechange = handleSignalingStateChangeEvent;
  pc.ontrack = handleTrackEvent;
  pc.ondatachannel = handleDataChannel;
};

// 处理 ICE 候选项事件
const handleICECandidateEvent = (event: RTCPeerConnectionIceEvent) => {
  if (event.candidate) {
    sendToServer({
      msg_type: MessageType.NEW_ICE_CANDIDATE,
      receiver: data.receiverAccount.id,
      msg: JSON.stringify(event.candidate),
      sender: data.account.id,
    });
  }
};

// 处理 ICE 连接状态变化事件
const handleICEConnectionStateChangeEvent = (event: Event) => {
  console.log("*** ICE 连接状态变为" + pc.iceConnectionState);
};

// 处理 ICE 聚集状态变化事件
const handleICEGatheringStateChangeEvent = (event: Event) => {
  console.log("*** ICE 聚集状态变为" + pc.iceGatheringState);
};

// 处理 WebRTC 信令状态变化事件
const handleSignalingStateChangeEvent = (event: Event) => {
  console.log("*** WebRTC 信令状态变为: " + pc.signalingState);
};

// 获取数据流事件处理
const handleTrackEvent = (event: RTCTrackEvent) => {
  desktop.value!.srcObject = event.streams[0];

  document.onkeydown = (e: KeyboardEvent) => {
    sendToClient({
      type: InputEventType.KEY_EVENT,
      data: {
        eventType: KeyboardStatus.MOUSE_DOWN,
        key: e.key,
      },
    });
  };

  document.onkeyup = (e: KeyboardEvent) => {
    sendToClient({
      type: InputEventType.KEY_EVENT,
      data: {
        eventType: KeyboardStatus.MOUSE_UP,
        key: e.key,
      },
    });
  };
};

// 数据通道事件处理
const handleDataChannel = (e: RTCDataChannelEvent) => {
  dc = e.channel;
  dc.onopen = (e: Event) => {
    console.log("数据通道已打开");
  };

  dc.onmessage = (event: MessageEvent) => {
    remoteDesktopDpi = JSON.parse(event.data);
  };

  dc.onclose = (e: Event) => {
    console.log("数据通道已关闭");
  };

  console.log("数据通道:", dc);
};

// 初始化 WebRTC 数据通道
const initRTCDataChannel = () => {
  dc = pc.createDataChannel("my channel", {
    ordered: true,
  });

  dc.onopen = (e: Event) => {
    console.log("数据通道已打开");
    dc.send(
      JSON.stringify({
        width: window.screen.width * window.devicePixelRatio,
        height: window.screen.height * window.devicePixelRatio,
      })
    );
    console.log("数据通道:", dc);
  };

  dc.onmessage = (event: MessageEvent) => {
    let msg: Record<string, any> = JSON.parse(event.data);
    switch (msg.type) {
      case InputEventType.MOUSE_EVENT:
        handleMouseEvent(msg.data);
        break;
      case InputEventType.KEY_EVENT:
        handleKeyboardEvent(msg.data);
        break;
    }
  };

  dc.onclose = (e: Event) => {
    console.log("数据通道已关闭");
  };

  console.log("数据通道:", dc);
};

// 发送视频通话邀请
const sendOffer = async () => {
  const offer = await pc.createOffer();

  await pc.setLocalDescription(offer);

  sendToServer({
    msg_type: MessageType.VIDEO_OFFER,
    receiver: data.receiverAccount.id,
    msg: JSON.stringify(pc.localDescription),
    sender: data.account.id,
  });
};

/********************************* user event *************************************/

// 请求远程桌面
const remoteDesktop = async () => {
  if (!data.receiverAccount.id || !data.receiverAccount.password) {
    alert("请输入ID和密码");
    return;
  }

  appWindow.setFullscreen(true);

  // 显示远程桌面面板
  data.isShowRemoteDesktop = true;

  //将下下哦i发给被远程者
  sendToServer({
    msg_type: MessageType.REMOTE_DESKTOP,
    receiver: data.receiverAccount.id,
    msg: data.receiverAccount.password,
    sender: data.account.id,
  });
};

// 关闭远程桌面
const closeRemoteDesktop = async () => {
  appWindow.setFullscreen(false);
  data.isShowRemoteDesktop = false;

  close();

  sendToServer({
    msg_type: MessageType.CLOSE_REMOTE_DESKTOP,
    receiver: data.receiverAccount.id,
    msg: data.receiverAccount.password,
    sender: data.account.id,
  });
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

/********************************* common *************************************/

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

// 关闭远程桌面
const close = () => {
  if (desktop.value!.srcObject) {
    const tracks = desktop.value!.srcObject as MediaStream;
    tracks.getTracks().forEach((track: MediaStreamTrack) => track.stop());
    desktop.value!.srcObject = null;
  } else {
    webcamStream.getTracks().forEach((track: MediaStreamTrack) => track.stop());
  }
  // 关闭 Peer 连接
  pc.close();
};

// 发送消息给服务器
const sendToServer = (msg: Record<string, any>) => {
  let msgJSON = JSON.stringify(msg);
  ws.send(msgJSON);
};

// 发送消息给客户端
const sendToClient = (msg: Record<string, any>) => {
  let msgJSON = JSON.stringify(msg);
  dc.readyState == "open" && dc.send(msgJSON);
};

</script>

<template>
  <div class="sidebar">
    <div>
      <p>
        id: <span>{{ data.account.id }}</span>
      </p>
      <p>
        password: <span>{{ data.account.password }}</span>
      </p>
    </div>
  </div>
  <div class="form">
    <input v-model="data.receiverAccount.id" type="text" placeholder="请输入对方id" />
    <input v-model="data.receiverAccount.password" type="text" placeholder="请输入对方密码" />
    <button @click="remoteDesktop()">发起远程</button>
  </div>
  <video v-show="data.isShowRemoteDesktop" @mousedown="mouseDown($event)" @mouseup="mouseUp($event)"
    @mousemove="mouseMove($event)" @wheel="wheel($event)" @contextmenu.prevent="rightClick($event)" class="desktop"
    ref="desktop" autoplay></video>
  <button v-if="data.isShowRemoteDesktop" class="close-btn" @click="closeRemoteDesktop()">
    关闭
  </button>
</template>

<style lang="less" scoped>
.sidebar {
  width: 100%;
  height: 160px;
  background: #1b1b1c;
  color: #fafafa;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  border-bottom: 1px solid #252525;
  box-sizing: border-box;

  >div {
    background: #242425;
    padding: 10px 20px;
    border-radius: 10px;

    p {
      line-height: 28px;
      font-size: 16px;

      span {
        font-size: 18px;
        font-weight: 600;
      }
    }
  }
}

.form {
  height: calc(100% - 160px);
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background: #1b1b1c;

  button {
    width: 280px;
    height: 34px;
    background: #00c1cd;
  }
}

input {
  width: 280px;
  outline: none;
  border: 1px solid #252525;
  padding: 0 10px;
  height: 34px;
  box-sizing: border-box;
  border-radius: 5px;
  margin-bottom: 30px;
}

button {
  outline: none;
  border: none;
  color: #fff;
  border-radius: 5px;
}

.desktop {
  width: 100%;
  height: 100%;
  position: fixed;
  top: 0;
  left: 0;
  background: #121212;
  cursor: none;
}

.close-btn {
  width: 40px;
  height: 24px;
  position: fixed;
  right: 20px;
  bottom: 20px;
  z-index: 1;
  background: #d71526;
  font-size: 12px;
}
</style>

<script setup lang="ts">
// @ts-ignore
import { Editor as MonacoTreeEditor, useMessage, useHotkey, useMonaco, type Files } from '~lib'
import { ComputedRef, onMounted, onUnmounted, ref,onBeforeMount,reactive } from 'vue'
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
import { handleKeyboardEvent, handleMouseEvent } from "../common/InputEvent";
// import 'video.js/dist/video-js.css';
// import videojs from 'video.js';
import {
  MouseStatus,
  WheelStatus,
  KeyboardStatus,
  MessageType,
  InputEventType,
} from "../common/Constans";
import { Editor } from 'monaco-tree-editor'
const data = reactive({
  account: {
    id: "00:FF:59:CB:C2:BE",
    password: "1",
  },
  receiverAccount: {
    id: "00:FF:59:CB:C2:BE",
    password: "1",
  },
  isShowRemoteDesktop: false,
});
let dc: RTCDataChannel;
let pc: RTCPeerConnection;
// WebSocket 连接和RTC其他变量
let ws: WebSocket;
let webcamStream: MediaStream;
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
const handleKeyDown = (event: KeyboardEvent) => {
  console.log("ss");
  if (event.key === 'F1') {
    event.preventDefault();
    startRecord();
  }
};
//共享屏幕+远程控制开始
const initWebSocket = () => {
  console.log("初始化initWebSocket");
  ws = new WebSocket(`ws://10.134.169.24:8081/conn/${data.account.id}`);
 
  ws.onopen = (e: Event) => {
    // 向服务器发送心跳消息
    setInterval(() => {
      sendToServer({
        msg_type: "heartbeat",
        receiver: "",
        sender: "",
        msg: "",
      });
    }, 1000 * 60);
  };
  ws.onmessage = async (e: MessageEvent) => {
    const msg: Record<string, any> = JSON.parse(e.data);
    switch (msg.msg_type) {
      case MessageType.VIDEO_OFFER: // 视频通话邀请
        handleVideoOfferMsg(msg);
        break;
      case MessageType.VIDEO_ANSWER: // 对方已接受邀请
        handleVideoAnswerMsg(msg);
        break;
      case MessageType.NEW_ICE_CANDIDATE: // 收到新的 ICE 候选项
        handleNewICECandidateMsg(msg);
        break;
      case MessageType.REMOTE_DESKTOP:
        handleRemoteDesktopRequest(msg);
        console.log("收到了handle消息");
        break;
      case MessageType.CLOSE_REMOTE_DESKTOP:
        close();
        break;
    }
  };
  ws.onerror = (e: Event) => {
    console.log("WebSocket 连接错误:", e);
  };
};
const handleVideoOfferMsg = async (msg: Record<string, any>) => {
  data.receiverAccount.id = msg.sender;
  await initRTCPeerConnection();
  const desc = new RTCSessionDescription(JSON.parse(msg.msg));
  await pc.setRemoteDescription(desc);
  await pc.setLocalDescription(await pc.createAnswer());
  sendToServer({
    msg_type: MessageType.VIDEO_ANSWER,
    receiver: data.receiverAccount.id,
    msg: JSON.stringify(pc.localDescription),
    sender: data.account.id,
  });
};
// 处理视频回应消息
const handleVideoAnswerMsg = async (msg: Record<string, any>) => {
  const desc = new RTCSessionDescription(JSON.parse(msg.msg));
  await pc.setRemoteDescription(desc).catch(reportError);
};
// 处理新的 ICE 候选项消息
const handleNewICECandidateMsg = async (msg: Record<string, any>) => {
  const candidate = new RTCIceCandidate(JSON.parse(msg.msg));
  try {
    await pc.addIceCandidate(candidate);
  } catch (err) {
    reportError(err);
  }
};
// 处理远程桌面请求消息
const handleRemoteDesktopRequest = async (msg: Record<string, any>) => {
  if (msg.msg != data.account.password) {
    console.log("密码错误！");
    return;
  }
  data.receiverAccount.id = msg.sender;
  await initRTCPeerConnection();
  initRTCDataChannel();
  // 获取本地桌面流
  webcamStream = await navigator.mediaDevices.getDisplayMedia({
    video: true,
    audio: false,
  });
  webcamStream.getTracks().forEach((track: MediaStreamTrack) =>
    pc.addTrack(track, webcamStream)
  );
  sendOffer();
};
const initRTCPeerConnection = () => {
  const iceServer: object = {
    iceServers: [
      {
        url: "stun:stun.l.google.com:19302",
      },
      {
        url: "turn:numb.viagenie.ca",
        username: "webrtc@live.com",
        credential: "muazkh",
      },
    ],
  };
  pc = new RTCPeerConnection(iceServer);
  pc.onicecandidate = handleICECandidateEvent;
  pc.oniceconnectionstatechange = handleICEConnectionStateChangeEvent;
  pc.onicegatheringstatechange = handleICEGatheringStateChangeEvent;
  pc.onsignalingstatechange = handleSignalingStateChangeEvent;
  pc.ontrack = handleTrackEvent;
  pc.ondatachannel = handleDataChannel;
  console.log("初始化pc通道成功");
};
// 处理 ICE 候选项事件
const handleICECandidateEvent = (event: RTCPeerConnectionIceEvent) => {
  if (event.candidate) {
    sendToServer({
      msg_type: MessageType.NEW_ICE_CANDIDATE,
      receiver: data.receiverAccount.id,
      msg: JSON.stringify(event.candidate),
      sender: data.account.id,
    });
  }
};
// 处理 ICE 连接状态变化事件
const handleICEConnectionStateChangeEvent = (event: Event) => {
  console.log("*** ICE 连接状态变为" + pc.iceConnectionState);
};
// 处理 ICE 聚集状态变化事件
const handleICEGatheringStateChangeEvent = (event: Event) => {
  console.log("*** ICE 聚集状态变为" + pc.iceGatheringState);
};
// 处理 WebRTC 信令状态变化事件
const handleSignalingStateChangeEvent = (event: Event) => {
  console.log("*** WebRTC 信令状态变为: " + pc.signalingState);
};
// 获取数据流事件处理
const handleTrackEvent = (event: any) => {
  console.log(event.streams);

  const stream = event.streams[0];
  addVideo(stream);

  // 使用 nextTick 来确保 DOM 更新
  nextTick(() => {
    const elems = videoElements.value; // 确保你有正确的 ref 指向视频元素数组
    const elem: any = elems[elems.length - 1];
    if (elem) {
      elem.srcObject = stream;
    } else {
      console.error("Video element not found");
    }

    document.onkeydown = (e: KeyboardEvent) => {
      sendToClient({
        type: InputEventType.KEY_EVENT,
        data: {
          eventType: KeyboardStatus.MOUSE_DOWN,
          key: e.key,
        },
      }); 
    };

    document.onkeyup = (e: KeyboardEvent) => {
      sendToClient({
        type: InputEventType.KEY_EVENT,
        data: {
          eventType: KeyboardStatus.MOUSE_UP,
          key: e.key,
        },
      });
    };
  });
};
const sendToClient = (msg: Record<string, any>) => {
  let msgJSON = JSON.stringify(msg);
  dc.readyState == "open" && dc.send(msgJSON);
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
hotkeyStore.listen('root', (event: KeyboardEvent) => { })
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

const handleDataChannel = (e: RTCDataChannelEvent) => {
  dc = e.channel;
  dc.onopen = (e: Event) => {
    console.log("数据通道已打开");
  };
  dc.onmessage = (event: MessageEvent) => {
    console.log("收到数据通道消息:", event.data);
  };
  dc.onclose = (e: Event) => {
    console.log("数据通道已关闭");
  };
  console.log("数据通道:", dc);
};
const initRTCDataChannel = () => {
  dc = pc.createDataChannel("my channel", {
    ordered: true,
  });
  dc.onopen = (e: Event) => {
    console.log("数据通道已打开");
    dc.send(
      JSON.stringify({
        width: window.screen.width * window.devicePixelRatio,
        height: window.screen.height * window.devicePixelRatio,
      })
    );
    console.log("数据通道:", dc);
  };
  dc.onmessage = (event: MessageEvent) => {
    let msg: Record<string, any> = JSON.parse(event.data);
    switch (msg.type) {
      case InputEventType.MOUSE_EVENT:
        handleMouseEvent(msg.data);
        console.log("检测到鼠标");
        break;
      case InputEventType.KEY_EVENT:
        handleKeyboardEvent(msg.data);
        break;
    }
  };
  dc.onclose = (e: Event) => {
    console.log("数据通道已关闭");
  };
  console.log("数据通道:", dc);
};
const sendOffer = async () => {
  const offer = await pc.createOffer();
  await pc.setLocalDescription(offer);
  sendToServer({
    msg_type: MessageType.VIDEO_OFFER,
    receiver: data.receiverAccount.id,
    msg: JSON.stringify(pc.localDescription),
    sender: data.account.id,
  });
};
const remoteDesktop = async () => {

  // 显示远程桌面面板
  data.isShowRemoteDesktop = true;
  //发给被远程者

};

const closeRemoteDesktop = async () => {
  data.isShowRemoteDesktop = false;
  close();
  sendToServer({
    msg_type: MessageType.CLOSE_REMOTE_DESKTOP,
    receiver: data.receiverAccount.id,
    msg: data.receiverAccount.password,
    sender: data.account.id,
  });
};
const close = () => {
  if (desktop.value!.srcObject) {
    const tracks = desktop.value!.srcObject as MediaStream;
    tracks.getTracks().forEach((track: MediaStreamTrack) => track.stop());
    desktop.value!.srcObject = null;
  } else {
    webcamStream.getTracks().forEach((track: MediaStreamTrack) => track.stop());
  }
  // 关闭 Peer 连接
  pc.close();
};
// 发送消息给服务器
const sendToServer = (msg: Record<string, any>) => {
  let msgJSON = JSON.stringify(msg);
  ws.send(msgJSON);
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
    sendToServer({
    msg_type: MessageType.REMOTE_DESKTOP,
    receiver: data.receiverAccount.id,
    msg: data.receiverAccount.password,
    sender: data.account.id,
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
  // 直接使用原始坐标发送鼠标事件
  let data = {
    x: x,  // 使用视频显示元素上的直接坐标
    y: y,
    eventType: eventType,
  };
};

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
onBeforeMount(async () => {

  initWebSocket();
});

</script>

<template>

  <MonacoTreeEditor :font-size="16" :files="files" :sider-min-width="240" filelist-title="源代码存储管理库" language="en-US"
    @reload="handleReload" @new-file="handleNewFile" @new-folder="handleNewFolder" @save-file="handleSaveFile"
    @delete-file="handleDeleteFile" @delete-folder="handleDeleteFolder" @rename-file="handleRename"
    @rename-folder="handleRename" :file-menu="fileMenu" :folder-menu="folderMenu"
    @contextmenu-select="handleContextMenuSelect" :settings-menu="settingsMenu" @drag-in-editor="handleDragInEditor"
    ref="editorRef">
  </MonacoTreeEditor>

  <video ref="desktop" width=640 height=480 controls class="resizable-video" style="background-color: black;"
    @mousedown="mouseDown($event)" @mouseup="mouseUp($event)" @mousemove="mouseMove($event)" @wheel="wheel($event)"
    @contextmenu.prevent="rightClick($event)" autoplay></video>

  <button @click="startScreenSharing()">共</button>
  <button v-if="data.isShowRemoteDesktop" class="close-btn" @click="closeRemoteDesktop()">
    关闭
  </button>
</template>

<style>
/* 添加可拉伸样式类 */
.resizable-video {
  resize: both;
  /* 允许水平和垂直拉伸 */
  overflow: hidden;
  /* 隐藏溢出部分 */
  border: 1px solid #ccc;
  /* 添加边框 */
}
</style>