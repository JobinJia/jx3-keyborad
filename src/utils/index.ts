import { FileEntry } from "../components/types/userDataDir";

// 寻找目标节点并返回其路径
export function findPath(data: FileEntry[], targetName: string): string | null {
  function helper(entries: FileEntry[], currentPath: string): string | null {
    for (const entry of entries) {
      const newPath = `${currentPath}/${entry.name}`;
      if (entry.name === targetName) {
        return newPath;
      }
      if (entry.is_dir && entry?.children?.length > 0) {
        const result = helper(entry.children, newPath);
        if (result) {
          return result;
        }
      }
    }
    return null;
  }

  const result = helper(data, '');
  return result ? result.substring(1) : null;
}

