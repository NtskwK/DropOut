<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";

  export let visible = false;

  interface DownloadEvent {
    file: string;
    downloaded: number; // in bytes
    total: number; // in bytes
    status: string;
  }

  let currentFile = "";
  let progress = 0; // percentage 0-100
  let totalFiles = 0;
  let statusText = "Preparing...";
  let unlistenProgress: () => void;
  let unlistenStart: () => void;
  let unlistenComplete: () => void;
  let downloadedBytes = 0;
  let totalBytes = 0;

  onMount(async () => {
    unlistenStart = await listen<number>("download-start", (event) => {
      visible = true;
      totalFiles = event.payload;
      progress = 0;
      statusText = "Starting download...";
      currentFile = "";
    });

    unlistenProgress = await listen<DownloadEvent>(
      "download-progress",
      (event) => {
        const payload = event.payload;
        currentFile = payload.file;

        // Simple file progress for now. Global progress would require tracking all files.
        // For single file (Client jar), this is accurate.
        downloadedBytes = payload.downloaded;
        totalBytes = payload.total;

        statusText = payload.status;

        if (payload.total > 0) {
          progress = (payload.downloaded / payload.total) * 100;
        }
      }
    );

    unlistenComplete = await listen("download-complete", () => {
      statusText = "Done!";
      progress = 100;
      setTimeout(() => {
        visible = false;
      }, 2000);
    });
  });

  onDestroy(() => {
    if (unlistenProgress) unlistenProgress();
    if (unlistenStart) unlistenStart();
    if (unlistenComplete) unlistenComplete();
  });

  function formatBytes(bytes: number) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }
</script>

{#if visible}
  <div
    class="fixed bottom-28 right-8 z-50 w-80 bg-zinc-900/90 backdrop-blur-md border border-zinc-700 rounded-lg shadow-2xl p-4 animate-in slide-in-from-right-10 fade-in duration-300"
  >
    <div class="flex items-center justify-between mb-2">
      <h3 class="text-white font-bold text-sm">Downloads</h3>
      <span class="text-xs text-zinc-400">{statusText}</span>
    </div>

    <div class="text-xs text-zinc-300 truncate mb-1" title={currentFile}>
      {currentFile || "Waiting..."}
    </div>

    <!-- Progress Bar -->
    <div class="w-full bg-zinc-800 rounded-full h-2 mb-2 overflow-hidden">
      <div
        class="bg-gradient-to-r from-green-500 to-emerald-400 h-2 rounded-full transition-all duration-200"
        style="width: {progress}%"
      ></div>
    </div>

    <div class="flex justify-between text-[10px] text-zinc-500 font-mono">
      <span>{formatBytes(downloadedBytes)} / {formatBytes(totalBytes)}</span>
      <span>{Math.round(progress)}%</span>
    </div>
  </div>
{/if}
