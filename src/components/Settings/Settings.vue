<script lang="ts" setup>
import { useUserDataDir } from "../../composables/userDataDir.ts";

const active = defineModel('active')
const { jx3UserDataPath, selectDirectory } = useUserDataDir()
</script>

<template>
  <n-drawer v-model:show="active" :width="502" placement="right">
    <n-drawer-content title="基础设置">
      <h5> userdata路径：{{ jx3UserDataPath }}</h5>
      <n-button @click="selectDirectory">设置路径</n-button>
    </n-drawer-content>
  </n-drawer>
</template>

<style lang="less" scoped></style>
