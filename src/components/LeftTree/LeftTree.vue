<script lang="ts" setup>
import { ref, watchPostEffect, h, watch } from 'vue';
import { FileEntry } from '../types/userDataDir';
import { useStorage } from '@vueuse/core';
import { invoke } from '@tauri-apps/api/core';
import { JX3_USER_DATA_DIR_PATH } from '../../common/constants';
import { NCheckbox, useMessage } from 'naive-ui';
import { findPath } from "../../utils";
import { EmitSelect } from "../types";

const emit = defineEmits<{
  source: [source: EmitSelect]
}>()

const jx3UserDataPath = useStorage(JX3_USER_DATA_DIR_PATH, '', localStorage)
const message = useMessage()

const fileEntries = ref<FileEntry[]>([]);
const source = ref('')

watch(source, (val) => {
  const path = findPath(fileEntries.value, val)
  emit('source', {
    name: val,
    path: path as string
  })
})

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
    source.value = node!.name;
  } else {
    message.success(`取消选择${node?.name}`)
  }
}

</script>

<template>
  <div class="wrapper">
    <n-input v-model:value="pattern" placeholder="搜索源账号/角色名称" />
    <div class="tree">
      <n-tree :pattern="pattern" :data="fileEntries" block-line key-field="id" label-field="name" :filter="handleFilter"
        :show-irrelevant-nodes="false" expand-on-click :render-prefix="renderPrefix"
        :on-update:selected-keys="handleSelectedKeys"></n-tree>
    </div>
  </div>
</template>

<style lang="less" scoped>
.wrapper {
  width: 30%;
  height: 100%;
  position: relative;
  box-sizing: border-box;

  .tree {
    width: 100%;
    margin-top: 10px;
    height: calc(100vh - 80px);
    overflow: scroll;
    box-sizing: border-box;
  }
}
</style>
