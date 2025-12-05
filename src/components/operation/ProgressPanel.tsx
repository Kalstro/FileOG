import { Progress } from "@/components/ui/progress";
import { Card } from "@/components/ui/card";
import { X } from "lucide-react";
import { Button } from "@/components/ui/button";

interface ProgressPanelProps {
  isActive: boolean;
  currentFile?: string;
  completed: number;
  total: number;
  onCancel?: () => void;
}

export function ProgressPanel({
  isActive,
  currentFile,
  completed,
  total,
  onCancel,
}: ProgressPanelProps) {
  if (!isActive) return null;

  const percentage = total > 0 ? (completed / total) * 100 : 0;

  return (
    <Card className="fixed bottom-4 right-4 w-80 p-4 shadow-lg z-50">
      <div className="space-y-3">
        <div className="flex items-center justify-between">
          <span className="text-sm font-medium">正在处理</span>
          <div className="flex items-center gap-2">
            <span className="text-sm text-muted-foreground">
              {completed}/{total}
            </span>
            {onCancel && (
              <Button
                variant="ghost"
                size="icon"
                className="h-6 w-6"
                onClick={onCancel}
              >
                <X className="h-4 w-4" />
              </Button>
            )}
          </div>
        </div>

        <Progress value={percentage} className="h-2" />

        {currentFile && (
          <p className="truncate text-xs text-muted-foreground">
            {currentFile}
          </p>
        )}
      </div>
    </Card>
  );
}
