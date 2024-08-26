<script setup lang="ts">
import { NConfigProvider, NMessageProvider } from 'naive-ui'
import LeftTree from './components/LeftTree/LeftTree.vue';
import RightTree from './components/RightTree/RightTree.vue';
import Result from './components/Result/Result.vue';
import { ref } from 'vue';
import { SettingsOutline } from '@vicons/ionicons5';
import { useStorage } from '@vueuse/core';
import { JX3_USER_DATA_DIR_PATH } from './common/constants';
import Settings from './components/Settings/Settings.vue'
import { EmitSelect } from "./components/types";

const active = ref(false)
const jx3UserDataPath = useStorage(JX3_USER_DATA_DIR_PATH, '', localStorage)

const userSelect = ref({
  source: '',
  sourcePath: '',
  target: '',
  targetPath: '',
})

function setSource(val: EmitSelect) {
  const { name, path } = val;
  userSelect.value.source = name
  userSelect.value.sourcePath = path
}

function setTarget(val: EmitSelect) {
  const { name, path } = val;
  userSelect.value.target = name
  userSelect.value.targetPath = path
}
</script>

<template>
  <div class="app">
    <NConfigProvider>
      <NMessageProvider>
        <n-float-button :right="10" :top="10" shape="square"  @click="active = !active">
          <n-icon>
            <SettingsOutline />
          </n-icon>
        </n-float-button>
        <div v-if="!jx3UserDataPath">
          先点设置，在弹出的面板中设置 userdata 的路径
        </div>
        <div v-if="jx3UserDataPath" class="content">
          <LeftTree @source="setSource" />
          <RightTree style="margin-left: 20px" @target="setTarget" />
          <Result style="margin-left: 20px" :user-select="userSelect"></Result>
        </div>
        <Settings v-model:active="active" />
      </NMessageProvider>
    </NConfigProvider>
  </div>
</template>

<style lang="less">
.app {
  width: 100vw;
  height: 100vh;
  box-sizing: border-box;
  padding: 20px;

  .content {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: row;
  }
}
</style>
