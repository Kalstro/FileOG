import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Settings, Sparkles, FolderTree, FileText, Palette, Plus, Trash2, Save, Zap } from "lucide-react";
import { toast } from "sonner";
import { testLlmConnection } from "@/services/llm";

interface LlmConfig {
  provider: string;
  api_key: string;
  api_endpoint: string;
  model: string;
  supports_vision: boolean;
}

interface LlmSettings {
  enabled: boolean;
  config: LlmConfig;
}

interface PromptSettings {
  filename_prompt: string;
  text_content_prompt: string;
  image_prompt: string;
}

interface AppSettings {
  theme: string;
  language: string;
  llm: LlmSettings;
  prompts: PromptSettings;
}

interface CategoryRule {
  rule_type: string;
  pattern: string;
  priority: number;
}

interface Category {
  id: string;
  name: string;
  description?: string;
  target_folder: string;
  rules: CategoryRule[];
  color?: string;
  icon?: string;
}

interface SettingsDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

const defaultPrompts: PromptSettings = {
  filename_prompt: `分析以下文件名，判断该文件应该属于哪个分类。
文件名: {{filename}}
可用分类: {{categories}}
请只返回最匹配的分类名称，不要包含其他内容。`,
  text_content_prompt: `分析以下文件内容，判断该文件应该属于哪个分类。
文件名: {{filename}}
文件内容（前1000字符）:
{{content}}
可用分类: {{categories}}
请只返回最匹配的分类名称，不要包含其他内容。`,
  image_prompt: `分析这张图片的内容，判断它应该属于哪个分类。
可用分类: {{categories}}
请只返回最匹配的分类名称，不要包含其他内容。`,
};

