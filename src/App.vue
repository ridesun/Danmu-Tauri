<script setup lang="ts">

import { computed } from "vue";
import { useOsTheme, darkTheme } from 'naive-ui'


const osThemeRef = useOsTheme();
var theme = computed(() => (osThemeRef.value === 'dark' ? darkTheme : null));
</script>

<template>
  <div class="container">
    <n-config-provider :theme="theme">
      <RouterView>
      </RouterView>
    </n-config-provider>

  </div>
</template>

<style scoped></style>
