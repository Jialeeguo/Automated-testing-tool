<template>
  <div style="background-color: rgb(245, 242, 242);">
    <div>

      <div style="background-color: rgb(245, 242, 242);">
        <div style="border: 2px solid white; margin: 5px; padding: 10px;">
          <label for="lang" style="font-size: 13px; color:rgb(168, 163, 163);">配置</label>
          <div style="display: flex; align-items: center;">



            <label for="lang" class="ziti">脚本</label>
            <select name="languages" id="lang" style="width: 140px;" value="请选择回放文件夹">

              <option value="请选择回放文件夹">{{ selectedFileName }}</option>
            </select>
            <button @click="selectPlaybackFile"
              style="margin-left: 7px; background-color: rgb(245, 242, 242); height: 24px; width: 50px; color: red; font-size: 15px; text-align: center; vertical-align:middle;line-height: 1px; border:1px; border-style: solid; border-radius: 3px; border-color: red; ">...</button>

            <div style="margin: auto;"></div>

            <form action="#">
              <label for="lang" class="ziti">执行次数</label>
              <select name="languages" id="lang" style="width: 200px; " value="5">
                <option value="1">1</option>
                <option value="2">2</option>
                <option value="3">3</option>
                <option value="4">4</option>
                <option value="5">5</option>
                <option value="6">6</option>
                <option value="7">7</option>
                <option value="8">8</option>
                <option value="9">9</option>
                <option value="10">10</option>
              </select>

            </form>

          </div>



          <!-- <input type="file" id="docpicker" accept=".txt" /> -->

          <div style="display: flex; ">
            <form @keydown.prevent="handleKeyDown">
              <label for="lang" class="ziti">开始/暂停执行</label>
              <select v-model="selectedFunctionKey" id="lang" style="width: 200px;">
                <option :value="null" disabled>请选择功能键</option>
                <option value="F1">F1</option>
                <option value="F2">F2</option>
                <option value="F3">F3</option>
                <option value="F4">F4</option>
                <option value="F5">F5</option>
                <option value="F6">F6</option>
                <option value="F7">F7</option>
                <option value="F8">F8</option>
              </select>
            </form>

            <div style="margin: auto;"></div>
            <form action="#">
              <label for="action" class="ziti">开始/暂停录制</label>
              <select v-model="selectedFunctionKey1" id="lang" style="width: 200px;">
                <option :value="null" disabled>请选择功能键</option>
                <option value="F1">F1</option>
                <option value="F2" selected>F2</option>
                <option value="F3">F3</option>
                <option value="F4">F4</option>
                <option value="F5">F5</option>
                <option value="F6">F6</option>
                <option value="F7">F7</option>
                <option value="F8">F8</option>
              </select>
            </form>
          </div>
          <div style="display: flex; ">
            <form action="#">
              <label for="lang" class="ziti">终止录制</label>
              <select v-model="selectedFunctionKey2" id="lang" style="width: 200px;">
                <option :value="null" disabled>请选择功能键</option>
                <option value="F1">F1</option>
                <option value="F2">F2</option>
                <option value="F3" selected>F3</option>
                <option value="F4">F4</option>
                <option value="F5">F5</option>
                <option value="F6">F6</option>
                <option value="F7">F7</option>
                <option value="F8">F8</option>
              </select>
            </form>
            <div style="margin: auto;"></div>
            <form action="#">

              <label for="lang" class="ziti">截图</label>
              <select v-model="selectedFunctionKey3" name="languages" id="lang" style="width: 200px;">
                <option :value="null" disabled>请选择功能键</option>
                <option value="F1">F1</option>
                <option value="F2">F2</option>
                <option value="F3">F3</option>
                <option value="F4">F4</option>
                <option value="F5">F5</option>
                <option value="F6">F6</option>
                <option value="F7">F7</option>
                <option value="F8">F8</option>
              </select>
            </form>
          </div>
          <div>
          </div>
        </div>
      </div>
      <div>
        <!-- 添加日志输出文本框 -->

      </div>
      <div style="background-color: rgb(245, 242, 242);">
        <div style="border: 2px solid white; margin: 5px; padding: 10px;">

          <div class="buttons-container">
            <!-- 添加按钮组 -->
            <button @click="recording ? stopRecord() : startRecord()" :disabled="screenshotting || isPlaybacking"
              class="button-font" id="startrecord" @mouseover="handleButtonMouseOver" @mouseout="handleButtonMouseOut">
              {{ recording ? '终止录制 ' : '开始录制 ' }}
            </button>

            <button @click="playBack" :disabled="recording || isPlaybacking" class="button-font"
              style="margin-left: 12px;">启动</button>

            <button @click="pause ? resumeRecord() : pauseRecord()" :disabled="!recording" class="button-font"
              id="pauserecord" style="margin-left: 12px;">
              {{ pause ? '恢复录制 ' : '暂停录制 ' }}
            </button>

            <button @click="startScreenshot" :disabled="!recording || isPlaybacking" class="button-font"
              id="startScreenshot" style="margin-left: 12px;">截图</button>
          </div>
        </div>
      </div>
    </div>
    <div style="border: 2px solid white; margin: 5px; padding: 10px;">
      <div class="log-container">
        <div class="log_log" style="font-size: 13px; color:rgb(146, 142, 142); ">操作步骤</div>
        <div style="margin: 0 167px;"></div>
        <div class="log_log" style="font-size: 13px; color:rgb(146, 142, 142);">对比脚本结果</div>
      </div>

      <div style="display: flex;">
        <textarea v-model="log" rows="15" readonly class="log" id="steps"></textarea>
        <div style="margin: 0 5px;"></div>
        <textarea v-model="logs" rows="15" readonly class="log" id="steps1"></textarea>
      </div>
      <!-- 其他内容保持不变 -->
    </div>
  </div>
