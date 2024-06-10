<template>
  <div>
    <button @click="goBack">Go Back</button>

    <button @click="startScreenshot" :disabled="!recording || isPlaybacking" class="button-font" id="startScreenshot"
      style="margin-left: 5px;">截图
    </button>

    <button @click="pause ? resumeRecord() : pauseRecord()" :disabled="!recording" class="button-font" id="pauserecord"
      style="margin-left: 5px;">
      {{ pause ? '恢复录制 ' : '暂停录制 ' }}
    </button>
    <textarea v-model="secondLog" rows="2" readonly class="log" id="steps"></textarea>
  </div>
</template>

<script>
import { getCurrent, getAll } from '@tauri-apps/api/window';
import { emit } from '@tauri-apps/api/event';

export default {
  name: '录制操作窗口',
  data() {
    return {
      recording: true, // 假设初始值为 true，实际值根据你的逻辑设置
      isPlaybacking: false, // 假设初始值为 false，实际值根据你的逻辑设置
      pause: false
    };
  },
  methods: {
    async goBack() {
      // 获取当前窗口
      const currentWindow = getCurrent();
      // 获取所有窗口
      const allWindows = await getAll();
      // 获取主窗口
      const mainWindow = allWindows.find(win => win.label === 'main');

      if (mainWindow) {
        // 显示主窗口
        await mainWindow.show();
      }
      // 隐藏当前窗口
      await currentWindow.close();
    },
    async startScreenshot() {
      await emit('start-screenshot');
    },
    async pauseRecord() {
      await emit('pause-record');
      this.pause = true;
      this.secondLog += `${'录制已恢复'}\n`;
    },
    async resumeRecord() {
      await emit('resume-record');
      this.pause = false;
      this.secondLog += `${'录制被暂停，再次点击按钮将恢复录制'}\n`;
    }
  }
};
</script>

<style scoped>
h1 {
  color: #42b983;
}
</style>