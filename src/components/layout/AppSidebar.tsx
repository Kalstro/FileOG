import { useState, useEffect } from "react";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarHeader,
  SidebarFooter,
} from "@/components/ui/sidebar";
import {
  FolderSearch,
  FileText,
  Image,
  Video,
  Music,
  Code,
  Archive,
  File,
  History,
  Undo2,
  Settings,
  Copy,
  X,
  type LucideIcon,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import { cn } from "@/lib/utils";

// Icon mapping for category names
const categoryIconMap: Record<string, LucideIcon> = {
  "文档": FileText,
  "documents": FileText,
  "图片": Image,
  "images": Image,
  "视频": Video,
  "videos": Video,
  "音乐": Music,
  "music": Music,
  "代码": Code,
  "code": Code,
  "压缩包": Archive,
  "archives": Archive,
  "其他": File,
  "others": File,
};

// Default categories with non-purple colors
const defaultCategories = [
  { id: "documents", name: "文档", color: "#3B82F6" },  // Blue
  { id: "images", name: "图片", color: "#10B981" },    // Green
  { id: "videos", name: "视频", color: "#F97316" },    // Orange (was purple)
  { id: "music", name: "音乐", color: "#F59E0B" },     // Amber
  { id: "code", name: "代码", color: "#EC4899" },      // Pink
  { id: "archives", name: "压缩包", color: "#06B6D4" }, // Cyan (was purple)
  { id: "others", name: "其他", color: "#71717A" },    // Gray
];

interface Category {
  id: string;
  name: string;
  color?: string;
}

interface OperationBatch {
  id: string;
  operations: unknown[];
  created_at: number;
  description: string;
}

interface AppSidebarProps {
  onScan?: () => void;
  onOpenSettings?: () => void;
  onFilterChange?: (category: string | null) => void;
  onFindDuplicates?: () => void;
  activeFilter?: string | null;
}

export function AppSidebar({
  onScan,
  onOpenSettings,
  onFilterChange,
  onFindDuplicates,
  activeFilter,
}: AppSidebarProps) {
  const [historyCount, setHistoryCount] = useState(0);
  const [categories, setCategories] = useState<Category[]>(defaultCategories);

  useEffect(() => {
    loadHistoryCount();
    loadCategories();
  }, []);

  const loadCategories = async () => {
    try {
      const result = await invoke<Category[]>("get_categories");
      if (result && result.length > 0) {
        setCategories(result);
      }
    } catch {
      // Use default categories if loading fails
    }
  };

  const loadHistoryCount = async () => {
    try {
      const history = await invoke<OperationBatch[]>("get_operation_history", { limit: 100 });
      const total = history.reduce((acc, batch) => acc + batch.operations.length, 0);
      setHistoryCount(total);
    } catch {
      // Silently fail if history can't be loaded
    }
  };

  const handleUndo = async () => {
    try {
      const result = await invoke<unknown[]>("undo_operations", { steps: 1 });
      if (result.length > 0) {
        toast.success(`已撤销 ${result.length} 个操作`);
        loadHistoryCount();
      } else {
        toast.info("没有可撤销的操作");
      }
    } catch (e) {
      toast.error(`撤销失败: ${e}`);
    }
  };

  const handleShowHistory = async () => {
    try {
      const history = await invoke<OperationBatch[]>("get_operation_history", { limit: 50 });
      const total = history.reduce((acc, batch) => acc + batch.operations.length, 0);
      toast.info(`历史记录中共有 ${total} 个操作`);
    } catch (e) {
      toast.error(`加载历史失败: ${e}`);
    }
  };

  const handleCategoryClick = (categoryName: string) => {
    if (activeFilter === categoryName) {
      onFilterChange?.(null);
    } else {
      onFilterChange?.(categoryName);
    }
  };

  return (
    <Sidebar>
      <SidebarHeader className="p-4">
        <Button
          className="w-full justify-start gap-2"
          variant="outline"
          onClick={onScan}
        >
          <FolderSearch className="h-4 w-4" />
          选择文件夹扫描
        </Button>
      </SidebarHeader>

      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel className="flex items-center justify-between">
            <span>分类筛选</span>
            {activeFilter && (
              <button
                onClick={() => onFilterChange?.(null)}
                className="text-xs text-muted-foreground hover:text-foreground flex items-center gap-1"
              >
                <X className="h-3 w-3" />
                清除
              </button>
            )}
          </SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {categories.map((category) => {
                const Icon = categoryIconMap[category.name] || categoryIconMap[category.id] || File;
                const isActive = activeFilter === category.name;
                return (
                  <SidebarMenuItem key={category.id}>
                    <SidebarMenuButton
                      onClick={() => handleCategoryClick(category.name)}
                      className={cn(
                        "transition-colors",
                        isActive && "bg-accent text-accent-foreground font-medium"
                      )}
                    >
                      <Icon
                        className="h-4 w-4"
                        style={{ color: category.color || "#71717A" }}
                      />
                      <span>{category.name}</span>
                      {isActive && (
                        <span className="ml-auto text-xs bg-primary/10 text-primary px-1.5 py-0.5 rounded">
                          筛选中
                        </span>
                      )}
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                );
              })}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>工具</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton onClick={onFindDuplicates}>
                  <Copy className="h-4 w-4" />
                  <span>查找重复文件</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton onClick={handleShowHistory}>
                  <History className="h-4 w-4" />
                  <span>历史记录</span>
                  {historyCount > 0 && (
                    <span className="ml-auto text-xs text-muted-foreground">{historyCount}</span>
                  )}
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton onClick={handleUndo}>
                  <Undo2 className="h-4 w-4" />
                  <span>撤销操作</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter className="p-4">
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton onClick={onOpenSettings}>
              <Settings className="h-4 w-4" />
              <span>设置</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
    </Sidebar>
  );
}
