<script>
  import { onDestroy } from 'svelte';
  
  export let compact = false;

  import {
    currentPosition,
    totalDuration,
    progress,
    formatTime,
    seekToPosition,
    endSeeking
  } from '../stores/player.js';

  let isDragging = false;
  let timelineElement;
  let dragPosition = null;

  function getProgressFromEvent(event) {
    if (!timelineElement) return 0;
    const rect = timelineElement.getBoundingClientRect();
    const x = (event.touches ? event.touches[0].clientX : event.clientX) - rect.left;
    const clampedX = Math.max(0, Math.min(rect.width, x));
    return (clampedX / rect.width) * 100;
  }

  function handleMove(event) {
    if (!isDragging) return;
    const newProgress = getProgressFromEvent(event);
    const newPosition = ($totalDuration * newProgress) / 100;
    dragPosition = newPosition;
    seekToPosition(newPosition);
    event.preventDefault();
  }

  async function handleEnd(event) {
    if (!isDragging) return;
    const finalPosition = dragPosition;
    await endSeeking();
    isDragging = false;
    dragPosition = null;
    if (finalPosition !== null) {
      currentPosition.set(finalPosition);
    }
    window.removeEventListener('mousemove', handleMove);
    window.removeEventListener('mouseup', handleEnd);
    window.removeEventListener('touchmove', handleMove);
    window.removeEventListener('touchend', handleEnd);
    event.preventDefault();
  }

  function handleStart(event) {
    isDragging = true;
    const newProgress = getProgressFromEvent(event);
    const newPosition = ($totalDuration * newProgress) / 100;
    dragPosition = newPosition;
    seekToPosition(newPosition);
    window.addEventListener('mousemove', handleMove);
    window.addEventListener('mouseup', handleEnd);
    window.addEventListener('touchmove', handleMove);
    window.addEventListener('touchend', handleEnd);
    event.preventDefault();
  }

  onDestroy(() => {
    window.removeEventListener('mousemove', handleMove);
    window.removeEventListener('mouseup', handleEnd);
    window.removeEventListener('touchmove', handleMove);
    window.removeEventListener('touchend', handleEnd);
  });

  $: displayPosition = isDragging && dragPosition !== null ? dragPosition : $currentPosition;
  $: displayProgress = isDragging && dragPosition !== null 
    ? ($totalDuration > 0 ? (dragPosition / $totalDuration) * 100 : 0)
    : $progress;
</script>

<div class="mt-2 {compact ? 'py-1' : ''}">
  <div class="flex items-center gap-2">
    <span class="text-[11px] text-white/70 min-w-[40px]">
      {formatTime(displayPosition)}
    </span>

    <div
      bind:this={timelineElement}
      class="group flex-1 h-1 bg-white/20 rounded-full relative cursor-pointer"
      role="slider"
      tabindex="0"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-valuenow={displayProgress}
      aria-label="Playback progress"
      on:mousedown={handleStart}
      on:touchstart={handleStart}
    >
      <div
        class="h-full bg-white rounded-full relative group-hover:bg-[#1db954] {isDragging ? 'bg-[#1db954]' : ''}"
        style="width: {displayProgress}%; transition: none;"
      >
        <div
          class="absolute right-0 top-1/2 -translate-y-1/2 translate-x-1/2 w-3 h-3 bg-white rounded-full {isDragging ? 'opacity-100' : 'opacity-0 group-hover:opacity-100'} shadow-lg"
        ></div>
      </div>
    </div>

    <span class="text-[11px] text-white/70 min-w-[40px] text-right">
      {formatTime($totalDuration)}
    </span>
  </div>
</div>
