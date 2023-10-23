<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import InfiniteList from 'vue3-infinite-list';
import { useRoute } from 'vue-router';
const { params } = useRoute();

const danmus = ref<Danmu[]>([{ username: '', level: '', content: '', emotion: '', time: '' }]);
interface Danmu {
  username: string,
  level: string,
  content: string,
  emotion: string,
  time: string
}
var timestamp = (time: string) => {
  var timestamp = parseInt(time);
  var date = new Date(timestamp * 1000);
  let Y = date.getFullYear() + '-';
  let M = (date.getMonth() + 1 < 10 ? '0' + (date.getMonth() + 1) : date.getMonth() + 1) + '-';
  let D = date.getDate() + ' ';
  let h = date.getHours() + ':';
  let m = date.getMinutes() + ':';
  let s = date.getSeconds();
  return Y + M + D + h + m + s;
}
const id = params.id;

onMounted(() => {
  init();
  getdanmu();
});
const getdanmu = () => {
  setInterval(async () => {
    await invoke('get_danmu', { room_id: id }).then((data) => {
      danmus.value = data as Danmu[];
    })
  }, 500)
};

const init = async () => {
  await invoke('new_connect', { room_id: id });
}


</script>

<template>
  <InfiniteList :data="danmus" :width="'100%'" :height="500" :itemSize="50" v-slot="{ item }" class="list"
    :scrollToIndex="danmus.length - 1">
    <n-thing content-style="margin-top: 10px;">
      <!-- <n-image width="35" preview-disabled :src="item.avatar" /> -->
      <n-tag :bordered="false" type="info" size="small">
        {{ item.username }}
      </n-tag>
      <n-tag :bordered="false" type="info" size="small">
        {{ item.level }}
      </n-tag>
      <n-tag :bordered="false" type="info" size="small">
        {{ timestamp(item.time) }}
      </n-tag>
      {{ item.content }}
      <n-image width="35" preview-disabled v-if="item.emotion != ''" :src=item.emotion />
    </n-thing>
  </InfiniteList>
</template>

<style>
.list {
  text-align: left;

}

.list::-webkit-scrollbar {
  width: 7px;
}

.list::-webkit-scrollbar-thumb {
  background: #ccc;
  border-radius: 5px;
}

selector {
  font-family: v-sans, v-mono, other-fallbacks;
  font-weight: 500;
}
</style>
