<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { gameState } from "../stores/game.svelte";
  import ModLoaderSelector from "./ModLoaderSelector.svelte";

  let searchQuery = $state("");
  let normalizedQuery = $derived(
    searchQuery.trim().toLowerCase().replace(/。/g, ".")
  );

  // Filter by version type
  let typeFilter = $state<"all" | "release" | "snapshot" | "modded">("all");

  // Installed modded versions
  let installedFabricVersions = $state<string[]>([]);
  let isLoadingModded = $state(false);

  // Load installed modded versions
  async function loadInstalledModdedVersions() {
    isLoadingModded = true;
    try {
      installedFabricVersions = await invoke<string[]>(
        "list_installed_fabric_versions"
      );
    } catch (e) {
      console.error("Failed to load installed fabric versions:", e);
    } finally {
      isLoadingModded = false;
    }
  }

  // Load on mount
  $effect(() => {
    loadInstalledModdedVersions();
  });

  // Combined versions list (vanilla + modded)
  let allVersions = $derived(() => {
    const moddedVersions = installedFabricVersions.map((id) => ({
      id,
      type: "fabric",
      url: "",
      time: "",
      releaseTime: new Date().toISOString(),
    }));
    return [...moddedVersions, ...gameState.versions];
  });

  let filteredVersions = $derived(() => {
    let versions = allVersions();

    // Apply type filter
    if (typeFilter === "release") {
      versions = versions.filter((v) => v.type === "release");
    } else if (typeFilter === "snapshot") {
      versions = versions.filter((v) => v.type === "snapshot");
    } else if (typeFilter === "modded") {
      versions = versions.filter(
        (v) => v.type === "fabric" || v.type === "forge"
      );
    }

    // Apply search filter
    if (normalizedQuery.length > 0) {
      versions = versions.filter((v) =>
        v.id.toLowerCase().includes(normalizedQuery)
      );
    }

    return versions;
  });

  function getVersionBadge(type: string) {
    switch (type) {
      case "release":
        return { text: "Release", class: "bg-green-600" };
      case "snapshot":
        return { text: "Snapshot", class: "bg-yellow-600" };
      case "fabric":
        return { text: "Fabric", class: "bg-blue-600" };
      case "forge":
        return { text: "Forge", class: "bg-orange-600" };
      default:
        return { text: type, class: "bg-zinc-600" };
    }
  }

  function handleModLoaderInstall(versionId: string) {
    // Refresh the installed versions list
    loadInstalledModdedVersions();
    // Select the newly installed version
    gameState.selectedVersion = versionId;
  }

  // Get the base Minecraft version from selected version (for mod loader selector)
  let selectedBaseVersion = $derived(() => {
    const selected = gameState.selectedVersion;
    if (!selected) return "";

    // If it's a modded version, extract the base version
    if (selected.startsWith("fabric-loader-")) {
      // Format: fabric-loader-X.X.X-1.20.4
      const parts = selected.split("-");
      return parts[parts.length - 1];
    }
    if (selected.includes("-forge-")) {
      // Format: 1.20.4-forge-49.0.38
      return selected.split("-forge-")[0];
    }

    // Check if it's a valid vanilla version
    const version = gameState.versions.find((v) => v.id === selected);
    return version ? selected : "";
  });
</script>

