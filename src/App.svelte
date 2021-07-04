<script>
  import { invoke } from '@tauri-apps/api'
</script>

<button on:click={() => invoke('hide_window')}>Hide window</button>
