<script setup lang="ts">
import { NConfigProvider, NMessageProvider, useMessage } from 'naive-ui'
import LeftTree from './components/LeftTree/LeftTree.vue';
import { ref } from 'vue';
import { SettingsOutline } from '@vicons/ionicons5';
import { useUserDataDir } from './composables/userDataDir';
import { useStorage } from '@vueuse/core';
import { JX3_USER_DATA_DIR_PATH } from './common/constants';

// const message = useMessage();

// const { jx3UserDataPath, selectDirectory } = useUserDataDir(message)

const active = ref(false)
const jx3UserDataPath = useStorage(JX3_USER_DATA_DIR_PATH, '', localStorage)
</script>

<template>
   <div class="app">
    <NConfigProvider>
      <NMessageProvider>
        <n-space reverse>
          <n-button circle @click="active = !active">
            <template #icon>
              <n-icon><SettingsOutline /></n-icon>
            </template>
          </n-button>
        </n-space>
        <div v-if="!jx3UserDataPath">
          设置
        </div>
        <LeftTree v-if="jx3UserDataPath" />
        <n-drawer v-model:show="active" :width="502" placement="right">
          <n-drawer-content title="斯通纳">
            《斯通纳》是美国作家约翰·威廉姆斯在 1965 年出版的小说。
          </n-drawer-content>
        </n-drawer>
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
  }
</style>
