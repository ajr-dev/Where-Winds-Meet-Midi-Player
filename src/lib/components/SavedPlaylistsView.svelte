<script>
  import Icon from "@iconify/svelte";
  import { fade, fly } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { dndzone } from "svelte-dnd-action";
  import {
    savedPlaylists,
    createPlaylist,
    deletePlaylist,
    renamePlaylist,
    loadPlaylistToQueue,
    reorderPlaylists,
    removeFromSavedPlaylist,
    reorderSavedPlaylist,
    playMidi,
    playlist,
    activePlaylistId,
  } from "../stores/player.js";

  let showCreateModal = false;
  let newPlaylistName = "";
  let editingPlaylistId = null;
  let editingName = "";
  let selectedPlaylistId = null;
  const flipDurationMs = 200;

  // Get selected playlist reactively from store
  $: selectedPlaylist = selectedPlaylistId
    ? $savedPlaylists.find((p) => p.id === selectedPlaylistId)
    : null;

  // Track items for drag and drop in playlist detail view
  $: trackItems = selectedPlaylist
    ? selectedPlaylist.tracks.map((track, index) => ({
        ...track,
        id: track.path, // Use path as unique ID
        originalIndex: index,
      }))
    : [];

  // Items for drag and drop
  $: items = $savedPlaylists.map((pl, index) => ({
    ...pl,
    originalIndex: index,
  }));

  function handleCreate() {
    if (newPlaylistName.trim()) {
      createPlaylist(newPlaylistName.trim());
      newPlaylistName = "";
      showCreateModal = false;
    }
  }

  function startEditing(playlist) {
    editingPlaylistId = playlist.id;
    editingName = playlist.name;
  }

  function saveEdit() {
    if (editingName.trim() && editingPlaylistId) {
      renamePlaylist(editingPlaylistId, editingName.trim());
    }
    editingPlaylistId = null;
    editingName = "";
  }

  function handleDelete(id) {
    if (confirm("Delete this playlist?")) {
      deletePlaylist(id);
      if (selectedPlaylistId === id) {
        selectedPlaylistId = null;
      }
    }
  }

  function handleDndConsider(e) {
    items = e.detail.items;
  }

  function handleDndFinalize(e) {
    const newItems = e.detail.items;
    // Update the store with new order
    savedPlaylists.set(
      newItems.map(({ originalIndex, ...pl }) => pl)
    );
    items = newItems.map((item, index) => ({
      ...item,
      originalIndex: index,
    }));
  }

  async function handleLoadToQueue(playlist) {
    await loadPlaylistToQueue(playlist.id);
  }

  function goBack() {
    selectedPlaylistId = null;
  }

  // Track management functions
  function handleTrackDndConsider(e) {
    trackItems = e.detail.items;
  }

  function handleTrackDndFinalize(e) {
    if (!selectedPlaylistId) return;

    const newItems = e.detail.items;
    // Update the store with new track order and persist
    savedPlaylists.update((lists) => {
      const newLists = lists.map((p) => {
        if (p.id === selectedPlaylistId) {
          return {
            ...p,
            tracks: newItems.map(({ id, originalIndex, ...track }) => track),
          };
        }
        return p;
      });
      // Persist to localStorage
      try {
        localStorage.setItem("wwm-playlists", JSON.stringify(newLists));
      } catch (error) {
        console.error("Failed to save playlists:", error);
      }
      return newLists;
    });
  }

  function handleRemoveTrack(trackPath) {
    if (selectedPlaylistId) {
      removeFromSavedPlaylist(selectedPlaylistId, trackPath);
    }
  }

  async function handlePlayTrack(track) {
    // Add track to queue and play
    playlist.update((list) => {
      if (!list.find((f) => f.path === track.path)) {
        return [...list, track];
      }
      return list;
    });
    await playMidi(track.path);
  }
</script>

