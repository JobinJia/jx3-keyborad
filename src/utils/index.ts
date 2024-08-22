import { FileEntry } from "../components/types/userDataDir";

export function tranformData(data: FileEntry[]): FileEntry[] {
  return data.map((it) => {
    const { children = [] } = it;
    if (children.length) {
      const tarnsChild = tranformData(children);
      return {
        ...it,
        children: tarnsChild,
        checkboxDisabled: true,
      };
    }
    return {
      ...it,
      checkboxDisabled: false,
    }
  })
}
