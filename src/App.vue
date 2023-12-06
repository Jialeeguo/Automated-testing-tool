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
            <form action="#">
              <label for="lang" class="ziti">开始/暂停执行</label>
              <select name="languages" id="lang" v-model="selectedFunctionKey" style="width: 200px;">
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
              <select name="languages" id="lang" style="width: 200px;">
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
              <select name="languages" id="lang" style="width: 200px;">
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
              <select v-model="selectedFunctionKey4" name="languages" id="lang" style="width: 200px;">
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
            <button @click="startRecord" :disabled="screenshotting" class="button-font" id="startrecord"
              onmouseover="this.style.backgroundColor='#199991';" onmouseout="this.style.backgroundColor='#FFFFFF';">
              {{ recording ? '终止录制 ' : '开始录制 ' }}
            </button>
            <button @click="playBack" :disabled="recording" class="button-font" style="margin-left: 12px;">启动</button>


            <button @click="stopRecord" :disabled="!recording" class="button-font"
              style="margin-left: 12px;">暂停录制</button>
            <button @click="startScreenshot" :disabled="!recording" class="button-font" id="startScreenshot"
              style="margin-left: 12px;">截图

            </button>
  
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
      log: '',
      log_playback: '',
      screenshotting: false,
      selectedFileName: '请选择回放文件夹',
      textData: '', // 初始化文本数据
      filename: '',
      selectedFunctionKey: 'F5',//下拉框选择按钮回放按键,
      selectedFunctionKey4: 'F7',
      recordstart: true,//开始录制的状态，还没改好
      clickButton: false,
      logs:'',
    };
  },
  methods: {
    async startRecord() {
      listen('event-name', (event) => {
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${"提取文字执行成功！请继续操作。"} - [${currentTime}]\n`;
      });
      if (!this.recording) {
        this.log = '';
        //终止的话他就是true，然后传给后端
      }
      this.recordstart = !this.recordstart;
      console.log(this.recordstart);
      this.recording = !this.recording;
      if (this.recording) {
        console.log(this.recording + '高可儿');
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'录制已开始'} - [${currentTime}]\n`;
        await invoke('start_record', { recordstart: this.recordstart });

      } else {
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'录制结束,已保存到log文件夹下，下次录制时将日志被清空！'} - [${currentTime}]\n`;
        console.group("录制结束,已保存到log文件夹下，下次录制时日志将被清空！")

      }
    },

    async startScreenshot() {
      if(this.clickButton == false){
      this.clickButton = !this.clickButton
    }
      this.$nextTick(() => {
        const textarea = document.getElementById('steps');
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${"请等待几秒，正在提取图片文字..."} - [${currentTime}]\n`;
        // setTimeout(() => {
        //   const currentTime = new Date().toLocaleTimeString();
        //   this.log += `${"提取文字执行成功，请继续操作"} - [${currentTime}]\n`;
        // }, 10000)
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

      // 更新数据 market_id.txt 文件接口
      let xhr = new XMLHttpRequest(),
        okStatus = document.location.protocol === "file:" ? 0 : 200;

      xhr.open("GET", `../Automated-testing/result/${this.filename}/record.txt`, false);
      xhr.overrideMimeType("text/html;charset=utf-8"); // 默认为 utf-8
      xhr.onreadystatechange = () => {
        if (xhr.readyState === 4) {
          if (xhr.status === okStatus) {
            // 每隔一秒处理一行文本
            let lines = xhr.responseText.split('\n');
            let currentIndex = 0;


            let intervalId = setInterval(() => {
              if (currentIndex < lines.length) {
                this.log += `${lines[currentIndex]}\n`;
                currentIndex++;
              } else {
                clearInterval(intervalId); // 所有行处理完毕，清除定时器

                this.loadRecordResult();
              }
            }, 500);

          } else {
            // 如果请求失败，将错误消息添加到日志中
            this.log += "文件夹选择错误，找不到record.txt文件，请重新选择\n";
          }
        }
      };

      xhr.send(null);

      // 文本的内容
      const filePath = this.selectedFileName;
      invoke('playback_main', { filePath, selectedFunctionKey: this.selectedFunctionKey });
    },


    loadRecordResult() {
      this.log_playback = '';

      // 加载 record_result.txt 文件内容
      let resultXhr = new XMLHttpRequest(),
        resultOkStatus = document.location.protocol === "file:" ? 0 : 200;

      resultXhr.open("GET", `../Automated-testing/result/${this.filename}/record_result.txt`, false);
      resultXhr.overrideMimeType("text/html;charset=utf-8");

      resultXhr.onreadystatechange = () => {
        if (resultXhr.readyState === 4) {
          if (resultXhr.status === resultOkStatus) {
            // 如果 record_result.txt 的内容为空，输出日志

            // 将 record_result.txt 的内容设置到 textarea
            document.getElementById("steps1").value = resultXhr.responseText;

          } else {
            // 如果请求失败，将错误消息添加到日志中
            this.log = "没有检测到任何移动轨迹，请查看record.txt是否为空\n";
          }
        }
      };

      resultXhr.send(null);
    },



    handleKeyDown(event) {
      // 检查是否按下 F1 键
      if (event.key === "F1") {
        // 阻止默认事件，以避免浏览器刷新页面
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
      if (event.key === "F2") {

      
          event.preventDefault();
          // 执行开始/停止录制逻辑
          this.startScreenshot()

        
      }
      if (event.key === "F6") {
        // 阻止默认事件，以避免浏览器刷新页面
        event.preventDefault();
        // 调用 playback_main 方法
        this.playBack();
      }
    },

  },
  mounted() {
    // 监听键盘按下事件
    window.addEventListener("keydown", this.handleKeyDown);

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