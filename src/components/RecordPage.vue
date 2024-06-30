<template>
  <div class="container" @mouseleave="startHideTimer" @mouseenter="clearHideTimer">
    <button @click="goBack" class="button go-back">录制结束</button>

    <button @click="startScreenshot" :disabled="!recording || isPlaybacking" class="button screenshot button-font" id="startScreenshot"
      style="margin-left: 5px;">截图
    </button>

    <button @click="pause ? resumeRecord() : pauseRecord()" :disabled="!recording" class="button pause-resume button-font" id="pauserecord"
      style="margin-left: 5px;">
      {{ pause ? '恢复录制 ' : '暂停录制 ' }}
    </button>
    <textarea v-model="secondLog" rows="5" readonly class="log" id="steps"></textarea>
  </div>
</template>

<script>
import { getCurrent, getAll } from '@tauri-apps/api/window';
import { emit, listen } from '@tauri-apps/api/event';

export default {
  name: '录制操作窗口',
  data() {
    return {
      recording: true, // 假设初始值为 true，实际值根据你的逻辑设置
      isPlaybacking: false, // 假设初始值为 false，实际值根据你的逻辑设置
      pause: false,
      secondLog: '', // 添加 secondLog 属性
      hideTimer: null // 隐藏计时器
    };
  },
  mounted() {
    listen('sync-log', event => {
      console.log('Received log content:', event.payload);
      this.secondLog = event.payload;
    });

    // 监听 show-child-window 事件
    listen('show-child-window', async (event) => {
      // console.log('接收到的内容：', event.payload); // 输出接收到的内容
      const currentWindow = await getCurrent();
      await currentWindow.show();
      this.clearHideTimer();
      this.startHideTimer();
    });

  },
  methods: {
    async goBack() {
      try {
        // 获取当前窗口
        const currentWindow = await getCurrent();
        // 获取所有窗口
        const allWindows = await getAll();
        // 获取主窗口
        const mainWindow = allWindows.find(win => win.label === 'main');

        if (mainWindow) {
          // 显示主窗口
          await mainWindow.show();
          // 发出 stop-record 事件
          await emit('stop-record');
        }
        // 关闭当前窗口
        await currentWindow.close();
      } catch (error) {
        console.error('Error in goBack:', error);
      }
    },
    async startScreenshot() {
      try {
        await emit('start-screenshot');
      } catch (error) {
        console.error('Error in startScreenshot:', error);
      }
    },
    async pauseRecord() {
      try {
        await emit('pause-record');
        this.pause = true;
        this.secondLog += `${'录制被暂停'}\n`;
      } catch (error) {
        console.error('Error in pauseRecord:', error);
      }
    },
    async resumeRecord() {
      try {
        await emit('resume-record');
        this.pause = false;
        this.secondLog += `${'录制已恢复'}\n`;
      } catch (error) {
        console.error('Error in resumeRecord:', error);
      }
    },
    startHideTimer() {
      this.clearHideTimer();
      this.hideTimer = setTimeout(async () => {
        try {
          const currentWindow = await getCurrent();
          await currentWindow.hide();
        } catch (error) {
          console.error('Error in startHideTimer:', error);
        }
      }, 3000); // 3秒后隐藏窗口
    },
    clearHideTimer() {
      if (this.hideTimer) {
        clearTimeout(this.hideTimer);
        this.hideTimer = null;
      }
    }
  },
};
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 10px;
}

.button {
  background-color: #42b983;
  border: none;
  color: white;
  padding: 10px 20px;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  font-size: 16px;
  margin: 10px 10px;
  cursor: pointer;
  transition-duration: 0.4s;
}

.button:hover {
  background-color: white;
  color: black;
  border: 2px solid #42b983;
}

.go-back {
  background-color: #e74c3c;
}

.go-back:hover {
  background-color: white;
  color: black;
  border: 2px solid #e74c3c;
}

.screenshot {
  margin-left: 5px;
}

.pause-resume {
  margin-left: 5px;
}

.log {
  width: 350px;
  height: 50px;
  margin-left: 10px;
  border: 1px solid #ccc;
  border-radius: 4px;
  resize: none;
  font-size: 14px;
}
</style>
