import { useState, useCallback } from "react";
import { MainLayout } from "@/components/layout/MainLayout";
import { FileList, FileItemData } from "@/components/file/FileList";
import { ProgressPanel } from "@/components/operation/ProgressPanel";
import { SettingsDialog } from "@/components/settings/SettingsDialog";
import { Button } from "@/components/ui/button";
import { Copy, Trash2, FolderOutput, Sparkles } from "lucide-react";
import { invoke, Channel } from "@tauri-apps/api/core";
import { open, confirm } from "@tauri-apps/plugin-dialog";
import { classifyFiles, getFileExtension } from "@/services/llm";
import { toast } from "sonner";
import type { PlannedOperation } from "@/types";

interface ScanProgress {
  event: string;
  current_file?: string;
  scanned_count: number;
  total_count?: number;
}

interface OperationProgress {
  event: string;
  current_file?: string;
  completed_count: number;
  total_count: number;
  percentage: number;
}

function App() {
  const [files, setFiles] = useState<FileItemData[]>([]);
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const [isProcessing, setIsProcessing] = useState(false);
  const [isClassifying, setIsClassifying] = useState(false);
  const [progress, setProgress] = useState({ current: 0, total: 0, file: "" });
  const [settingsOpen, setSettingsOpen] = useState(false);

  const handleScan = useCallback(async () => {
    const selected = await open({ directory: true });
    if (!selected) return;

    setIsProcessing(true);
    setFiles([]);
    setProgress({ current: 0, total: 0, file: "" });

    const onProgress = new Channel<ScanProgress>();
    onProgress.onmessage = (msg) => {
      setProgress({
        current: msg.scanned_count,
        total: msg.total_count || 0,
        file: msg.current_file || "",
      });
    };

    try {
      const result = await invoke<FileItemData[]>("scan_directory", {
        options: { path: selected, recursive: true, include_hidden: false },
        onProgress,
      });
      setFiles(result);
    } catch (e) {
      console.error("Scan failed:", e);
      toast.error(`扫描失败: ${e}`);
    } finally {
      setIsProcessing(false);
    }
  }, []);

  const handleSelect = useCallback((id: string, selected: boolean) => {
    setSelectedIds((prev) => {
      const next = new Set(prev);
      if (selected) {
        next.add(id);
      } else {
        next.delete(id);
      }
      return next;
    });
  }, []);

  const handleSelectAll = useCallback(
    (selected: boolean) => {
      if (selected) {
        setSelectedIds(new Set(files.map((f) => f.id)));
      } else {
        setSelectedIds(new Set());
      }
    },
    [files]
  );

  const handleClassify = useCallback(async () => {
    if (selectedIds.size === 0) return;

    const selectedFiles = files.filter((f) => selectedIds.has(f.id));
    const filesToClassify = selectedFiles.map((f) => ({
      path: f.path,
      name: f.name,
      extension: getFileExtension(f.name),
      size: f.size,
    }));

    setIsClassifying(true);
    setProgress({ current: 0, total: filesToClassify.length, file: "正在分类..." });

    try {
      const results = await classifyFiles(filesToClassify);

      // Update files with classification results
      setFiles((prevFiles) =>
        prevFiles.map((file) => {
          const result = results.find((r) => r.file_path === file.name);
          if (result) {
            return {
              ...file,
              category: result.suggested_category,
            };
          }
          return file;
        })
      );
      toast.success(`成功分类 ${results.length} 个文件`);
    } catch (e) {
      console.error("Classification failed:", e);
      toast.error(`分类失败: ${e}`);
    } finally {
      setIsClassifying(false);
    }
  }, [files, selectedIds]);

  const handleMoveTo = useCallback(async () => {
    if (selectedIds.size === 0) return;

    const destination = await open({ directory: true, title: "选择目标文件夹" });
    if (!destination) return;

    const selectedFiles = files.filter((f) => selectedIds.has(f.id));
    const operations: PlannedOperation[] = selectedFiles.map((f) => ({
      file_id: f.id,
      file_name: f.name,
      operation_type: "move",
      source: f.path,
      destination: `${destination}/${f.name}`,
    }));

    setIsProcessing(true);
    setProgress({ current: 0, total: operations.length, file: "正在移动..." });

    const onProgress = new Channel<OperationProgress>();
    onProgress.onmessage = (msg) => {
      setProgress({
        current: msg.completed_count,
        total: msg.total_count,
        file: msg.current_file || "",
      });
    };

    try {
      await invoke("execute_operations", { operations, onProgress });
      // Remove moved files from list
      setFiles((prev) => prev.filter((f) => !selectedIds.has(f.id)));
      setSelectedIds(new Set());
      toast.success(`成功移动 ${operations.length} 个文件`);
    } catch (e) {
      console.error("Move failed:", e);
      toast.error(`移动失败: ${e}`);
    } finally {
      setIsProcessing(false);
    }
  }, [files, selectedIds]);

  const handleCopyTo = useCallback(async () => {
    if (selectedIds.size === 0) return;

    const destination = await open({ directory: true, title: "选择目标文件夹" });
    if (!destination) return;

    const selectedFiles = files.filter((f) => selectedIds.has(f.id));
    const operations: PlannedOperation[] = selectedFiles.map((f) => ({
      file_id: f.id,
      file_name: f.name,
      operation_type: "copy",
      source: f.path,
      destination: `${destination}/${f.name}`,
    }));

    setIsProcessing(true);
    setProgress({ current: 0, total: operations.length, file: "正在复制..." });

    const onProgress = new Channel<OperationProgress>();
    onProgress.onmessage = (msg) => {
      setProgress({
        current: msg.completed_count,
        total: msg.total_count,
        file: msg.current_file || "",
      });
    };

    try {
      await invoke("execute_operations", { operations, onProgress });
      toast.success(`成功复制 ${operations.length} 个文件`);
    } catch (e) {
      console.error("Copy failed:", e);
      toast.error(`复制失败: ${e}`);
    } finally {
      setIsProcessing(false);
    }
  }, [files, selectedIds]);

  const handleDelete = useCallback(async () => {
    if (selectedIds.size === 0) return;

    const confirmed = await confirm(
      `确定要删除选中的 ${selectedIds.size} 个文件吗？此操作不可撤销。`,
      { title: "确认删除", kind: "warning" }
    );
    if (!confirmed) return;

    const selectedFiles = files.filter((f) => selectedIds.has(f.id));
    const operations: PlannedOperation[] = selectedFiles.map((f) => ({
      file_id: f.id,
      file_name: f.name,
      operation_type: "delete",
      source: f.path,
      destination: "",
    }));

    setIsProcessing(true);
    setProgress({ current: 0, total: operations.length, file: "正在删除..." });

    const onProgress = new Channel<OperationProgress>();
    onProgress.onmessage = (msg) => {
      setProgress({
        current: msg.completed_count,
        total: msg.total_count,
        file: msg.current_file || "",
      });
    };

    try {
      await invoke("execute_operations", { operations, onProgress });
      // Remove deleted files from list
      setFiles((prev) => prev.filter((f) => !selectedIds.has(f.id)));
      setSelectedIds(new Set());
      toast.success(`成功删除 ${operations.length} 个文件`);
    } catch (e) {
      console.error("Delete failed:", e);
      toast.error(`删除失败: ${e}`);
    } finally {
      setIsProcessing(false);
    }
  }, [files, selectedIds]);

  return (
    <MainLayout onScan={handleScan} onOpenSettings={() => setSettingsOpen(true)}>
      <div className="space-y-4">
        {selectedIds.size > 0 && (
          <div className="flex items-center gap-2 rounded-lg border bg-muted/50 p-2">
            <span className="text-sm text-muted-foreground px-2">
              已选择 {selectedIds.size} 项
            </span>
            <div className="flex-1" />
            <Button
              size="sm"
              variant="outline"
              className="gap-1"
              onClick={handleMoveTo}
              disabled={isProcessing}
            >
              <FolderOutput className="h-4 w-4" />
              移动到
            </Button>
            <Button
              size="sm"
              variant="outline"
              className="gap-1"
              onClick={handleCopyTo}
              disabled={isProcessing}
            >
              <Copy className="h-4 w-4" />
              复制到
            </Button>
            <Button
              size="sm"
              variant="outline"
              className="gap-1 text-destructive"
              onClick={handleDelete}
              disabled={isProcessing}
            >
              <Trash2 className="h-4 w-4" />
              删除
            </Button>
            <Button
              size="sm"
              className="gap-1"
              onClick={handleClassify}
              disabled={isClassifying || isProcessing}
            >
              <Sparkles className="h-4 w-4" />
              {isClassifying ? "分类中..." : "AI 分类"}
            </Button>
          </div>
        )}

        <FileList
          files={files}
          selectedIds={selectedIds}
          onSelect={handleSelect}
          onSelectAll={handleSelectAll}
        />
      </div>

      <ProgressPanel
        isActive={isProcessing || isClassifying}
        currentFile={progress.file}
        completed={progress.current}
        total={progress.total}
        onCancel={() => {
          setIsProcessing(false);
          setIsClassifying(false);
        }}
      />

      <SettingsDialog open={settingsOpen} onOpenChange={setSettingsOpen} />
    </MainLayout>
  );
}

export default App;
