import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { Check, ChevronDown, Play, User } from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import { cn } from "@/lib/utils";
import { useAuthStore } from "@/models/auth";
import { useGameStore } from "@/stores/game-store";
import { useInstancesStore } from "@/stores/instances-store";
import { LoginModal } from "./login-modal";
import { Button } from "./ui/button";

interface InstalledVersion {
  id: string;
  type: string;
}

export function BottomBar() {
  const authStore = useAuthStore();
  const gameStore = useGameStore();
  const instancesStore = useInstancesStore();

  const [isVersionDropdownOpen, setIsVersionDropdownOpen] = useState(false);
  const [installedVersions, setInstalledVersions] = useState<
    InstalledVersion[]
  >([]);
  const [isLoadingVersions, setIsLoadingVersions] = useState(true);
  const [showLoginModal, setShowLoginModal] = useState(false);

  const loadInstalledVersions = useCallback(async () => {
    if (!instancesStore.activeInstanceId) {
      setInstalledVersions([]);
      setIsLoadingVersions(false);
      return;
    }

    setIsLoadingVersions(true);
    try {
      const versions = await invoke<InstalledVersion[]>(
        "list_installed_versions",
        { instanceId: instancesStore.activeInstanceId },
      );

      const installed = versions || [];
      setInstalledVersions(installed);

      // If no version is selected but we have installed versions, select the first one
      if (!gameStore.selectedVersion && installed.length > 0) {
        gameStore.setSelectedVersion(installed[0].id);
      }
    } catch (error) {
      console.error("Failed to load installed versions:", error);
    } finally {
      setIsLoadingVersions(false);
    }
  }, [
    instancesStore.activeInstanceId,
    gameStore.selectedVersion,
    gameStore.setSelectedVersion,
  ]);

  useEffect(() => {
    loadInstalledVersions();

    // Listen for backend events that should refresh installed versions.
    let unlistenDownload: UnlistenFn | null = null;
    let unlistenVersionDeleted: UnlistenFn | null = null;

    (async () => {
      try {
        unlistenDownload = await listen("download-complete", () => {
          loadInstalledVersions();
        });
      } catch (err) {
        // best-effort: do not break UI if listening fails
        // eslint-disable-next-line no-console
        console.warn("Failed to attach download-complete listener:", err);
      }

      try {
        unlistenVersionDeleted = await listen("version-deleted", () => {
          loadInstalledVersions();
        });
      } catch (err) {
        // eslint-disable-next-line no-console
        console.warn("Failed to attach version-deleted listener:", err);
      }
    })();

    return () => {
      try {
        if (unlistenDownload) unlistenDownload();
      } catch {
        // ignore
      }
      try {
        if (unlistenVersionDeleted) unlistenVersionDeleted();
      } catch {
        // ignore
      }
    };
  }, [loadInstalledVersions]);

  const selectVersion = (id: string) => {
    if (id !== "loading" && id !== "empty") {
      gameStore.setSelectedVersion(id);
      setIsVersionDropdownOpen(false);
    }
  };

  const handleStartGame = async () => {
    // await gameStore.startGame(
    //   authStore.currentAccount,
    //   authStore.openLoginModal,
    //   instancesStore.activeInstanceId,
    //   uiStore.setView,
    // );
  };

  const getVersionTypeColor = (type: string) => {
    switch (type) {
      case "release":
        return "bg-emerald-500";
      case "snapshot":
        return "bg-amber-500";
      case "old_beta":
        return "bg-rose-500";
      case "old_alpha":
        return "bg-violet-500";
      default:
        return "bg-gray-500";
    }
  };

  const versionOptions = isLoadingVersions
    ? [{ id: "loading", type: "loading", label: "Loading..." }]
    : installedVersions.length === 0
      ? [{ id: "empty", type: "empty", label: "No versions installed" }]
      : installedVersions.map((v) => ({
          ...v,
          label: `${v.id}${v.type !== "release" ? ` (${v.type})` : ""}`,
        }));

  return (
    <div className="absolute bottom-0 left-0 right-0 bg-linear-to-t from-black/30 via-transparent to-transparent p-4 z-10">
      <div className="max-w-7xl mx-auto">
        <div className="flex items-center justify-between bg-white/5 dark:bg-black/20 backdrop-blur-xl border border-white/10 dark:border-white/5 p-3 shadow-lg">
          <div className="flex items-center gap-4">
            <div className="flex flex-col">
              <span className="text-xs font-mono text-zinc-400 uppercase tracking-wider">
                Active Instance
              </span>
              <span className="text-sm font-medium text-white">
                {instancesStore.activeInstance?.name || "No instance selected"}
              </span>
            </div>
          </div>

          <div className="flex items-center gap-3">
            {authStore.account ? (
              <Button
                className={cn(
                  "px-4 py-2 shadow-xl",
                  "bg-emerald-600! hover:bg-emerald-500!",
                )}
                size="lg"
                onClick={handleStartGame}
              >
                <Play />
                Start
              </Button>
            ) : (
              <Button
                className="px-4 py-2"
                size="lg"
                onClick={() => setShowLoginModal(true)}
              >
                <User /> Login
              </Button>
            )}
          </div>
        </div>
      </div>

      <LoginModal
        open={showLoginModal}
        onOpenChange={() => setShowLoginModal(false)}
      />
    </div>
  );
}
