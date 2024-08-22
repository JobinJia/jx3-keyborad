export interface FileEntry {
  id: string
  name: string
  is_dir: boolean
  children: FileEntry[]
  [key: string]: any
}