<div class="p-8 h-full overflow-y-auto bg-zinc-900">
  <h2 class="text-3xl font-bold mb-6">Versions</h2>

  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Left: Version List -->
    <div class="lg:col-span-2 space-y-4">
      <!-- Search and Filters -->
      <div class="flex gap-3">
        <input
          type="text"
          placeholder="Search versions..."
          class="flex-1 p-3 bg-zinc-800 border border-zinc-700 rounded text-white focus:outline-none focus:border-green-500 transition-colors"
          bind:value={searchQuery}
        />
      </div>

      <!-- Type Filter Tabs -->
      <div class="flex gap-1 bg-zinc-800 rounded-lg p-1">
        <button
          class="flex-1 px-3 py-2 rounded-md text-sm font-medium transition-colors {typeFilter ===
          'all'
            ? 'bg-zinc-700 text-white'
            : 'text-zinc-400 hover:text-white'}"
          onclick={() => (typeFilter = "all")}
        >
          All
        </button>
        <button
          class="flex-1 px-3 py-2 rounded-md text-sm font-medium transition-colors {typeFilter ===
          'release'
            ? 'bg-green-600 text-white'
            : 'text-zinc-400 hover:text-white'}"
          onclick={() => (typeFilter = "release")}
        >
          Releases
        </button>
        <button
          class="flex-1 px-3 py-2 rounded-md text-sm font-medium transition-colors {typeFilter ===
          'snapshot'
            ? 'bg-yellow-600 text-white'
            : 'text-zinc-400 hover:text-white'}"
          onclick={() => (typeFilter = "snapshot")}
        >
          Snapshots
        </button>
        <button
          class="flex-1 px-3 py-2 rounded-md text-sm font-medium transition-colors {typeFilter ===
          'modded'
            ? 'bg-purple-600 text-white'
            : 'text-zinc-400 hover:text-white'}"
          onclick={() => (typeFilter = "modded")}
        >
          Modded
        </button>
      </div>

      <!-- Version List -->
      <div class="grid gap-2 max-h-[calc(100vh-320px)] overflow-y-auto pr-2">
        {#if gameState.versions.length === 0}
          <div class="text-zinc-500">Loading versions...</div>
        {:else if filteredVersions().length === 0}
          <div class="text-zinc-500">
            {#if normalizedQuery.length > 0}
              No versions found matching "{searchQuery}"
            {:else}
              No versions in this category
            {/if}
          </div>
        {:else}
          {#each filteredVersions() as version}
            {@const badge = getVersionBadge(version.type)}
            <button
              class="flex items-center justify-between p-4 bg-zinc-800 rounded hover:bg-zinc-700 transition text-left border border-zinc-700 {gameState.selectedVersion ===
              version.id
                ? 'border-green-500 bg-zinc-800/80 ring-1 ring-green-500'
                : ''}"
              onclick={() => (gameState.selectedVersion = version.id)}
            >
              <div class="flex items-center gap-3">
                <span
                  class="px-2 py-0.5 rounded text-xs font-medium {badge.class}"
                >
                  {badge.text}
                </span>
                <div>
                  <div class="font-bold font-mono">{version.id}</div>
                  {#if version.releaseTime && version.type !== "fabric" && version.type !== "forge"}
                    <div class="text-xs text-zinc-400">
                      {new Date(version.releaseTime).toLocaleDateString()}
                    </div>
                  {/if}
                </div>
              </div>
              {#if gameState.selectedVersion === version.id}
                <div class="text-green-500 font-bold text-sm">SELECTED</div>
              {/if}
            </button>
          {/each}
        {/if}
      </div>
    </div>

    <!-- Right: Mod Loader Panel -->
    <div class="space-y-4">
      <!-- Selected Version Info -->
      {#if gameState.selectedVersion}
        <div class="bg-zinc-800 rounded-lg p-4 border border-zinc-700">
          <h3 class="text-sm font-semibold text-zinc-400 mb-2">Selected</h3>
          <p class="font-mono text-lg text-green-400">
            {gameState.selectedVersion}
          </p>
        </div>
      {/if}

      <!-- Mod Loader Selector -->
      <ModLoaderSelector
        selectedGameVersion={selectedBaseVersion()}
        onInstall={handleModLoaderInstall}
      />

      <!-- Help Text -->
      <div class="bg-zinc-800/50 rounded-lg p-4 border border-zinc-700/50">
        <h4 class="text-sm font-semibold text-zinc-400 mb-2">💡 Tip</h4>
        <p class="text-xs text-zinc-500">
          Select a vanilla Minecraft version, then use the Mod Loader panel to
          install Fabric or Forge. Installed modded versions will appear in the
          list with colored badges.
        </p>
      </div>
    </div>
  </div>
</div>