export function SettingsDialog({ open, onOpenChange }: SettingsDialogProps) {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [categories, setCategories] = useState<Category[]>([]);
  const [saving, setSaving] = useState(false);
  const [testing, setTesting] = useState(false);

  useEffect(() => {
    if (open) {
      loadSettings();
      loadCategories();
    }
  }, [open]);

  const loadSettings = async () => {
    try {
      const result = await invoke<AppSettings>("get_settings");
      setSettings(result);
    } catch (error) {
      console.error("Failed to load settings:", error);
      // Use default settings if none exist
      setSettings({
        theme: "system",
        language: "zh-CN",
        llm: {
          enabled: false,
          config: {
            provider: "openai",
            api_key: "",
            api_endpoint: "https://api.openai.com/v1",
            model: "gpt-4o-mini",
            supports_vision: true,
          },
        },
        prompts: defaultPrompts,
      });
    }
  };

  const loadCategories = async () => {
    try {
      const result = await invoke<Category[]>("get_categories");
      setCategories(result);
    } catch (error) {
      console.error("Failed to load categories:", error);
    }
  };

  const saveSettings = async () => {
    if (!settings) return;
    setSaving(true);
    try {
      await invoke("save_settings", { settings });
      toast.success("设置已保存");
    } catch (error) {
      console.error("Failed to save settings:", error);
      toast.error("保存设置失败");
    } finally {
      setSaving(false);
    }
  };

  const saveCategories = async () => {
    setSaving(true);
    try {
      await invoke("save_categories", { categories });
      toast.success("分类已保存");
    } catch (error) {
      console.error("Failed to save categories:", error);
      toast.error("保存分类失败");
    } finally {
      setSaving(false);
    }
  };

  const addCategory = () => {
    const newCategory: Category = {
      id: crypto.randomUUID(),
      name: "新分类",
      description: "",
      target_folder: "NewCategory",
      rules: [{
        rule_type: "extension",
        pattern: "",
        priority: 1,
      }],
      color: "#3B82F6",
      icon: "folder",
    };
    setCategories([...categories, newCategory]);
  };

  const removeCategory = (id: string) => {
    setCategories(categories.filter((c) => c.id !== id));
  };

  const updateCategory = (id: string, updates: Partial<Category>) => {
    setCategories(
      categories.map((c) => (c.id === id ? { ...c, ...updates } : c))
    );
  };

  if (!settings) return null;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-3xl max-h-[85vh]">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Settings className="h-5 w-5" />
            设置
          </DialogTitle>
        </DialogHeader>

        <Tabs defaultValue="llm" className="mt-4">
          <TabsList className="grid w-full grid-cols-4">
            <TabsTrigger value="llm" className="flex items-center gap-2">
              <Sparkles className="h-4 w-4" />
              LLM
            </TabsTrigger>
            <TabsTrigger value="categories" className="flex items-center gap-2">
              <FolderTree className="h-4 w-4" />
              分类
            </TabsTrigger>
            <TabsTrigger value="prompts" className="flex items-center gap-2">
              <FileText className="h-4 w-4" />
              提示词
            </TabsTrigger>
            <TabsTrigger value="appearance" className="flex items-center gap-2">
              <Palette className="h-4 w-4" />
              外观
            </TabsTrigger>
          </TabsList>

          {/* LLM Settings */}
          <TabsContent value="llm" className="mt-4">
            <ScrollArea className="h-[400px] pr-4">
              <div className="space-y-6">
                <div className="flex items-center justify-between">
                  <div className="space-y-0.5">
                    <Label>启用 LLM 智能分类</Label>
                    <p className="text-sm text-muted-foreground">
                      使用大语言模型智能识别和分类文件
                    </p>
                  </div>
                  <Switch
                    checked={settings.llm.enabled}
                    onCheckedChange={(checked) =>
                      setSettings({
                        ...settings,
                        llm: { ...settings.llm, enabled: checked },
                      })
                    }
                  />
                </div>

                <Separator />

                <div className="space-y-4">
                  <div className="space-y-2">
                    <Label>提供商</Label>
                    <Select
                      value={settings.llm.config.provider}
                      onValueChange={(value) =>
                        setSettings({
                          ...settings,
                          llm: {
                            ...settings.llm,
                            config: { ...settings.llm.config, provider: value },
                          },
                        })
                      }
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="openai">OpenAI</SelectItem>
                        <SelectItem value="anthropic">Anthropic (Claude)</SelectItem>
                        <SelectItem value="ollama">Ollama (本地)</SelectItem>
                        <SelectItem value="custom">自定义端点</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>

                  <div className="space-y-2">
                    <Label>API 端点</Label>
                    <Input
                      value={settings.llm.config.api_endpoint}
                      onChange={(e) =>
                        setSettings({
                          ...settings,
                          llm: {
                            ...settings.llm,
                            config: {
                              ...settings.llm.config,
                              api_endpoint: e.target.value,
                            },
                          },
                        })
                      }
                      placeholder="https://api.openai.com/v1"
                    />
                  </div>

                  <div className="space-y-2">
                    <Label>API Key</Label>
                    <Input
                      type="password"
                      value={settings.llm.config.api_key}
                      onChange={(e) =>
                        setSettings({
                          ...settings,
                          llm: {
                            ...settings.llm,
                            config: {
                              ...settings.llm.config,
                              api_key: e.target.value,
                            },
                          },
                        })
                      }
                      placeholder="sk-..."
                    />
                  </div>

                  <div className="space-y-2">
                    <Label>模型</Label>
                    <Input
                      value={settings.llm.config.model}
                      onChange={(e) =>
                        setSettings({
                          ...settings,
                          llm: {
                            ...settings.llm,
                            config: {
                              ...settings.llm.config,
                              model: e.target.value,
                            },
                          },
                        })
                      }
                      placeholder="gpt-4o-mini"
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <div className="space-y-0.5">
                      <Label>支持图片识别</Label>
                      <p className="text-sm text-muted-foreground">
                        该模型是否支持 Vision 功能
                      </p>
                    </div>
                    <Switch
                      checked={settings.llm.config.supports_vision}
                      onCheckedChange={(checked) =>
                        setSettings({
                          ...settings,
                          llm: {
                            ...settings.llm,
                            config: {
                              ...settings.llm.config,
                              supports_vision: checked,
                            },
                          },
                        })
                      }
                    />
                  </div>
                </div>

                <div className="flex justify-end gap-2">
                  <Button
                    variant="outline"
                    onClick={async () => {
                      if (!settings) return;
                      setTesting(true);
                      // Save settings first to ensure test uses current values
                      try {
                        await invoke("save_settings", { settings });
                        const result = await testLlmConnection();
                        toast.success(result);
                      } catch (error) {
                        console.error("Test failed:", error);
                        toast.error(`连接测试失败: ${error}`);
                      } finally {
                        setTesting(false);
                      }
                    }}
                    disabled={testing || saving}
                  >
                    <Zap className="h-4 w-4 mr-2" />
                    {testing ? "测试中..." : "测试连接"}
                  </Button>
                  <Button onClick={saveSettings} disabled={saving}>
                    <Save className="h-4 w-4 mr-2" />
                    保存设置
                  </Button>
                </div>
              </div>
            </ScrollArea>
          </TabsContent>

          {/* Categories Settings */}
          <TabsContent value="categories" className="mt-4">
            <ScrollArea className="h-[400px] pr-4">
              <div className="space-y-4">
                {categories.map((category) => (
                  <div
                    key={category.id}
                    className="flex items-start gap-4 p-4 border rounded-lg"
                  >
                    <div
                      className="w-4 h-4 rounded mt-2"
                      style={{ backgroundColor: category.color }}
                    />
                    <div className="flex-1 space-y-3">
                      <div className="flex gap-4">
                        <div className="flex-1">
                          <Label className="text-xs">名称</Label>
                          <Input
                            value={category.name}
                            onChange={(e) =>
                              updateCategory(category.id, { name: e.target.value })
                            }
                          />
                        </div>
                        <div className="w-24">
                          <Label className="text-xs">颜色</Label>
                          <Input
                            type="color"
                            value={category.color}
                            onChange={(e) =>
                              updateCategory(category.id, { color: e.target.value })
                            }
                            className="h-9 p-1"
                          />
                        </div>
                      </div>
                      <div>
                        <Label className="text-xs">目标文件夹</Label>
                        <Input
                          value={category.target_folder}
                          onChange={(e) =>
                            updateCategory(category.id, {
                              target_folder: e.target.value,
                            })
                          }
                          placeholder="Documents"
                        />
                      </div>
                      <div>
                        <Label className="text-xs">匹配规则（扩展名，逗号分隔）</Label>
                        <Input
                          value={category.rules.map(r => r.pattern).join(", ")}
                          onChange={(e) =>
                            updateCategory(category.id, {
                              rules: e.target.value
                                .split(",")
                                .map((s, i) => ({
                                  rule_type: "extension",
                                  pattern: s.trim(),
                                  priority: i + 1,
                                }))
                                .filter(r => r.pattern),
                            })
                          }
                          placeholder=".pdf, .doc, .docx"
                        />
                      </div>
                    </div>
                    <Button
                      variant="ghost"
                      size="icon"
                      className="text-destructive"
                      onClick={() => removeCategory(category.id)}
                    >
                      <Trash2 className="h-4 w-4" />
                    </Button>
                  </div>
                ))}

                <Button variant="outline" className="w-full" onClick={addCategory}>
                  <Plus className="h-4 w-4 mr-2" />
                  添加分类
                </Button>

                <div className="flex justify-end">
                  <Button onClick={saveCategories} disabled={saving}>
                    <Save className="h-4 w-4 mr-2" />
                    保存分类
                  </Button>
                </div>
              </div>
            </ScrollArea>
          </TabsContent>

          {/* Prompts Settings */}
          <TabsContent value="prompts" className="mt-4">
            <ScrollArea className="h-[400px] pr-4">
              <div className="space-y-6">
                <div className="space-y-2">
                  <Label>文件名分析提示词</Label>
                  <p className="text-xs text-muted-foreground">
                    可用变量: {"{{filename}}"}, {"{{categories}}"}
                  </p>
                  <Textarea
                    value={settings.prompts.filename_prompt}
                    onChange={(e) =>
                      setSettings({
                        ...settings,
                        prompts: {
                          ...settings.prompts,
                          filename_prompt: e.target.value,
                        },
                      })
                    }
                    rows={5}
                  />
                </div>

                <div className="space-y-2">
                  <Label>文本内容分析提示词</Label>
                  <p className="text-xs text-muted-foreground">
                    可用变量: {"{{filename}}"}, {"{{content}}"}, {"{{categories}}"}
                  </p>
                  <Textarea
                    value={settings.prompts.text_content_prompt}
                    onChange={(e) =>
                      setSettings({
                        ...settings,
                        prompts: {
                          ...settings.prompts,
                          text_content_prompt: e.target.value,
                        },
                      })
                    }
                    rows={6}
                  />
                </div>

                <div className="space-y-2">
                  <Label>图片分析提示词</Label>
                  <p className="text-xs text-muted-foreground">
                    可用变量: {"{{categories}}"}
                  </p>
                  <Textarea
                    value={settings.prompts.image_prompt}
                    onChange={(e) =>
                      setSettings({
                        ...settings,
                        prompts: {
                          ...settings.prompts,
                          image_prompt: e.target.value,
                        },
                      })
                    }
                    rows={4}
                  />
                </div>

                <Button
                  variant="outline"
                  onClick={() =>
                    setSettings({ ...settings, prompts: defaultPrompts })
                  }
                >
                  重置为默认提示词
                </Button>

                <div className="flex justify-end">
                  <Button onClick={saveSettings} disabled={saving}>
                    <Save className="h-4 w-4 mr-2" />
                    保存设置
                  </Button>
                </div>
              </div>
            </ScrollArea>
          </TabsContent>

          {/* Appearance Settings */}
          <TabsContent value="appearance" className="mt-4">
            <div className="space-y-6">
              <div className="space-y-2">
                <Label>主题</Label>
                <Select
                  value={settings.theme}
                  onValueChange={(value) =>
                    setSettings({ ...settings, theme: value })
                  }
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="light">浅色</SelectItem>
                    <SelectItem value="dark">深色</SelectItem>
                    <SelectItem value="system">跟随系统</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label>语言</Label>
                <Select
                  value={settings.language}
                  onValueChange={(value) =>
                    setSettings({ ...settings, language: value })
                  }
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="zh-CN">简体中文</SelectItem>
                    <SelectItem value="en-US">English</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="flex justify-end">
                <Button onClick={saveSettings} disabled={saving}>
                  <Save className="h-4 w-4 mr-2" />
                  保存设置
                </Button>
              </div>
            </div>
          </TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  );
}
