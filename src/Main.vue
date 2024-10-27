<template>
    <main class="container">
      <span>getName(): {{ appname }} - getTauriVersion(): {{ appversion }}</span>
      <button @click="showApp">show</button>
      <button @click="hideApp">hide</button>
      <button @click="applyTheme('dark')">setDarkTheme</button>
      <button @click="applyTheme('light')">setTheme</button>
      <button @click="newWindow">newWindow</button>
      <button @click="newWindow1">newWindow1</button>
      <button @click="newWindow1CallRust">newWindow1CallRust</button>
      <span>{{ newWindow1CallRusttext }}</span>
      <button @click="close1">close1</button>
  
      <button @click="MainSendnewWindows">MainSendnewWindows</button>
      <button @click="fromRust">fromRust</button>
      <button @click="hideApp">hide</button>
    </main>
  </template>
  
  <script setup>
  import { ref, onMounted } from 'vue';
  
  // 定义响应式变量
  let appname = ref('');
  let appversion = ref('');
  let newWindow1CallRusttext = ref('');
  
  // 导入 Tauri API 函数
  import { getName, getTauriVersion, getVersion, hide, show, setTheme } from '@tauri-apps/api/app';
  import { invoke } from '@tauri-apps/api/core';
  import { emit } from '@tauri-apps/api/event';
  
  // 异步函数获取应用名称
  async function getMyName() {
    try {
      const name = await getName();
      const tauriVersion = await getTauriVersion();
      appname.value = name;
      console.log("App Name:", name,tauriVersion);
    } catch (error) {
      console.error("Error retrieving name:", error);
    }
  }
  
  // 异步函数获取 Tauri 版本
  async function getAppVersion() {
    try {
      const version = await getTauriVersion();
      appversion.value = version;
      console.log("Tauri Version:", version);
    } catch (error) {
      console.error("Error retrieving Tauri version:", error);
    }
  }
  
  
  // 在组件挂载时调用获取名称和版本
  onMounted(() => {
    getMyName();
    getAppVersion();
    myTray();
    // CreateTraymenu()
  });

  async function CreateTraymenu() {
    console.log('start create traymenu...')
    
    let webview = new WebviewWindow("traymenu", {
        url: "#/home",
        title: "消息通知",
        width: 150,
        height: 200,
        visible: false,

    })

    // 托盘消息事件
    webview.once('tauri://created', function () {
              console.log("createAboutWindow1");
          });
    await webview.listen('tauri://error', async(error) => {
        console.log('traymenu error!', error)
    })

    console.log(await getAllWebviewWindows())
    // 监听托盘事件
    webview.listen('tray_contextmenu', async (event) => {
        console.log(event)

        const win = await WebviewWindow.getByLabel('traymenu')
        if(!win) return

        let position = event.payload
        if(win) {
            await win.setAlwaysOnTop(true)
            await win.setFocus()
            await win.setPosition(new LogicalPosition(position.x, position.y - 100))
            await win.show()
        }
    })
}



  import { TrayIcon } from '@tauri-apps/api/tray';
import { defaultWindowIcon } from '@tauri-apps/api/app';

import { Menu } from '@tauri-apps/api/menu';

async function onTrayMenuClick(itemId) {
  console.log(itemId);
  let allws = await getAllWebviewWindows()
  console.log("tr",allws)
  allws[0].show()
}

async function myTray(){
  const menu = await Menu.new({
  items: [
      {
        id: 'quit',
        text: 'Quit',
        action: onTrayMenuClick,
      },
    ],
  });

  const options = {
    icon: await defaultWindowIcon(),
    menu,
    menuOnLeftClick: true,
    action: (event) => {
      switch (event.type) {
        case 'Click':
          console.log(
            `mouse ${event.button} button pressed, state: ${event.buttonState}`
          );
          
          break;
        case 'DoubleClick':
          console.log(`mouse ${event.button} button pressed`);
          emit("tray_contextmenu",{'event':event.rect})
          break;
        case 'Enter':
          console.log(
            `mouse hovered tray at ${event.rect.position.x}, ${event.rect.position.y}`
          );
          break;
        case 'Move':
          console.log(
            `mouse moved on tray at ${event.rect.position.x}, ${event.rect.position.y}`
          );
          break;
        case 'Leave':
          console.log(
            `mouse left tray at ${event.rect.position.x}, ${event.rect.position.y}`
          );
          break;
      }
    }
  };

  const tray = await TrayIcon.new(options);

  console.log("tray",tray);
}
  
  // 可选函数示例：隐藏应用窗口
  async function hideApp() {
    try {
      let allws = await getAllWebviewWindows()
      allws[0].hide()
      console.log("App window hidden");
    } catch (error) {
      console.error("Error hiding app:", error);
    }
  }
  
  // 可选函数示例：显示应用窗口
  async function showApp() {
    try {
      await show();
      console.log("App window shown");
    } catch (error) {
      console.error("Error showing app:", error);
    }
  }
  
  // 可选函数示例：设置主题
  async function applyTheme(theme) {
    try {
      await setTheme(theme);
      console.log(`Theme set to ${theme}`);
    } catch (error) {
      console.error("Error setting theme:", error);
    }
  }
  
  
  import { WebviewWindow ,getAllWebviewWindows} from "@tauri-apps/api/webviewWindow"
  async function newWindow(){
    const webview = new WebviewWindow('home', {
              label: "home",            // 窗口唯一label
              width: 400,
              height: 280,
              minWidth: 400,
              minHeight: 280,
              url: '#/home',
              decorations: true
          });
          webview.once('tauri://created', function () {
              console.log("createAboutWindow1");
          });
          webview.once('tauri://error', function (e) {
              console.log("createAboutWindow2",e);
          });
  
  }
  
  async function newWindow1(){
    const webview = new WebviewWindow('about', {
              label: "about",            // 窗口唯一label
              width: 400,
              height: 280,
              minWidth: 400,
              minHeight: 280,
              url: 'https://deelmind.com/',
              decorations: true
          });
          webview.once('tauri://created', function () {
              console.log("createAboutWindow1");
          });
          webview.once('tauri://error', function (e) {
              console.log("createAboutWindow2",e);
          });
  }
  
  async function close1(){
    let allws = await getAllWebviewWindows()
    allws[0].close()
    console.log(allws)
  }
  
  
//   async function MainSendnewWindows(){
//     const sendMessage = () => {
//         emit('message-from-main', { content: 'Hello from main window!' });
//     };
//   }

  const MainSendnewWindows = () => {
        emit('message-from-main', { content: 'Hello from main window!' });
    };
  
  async function newWindow1CallRust(){
    let allws = await getAllWebviewWindows();
    console.log(allws[0]);
    newWindow1CallRusttext.value = await invoke("newWindow1CallRust",{name:"newWindow1CallRust"});
  }

  import { listen } from '@tauri-apps/api/event';
  listen('vue-event', (event) => {
      console.log('Received event:', event.payload);
    });
  
  async function fromRust(){
    await invoke("trigger_vue_function");
  }
  
  </script>
  
  <style>
  .container {
    font-family: Arial, sans-serif;
    font-size: 1.2em;
    margin: 20px;
  }
  </style>
  