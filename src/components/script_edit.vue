<template>
    <div>
      <button @click="updateRecords">脚本保存</button>
      <table>
        <thead>
          <tr>
            <th>时间(ms)</th>
            <th>记录动作</th>
            <th>鼠标坐标/键盘按键</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(entry, index) in result" :key="index">
            <td v-for="(value, idx) in entry" :key="idx">
              <input v-model="result[index][idx]" />
            </td>
            <td>
              <button @click="addRow(index)">添加</button>
              <button @click="deleteRow(index)">删除</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>
  
  <script>
  export default {
    data() {
      return {
        result: [],
        path: '',
      };
    },
    mounted() {
      const queryParams = new URLSearchParams(window.location.search);
      const resultParam = queryParams.get('result');
      this.path = queryParams.get('filePath');
      const resultString = decodeURIComponent(resultParam);
      this.result = JSON.parse(resultString);
    },
    methods: {
      addRow(index) {
        const newRow = this.result[index].map(() => '');
        this.result.splice(index + 1, 0, newRow);
      },
      deleteRow(index) {
        this.result.splice(index, 1);
      },
      collectTableData() {
        const data = [];
        const rows = document.querySelectorAll('table tbody tr');
        rows.forEach(row => {
          const rowData = [];
          row.querySelectorAll('td input').forEach(input => {
            rowData.push(input.value);
          });
          data.push(rowData);
        });
        return data;
      },
      updateRecords() {
        const script = this.collectTableData();
        console.log("保存内容：", script);

        invoke('script_write_back', { path: this.path, script })
          .then(response => {
            console.log("保存成功");
          })
          .catch(error => {
            console.error("保存出错");
          });
      },
    },
  };
  </script>
  