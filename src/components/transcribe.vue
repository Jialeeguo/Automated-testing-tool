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
              style="margin-left: 7px; background-color: rgb(245, 242, 242); height: 24px; width: 50px; color: red; font-size: 15px; text-align: center; vertical-align:middle;
              line-height:1px; border:1px; border-style: solid; border-radius:3px; border-color: rgb(226, 217, 217); ">...</button>
            <div style="margin: auto;"></div>
            <!--The drop-down box corresponds-->
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
      </div>
      <!--button-->
      <div style="background-color: rgb(245, 242, 242);">
        <div style="border: 2px solid #e3fdf5; margin: 5px; padding: 10px;">
          <div class="buttons-container">
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
        <button @click="recordWindow" class="button-font" style="width:80px; font-size:13px;">脚本编辑</button>
        <button @click="goToIde" class="button-font" style="width:80px; font-size:13px;">IDE界面</button>
      </div>

      <div style="display: flex;">
        <textarea v-model="log" rows="15" readonly class="log" id="steps"></textarea>
        <div style="margin: 0 5px;"></div>
        <textarea v-model="logs" rows="15" readonly class="log" id="steps1"></textarea>
      </div>
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

export default {
  data() {
    return {
      recording: false,
      pause: false,
      pause1: true, //F3，对应按钮的pause
      log: '',//左边截图框
      log_playback: '',
      screenshotting: false,
      selectedFileName: '请选择回放文件夹',
      textData: '', // 初始化文本数据
      filename: '',
      selectedFunctionKey10: 'F1',
      selectedFunctionKey3: 'F2',
      selectedFunctionKey1: 'F4',
      selectedFunctionKey5: 'F6',
      selectedFolder: null,//通过不通过选择
      Chinese: '8',
      recordstart: true,
      clickButton: false,
      isMarui: false,
      logs: '', //右边截图框
      isPlaybacking: false,//是否正在执行回放
      loggingEnabled: false,
      back: false,
      back1: false, //脚本编辑之前要判断是否选择了回放的路径
      recordStateChangeTime: null,
      selectedContent: false,//通过不通过状态选择，如果没有回放状态则为false，输出日志
      isTruescreenshotting: true, //判断在截图过程中是否可以终止录制
      pressedKey: '', //监听键盘按键
      isRecording: false,
      startTime: null,
      elapsedTime: 0,
    };
  },
  methods: {
    async goToIde() {

      // Create a unique label for each webview

      const webview = new WebviewWindow('uniqueLabel', {
        url: '#/ide',
      });

      webview.once('tauri://created', function () {
        // Webview created successfully
      });

      webview.once('tauri://error', function (e) {
        // An error occurred during webview window creation
        console.error('Webview error:', e);
      });


    },

    async startRecord() {
      // 开始录制
      this.startTime = new Date();
      this.isRecording = true;
      this.elapsedTime = 0;
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
        // const currentTime = new Date().toLocaleTimeString();
        // const elapsedTime = new Date() - this.recordStateChangeTime;
        // this.log += `本次录制时长 ${elapsedTime} 毫秒\n`;
        // this.log += `${'录制结束,生成脚本已保存到log文件夹下，下次录制时本次日志操作提示被清空！'} - [${currentTime}]\n`;
        this.recordStateChangeTime = null;
      }
    },

    async stopRecord() {
      //停止录制
      this.isTruescreenshotting = false;
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
        this.recordStateChangeTime = null;
      }

    },

    async resumeRecord() {
      //恢复回放
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
      //暂停回放
      if (this.recording) {
        this.pause = true;
        if (!this.loggingEnabled) {
          this.logIntervalId = setInterval(async () => {
            const currentTime = new Date().toLocaleTimeString();
            this.log += `${'录制被暂停，再次点击按钮将恢复录制'} - [${currentTime}]\n`;
          }, 1000);
        }
        invoke('pause_record');
      } else {
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'不在录制过程中，无法暂停录制'} - [${currentTime}]\n`;
      }
    },

    async startScreenshot() {
      //截屏
      if (this.recording) {
        if (this.clickButton == false) {
          this.clickButton = !this.clickButton
        }
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
    //1.0版本中选择文件夹的代码，已移用到recordWindow()。

    // async selectPlaybackFile() {
    //   //选择回放文件夹
    //   this.back1 = true;
    //   const selected = await open({
    //     directory: true,
    //     multiple: false,
    //     defaultPath: await appConfigDir(),
    //   });
    //   if (Array.isArray(selected)) {
    //   } else if (selected === null) {
    //   } else {
    //   }
    //   this.selectedFileName = selected;
    //   //获取文件路径的最后一个文件名，用来xhr.open(点击哪个就放哪个)
    //   const fullPath = selected;
    //   this.filename = fullPath.replace(/^.*[\\\/]/, '');
    // },


    playBack() {
      //回放函数
      this.logs = '';
      this.back = true;
      const filePath = this.selectedFileName;
      if (!invoke('check_file_exists', { filePath })) {
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'没有选择文件夹,请选择文件夹!'} - [${currentTime}]\n`;
      }
      invoke('dir_confirm', { filePath })
        .then((result) => {
          if (result && result.length > 0) {
            // 文件内容不为空的处理逻辑
            this.log = '';
            this.isPlaybacking = true;
            let currentIndex = 0;
            this.selectedContent = true;
            let intervalId = setInterval(() => {
              if (currentIndex < result.length) {
                this.log += `${result[currentIndex].join(' ')}\n`;
                currentIndex++;
              } else {
                clearInterval(intervalId);
                this.isPlaybacking = false;
                this.loadRecordResult();
              }
            }, 500);
          } else {
            const currentTime = new Date().toLocaleTimeString();
            this.log += `${'record.txt为空,请检查record.txt或目录'} - [${currentTime}]\n`;
            this.isPlaybacking = false;
          }
        })
        .catch((error) => {
          console.error('前端脚本读取出错:', error);
        });
      //选择语言
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
      this.back = false;
      invoke('playback_main', { filePath, lang: langValue });
    },

    async loadRecordResult() {
      // 加载 record_result.txt 文件内容
      this.log_playback = '';
      let resultXhr = new XMLHttpRequest(),
        resultOkStatus = document.location.protocol === "file:" ? 0 : 200;
      resultXhr.open("GET", `../Automated-testing/result/${this.filename}/record_result.txt`, false);
      resultXhr.overrideMimeType("text/html;charset=utf-8");
      resultXhr.onreadystatechange = () => {
        if (resultXhr.readyState === 4) {
          if (resultXhr.status === resultOkStatus) {
            this.$nextTick(() => {
              document.getElementById("steps1").value = resultXhr.responseText;
            });
          } else {
            // // 如果请求失败，将错误消息添加到日志中
            // this.log = "没有检测到任何移动轨迹，请查看record.txt是否为空\n";
          }
        }
      };
      resultXhr.send(null);
    },

    async recordWindow() {
      // 选择回放文件夹
      const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: await appConfigDir(),
      });
      const filePath = selected;
      console.log(filePath);
      this.selectedFileName = filePath;
      this.filename = filePath.replace(/^.*[\\\/]/, '');
      // 调用后端 `read_a_record` 命令，传递文件路径
      invoke('read_a_record', { filePath })
        .then((result) => {
          const promiseResult = result;
          const resultString = JSON.stringify(promiseResult);
          const resultEncoded = encodeURIComponent(resultString);
          const webview = new WebviewWindow('theUniqueLabel', {
            url: `script_edit.html?result=${resultEncoded}&filePath=${filePath}`, // 将结果作为查询参数传递
          });
          webview.once('tauri://created', function () {

          });

          webview.once('tauri://error', function (e) {

          });
        })
        .catch((error) => {
          console.error('An error occurred:', error);
        });
    },
    handleKeyDown(event) {
      //监听各种
      if (this.isPlaybacking) {
        const currentTime = new Date().toLocaleTimeString();
        this.log += `${'正在回放中，请等待回放完毕再进行录制'} - [${currentTime}]\n`;
      } else {
        const selectedValue = this.selectedFunctionKey10;
        if (event.key === selectedValue) {
          event.preventDefault();
          // this.startRecord();
          this.$nextTick(() => {
            const textarea = document.getElementById('steps');
            const currentTime = new Date().toLocaleTimeString();
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
      if (event.ctrlKey && event.key === 'o') {
        event.preventDefault();
        this.recordWindow();
      }

      if (event.ctrlKey && event.key === 'q') {
        event.preventDefault();
        this.startRecord();
      }
      if (event.ctrlKey && event.key === 'w') {
        event.preventDefault();
        this.stopRecord();
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
            this.log += `${'正在录制中，请关                                                                                闭录制并选择文件夹进行回放'} - [${currentTime}]\n`;
          }
        }
      }
      if (this.isRecording && event.key !== 'F1') {
        const currentTime = new Date();
        this.elapsedTime = currentTime - this.startTime;
        this.pressedKey = event.key;
        this.log += `${this.elapsedTime}ms,捕捉到键盘输入${this.pressedKey}\n`;
      }
    },

  },

  mounted() {
    // invoke('close_splashscreen');
    window.addEventListener("keydown", this.handleKeyDown);
    // listen('tran', (event) => {
    //   this.startRecord();
    // });
    listen('screen', (event) => {
      this.startScreenshot();
    });
    listen('opening', (event) => {
      this.recordWindow();
    });
    listen('run', (event) => {
      this.playBack();
    });
    listen('press-listen-keyboard', (event) => {

      console.log("press-listen-keyboard");
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