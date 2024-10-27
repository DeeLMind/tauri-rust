<template>
    <main class="container">
        home
        <button @click="newWindow1CallRust">newWindow1CallRust</button>
        <span>{{ newWindow1CallRusttext }}</span>
    </main>
  </template>
  
  <script setup>
// 子窗口
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

let newWindow1CallRusttext = ref('');

const receiveMessage = () => {
  listen('message-from-main', (event) => {
    console.log('Received:', event.payload);
  });
};


// 调用函数开始监听
receiveMessage();

async function newWindow1CallRust(){
    newWindow1CallRusttext.value = await invoke("newWindow1CallRust",{name:"newWindow1CallRust"});
  }

  </script>
  
  <style>
  .container {
    font-family: Arial, sans-serif;
    font-size: 1.2em;
    margin: 20px;
  }
  </style>
  