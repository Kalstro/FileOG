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
} from "lucide-react";
import { Button } from "@/components/ui/button";

const categories = [
  { id: "documents", name: "文档", icon: FileText, color: "#3B82F6" },
  { id: "images", name: "图片", icon: Image, color: "#10B981" },
  { id: "videos", name: "视频", icon: Video, color: "#8B5CF6" },
  { id: "music", name: "音乐", icon: Music, color: "#F59E0B" },
  { id: "code", name: "代码", icon: Code, color: "#EC4899" },
  { id: "archives", name: "压缩包", icon: Archive, color: "#6366F1" },
  { id: "others", name: "其他", icon: File, color: "#71717A" },
];

interface AppSidebarProps {
  onScan?: () => void;
  onOpenSettings?: () => void;
}

export function AppSidebar({ onScan, onOpenSettings }: AppSidebarProps) {
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
          <SidebarGroupLabel>分类</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {categories.map((category) => (
                <SidebarMenuItem key={category.id}>
                  <SidebarMenuButton>
                    <category.icon 
                      className="h-4 w-4" 
                      style={{ color: category.color }} 
                    />
                    <span>{category.name}</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>操作</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton>
                  <History className="h-4 w-4" />
                  <span>历史记录</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton>
                  <Undo2 className="h-4 w-4" />
                  <span>撤销</span>
                  <span className="ml-auto text-xs text-muted-foreground">0</span>
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
