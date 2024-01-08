<template>
  <div style="background-color: rgb(245, 242, 242);  background: linear-gradient(135deg, #e3fdf5, #ffe6fa);">

    <div>

      <div style="background-color: rgb(245, 242, 242);  background: linear-gradient(135deg, #e3fdf5, #ffe6fa);">
        <div style="border: 2px solid #e3fdf5; margin: 5px; padding: 10px;">
          <label for="lang" style="font-size: 13px; color:rgb(168, 163, 163);  ">配置</label>
          <div style="float:right;font-size: 13px; color:rgb(165, 2, 2); ">注意：功能键选择不能重合，否则会崩溃</div>
          <div style="display: flex; align-items: center;">



            <label for="lang" class="ziti" style="color: rgb(0, 0, 0);">脚本</label>
            <select name="languages" id="lang" style="width: 140px;" value="请选择回放文件夹">

              <option value="请选择回放文件夹">{{ selectedFileName }}</option>
            </select>
            <button @click="selectPlaybackFile"
              style="margin-left: 7px; background-color: rgb(245, 242, 242); height: 24px; width: 50px; color: red; font-size: 15px; text-align: center; vertical-align:middle;line-height: 1px; border:1px; border-style: solid; border-radius: 3px; border-color: rgb(226, 217, 217); ">...</button>

            <div style="margin: auto;"></div>

            <form action="#">
              <label for="lang" class="ziti">翻译对应语种</label>
              <select v-model="Chinese" id="lang" style="width: 200px; ">
                <option :value="null" disabled>默认为中文</option>
                <option value="1">自动识别</option>
                <option value="2">英语</option>
                <option value="3">西班牙语</option>
                <option value="4">阿拉伯语</option>
                <option value="5">葡萄牙语</option>
                <option value="6">俄语</option>
                <option value="7">法语</option>
                <option value="8">中文</option>
              </select>

            </form>

          </div>



          <!-- <input type="file" id="docpicker" accept=".txt" /> -->

          <div style="display: flex; ">
            <form action="#">
              <label for="action" class="ziti">开始/终止录制</label>
              <select v-model="selectedFunctionKey10" id="lang" style="width: 200px;">
                <option :value="null" disabled>请选择功能键</option>
                <option value="F1">F1</option>

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
          <div style="display: flex; ">
            <form action="#">
              <label for="lang" class="ziti">暂停录制/恢复录制</label>
              <select v-model="selectedFunctionKey1" id="lang" style="width: 200px;">
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
            <form @keydown.prevent="handleKeyDown">
              <label for="lang" class="ziti">启动</label>
              <select v-model="selectedFunctionKey5" id="lang" style="width: 200px;">
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
        <div style="border: 2px solid #e3fdf5; margin: 5px; padding: 10px;">

          <div class="buttons-container">
            <!-- 添加按钮组 -->
            <button @click="recording ? stopRecord() : startRecord()" :disabled="screenshotting || isPlaybacking"
              class="button-font" id="startrecord" @mouseover="handleButtonMouseOver" @mouseout="handleButtonMouseOut">
              {{ recording ? '终止录制 ' : '开始录制 ' }}
            </button>
            <button @click="playBack" :disabled="recording || isPlaybacking" class="button-font"
              style="margin-left: 5px;">启动</button>
            <button @click="pause ? resumeRecord() : pauseRecord()" :disabled="!recording" class="button-font"
              id="pauserecord" style="margin-left: 5px;">
              {{ pause ? '恢复录制 ' : '暂停录制 ' }}
            </button>

            <button @click="startScreenshot" :disabled="!recording || isPlaybacking" class="button-font"
              id="startScreenshot" style="margin-left: 5px;">截图</button>
          </div>
        </div>
      </div>
    </div>
    <div style="border: 2px solid #e3fdf5; margin: 5px; padding: 10px;">
      <div class="log-container">
        <div class="log_log" style="font-size: 13px; color:rgb(146, 142, 142); ">操作步骤</div>
        <div style="margin: 0 167px;"></div>
        <div class="log_log" style="font-size: 13px; color:rgb(146, 142, 142);">对比脚本结果</div>
        <div>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</div>
        <form action="#">
          <select name="languages" id="lang" style="width: 140px;" v-model="selectedFolder" @change="playbackConfirm">
            <option :value="null" value="请选择回放文件夹" disabled selected>请判定测试用例状态</option>
            <option value="F1">通过</option>
            <option value="F2">不通过</option>
            <option value="F3">待定</option>
          </select>
        </form>
        <button @click="recordWindow">脚本编辑</button>
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
import { WebviewWindow } from '@tauri-apps/api/window'
import { appWindow } from "@tauri-apps/api/window"
import { open, ask, message } from '@tauri-apps/api/dialog';
import { appConfigDir } from '@tauri-apps/api/path';
// Open a selection dialog for directoriesChinese
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
      selectedFunctionKey1: 'F4',
      selectedFolder: null,//通过不通过选择
      Chinese: '8',
      selectedFunctionKey3: 'F2',
      selectedFunctionKey5: 'F6',
      selectedFunctionKey10: 'F1',//开始/终止录制默认
      recordstart: true,
      clickButton: false,
      isMarui: false,
      logs: '',
      isPlaybacking: false,//是否正在执行回放
      loggingEnabled: false,
      back: false,
      recordStateChangeTime: null,
      selectedContent: false,//通过不通过状态选择，如果没有回放状态则为false，输出日志
    };
  },
  methods: {

    async startRecord() {


      if (!this.recording) {
        this.log = '';

      }
      this.recordstart = !this.recordstart;
      const recordChangeTime = new Date();
      this.recording = !this.recording;
      if (this.recording) {
        this.logs = '';
        if (!this.recordStateChangeTime) {

          this.recordStateChangeTime = new Date();
        }
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'录制已开始'} - [${currentTime}]\n`;
        await invoke('start_record', { recordstart: this.recordstart });
      } else {
        console.log(this.recording + '是');
        const currentTime = new Date().toLocaleTimeString();


        const elapsedTime = new Date() - this.recordStateChangeTime;


        this.log += `本次录制时长 ${elapsedTime} 毫秒\n`;

        this.log += `${'录制结束,生成脚本已保存到log文件夹下，下次录制时本次日志操作提示被清空！'} - [${currentTime}]\n`;

        // 重置状态变化的时间戳，以便下一次记录
        this.recordStateChangeTime = null;
      }
    },

    async stopRecord() {
      this.logs = '';
      const recordChangeTime = new Date();
      this.recording = !this.recording;
      this.recordstart = !this.recordstart;

      if (this.pause) {
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'正在暂停录制中，无法终止录制，请先恢复录制再终止录制'} - [${currentTime}]\n`;

      } else {
        if (!this.recordStateChangeTime) {

          this.recordStateChangeTime = new Date();
        }
        invoke('record_end');
        const currentTime = new Date().toLocaleTimeString();


        const elapsedTime = new Date() - this.recordStateChangeTime;


        this.log += `本次录制时长 ${elapsedTime} 毫秒\n`;

        this.log += `${'录制结束,生成脚本已保存到log文件夹下，下次录制时本次日志操作提示被清空！'} - [${currentTime}]\n`;

        // 重置状态变化的时间戳，以便下一次记录
        this.recordStateChangeTime = null;
      }
    },

    async resumeRecord() {
      this.pause = false;

      clearInterval(this.logIntervalId);
      const currentTime = new Date().toLocaleTimeString();
      if (this.recording) {
        this.log += `${'录制已恢复'} - [${currentTime}]\n`;
        invoke('resume_record');
      } else {
        this.log += `${'不在录制过程中，无法暂停录制'} - [${currentTime}]\n`;
      }

    },

    async pauseRecord() {
      console.log('ss');
      if (this.recording) {
        this.pause = true;

        if (!this.loggingEnabled) {
          this.logIntervalId = setInterval(async () => {
            const currentTime = new Date().toLocaleTimeString();
            this.log += `${'录制被暂停，再次点击按钮将恢复录制'} - [${currentTime}]\n`;

            // 在每次输出后检查 loggingEnabled 是否为 true

          }, 1000);
        }

        invoke('pause_record');
      } else {
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'不在录制过程中，无法暂停录制'} - [${currentTime}]\n`;
      }
    },



    async startScreenshot() {
      if (this.recording) {
        if (this.clickButton == false) {
          this.clickButton = !this.clickButton
        }

        // 如果没有在录制，输出“没有录制”


        this.$nextTick(() => {
          const textarea = document.getElementById('steps');
          const currentTime = new Date().toLocaleTimeString();
          this.log += `${"请等待几秒，正在提取图片文字..."} - [${currentTime}]\n`;
          textarea.scrollTop = textarea.scrollHeight;

        });


        await invoke('start_screen', { clickButton: this.clickButton });
      } else {
        this.$nextTick(() => {
          if (this.back == false) {
            const textarea = document.getElementById('steps');
            const currentTime = new Date().toLocaleTimeString();
            console.log("ss");
            this.log += `${"不在录制过程中，无法截图"} - [${currentTime}]\n`;
            textarea.scrollTop = textarea.scrollHeight;
          }

        });

      }
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
    //回放函数
    playBack() {
      this.logs = '';
      this.back = true;
      let xhr = new XMLHttpRequest(),
        okStatus = document.location.protocol === "file:" ? 0 : 200;

      xhr.open("GET", `../Automated-testing/result/${this.filename}/record.txt`, false);
      xhr.overrideMimeType("text/html;charset=utf-8"); // 默认为 utf-8
      xhr.onreadystatechange = () => {
        if (xhr.readyState === 4) {
          if (xhr.status === okStatus) {

            // 请求成功，但是文件内容为空
            if (xhr.responseText.trim() === "") {
              const currentTime = new Date().toLocaleTimeString();
              this.log += `${'没有选择文件夹或record.txt为空,请选择文件夹或检查record.txt是否为空'} - [${currentTime}]\n`;

            } else {
              this.log = '';
              this.isPlaybacking = true;
              // 每隔一秒处理一行文本
              let lines = xhr.responseText.split('\n');
              let currentIndex = 0;
              this.selectedContent = true;
              let intervalId = setInterval(() => {
                if (currentIndex < lines.length) {
                  this.log += `${lines[currentIndex]}\n`;
                  currentIndex++;
                } else {
                  clearInterval(intervalId); // 所有行处理完毕，清除定时器


                  this.isPlaybacking = false; // 在处理完毕后设置为 false

                  this.loadRecordResult();
                }
              }, 500);
            }
          } else {
            // 如果请求失败，将错误消息添加到日志中steps
            if (!this.recording) {
              const currentTime = new Date().toLocaleTimeString();
              this.log += `${'文件夹选择错误，文件夹下没有包含record.txt,请重新选择'} - [${currentTime}]\n`;

            }
            this.isPlaybacking = false; // 在处理失败后也要设置为 false
          }
        }
      };


      xhr.send(null);

      // 调用回放后端函数
      const filePath = this.selectedFileName;
      let langValue;
      switch (this.Chinese) {
        case '1':
          langValue = 'auto';
          break;
        case '8':
          langValue = 'zh';
          break;
        case '2':
          langValue = 'en';
          break;
        case '7':
          langValue = 'fra';
          break;
        case '6':
          langValue = 'ru';
          break;
        case '3':
          langValue = 'spa';
          break;
        case '5':
          langValue = 'pt';
          break;
        case '4':
          langValue = 'ara';
          break;
        default:
          langValue = '';
          break;
      }

      console.log('txp:', this.Chinese);
      console.log('langValue:', langValue);
      this.back = false;
      invoke('playback_main', { filePath, lang: langValue, selectedFunctionKey: this.selectedFunctionKey });
    },

    //通过不通过触发函数
    playbackConfirm() {
      const filePath = this.selectedFileName;
      let playbackResult;
      switch (this.selectedFolder) {
        case 'F1':
          playbackResult = '通过';
          break;
        case 'F2':
          playbackResult = '不通过';
          break;
        case 'F3':
          playbackResult = '待定';
          break;
        default:
          langValue = '';
          break;
      }
      console.log('filePath:', filePath);
      console.log('playbackResult:', playbackResult);
      if (this.selectedContent) {
        invoke('playback_confirm', { filePath, playbackResult });
        this.selectedContent = false;
      } else {
        const currentTime = new Date().toLocaleTimeString();
        this.logs = '';
        this.logs += `${'没有回放任何脚本，不能判别测试用例状态，请回放脚本后再选择'} - [${currentTime}]\n`;

      }
    },


    async loadRecordResult() {
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
            this.$nextTick(() => {
              // 将 record_result.txt 的内容设置到 textarea
              document.getElementById("steps1").value = resultXhr.responseText;
            });

          } else {
            // // 如果请求失败，将错误消息添加到日志中
            // this.log = "没有检测到任何移动轨迹，请查看record.txt是否为空\n";
          }
        }
      };


      resultXhr.send(null);

      // const yes2 = await ask('测试用例状态通通过', { title: 'Tauri', type: 'warning' });
    },



    //脚本编辑窗口
    recordWindow() {
      console.log("record");
      const filePath = this.selectedFileName;
      console.log(filePath);
      invoke('read_a_record', { filePath })
        .then((result) => {
          console.log(result);
          // 获取 PromiseResult 数组
          const promiseResult = result;

          // 将结果写入新窗口
          const resultString = JSON.stringify(promiseResult);
          const resultEncoded = encodeURIComponent(resultString);
          console.log(resultString);

          const webview = new WebviewWindow('theUniqueLabel', {
            url: `script_edit.html?result=${resultEncoded}&filePath=${filePath}`, // 将结果作为查询参数传递
          });

          // 监听 webview 窗口创建成功事件
          webview.once('tauri://created', function () {
            // webview 窗口成功创建
          });
          // 监听 webview 窗口创建失败事件
          webview.once('tauri://error', function (e) {
            // 在 webview 窗口创建过程中发生错误
          });
        })
        .catch((error) => {
          console.error('An error occurred:', error);
        });
    },


    handleKeyDown(event) {


      if (this.isPlaybacking) {
        //正在回放就不能录制
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'正在回放中，请等待回放完毕再进行录制'} - [${currentTime}]\n`;


      } else {
        const selectedValue = this.selectedFunctionKey10;
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

      const selectedValue5 = this.selectedFunctionKey3;
      if (event.key === selectedValue5) {


        event.preventDefault();

        this.startScreenshot()
      }



      //开始恢复暂停下拉框监听

      const selectedValue = this.selectedFunctionKey1;
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

      //回放下拉框监听
      if (!this.recording) {
        const selectedValue = this.selectedFunctionKey5;
        if (event.key === selectedValue) {
          event.preventDefault();
          this.playBack();
        } else {
          if (this.recording) {
            const currentTime = new Date().toLocaleTimeString();
            this.log += `${'正在录制中，请关闭录制并选择文件夹进行回放'} - [${currentTime}]\n`;
          }
        }
      }
    },

  },
  mounted() {
    invoke('close_splashscreen');
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
textarea {
  width: 50%;
  margin-top: 10px;
  background-color: #e6e2e2;

}

button {
  margin: 5px;
  padding: 10px;
  cursor: pointer;
  background: linear-gradient(135deg, #e3fdf5, #ffe6fa);
  background-color: rgb(230, 227, 227);
  width: 172px;
  color: rgb(138, 123, 123);
  border: 1.5px;
  border-radius: 10px;
  border-style: solid;
  border-color: rgb(226, 217, 217);
  font-size: large;
  transition: background-color 0.5s, color 0.3s
}

.button-font:hover {
  background-color: #817b7b;
  background: linear-gradient(135deg, #abecd6, #fbed96);
  color: #ffffff;

}

.button-font:disabled {
  background-color: #c2baba;
  background: linear-gradient(135deg, #a8caba, #d8ccd5);
  color: rgb(145, 137, 137);

  cursor: not-allowed;

}

select {
  margin-bottom: 10px;
  margin-top: 10px;
  font-family: cursive, sans-serif;
  outline: 0;
  background: #ffffff;
  color: rgb(255, 0, 191);
  border: 1px solid rgb(226, 217, 217);
  padding: 4px;
  border-radius: 9px;
  background: linear-gradient(135deg, #e3fdf5, #ffe6fa);

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
  color: rgb(255, 0, 191);
  font-size: large;
  font-weight: 700;
  background-color: rgb(245, 242, 242);
  background: linear-gradient(135deg, #e3fdf5, #ffe6fa);
}

.log-container {
  display: flex;
  align-items: baseline;
  background: linear-gradient(135deg, #e3fdf5, #ffe6fa);

}

.log {
  display: flex;
  align-items: baseline;
  color: rgb(255, 0, 191);
  background-color: rgb(255, 255, 255);
  ;
  border: 7px solid rgb(241, 240, 240);
  padding: 4px;
  border-radius: 9px;
  box-shadow: 0 0 0 2px rgb(226, 217, 217);
  font-family: 'Gill Sans', 'Gill Sans MT', Calibri, 'Trebuchet MS', sans-serif;
  background: linear-gradient(135deg, #e9defa, #dbeffb);
  background: linear-gradient(135deg, #e3fdf5, #ffe6fa);
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