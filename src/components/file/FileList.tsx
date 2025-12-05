import { FileText, Image, Video, Music, Code, Archive, File } from "lucide-react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Checkbox } from "@/components/ui/checkbox";
import { cn } from "@/lib/utils";

export interface FileItemData {
  id: string;
  name: string;
  path: string;
  size: number;
  fileType: string;
  category?: string;
}

interface FileListProps {
  files: FileItemData[];
  selectedIds: Set<string>;
  onSelect: (id: string, selected: boolean) => void;
  onSelectAll: (selected: boolean) => void;
}

const fileTypeIcons: Record<string, React.ElementType> = {
  document: FileText,
  image: Image,
  video: Video,
  audio: Music,
  code: Code,
  archive: Archive,
  other: File,
};

const fileTypeColors: Record<string, string> = {
  document: "text-blue-500",
  image: "text-green-500",
  video: "text-orange-500",
  audio: "text-amber-500",
  code: "text-pink-500",
  archive: "text-cyan-500",
  other: "text-zinc-500",
};

function formatFileSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
}

export function FileList({ files, selectedIds, onSelect, onSelectAll }: FileListProps) {
  const allSelected = files.length > 0 && selectedIds.size === files.length;
  const someSelected = selectedIds.size > 0 && selectedIds.size < files.length;

  if (files.length === 0) {
    return (
      <div className="flex h-[400px] items-center justify-center text-muted-foreground">
        <div className="text-center">
          <File className="mx-auto h-12 w-12 opacity-50" />
          <p className="mt-2">选择一个文件夹开始扫描</p>
        </div>
      </div>
    );
  }

  return (
    <div className="rounded-lg border">
      <div className="flex items-center gap-3 border-b bg-muted/50 px-4 py-2">
        <Checkbox
          checked={allSelected}
          onCheckedChange={(checked) => onSelectAll(!!checked)}
          aria-label="全选"
          className={cn(someSelected && "opacity-50")}
        />
        <span className="text-sm font-medium">
          {selectedIds.size > 0 ? `已选择 ${selectedIds.size} 项` : `共 ${files.length} 个文件`}
        </span>
      </div>
      <ScrollArea className="h-[500px]">
        <div className="divide-y">
          {files.map((file) => {
            const Icon = fileTypeIcons[file.fileType.toLowerCase()] || File;
            const colorClass = fileTypeColors[file.fileType.toLowerCase()] || "text-zinc-500";
            
            return (
              <div
                key={file.id}
                className="flex items-center gap-3 px-4 py-3 hover:bg-muted/50 transition-colors"
              >
                <Checkbox
                  checked={selectedIds.has(file.id)}
                  onCheckedChange={(checked) => onSelect(file.id, !!checked)}
                  aria-label={`选择 ${file.name}`}
                />
                <Icon className={cn("h-5 w-5 shrink-0", colorClass)} />
                <div className="min-w-0 flex-1">
                  <p className="truncate text-sm font-medium">{file.name}</p>
                  <p className="truncate text-xs text-muted-foreground">{file.path}</p>
                </div>
                <div className="text-right">
                  <p className="text-sm text-muted-foreground">{formatFileSize(file.size)}</p>
                  {file.category && (
                    <p className="text-xs text-primary">{file.category}</p>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </ScrollArea>
    </div>
  );
}
