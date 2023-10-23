<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import QRCode from 'qrcodejs2-fix';


const status = ref('打开手机客户端扫码登录');
const value = ref(0);
onMounted(() => {
    login();
});

const newwindow = () => {
    invoke('newdanmu', { room_id: value.value.toString() });
}

var login = async () => {
    const get_qr = setInterval(() => {
        invoke('getqr').then((url) => {
            document.getElementById("qrCodeBox")!.innerHTML = "";
            new QRCode(document.getElementById("qrCodeBox"), {
                text: url,
                width: 300,
                height: 300,
                correctLevel: QRCode.CorrectLevel.H
            });
        });
    }, 500);
    await invoke('login').then(() => { status.value = '输入房间号点击新窗口'; document.getElementById("qrCodeBox")!.innerHTML = ""; }).catch((error) => status.value = error);
    clearInterval(get_qr);
};
</script>

<template>
    <div id="qrCodeBox" class="qr"></div>
    <n-tag>
        {{ status }}
    </n-tag>
    <div class="num">
        <n-input-number v-model:value="value" clearable :keyboard="{ ArrowUp: false, ArrowDown: false }"
            :show-button="false" />
    </div>
    <n-button @click="newwindow()" type="primary">新窗口</n-button>
</template>

<style>
.qr {
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 20px;
}

.num {
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 20px;
}
</style>