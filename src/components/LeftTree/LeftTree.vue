<script lang="ts" setup>
import { ref, watchPostEffect, h } from 'vue';
import { FileEntry } from '../types/userDataDir';
import { useStorage } from '@vueuse/core';
import { invoke } from '@tauri-apps/api/core';
import { JX3_USER_DATA_DIR_PATH } from '../../common/constants';
import { tranformData } from '../../utils';
import { NCheckbox, useMessage } from 'naive-ui';

const jx3UserDataPath = useStorage(JX3_USER_DATA_DIR_PATH, '', localStorage)
const message = useMessage()

const fileEntries = ref<FileEntry[]>([]);
const selectedAccount = ref('')

async function readeDir() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const originData: FileEntry[] = await invoke("list_directory_contents", { path: jx3UserDataPath.value });
  fileEntries.value = originData // tranformData(originData);
  console.log(fileEntries.value);
}

watchPostEffect(() => {
  if (jx3UserDataPath.value) {
    console.log(jx3UserDataPath.value);
    readeDir()
  }
})

const pattern = ref('')

function handleFilter(ptn: string, node: FileEntry) {
  const { name } = node;
  return name.indexOf(ptn) !== -1;
}

function renderPrefix(info: { option: FileEntry, checked: boolean, selected: boolean }) {
  const { option, selected } = info
  if (option?.children) {
    return null;
  }
  return h(NCheckbox, {
    checked: selected,
    'onUpdate:checked': (value: boolean) => {
      info.selected = value;
      // if (value) {
      //   selectedAccount.value = option.name;
      //   message.success(`选择${option.name}`)
      // } else {
      //   message.success(`取消选择${option.name}`)
      // }
    }
  })
}


function handleSelectedKeys(keys: Array<string | number>, option: Array<FileEntry | null>, meta: { node: FileEntry | null, action: 'select' | 'unselect' }) {
  const { node, action } = meta;
  if (node?.children) {
    return;
  }
  if (action === 'select') {
    message.success(`选择${node?.name}`)
  } else {
    message.success(`取消选择${node?.name}`)
  }
}

</script>

<template>
  <div class="wrapper">
    <n-input v-model:value="pattern" placeholder="搜索账号/角色名称" />
    <div class="tree">
      <n-tree
      :pattern="pattern"
      :data="fileEntries"
      block-line
      key-field="id"
      label-field="name"
      :filter="handleFilter"
      :show-irrelevant-nodes="false"
      expand-on-click
      :render-prefix="renderPrefix"
      :on-update:selected-keys="handleSelectedKeys"
    ></n-tree>
    </div>
  </div>
</template>

<style lang="less" scoped>
  .wrapper {
    width: 30%;
    height: 100vh;
    position: relative;
    .tree {
      width: 100%;
      height: 100%;
      overflow: scroll;
    }
  }
</style>
