import { useStorage } from "@vueuse/core";
import { open } from '@tauri-apps/plugin-dialog';
import { JX3_USER_DATA_DIR_PATH } from "../common/constants";
import { MessageApiInjection } from "naive-ui/es/message/src/MessageProvider";

export function useUserDataDir(message: MessageApiInjection) {
  const jx3UserDataPath = useStorage(JX3_USER_DATA_DIR_PATH, '', localStorage)

  async function selectDirectory() {
    const selected = await open({
      directory: true,
    });
    if (selected) {
      console.log('Selected directory:', selected);
      // Handle the selected directory path
      jx3UserDataPath.value = selected;
      message.success(`你选择了${selected}`)
    } else {
      console.log('No directory selected');
    }
  }

  return {
    jx3UserDataPath,
    selectDirectory
  }
}
