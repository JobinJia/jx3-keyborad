<script setup lang="ts">
import { ref, watchPostEffect } from 'vue'
import { useUserDataDir } from "../../composables/userDataDir.ts";
import { invoke } from "@tauri-apps/api/core";
import { useMessage } from 'naive-ui'
const props = defineProps<{
  userSelect: {
    target: string
    targetPath: string
    source: string
    sourcePath: string
  }
}>()

const message = useMessage();

const formValue = ref({
  source: '',
  target: '',
})

watchPostEffect(() => {
  formValue.value.target = props.userSelect.target;
  formValue.value.source = props.userSelect.source;
})

const { jx3UserDataPath } = useUserDataDir()

async function handleGoGO() {
  const params = {
    source_path: `${jx3UserDataPath.value}/${props.userSelect.sourcePath}`,
    target_path: `${jx3UserDataPath.value}/${props.userSelect.targetPath}`,
  }
  const result: boolean = await invoke('cp_source_to_target', { params })
  if (result) {
    message.success('操作成功')
  } else {
    message.error('复制失败')
  }
}
</script>

<template>
  <div class="wrapper">
    <n-alert title="提示" type="info" :bordered="false" v-if="formValue.target && formValue.source">
      角色 <b>{{ formValue.target }}</b> 使用 <b> {{ formValue.source }} </b> 的键位
    </n-alert>
    <n-form style="margin-top: 20px" ref="formRef" :label-width="80" :model="formValue" size="small"
      label-placement="left" label-align="left">
      <n-form-item label="源角色" path="user.name">
        <n-input v-model:value="formValue.source" disabled />
      </n-form-item>
      <n-form-item label="目标角色" path="user.age">
        <n-input v-model:value="formValue.target" disabled />
      </n-form-item>
      <n-form-item>
        <n-button type="primary" @click="handleGoGO" :disabled="!(formValue.target && formValue.source)">
          确认替换
        </n-button>
      </n-form-item>
    </n-form>
    <n-alert :show-icon="false" title="常见问题及方案">
      <p> <b> 自己带键位的账号 </b>需要在游戏里 <b>关闭同步到服务器</b>，这样键位在才能本地得到保存 </p>
      <p> 如果是 <b> 新账号 ，登入到游戏角色选择界面后，选中需要改键位的角色，别进入游戏，停在这个游戏界面。</b> 然后点击刷新就能搜索到这个角色了 </p>
      <p> 这玩意儿本质就是找到对应目录进行复制粘贴，没啥难度，做这个主要是哥们儿很烦每次手动改键位，眼都找瞎了 </p>
    </n-alert>
  </div>
</template>

<style scoped lang="less">
.wrapper {
  width: 30%;
  height: 100%;
  position: relative;
  box-sizing: border-box;
}
</style>