<div class="h-full flex flex-col">
  {#if selectedPlaylist}
    <!-- Playlist Detail View -->
    <div class="mb-4" in:fly={{ x: 20, duration: 200 }}>
      <button
        class="flex items-center gap-2 text-white/60 hover:text-white mb-4 transition-colors"
        onclick={goBack}
      >
        <Icon icon="mdi:arrow-left" class="w-5 h-5" />
        <span class="text-sm">Back to Playlists</span>
      </button>

      <div class="flex items-center gap-4 mb-4">
        <div
          class="w-16 h-16 rounded-lg bg-white/10 flex items-center justify-center"
        >
          <Icon icon="mdi:playlist-music" class="w-8 h-8 text-[#1db954]" />
        </div>
        <div class="flex-1 min-w-0">
          {#if editingPlaylistId === selectedPlaylist.id}
            <input
              type="text"
              bind:value={editingName}
              class="bg-white/10 border border-white/20 rounded px-2 py-1 text-lg font-bold w-full"
              onblur={saveEdit}
              onkeydown={(e) => e.key === "Enter" && saveEdit()}
              autofocus
            />
          {:else}
            <h2
              class="text-2xl font-bold truncate cursor-pointer hover:text-[#1db954] transition-colors"
              onclick={() => startEditing(selectedPlaylist)}
              title="Click to rename"
            >
              {selectedPlaylist.name}
            </h2>
          {/if}
          <p class="text-sm text-white/60">
            {selectedPlaylist.tracks.length} songs
          </p>
        </div>
      </div>

      <div class="flex gap-2">
        <button
          class="spotify-button spotify-button--primary flex items-center gap-2"
          onclick={() => handleLoadToQueue(selectedPlaylist)}
          disabled={selectedPlaylist.tracks.length === 0}
        >
          <Icon icon="mdi:play" class="w-5 h-5" />
          Play
        </button>
        <button
          class="spotify-button spotify-button--secondary flex items-center gap-2"
          onclick={() => handleDelete(selectedPlaylist.id)}
        >
          <Icon icon="mdi:delete-outline" class="w-4 h-4" />
        </button>
      </div>
    </div>

    <!-- Playlist Tracks with Drag and Drop -->
    <div
      class="flex-1 overflow-y-auto pr-2"
      use:dndzone={{
        items: trackItems,
        flipDurationMs,
        dropTargetStyle: { outline: "none" },
      }}
      onconsider={handleTrackDndConsider}
      onfinalize={handleTrackDndFinalize}
    >
      {#each trackItems as track, index (track.id)}
        <div
          class="group spotify-list-item flex items-center gap-3 py-2 mb-1 cursor-grab active:cursor-grabbing hover:bg-white/5"
          animate:flip={{ duration: flipDurationMs }}
        >
          <!-- Drag Handle -->
          <div class="text-white/30 hover:text-white/60 transition-colors flex-shrink-0">
            <Icon icon="mdi:drag-vertical" class="w-4 h-4" />
          </div>

          <!-- Track Number / Play Button -->
          <div class="w-6 flex items-center justify-center flex-shrink-0">
            <span class="text-sm text-white/40 group-hover:hidden">{index + 1}</span>
            <button
              class="hidden group-hover:flex items-center justify-center w-6 h-6 rounded-full bg-[#1db954] hover:scale-110 transition-transform"
              onclick={() => handlePlayTrack(track)}
              title="Play"
            >
              <Icon icon="mdi:play" class="w-4 h-4 text-black" />
            </button>
          </div>

          <!-- Track Info -->
          <div
            class="flex-1 min-w-0 cursor-pointer"
            onclick={() => handlePlayTrack(track)}
          >
            <p class="text-sm font-medium text-white truncate group-hover:text-[#1db954] transition-colors">
              {track.name}
            </p>
            <p class="text-xs text-white/40">MIDI Track</p>
          </div>

          <!-- Duration -->
          <div class="text-sm text-white/40 flex-shrink-0 tabular-nums">
            {track.duration
              ? `${Math.floor(track.duration / 60)}:${String(Math.floor(track.duration % 60)).padStart(2, "0")}`
              : "--:--"}
          </div>

          <!-- Remove Button -->
          <button
            class="p-1.5 rounded-full text-white/30 opacity-0 group-hover:opacity-100 hover:text-red-400 hover:bg-red-500/20 transition-all flex-shrink-0"
            onclick={(e) => {
              e.stopPropagation();
              handleRemoveTrack(track.path);
            }}
            title="Remove from playlist"
          >
            <Icon icon="mdi:close" class="w-4 h-4" />
          </button>
        </div>
      {/each}
    </div>

    {#if selectedPlaylist.tracks.length === 0}
      <div class="flex-1 flex flex-col items-center justify-center text-white/40 py-12">
        <Icon
          icon="mdi:music-note-plus"
          class="w-12 h-12 mb-4 opacity-50"
        />
        <p class="text-sm">This playlist is empty</p>
        <p class="text-xs mt-1">Add songs from your library</p>
      </div>
    {/if}

    {#if selectedPlaylist.tracks.length > 1}
      <div
        class="pt-4 mt-4 border-t border-white/10 flex items-center justify-center gap-2 text-white/30"
      >
        <Icon icon="mdi:gesture-swipe-vertical" class="w-4 h-4" />
        <p class="text-xs">Drag to reorder</p>
      </div>
    {/if}
  {:else}
    <!-- Playlists List View -->
    <div class="mb-6">
      <div class="flex items-center justify-between mb-4">
        <div>
          <h2 class="text-2xl font-bold">Playlists</h2>
          <p class="text-sm text-white/60">
            {$savedPlaylists.length} playlists
          </p>
        </div>
        <button
          class="spotify-button spotify-button--secondary flex items-center gap-2"
          onclick={() => (showCreateModal = true)}
        >
          <Icon icon="mdi:plus" class="w-4 h-4" />
          New
        </button>
      </div>
    </div>

    <!-- Playlists Grid with Drag and Drop -->
    {#if $savedPlaylists.length > 0}
      <div
        class="flex-1 overflow-y-auto"
        use:dndzone={{
          items,
          flipDurationMs,
          dropTargetStyle: { outline: "none" },
        }}
        onconsider={handleDndConsider}
        onfinalize={handleDndFinalize}
      >
        {#each items as playlist (playlist.id)}
          <div
            class="group spotify-card mb-2 cursor-grab active:cursor-grabbing"
            animate:flip={{ duration: flipDurationMs }}
          >
            <div class="flex items-center gap-4">
              <!-- Drag Handle -->
              <div class="text-white/30 hover:text-white/60 transition-colors">
                <Icon icon="mdi:drag-vertical" class="w-5 h-5" />
              </div>

              <!-- Playlist Icon -->
              <div
                class="w-12 h-12 rounded bg-white/10 flex items-center justify-center flex-shrink-0"
              >
                <Icon icon="mdi:playlist-music" class="w-6 h-6 text-[#1db954]" />
              </div>

              <!-- Playlist Info -->
              <div
                class="flex-1 min-w-0 cursor-pointer"
                onclick={() => (selectedPlaylistId = playlist.id)}
              >
                {#if editingPlaylistId === playlist.id}
                  <input
                    type="text"
                    bind:value={editingName}
                    class="bg-white/10 border border-white/20 rounded px-2 py-1 text-sm font-semibold w-full"
                    onblur={saveEdit}
                    onkeydown={(e) => e.key === "Enter" && saveEdit()}
                    onclick={(e) => e.stopPropagation()}
                    autofocus
                  />
                {:else}
                  <p class="font-semibold text-white truncate hover:text-[#1db954] transition-colors">
                    {playlist.name}
                  </p>
                {/if}
                <p class="text-xs text-white/50">
                  {playlist.tracks.length} songs
                </p>
              </div>

              <!-- Actions -->
              <div
                class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
              >
                <button
                  class="p-2 rounded-full hover:bg-white/10 text-white/60 hover:text-white transition-all"
                  onclick={(e) => {
                    e.stopPropagation();
                    handleLoadToQueue(playlist);
                  }}
                  title="Play playlist"
                >
                  <Icon icon="mdi:play" class="w-5 h-5" />
                </button>
                <button
                  class="p-2 rounded-full hover:bg-white/10 text-white/60 hover:text-white transition-all"
                  onclick={(e) => {
                    e.stopPropagation();
                    startEditing(playlist);
                  }}
                  title="Rename"
                >
                  <Icon icon="mdi:pencil" class="w-4 h-4" />
                </button>
                <button
                  class="p-2 rounded-full hover:bg-red-500/20 text-white/60 hover:text-red-400 transition-all"
                  onclick={(e) => {
                    e.stopPropagation();
                    handleDelete(playlist.id);
                  }}
                  title="Delete"
                >
                  <Icon icon="mdi:delete-outline" class="w-4 h-4" />
                </button>
              </div>
            </div>
          </div>
        {/each}
      </div>

      <div
        class="pt-4 mt-4 border-t border-white/10 flex items-center justify-center gap-2 text-white/30"
      >
        <Icon icon="mdi:gesture-swipe-vertical" class="w-4 h-4" />
        <p class="text-xs">Drag to reorder</p>
      </div>
    {:else}
      <div
        class="flex-1 flex flex-col items-center justify-center text-white/40 py-16"
        transition:fade
      >
        <div
          class="w-20 h-20 rounded-full bg-white/5 flex items-center justify-center mb-6"
        >
          <Icon icon="mdi:playlist-plus" class="w-10 h-10 opacity-50" />
        </div>
        <p class="text-lg font-semibold mb-2 text-white/60">No playlists yet</p>
        <p class="text-sm mb-4">Create a playlist to organize your music</p>
        <button
          class="spotify-button spotify-button--primary flex items-center gap-2"
          onclick={() => (showCreateModal = true)}
        >
          <Icon icon="mdi:plus" class="w-4 h-4" />
          Create Playlist
        </button>
      </div>
    {/if}
  {/if}
</div>

<!-- Create Playlist Modal -->
{#if showCreateModal}
  <div
    class="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
    transition:fade={{ duration: 150 }}
    onclick={() => (showCreateModal = false)}
  >
    <div
      class="bg-[#282828] rounded-xl p-6 w-80 shadow-2xl border border-white/10"
      transition:fly={{ y: 20, duration: 200 }}
      onclick={(e) => e.stopPropagation()}
    >
      <h3 class="text-xl font-bold mb-4">Create Playlist</h3>
      <input
        type="text"
        placeholder="Playlist name"
        bind:value={newPlaylistName}
        class="w-full bg-white/10 border border-white/20 rounded-lg px-4 py-3 text-white placeholder-white/40 focus:outline-none focus:ring-2 focus:ring-[#1db954] mb-4"
        onkeydown={(e) => e.key === "Enter" && handleCreate()}
        autofocus
      />
      <div class="flex gap-2 justify-end">
        <button
          class="spotify-button spotify-button--secondary"
          onclick={() => (showCreateModal = false)}
        >
          Cancel
        </button>
        <button
          class="spotify-button spotify-button--primary"
          onclick={handleCreate}
          disabled={!newPlaylistName.trim()}
        >
          Create
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  :global(.spotify-card) {
    outline: none !important;
  }
</style>