</template>

<script>
import { ref, reactive, onBeforeMount } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window"
import { open } from '@tauri-apps/api/dialog';
import { appConfigDir } from '@tauri-apps/api/path';
// Open a selection dialog for directories
import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import { listen } from '@tauri-apps/api/event';
// Read the text file in the `$APPCONFIG/app.conf` path
// const contents = await readTextFile('record.txt', { dir: BaseDirectory.AppConfig });
export default {
  data() {
    return {
      recording: false,
      pause: false,
      pause1: true, //F3，对应按钮的pause
      log: '',
      log_playback: '',
      screenshotting: false,
      selectedFileName: '请选择回放文件夹',
      textData: '', // 初始化文本数据
      filename: '',
      selectedFunctionKey: 'F1',//下拉框选择按钮回放按键,
      selectedFunctionKey1: 'F3',
      selectedFunctionKey2: 'F4',
      selectedFunctionKey3: 'F2',
      recordstart: true,
      clickButton: false,
      isMarui: false,
      logs: '',
      isPlaybacking: false,//是否正在执行回放
      loggingEnabled: false,

    };
  },
  methods: {

    async startRecord() {
      // 添加新的事件监听器

      if (!this.recording) {
        this.log = '';

      }
      this.recordstart = !this.recordstart;

      this.recording = !this.recording;
      if (this.recording) {
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'录制已开始'} - [${currentTime}]\n`;
        await invoke('start_record', { recordstart: this.recordstart });
      } else {
        console.log(this.recording + '是');
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'录制结束,已保存到log文件夹下，下次录制时将日志被清空！'} - [${currentTime}]\n`;


      }
    },

    async stopRecord() {
      this.recording = !this.recording;
      this.recordstart = !this.recordstart;
      console.log(this.pause);
      if (this.pause) {
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'正在暂停录制中，无法终止录制，请先恢复录制再终止录制'} - [${currentTime}]\n`;

      } else {

        invoke('record_end');
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'录制结束,已保存到log文件夹下，下次录制时将日志被清空！'} - [${currentTime}]\n`;
      }
    },

    async resumeRecord() {
      this.pause = false;

      clearInterval(this.logIntervalId);
      const currentTime = new Date().toLocaleTimeString();
      this.log += `${'录制已恢复'} - [${currentTime}]\n`;

      invoke('resume_record');
    },

    async pauseRecord() {
      this.pause = true;

      if (!this.loggingEnabled) {
        this.logIntervalId = setInterval(async () => {
          const currentTime = new Date().toLocaleTimeString();
          this.log += `${'录制被暂停，再次点击按钮或F4将恢复录制'} - [${currentTime}]\n`;

          // 在每次输出后检查 loggingEnabled 是否为 true

        }, 1000);
      }

      invoke('pause_record');
    },



    async startScreenshot() {

      if (this.clickButton == false) {
        this.clickButton = !this.clickButton
      }
      this.$nextTick(() => {
        const textarea = document.getElementById('steps');
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${"请等待几秒，正在提取图片文字..."} - [${currentTime}]\n`;
        textarea.scrollTop = textarea.scrollHeight;
        if (!this.recording) {
          // 如果没有在录制，输出“没有录制”
          const currentTime = new Date().toLocaleTimeString();
          this.log = '';
          this.log += `${"不在录制过程中，无法截图"} - [${currentTime}]\n`;
        }
      });


      await invoke('start_screen', { clickButton: this.clickButton });

    },
    //选择回放文件夹
    async selectPlaybackFile() {
      const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: await appConfigDir(),
      });
      if (Array.isArray(selected)) {
        // user selected multiple directories
      } else if (selected === null) {
        // user cancelled the selection
      } else {
        // user selected a single directory
      }
      this.selectedFileName = selected;
      //获取文件路径的最后一个文件名，用来xhr.open(点击哪个就放哪个)
      const fullPath = selected;
      this.filename = fullPath.replace(/^.*[\\\/]/, '');


    },

    playBack() {
      this.log = '';

      let xhr = new XMLHttpRequest(),
        okStatus = document.location.protocol === "file:" ? 0 : 200;

      xhr.open("GET", `../Automated-testing/result/${this.filename}/record.txt`, false);
      xhr.overrideMimeType("text/html;charset=utf-8"); // 默认为 utf-8
      xhr.onreadystatechange = () => {
        if (xhr.readyState === 4) {
          if (xhr.status === okStatus) {
            // 请求成功，但是文件内容为空
            if (xhr.responseText.trim() === "") {
              this.log += "没有选择文件夹或record.txt为空,请选择文件夹或检查record.txt是否为空\n";
            } else {
              this.isPlaybacking = true;
              // 每隔一秒处理一行文本
              let lines = xhr.responseText.split('\n');
              let currentIndex = 0;

              let intervalId = setInterval(() => {
                if (currentIndex < lines.length) {
                  this.log += `${lines[currentIndex]}\n`;
                  currentIndex++;
                } else {
                  clearInterval(intervalId); // 所有行处理完毕，清除定时器
                  this.isPlaybacking = false; // 在处理完毕后设置为 false
                }
              }, 500);
            }
          } else {
            // 如果请求失败，将错误消息添加到日志中
            if (!this.recording) {
              this.log += "文件夹选择错误，文件夹下没有包含record.txt,请重新选择\n";
            }
            this.isPlaybacking = false; // 在处理失败后也要设置为 false
          }
        }
      };
      // this.loadRecordResult();

      xhr.send(null);

      // 文本的内容
      const filePath = this.selectedFileName;
      invoke('playback_main', { filePath, selectedFunctionKey: this.selectedFunctionKey });
    },






    handleKeyDown(event) {


      if (this.isPlaybacking) {
        //正在回放就不能录制
        this.log += "正在回放中，请等待回放完毕再进行录制\n";
      } else {
        const selectedValue = this.selectedFunctionKey;
        if (event.key === selectedValue) {

          event.preventDefault();
          // 执行开始/停止录制逻辑
          this.startRecord();

          // 更新 log 数据
          this.$nextTick(() => {
            const textarea = document.getElementById('steps');
            const currentTime = new Date().toLocaleTimeString();

            // 添加特殊的日志，但仅当还没有添加过时
            if (!this.recording && !this.hasRefreshLog) {

            } else if (this.recording == false) {
              this.log += `${'录制结束,下次录制将刷新日志！'} - [${currentTime}]\n`;

            }

            textarea.scrollTop = textarea.scrollHeight;
          });
        }
      }
      //截图下拉框监听
      if (this.recording) {
        const selectedValue = this.selectedFunctionKey3;
        if (event.key === selectedValue) {


          event.preventDefault();

          this.startScreenshot()
        }

      } else {
        if (this.recording) {
          this.log += "不在录制过程中，请在录制过程中截图\n";
        }
      }


      if (this.recording) {
        const selectedValue = this.selectedFunctionKey1;
        this.isMarui = !this.isMarui;
        if (event.key === selectedValue) {
          if (this.pause1) {
            this.pauseRecord();
            this.pause1 = !this.pause1;
          }
          else {
            this.resumeRecord();
            this.pause1 = !this.pause1;
          }
        }
      } else {
        if(!this.isMarui){
          this.log += "不在录制过程中，无法暂停录制\n";
        }
      }

      if (event.key === "F6") {


        if (!this.recording) {

          // 阻止默认事件，以避免浏览器刷新页面
          event.preventDefault();
          // 调用 playback_main 方法
          this.playBack();
        } else {
          this.log += "正在录制中，请关闭录制并选择文件夹进行回放\n";
        }
      }
    },

  },
  mounted() {
    // 监听键盘按下事件
    window.addEventListener("keydown", this.handleKeyDown);
    listen('event-name', (event) => {
      const currentTime = new Date().toLocaleTimeString();
      this.log += `${"提取文字执行成功！请继续操作。"} - [${currentTime}]\n`;
    });
  },
  beforeDestroy() {
    // 在组件销毁前移除事件监听
    window.removeEventListener("keydown", this.handleKeyDown);
  },
};



</script>

<style scoped>
/* 添加一些基本样式，你可以根据需要进行修改 */
textarea {
  width: 50%;
  margin-top: 10px;
  background-color: #e6e2e2;
}

button {
  margin: 5px;
  padding: 10px;
  cursor: pointer;
  background-color: rgb(230, 227, 227);
  width: 172px;
  color: red;
  border: 1.5px;
  border-radius: 10px;
  border-style: solid;
  border-color: red;
  font-size: large;
  transition: background-color 0.5s, color 0.3s
}

.button-font:hover {
  background-color: #e60000;
  /* 悬停时背景色变化 */
  color: #ffffff;
  /* 悬停时文字颜色变化 */
}

.button-font:disabled {
  background-color: #c2baba;
  /* 禁用状态的背景色 */
  color: #fc0000;
  /* 禁用状态的文字颜色 */
  cursor: not-allowed;
  /* 显示禁用状态的鼠标样式 */
}

select {
  margin-bottom: 10px;
  margin-top: 10px;
  font-family: cursive, sans-serif;
  outline: 0;
  background: #ffffff;
  color: red;
  border: 1px solid red;
  padding: 4px;
  border-radius: 9px;
}

.ziti {
  font-family: 'Gill Sans', 'Gill Sans MT', Calibri, 'Trebuchet MS', sans-serif;
  color: rgb(47, 51, 50);
  font-size: smaller;
}

.log_log {
  font-family: 'Gill Sans', 'Gill Sans MT', Calibri, 'Trebuchet MS', sans-serif;
  color: rgb(47, 51, 50);
  font-size: smaller;
  color: rgb(255, 0, 0);

}

.button-font {
  font-family: Cambria, Cochin, Georgia, Times, 'Times New Roman', serif;
  color: rgb(236, 8, 20);
  font-size: large;
  font-weight: 700;
  background-color: rgb(245, 242, 242);

}

.log-container {
  display: flex;
  align-items: baseline;

  /* 这里使用 baseline 对齐文字的基线，你也可以使用其他值如 center，flex-start 等 */
}

.log {
  display: flex;
  align-items: baseline;
  color: rgb(255, 0, 0);
  background-color: rgb(255, 255, 255);
  ;
  border: 7px solid rgb(241, 240, 240);
  padding: 4px;
  border-radius: 9px;
  box-shadow: 0 0 0 2px rgb(255, 0, 0);
  font-family: 'Gill Sans', 'Gill Sans MT', Calibri, 'Trebuchet MS', sans-serif;
  /* 这里使用 baseline 对齐文字的基线，你也可以使用其他值如 center，flex-start 等 */
}

button:disabled {
  background-color: #c2baba;
  cursor: not-allowed;
}


body {
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto;
  height: 100vh;
  background-color: #f1f1f1;
}

input {
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto;
}



label {
  width: 150px;
  text-align: left;
  display: inline-block;
}
</style>